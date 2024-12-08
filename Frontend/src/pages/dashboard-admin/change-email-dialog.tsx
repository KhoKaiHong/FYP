import { createForm } from "@tanstack/solid-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo, createSignal } from "solid-js";
import showErrorToast from "@/components/error-toast";
import { AdminUpdatePayload } from "@/types/admin";
import { updateAdmin } from "@/api/admin";
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

function ChangeEmailDialog() {
  const { refreshUser } = useUser();

  const [duplicateEmail, setDuplicateEmail] = createSignal(false);

  const form = createForm(() => ({
    defaultValues: {
      email: "",
    },
    onSubmit: async ({ value }) => {
      const adminUpdatePayload: AdminUpdatePayload = {
        email: value.email,
      };
      const response = await updateAdmin(adminUpdatePayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Email update successful." });
          refreshUser();
        },
        (error) => {
          if (
            error.message === "DUPLICATE_RECORD" &&
            error.detail === "email"
          ) {
            setDuplicateEmail(true);
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

  const emailSchema = z
    .string()
    .min(1, "Email is required")
    .email("Please enter a valid email address")
    .max(64, "Email must be at most 64 characters");

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
          <DialogTitle>Change email</DialogTitle>
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
              name="email"
              validators={{ onChange: emailSchema }}
              children={(field) => {
                const hasError = createMemo(() => {
                  return (
                    (field().state.meta.errors.length > 0 &&
                      field().state.meta.isTouched) ||
                    duplicateEmail()
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
                      setDuplicateEmail(false);
                    }}
                  >
                    <TextFieldLabel>Email</TextFieldLabel>
                    <TextField type="email" />
                    <TextFieldErrorMessage>
                      {duplicateEmail()
                        ? "Email already registered"
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

export default ChangeEmailDialog;
