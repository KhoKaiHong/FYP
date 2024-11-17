import { createForm } from "@tanstack/solid-form";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo, createSignal, Suspense } from "solid-js";
import { Eye, EyeOff } from "lucide-solid";
import { listBloodTypes } from "@/routes/blood-types";
import { listDistricts } from "@/routes/districts";
import { createResource } from "solid-js";
import showErrorToast from "@/components/error-toast";
import { State } from "@/types/states";
import {
  Select,
  SelectContent,
  SelectErrorMessage,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Combobox,
  ComboboxContent,
  ComboboxErrorMessage,
  ComboboxInput,
  ComboboxItem,
  ComboboxTrigger,
} from "@/components/ui/combobox";
import { Skeleton } from "@/components/ui/skeleton";
import { District } from "@/types/districts";
import { UserRegisterPayload } from "@/types/register";
import { userRegister } from "@/routes/register";
import showSuccessToast from "@/components/success-toast";
import { Button } from "@/components/ui/button";
import { useNavigate } from "@solidjs/router";

function UserRegisterForm() {
  const navigate = useNavigate();

  async function getFormData() {
    try {
      const [bloodTypesResponse, districtsResponse] = await Promise.all([
        listBloodTypes(),
        listDistricts(),
      ]);

      const bloodTypes = bloodTypesResponse.match(
        (data) => data.data.bloodTypes,
        (error) => {
          console.error("Error fetching states and districts.", error);
          return null;
        }
      );

      const statesAndDistricts = districtsResponse.match(
        (data) => {
          const districts = data.data.districts;
          const statesMap = districts.reduce((map, district) => {
            map.set(district.stateId, district.stateName);
            return map;
          }, new Map<number, string>());

          const states = Array.from(statesMap, ([id, name]) => ({
            id,
            name,
          })) as State[];

          return { states, districts };
        },
        (error) => {
          console.error("Error fetching states and districts.", error);
          return null;
        }
      );

      if (!bloodTypes || !statesAndDistricts) {
        showErrorToast({
          errorTitle: "Error loading registration form.",
          error: { message: "SERVICE_ERROR" },
        });
        return null;
      }

      return {
        bloodTypes,
        states: statesAndDistricts.states,
        districts: statesAndDistricts.districts,
      };
    } catch (err) {
      console.error("Error loading registration form: ", err);
      showErrorToast({
        errorTitle: "Error loading registration form.",
        error: { message: "UNKNOWN_ERROR" },
      });
      return null;
    }
  }

  const [formData] = createResource(getFormData);
  const bloodTypes = () => formData()?.bloodTypes ?? null;
  const states = () => formData()?.states ?? null;
  const districts = () => formData()?.districts ?? null;

  const [duplicateIcNumber, setDuplicateIcNumber] = createSignal(false);
  const [duplicateEmail, setDuplicateEmail] = createSignal(false);
  const [duplicatePhoneNumber, setDuplicatePhoneNumber] = createSignal(false);

  const form = createForm(() => ({
    defaultValues: {
      icNumber: "",
      password: "",
      passwordConfirm: "",
      name: "",
      email: "",
      phoneNumber: "",
      bloodType: "",
      stateId: 0,
      districtId: 0,
    },
    onSubmit: async ({ value }) => {
      const userRegisterPayload: UserRegisterPayload = {
        icNumber: value.icNumber,
        password: value.password,
        name: value.name,
        email: value.email,
        phoneNumber: "+60" + value.phoneNumber,
        bloodType: value.bloodType,
        stateId: value.stateId,
        districtId: value.districtId,
      };
      const response = await userRegister(userRegisterPayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "Register successful" });
          navigate("/login/user");
        },
        (error) => {
          if (error.message === "DUPLICATE_RECORD") {
            if (error.detail === "IC number") {
              setDuplicateIcNumber(true);
            } else if (error.detail === "email") {
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

  const nameSchema = z
    .string()
    .min(1, "Name is required")
    .max(64, "Name must be at most 64 characters");

  const emailSchema = z
    .string()
    .min(1, "Email is required")
    .email("Please enter a valid email address")
    .max(64, "Email must be at most 64 characters");

  const phoneNumberSchema = z
    .string()
    .min(1, "Phone number is required")
    .regex(/^\d{1,2}-\d{7,8}$/, "Invalid phone number format");

  const bloodTypeSchema = createMemo(() => {
    const data = bloodTypes();

    if (!data) {
      return z.enum([""]);
    } else {
      return z.enum(data as [string, ...string[]], {
        message: "This field is required",
      });
    }
  });

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

  const [stateIdChosen, setStateIdChosen] = createSignal(0);

  const districtsAvailable = createMemo(() => {
    const data = districts();
    const stateId = stateIdChosen();

    if (!data || stateId === 0) {
      return [];
    } else {
      const districtsInState = data.filter(
        (district) => district.stateId === stateId
      );

      return districtsInState;
    }
  });

  const districtIdSchema = createMemo(() => {
    if (districtsAvailable().length === 0) {
      return z.number();
    } else {
      return z
        .number()
        .min(districtsAvailable()[0].districtId, {
          message: "This field is required",
        })
        .max(districtsAvailable()[districtsAvailable().length - 1].districtId, {
          message: "This field is required",
        });
    }
  });

  const [districtInput, setDistrictInput] = createSignal<District | null>(null);

  return (
    <div>
      <Suspense
        fallback={
          <div class="space-y-8 h-full min-h-80 py-3 ">
            <div class="space-y-8 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
              <Skeleton class="w-full h-14" />
              <Skeleton class="w-full h-14" />
            </div>
            <div class="space-y-8 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
              <Skeleton class="w-full h-14" />
              <Skeleton class="w-full h-14" />
            </div>
            <div class="space-y-8 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
              <Skeleton class="w-full h-14" />
              <Skeleton class="w-full h-14" />
            </div>
            <div class="space-y-8 md:space-y-0 md:grid md:grid-cols-3 md:gap-6">
              <Skeleton class="w-full h-14" />
              <Skeleton class="w-full h-14" />
              <Skeleton class="w-full h-14" />
            </div>
          </div>
        }
      >
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
                name="icNumber"
                validators={{ onChange: icNumberSchema }}
                children={(field) => {
                  const hasError = createMemo(() => {
                    return (
                      (field().state.meta.errors.length > 0 &&
                        field().state.meta.isTouched) ||
                      duplicateIcNumber()
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
                        setDuplicateIcNumber(false);
                      }}
                    >
                      <TextFieldLabel>IC Number</TextFieldLabel>
                      <TextField placeholder="e.g. 123456-78-9012" />
                      <TextFieldErrorMessage>
                        {duplicateIcNumber()
                          ? "IC Number already registered"
                          : field().state.meta.errors.join(", ").split(", ")[0]}
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
                        {field().state.meta.errors.join(", ").split(", ")[0]}
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

            <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-3 md:gap-6">
              <form.Field
                name="bloodType"
                validators={{ onChange: bloodTypeSchema() }}
                children={(field) => {
                  const hasError = createMemo(() => {
                    return (
                      field().state.meta.errors.length > 0 &&
                      field().state.meta.isTouched
                    );
                  });

                  const fieldValue = createMemo(() => {
                    if (field().state.value === "") {
                      return null;
                    } else {
                      return field().state.value;
                    }
                  });

                  return (
                    <div class="space-y-1">
                      <label
                        class={`text-sm font-medium ${
                          !bloodTypes() ? "cursor-not-allowed opacity-70" : ""
                        } ${hasError() ? "text-destructive" : ""}`}
                      >
                        Blood Type
                      </label>
                      <Select
                        class="space-y-1 w-full"
                        name={field().name}
                        options={bloodTypes() ?? []}
                        disabled={bloodTypes() ? false : true}
                        validationState={hasError() ? "invalid" : "valid"}
                        value={fieldValue()}
                        onChange={(e) => field().handleChange(e ?? "")}
                        onBlur={field().handleBlur}
                        placeholder="--"
                        itemComponent={(props) => (
                          <SelectItem item={props.item}>
                            {props.item.rawValue}
                          </SelectItem>
                        )}
                      >
                        <SelectTrigger class="w-full">
                          <SelectValue<string>>
                            {(state) => state.selectedOption()}
                          </SelectValue>
                        </SelectTrigger>
                        <SelectErrorMessage class="font-medium text-destructive text-xs">
                          {field().state.meta.errors.join(", ").split(", ")[0]}
                        </SelectErrorMessage>
                        <SelectContent />
                      </Select>
                    </div>
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
                        class={`text-sm font-medium ${
                          !states() ? "cursor-not-allowed opacity-70" : ""
                        } ${hasError() ? "text-destructive" : ""}`}
                      >
                        State
                      </label>
                      <Select
                        class="space-y-1 w-full"
                        name={field().name}
                        options={states() ?? []}
                        optionValue="id"
                        optionTextValue="name"
                        disabled={states() ? false : true}
                        validationState={hasError() ? "invalid" : "valid"}
                        onChange={(e) => {
                          field().handleChange(e?.id ?? 0);
                          setStateIdChosen(e?.id ?? 0);
                          setDistrictInput(null);
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
                          {field().state.meta.errors.join(", ").split(", ")[0]}
                        </SelectErrorMessage>
                        <SelectContent />
                      </Select>
                    </div>
                  );
                }}
              />
              <form.Field
                name="districtId"
                validators={{
                  onChange: districtIdSchema(),
                }}
                children={(field) => {
                  const hasError = createMemo(() => {
                    return (
                      field().state.meta.errors.length > 0 &&
                      field().state.meta.isTouched
                    );
                  });

                  const disabled = createMemo(() => {
                    return !districts() || districtsAvailable().length === 0;
                  });

                  return (
                    <div class="space-y-1">
                      <label
                        class={`text-sm font-medium ${
                          disabled() ? "cursor-not-allowed opacity-70" : ""
                        } ${hasError() ? "text-destructive" : ""}`}
                      >
                        District
                      </label>
                      <Combobox
                        class="space-y-1"
                        name={field().name}
                        options={districtsAvailable()}
                        value={districtInput()}
                        onChange={(selectedOption) => {
                          setDistrictInput(selectedOption);
                          field().handleChange(selectedOption?.districtId ?? 0);
                        }}
                        optionValue="districtId"
                        optionTextValue="districtName"
                        optionLabel="districtName"
                        disabled={disabled()}
                        validationState={hasError() ? "invalid" : "valid"}
                        onBlur={field().handleBlur}
                        placeholder="Enter district..."
                        itemComponent={(props) => (
                          <ComboboxItem item={props.item}>
                            {props.item.rawValue.districtName}
                          </ComboboxItem>
                        )}
                      >
                        <ComboboxTrigger>
                          <ComboboxInput
                            onBlur={(e) => {
                              e.currentTarget.value =
                                districtInput()?.districtName ?? "";
                            }}
                          />
                        </ComboboxTrigger>
                        <ComboboxErrorMessage class="font-medium text-destructive text-xs">
                          {field().state.meta.errors.join(", ").split(", ")[0]}
                        </ComboboxErrorMessage>
                        <ComboboxContent />
                      </Combobox>
                    </div>
                  );
                }}
              />
            </div>
          </div>
          <div class="flex items-center pt-6">
            <Button type="submit" disabled={formData() ? false : true}>
              Register
            </Button>
          </div>
        </form>
      </Suspense>
    </div>
  );
}

export default UserRegisterForm;
