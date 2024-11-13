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

function UserRegisterForm() {
  const form = createForm(() => ({
    defaultValues: {
      icNumber: "",
      password: "",
      passwordConfirm: "",
      name: "",
      email: "",
      phoneNumber: "",
      bloodType: "A+",
      stateId: "",
      districtId: "",
    },
    onSubmit: async ({ value }) => {
      // Do something with form data
      console.log(value);
    },
    validatorAdapter: zodValidator(),
  }));

  const icNumberSchema = z
    .string()
    .min(1, "IC number is required")
    .regex(
      /^\d{6}-\d{2}-\d{4}$/,
      "IC number must be in format: 123456-78-9012"
    );

  const passwordSchema = z
    .string()
    .min(8, "Password must be at least 8 characters")
    .max(32, "Password must be at most 32 characters");

  const passwordConfirmSchema = z
    .string()
    .min(8, "Password must be at least 8 characters")
    .max(32, "Password must be at most 32 characters")
    .refine((value) => value === form.getFieldValue("password"), {
      message: "Passwords do not match",
      path: ["passwordConfirm"],
    });

  const nameSchema = z
    .string()
    .min(1, "Name is required")
    .max(64, "Name must be at most 64 characters");

  const emailSchema = z
    .string()
    .min(1, "Email is required")
    .max(64, "Email must be at most 64 characters")
    .email("Email is not valid");

  const phoneNumberSchema = z
    .string()
    .min(1, "Phone number is required")
    .regex(/^\+60\d{1,2}-\d{7,8}$/, "Invalid phone number format");

  const bloodTypeSchema = z.enum([
    "A+",
    "A-",
    "B+",
    "B-",
    "O+",
    "O-",
    "AB+",
    "AB-",
  ]);

  const stateIdSchema = z.string().min(1, "State ID is required");

  const districtIdSchema = z.string().min(1, "District ID is required");

  const formSchema = z.object({
    icNumber: icNumberSchema,
    password: passwordSchema,
    passwordConfirm: passwordConfirmSchema,
    name: nameSchema,
    email: emailSchema,
    phoneNumber: phoneNumberSchema,
    bloodType: bloodTypeSchema,
    stateId: stateIdSchema,
    districtId: districtIdSchema,
  });

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
      >
        <div class="space-y-2">
          <form.Field
            name="icNumber"
            validators={{ onChange: icNumberSchema }}
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
                  <TextFieldLabel>IC Number</TextFieldLabel>
                  <TextField placeholder="e.g. 123456-78-9012" />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
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
                  <TextField type="password" />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
          <form.Field
            name="passwordConfirm"
            validators={{ onChange: passwordConfirmSchema }}
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
                  <TextFieldLabel>Confirm Password</TextFieldLabel>
                  <TextField type="password" />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
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
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
          <form.Field
            name="email"
            validators={{ onChange: emailSchema }}
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
                  <TextFieldLabel>Email</TextFieldLabel>
                  <TextField type="email" />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
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
                  <TextFieldLabel>Phone Number</TextFieldLabel>
                  <TextField type="tel" placeholder="e.g. +6012-3456789" />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
          <form.Field
            name="bloodType"
            validators={{ onChange: bloodTypeSchema }}
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
                  <TextFieldLabel>Blood Type</TextFieldLabel>
                  <TextField />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
          <form.Field
            name="stateId"
            validators={{ onChange: stateIdSchema }}
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
                  <TextFieldLabel>State ID</TextFieldLabel>
                  <TextField />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
          <form.Field
            name="districtId"
            validators={{ onChange: districtIdSchema }}
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
                  <TextFieldLabel>District ID</TextFieldLabel>
                  <TextField />
                  <TextFieldErrorMessage>
                    {field().state.meta.errors.join(", ")}
                  </TextFieldErrorMessage>
                </TextFieldRoot>
              );
            }}
          />
        </div>
        <button type="submit">Submit</button>
      </form>
    </div>
  );
}

export default UserRegisterForm;
