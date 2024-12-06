import { createForm } from "@tanstack/solid-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo, createSignal } from "solid-js";
import showErrorToast from "@/components/error-toast";
import { FacilityUpdatePayload } from "@/types/facility";
import { updateFacility } from "@/api/facility";
import showSuccessToast from "@/components/success-toast";
import { Button } from "@/components/ui/button";
import { useUser } from "@/context/user-context";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { DialogTriggerProps } from "@kobalte/core/dialog";
import { Pencil } from "lucide-solid";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";

function ChangePhoneNumberDialog() {
  const { refreshUser } = useUser();

  const [duplicatePhoneNumber, setDuplicatePhoneNumber] = createSignal(false);

  const form = createForm(() => ({
    defaultValues: {
      phoneNumber: "",
    },
    onSubmit: async ({ value }) => {
      const facilityUpdatePayload: FacilityUpdatePayload = {
        phoneNumber: "+60" + value.phoneNumber,
      };
      const response = await updateFacility(facilityUpdatePayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Phone number update successful." });
          refreshUser();
        },
        (error) => {
          if (
            error.message === "DUPLICATE_RECORD" &&
            error.detail === "phone number"
          ) {
            setDuplicatePhoneNumber(true);
          } else {
            showErrorToast({
              errorTitle: "Error performing update. Please try again.",
              error: error,
            });
          }
        }
      );
    },
    validatorAdapter: zodValidator(),
  }));

  const phoneNumberSchema = z
    .string()
    .min(1, "Phone number is required")
    .regex(/^\d{1,2}-\d{7,8}$/, "Invalid phone number format");

  return (
    <Dialog>
      <DialogTrigger
        as={(props: DialogTriggerProps) => (
          <Button variant="ghost" size={"icon"} {...props}>
            <Pencil size={18} />
          </Button>
        )}
      />
      <DialogContent class="md:max-w-xl">
        <DialogHeader>
          <DialogTitle>Change phone number</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <div class="space-y-2 pb-4">
            <form.Field
              name="phoneNumber"
              validators={{ onChange: phoneNumberSchema }}
              children={(field) => {
                const hasError = createMemo(() => {
                  return (
                    (field().state.meta.errors.length > 0 &&
                      field().state.meta.isTouched) ||
                    duplicatePhoneNumber()
                  );
                });

                return (
                  <TextFieldRoot
                    class="space-y-1"
                    name={field().name}
                    validationState={hasError() ? "invalid" : "valid"}
                    value={field().state.value}
                    onBlur={field().handleBlur}
                    onChange={(e) => {
                      field().handleChange(e);
                      setDuplicatePhoneNumber(false);
                    }}
                  >
                    <TextFieldLabel>Phone Number</TextFieldLabel>
                    <div class="flex rounded-lg ">
                      <div class="flex h-9 rounded-md border bg-transparent px-3 py-1 text-sm shadow-sm text-foreground rounded-e-none justify-center items-center">
                        +60
                      </div>
                      <TextField
                        class="rounded-s-none"
                        type="tel"
                        placeholder="12-3456789"
                      />
                    </div>
                    <TextFieldErrorMessage>
                      {duplicatePhoneNumber()
                        ? "Phone number already registered"
                        : field().state.meta.errors.join(", ").split(", ")[0]}
                    </TextFieldErrorMessage>
                  </TextFieldRoot>
                );
              }}
            />
          </div>
          <DialogFooter>
            <Button type="submit">Update</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

export default ChangePhoneNumberDialog;
