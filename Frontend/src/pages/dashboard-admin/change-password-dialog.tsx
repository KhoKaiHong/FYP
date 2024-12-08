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
import { Eye, EyeOff, Pencil } from "lucide-solid";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";

function ChangePasswordDialog() {
  const { refreshUser } = useUser();

  const [currentPasswordNotMatching, setCurrentPasswordNotMatching] =
    createSignal(false);

  const form = createForm(() => ({
    defaultValues: {
      currentPassword: "",
      password: "",
      passwordConfirm: "",
    },
    onSubmit: async ({ value }) => {
      const adminUpdatePayload: AdminUpdatePayload = {
        currentPassword: value.currentPassword,
        password: value.password,
      };
      const response = await updateAdmin(adminUpdatePayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Password update successful." });
          refreshUser();
        },
        (error) => {
          if (error.message === "CURRENT_PASSWORD_NOT_MATCHING") {
            setCurrentPasswordNotMatching(true);
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

  const currentPasswordSchema = z
    .string()
    .min(1, "This field is required")
    .max(32, "Maximum 32 characters allowed");

  const passwordSchema = z
    .string()
    .min(8, "Password must be at least 8 characters")
    .max(32, "Password must be at most 32 characters")
    .regex(
      /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[#?!@$%^&*-]).{8,32}$/,
      "Password must include at least one uppercase letter and one lowercase letter as well as one special character"
    );

  const passwordConfirmSchema = z
    .string()
    .min(8, "Password must be at least 8 characters")
    .max(32, "Password must be at most 32 characters")
    .refine((value) => value === form.getFieldValue("password"), {
      message: "Passwords do not match",
      path: ["passwordConfirm"],
    });

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
          <DialogTitle>Change password</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <div class="space-y-2 pb-4">
            <div class="space-y-2 md:space-y-0">
              <form.Field
                name="currentPassword"
                validators={{ onChange: currentPasswordSchema }}
                children={(field) => {
                  const hasError = createMemo(() => {
                    return (
                      (field().state.meta.errors.length > 0 &&
                        field().state.meta.isTouched) ||
                      currentPasswordNotMatching()
                    );
                  });

                  const [isVisible, setIsVisible] = createSignal(false);
                  function toggleVisibility() {
                    setIsVisible(!isVisible());
                  }

                  return (
                    <TextFieldRoot
                      class="space-y-1"
                      name={field().name}
                      validationState={hasError() ? "invalid" : "valid"}
                      value={field().state.value}
                      onBlur={field().handleBlur}
                      onChange={(e) => {
                        field().handleChange(e);
                        setCurrentPasswordNotMatching(false);
                      }}
                    >
                      <TextFieldLabel>Current Password</TextFieldLabel>
                      <div class="relative">
                        <TextField type={isVisible() ? "text" : "password"} />
                        <button
                          class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                          type="button"
                          onClick={toggleVisibility}
                          aria-label={
                            isVisible() ? "Hide password" : "Show password"
                          }
                          aria-pressed={isVisible()}
                          aria-controls="password"
                        >
                          {isVisible() ? (
                            <EyeOff
                              size={16}
                              strokeWidth={2}
                              aria-hidden="true"
                            />
                          ) : (
                            <Eye size={16} strokeWidth={2} aria-hidden="true" />
                          )}
                        </button>
                      </div>
                      <TextFieldErrorMessage>
                        {currentPasswordNotMatching()
                          ? "Current password does not match"
                          : field().state.meta.errors.join(", ").split(", ")[0]}
                      </TextFieldErrorMessage>
                    </TextFieldRoot>
                  );
                }}
              />
            </div>
            <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
              <form.Field
                name="password"
                validators={{ onChange: passwordSchema }}
                children={(field) => {
                  const hasError = createMemo(() => {
                    return (
                      field().state.meta.errors.length > 0 &&
                      field().state.meta.isTouched
                    );
                  });

                  const [isVisible, setIsVisible] = createSignal(false);
                  function toggleVisibility() {
                    setIsVisible(!isVisible());
                  }

                  return (
                    <TextFieldRoot
                      class="space-y-1"
                      name={field().name}
                      validationState={hasError() ? "invalid" : "valid"}
                      value={field().state.value}
                      onBlur={field().handleBlur}
                      onChange={field().handleChange}
                    >
                      <TextFieldLabel>New Password</TextFieldLabel>
                      <div class="relative">
                        <TextField type={isVisible() ? "text" : "password"} />
                        <button
                          class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                          type="button"
                          onClick={toggleVisibility}
                          aria-label={
                            isVisible() ? "Hide password" : "Show password"
                          }
                          aria-pressed={isVisible()}
                          aria-controls="password"
                        >
                          {isVisible() ? (
                            <EyeOff
                              size={16}
                              strokeWidth={2}
                              aria-hidden="true"
                            />
                          ) : (
                            <Eye size={16} strokeWidth={2} aria-hidden="true" />
                          )}
                        </button>
                      </div>
                      <TextFieldErrorMessage>
                        {field().state.meta.errors.join(", ").split(", ")[0]}
                      </TextFieldErrorMessage>
                    </TextFieldRoot>
                  );
                }}
              />
              <form.Field
                name="passwordConfirm"
                validators={{
                  onChangeListenTo: ["password"],
                  onChange: passwordConfirmSchema,
                }}
                children={(field) => {
                  const hasError = createMemo(() => {
                    return (
                      field().state.meta.errors.length > 0 &&
                      field().state.meta.isTouched
                    );
                  });

                  const [isVisible, setIsVisible] = createSignal(false);
                  function toggleVisibility() {
                    setIsVisible(!isVisible());
                  }

                  return (
                    <TextFieldRoot
                      class="space-y-1"
                      name={field().name}
                      validationState={hasError() ? "invalid" : "valid"}
                      value={field().state.value}
                      onBlur={field().handleBlur}
                      onChange={field().handleChange}
                    >
                      <TextFieldLabel>Confirm New Password</TextFieldLabel>
                      <div class="relative">
                        <TextField type={isVisible() ? "text" : "password"} />
                        <button
                          class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                          type="button"
                          onClick={toggleVisibility}
                          aria-label={
                            isVisible() ? "Hide password" : "Show password"
                          }
                          aria-pressed={isVisible()}
                          aria-controls="password"
                        >
                          {isVisible() ? (
                            <EyeOff
                              size={16}
                              strokeWidth={2}
                              aria-hidden="true"
                            />
                          ) : (
                            <Eye size={16} strokeWidth={2} aria-hidden="true" />
                          )}
                        </button>
                      </div>
                      <TextFieldErrorMessage>
                        {field().state.meta.errors.join(", ").split(", ")[0]}
                      </TextFieldErrorMessage>
                    </TextFieldRoot>
                  );
                }}
              />
            </div>
          </div>
          <DialogFooter>
            <Button type="submit">Update</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

export default ChangePasswordDialog;
