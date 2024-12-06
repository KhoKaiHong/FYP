import { createForm } from "@tanstack/solid-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo } from "solid-js";
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

function ChangeAddressDialog() {
  const { refreshUser } = useUser();

  const form = createForm(() => ({
    defaultValues: {
      address: "",
    },
    onSubmit: async ({ value }) => {
      const facilityUpdatePayload: FacilityUpdatePayload = {
        address: value.address,
      };
      const response = await updateFacility(facilityUpdatePayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Name update successful." });
          refreshUser();
        },
        (error) => {
          showErrorToast({
            errorTitle: "Error performing update. Please try again.",
            error: error,
          });
        }
      );
    },
    validatorAdapter: zodValidator(),
  }));

  const addressSchema = z
    .string()
    .min(1, "Address is required")
    .max(128, "Address must be at most 128 characters");

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
          <DialogTitle>Change address</DialogTitle>
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
              name="address"
              validators={{ onChange: addressSchema }}
              children={(field) => {
                const hasError = createMemo(() => {
                  return (
                    field().state.meta.errors.length > 0 &&
                    field().state.meta.isTouched
                  );
                });

                return (
                  <TextFieldRoot
                    class="space-y-1"
                    name={field().name}
                    validationState={hasError() ? "invalid" : "valid"}
                    value={field().state.value}
                    onBlur={field().handleBlur}
                    onChange={field().handleChange}
                  >
                    <TextFieldLabel>Address</TextFieldLabel>
                    <TextField />
                    <TextFieldErrorMessage>
                      {field().state.meta.errors.join(", ").split(", ")[0]}
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

export default ChangeAddressDialog;
