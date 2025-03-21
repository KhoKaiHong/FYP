import Navbar from "@/components/navigation-bar";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { createForm } from "@tanstack/solid-form";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import {
  createEffect,
  createMemo,
  createResource,
  createSignal,
  Show,
} from "solid-js";
import { Eye, EyeOff } from "lucide-solid";
import showErrorToast from "@/components/error-toast";
import { FacilityRegisterPayload } from "@/types/register";
import { facilityRegister } from "@/api/register";
import showSuccessToast from "@/components/success-toast";
import { Button } from "@/components/ui/button";
import { useUser } from "@/context/user-context";
import { useNavigate } from "@solidjs/router";
import { listStates } from "@/api/states";
import {
  Select,
  SelectContent,
  SelectErrorMessage,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { State } from "@/types/states";

function AddFacilityPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Admin")) {
      navigate("/", { resolve: false });
    }
  });

  async function fetchStates() {
    try {
      const statesResponse = await listStates();

      return statesResponse.match(
        (data) => {
          return data.data.states;
        },
        (error) => {
          showErrorToast({
            errorTitle: "Error loading add facility form.",
            error: { message: "SERVICE_ERROR" },
          });
          console.error("Error fetching states:", error);
          return null;
        }
      );
    } catch (err) {
      console.error("Error fetching states: ", err);
      showErrorToast({
        errorTitle: "Error loading add facility form.",
        error: { message: "UNKNOWN_ERROR" },
      });
      return null;
    }
  }

  const [states] = createResource(fetchStates);

  const [duplicateEmail, setDuplicateEmail] = createSignal(false);
  const [duplicatePhoneNumber, setDuplicatePhoneNumber] = createSignal(false);

  const form = createForm(() => ({
    defaultValues: {
      email: "",
      name: "",
      password: "",
      passwordConfirm: "",
      address: "",
      phoneNumber: "",
      stateId: 0,
    },
    onSubmit: async ({ value }) => {
      const facilityRegisterPayload: FacilityRegisterPayload = {
        email: value.email,
        name: value.name,
        password: value.password,
        address: value.address,
        phoneNumber: "+60" + value.phoneNumber,
        stateId: value.stateId,
      };
      const response = await facilityRegister(facilityRegisterPayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Blood collection facility added successfully" });
        },
        (error) => {
          if (error.message === "DUPLICATE_RECORD") {
            if (error.detail === "email") {
              setDuplicateEmail(true);
            } else if (error.detail === "phone number") {
              setDuplicatePhoneNumber(true);
            } else {
              showErrorToast({
                errorTitle: "Error adding blood collection facility. Please try again.",
                error: error,
              });
            }
          } else {
            showErrorToast({
              errorTitle: "Error adding blood collection facility. Please try again.",
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

  const phoneNumberSchema = z
    .string()
    .min(1, "Phone number is required")
    .regex(/^\d{1,2}-\d{7,8}$/, "Invalid phone number format");

  const stateIdSchema = createMemo(() => {
    const data = states();

    if (!data) {
      return z.number();
    } else {
      return z
        .number()
        .min(1, { message: "This field is required" })
        .max(data.length);
    }
  });

  const addressSchema = z
    .string()
    .min(1, "Address is required")
    .max(128, "Address must be at most 128 characters");

  return (
    <div>
      <Navbar />
      <div class="flex justify-center items-center min-h-[calc(100dvh-4rem)]">
        <Show when={states()} keyed>
          {(states) => (
            <div class="w-full max-w-4xl p-8">
              <Card>
                <CardHeader>
                  <CardTitle>Add Blood Collection Facility</CardTitle>
                  <CardDescription>
                    Add a blood collection facility here.
                  </CardDescription>
                </CardHeader>
                <CardContent class="space-y-2">
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
                                validationState={
                                  hasError() ? "invalid" : "valid"
                                }
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
                                    : field()
                                        .state.meta.errors.join(", ")
                                        .split(", ")[0]}
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
                                validationState={
                                  hasError() ? "invalid" : "valid"
                                }
                                value={field().state.value}
                                onBlur={field().handleBlur}
                                onChange={field().handleChange}
                              >
                                <TextFieldLabel>Name</TextFieldLabel>
                                <TextField />
                                <TextFieldErrorMessage>
                                  {
                                    field()
                                      .state.meta.errors.join(", ")
                                      .split(", ")[0]
                                  }
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

                            const [isVisible, setIsVisible] =
                              createSignal(false);
                            function toggleVisibility() {
                              setIsVisible(!isVisible());
                            }

                            return (
                              <TextFieldRoot
                                class="space-y-1"
                                name={field().name}
                                validationState={
                                  hasError() ? "invalid" : "valid"
                                }
                                value={field().state.value}
                                onBlur={field().handleBlur}
                                onChange={field().handleChange}
                              >
                                <TextFieldLabel>Password</TextFieldLabel>
                                <div class="relative">
                                  <TextField
                                    type={isVisible() ? "text" : "password"}
                                  />
                                  <button
                                    class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                                    type="button"
                                    onClick={toggleVisibility}
                                    aria-label={
                                      isVisible()
                                        ? "Hide password"
                                        : "Show password"
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
                                      <Eye
                                        size={16}
                                        strokeWidth={2}
                                        aria-hidden="true"
                                      />
                                    )}
                                  </button>
                                </div>
                                <TextFieldErrorMessage>
                                  {
                                    field()
                                      .state.meta.errors.join(", ")
                                      .split(", ")[0]
                                  }
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

                            const [isVisible, setIsVisible] =
                              createSignal(false);
                            function toggleVisibility() {
                              setIsVisible(!isVisible());
                            }

                            return (
                              <TextFieldRoot
                                class="space-y-1"
                                name={field().name}
                                validationState={
                                  hasError() ? "invalid" : "valid"
                                }
                                value={field().state.value}
                                onBlur={field().handleBlur}
                                onChange={field().handleChange}
                              >
                                <TextFieldLabel>
                                  Confirm Password
                                </TextFieldLabel>
                                <div class="relative">
                                  <TextField
                                    type={isVisible() ? "text" : "password"}
                                  />
                                  <button
                                    class="absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center rounded-e-lg text-muted-foreground/80 ring-offset-background transition-shadow hover:text-foreground focus-visible:border focus-visible:border-ring focus-visible:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                                    type="button"
                                    onClick={toggleVisibility}
                                    aria-label={
                                      isVisible()
                                        ? "Hide password"
                                        : "Show password"
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
                                      <Eye
                                        size={16}
                                        strokeWidth={2}
                                        aria-hidden="true"
                                      />
                                    )}
                                  </button>
                                </div>
                                <TextFieldErrorMessage>
                                  {
                                    field()
                                      .state.meta.errors.join(", ")
                                      .split(", ")[0]
                                  }
                                </TextFieldErrorMessage>
                              </TextFieldRoot>
                            );
                          }}
                        />
                      </div>

                      <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
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
                                validationState={
                                  hasError() ? "invalid" : "valid"
                                }
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
                                    : field()
                                        .state.meta.errors.join(", ")
                                        .split(", ")[0]}
                                </TextFieldErrorMessage>
                              </TextFieldRoot>
                            );
                          }}
                        />

                        <form.Field
                          name="stateId"
                          validators={{ onChange: stateIdSchema() }}
                          children={(field) => {
                            const hasError = createMemo(() => {
                              return (
                                field().state.meta.errors.length > 0 &&
                                field().state.meta.isTouched
                              );
                            });

                            return (
                              <div class="space-y-1">
                                <label
                                  class={`text-sm font-medium  ${
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  State
                                </label>
                                <Select
                                  class="space-y-1 w-full"
                                  name={field().name}
                                  options={states}
                                  optionValue="id"
                                  optionTextValue="name"
                                  validationState={
                                    hasError() ? "invalid" : "valid"
                                  }
                                  onChange={(e) => {
                                    field().handleChange(e?.id ?? 0);
                                  }}
                                  onBlur={field().handleBlur}
                                  placeholder="--"
                                  itemComponent={(props) => (
                                    <SelectItem item={props.item}>
                                      {props.item.rawValue.name}
                                    </SelectItem>
                                  )}
                                >
                                  <SelectTrigger class="w-full">
                                    <SelectValue<State>>
                                      {(state) => state.selectedOption().name}
                                    </SelectValue>
                                  </SelectTrigger>
                                  <SelectErrorMessage class="font-medium text-destructive text-xs">
                                    {
                                      field()
                                        .state.meta.errors.join(", ")
                                        .split(", ")[0]
                                    }
                                  </SelectErrorMessage>
                                  <SelectContent />
                                </Select>
                              </div>
                            );
                          }}
                        />
                      </div>

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
                                {
                                  field()
                                    .state.meta.errors.join(", ")
                                    .split(", ")[0]
                                }
                              </TextFieldErrorMessage>
                            </TextFieldRoot>
                          );
                        }}
                      />
                    </div>
                    <div class="flex items-center pt-6">
                      <Button type="submit">
                        Add Blood Collection Facility
                      </Button>
                    </div>
                  </form>
                </CardContent>
              </Card>
            </div>
          )}
        </Show>
      </div>
    </div>
  );
}

export default AddFacilityPage;
