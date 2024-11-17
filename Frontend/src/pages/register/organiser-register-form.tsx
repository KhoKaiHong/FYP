import { createForm } from "@tanstack/solid-form";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo, createSignal } from "solid-js";
import { Eye, EyeOff } from "lucide-solid";
import showErrorToast from "@/components/error-toast";
import { OrganiserRegisterPayload } from "@/types/register";
import { organiserRegister } from "@/api/register";
import showSuccessToast from "@/components/success-toast";
import { Button } from "@/components/ui/button";
import { useNavigate } from "@solidjs/router";

function OrganiserRegisterForm() {
  const navigate = useNavigate();

  const [duplicateEmail, setDuplicateEmail] = createSignal(false);
  const [duplicatePhoneNumber, setDuplicatePhoneNumber] = createSignal(false);

  const form = createForm(() => ({
    defaultValues: {
      email: "",
      phoneNumber: "",
      name: "",
      password: "",
      passwordConfirm: "",
    },
    onSubmit: async ({ value }) => {
      const organiserRegisterPayload: OrganiserRegisterPayload = {
        email: value.email,
        password: value.password,
        name: value.name,
        phoneNumber: "+60" + value.phoneNumber,
      };
      const response = await organiserRegister(organiserRegisterPayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Register successful" });
          navigate("/login/organiser");
        },
        (error) => {
          if (error.message === "DUPLICATE_RECORD") {
            if (error.detail === "email") {
              setDuplicateEmail(true);
            } else if (error.detail === "phone number") {
              setDuplicatePhoneNumber(true);
            } else {
              showErrorToast({
                errorTitle: "Error performing registration. Please try again.",
                error: error,
              });
            }
          } else {
            showErrorToast({
              errorTitle: "Error performing registration. Please try again.",
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

  const phoneNumberSchema = z
    .string()
    .min(1, "Phone number is required")
    .regex(/^\d{1,2}-\d{7,8}$/, "Invalid phone number format");

  const nameSchema = z
    .string()
    .min(1, "Name is required")
    .max(64, "Name must be at most 64 characters");

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
    <form
      onSubmit={(e) => {
        e.preventDefault();
        e.stopPropagation();
        form.handleSubmit();
      }}
    >
      <div class="space-y-2">
        <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
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
                  <TextFieldLabel>Password</TextFieldLabel>
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
                        <EyeOff size={16} strokeWidth={2} aria-hidden="true" />
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
                  <TextFieldLabel>Confirm Password</TextFieldLabel>
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
                        <EyeOff size={16} strokeWidth={2} aria-hidden="true" />
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
      <div class="flex items-center pt-6">
        <Button type="submit">Register</Button>
      </div>
    </form>
  );
}

export default OrganiserRegisterForm;
