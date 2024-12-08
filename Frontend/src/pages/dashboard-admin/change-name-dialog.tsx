import { createForm } from "@tanstack/solid-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo } from "solid-js";
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

function ChangeNameDialog() {
  const { refreshUser } = useUser();

  const form = createForm(() => ({
    defaultValues: {
      name: "",
    },
    onSubmit: async ({ value }) => {
      const adminUpdatePayload: AdminUpdatePayload = {
        name: value.name,
      };
      const response = await updateAdmin(adminUpdatePayload);
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

  const nameSchema = z
    .string()
    .min(1, "Name is required")
    .max(64, "Name must be at most 64 characters");

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
          <DialogTitle>Change name</DialogTitle>
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
              name="name"
              validators={{ onChange: nameSchema }}
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
                    <TextFieldLabel>Name</TextFieldLabel>
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

export default ChangeNameDialog;
