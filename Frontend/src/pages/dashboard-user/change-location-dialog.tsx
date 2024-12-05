import { createForm } from "@tanstack/solid-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import { createMemo, createSignal } from "solid-js";
import { listDistricts } from "@/api/districts";
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
import { District } from "@/types/districts";
import { UserUpdatePayload } from "@/types/user";
import { updateUser } from "@/api/user";
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

function ChangeLocationDialog() {
  const { refreshUser } = useUser();

  async function getStatesDistricts() {
    try {
      const districtsResponse = await listDistricts();

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

      if (!statesAndDistricts) {
        showErrorToast({
          errorTitle: "Error loading dialog.",
          error: { message: "SERVICE_ERROR" },
        });
        return null;
      }

      return {
        states: statesAndDistricts.states,
        districts: statesAndDistricts.districts,
      };
    } catch (err) {
      console.error("Error loading dialog: ", err);
      showErrorToast({
        errorTitle: "Error loading dialog.",
        error: { message: "UNKNOWN_ERROR" },
      });
      return null;
    }
  }

  const [statesDistricts] = createResource(getStatesDistricts);
  const states = () => statesDistricts()?.states ?? null;
  const districts = () => statesDistricts()?.districts ?? null;

  const form = createForm(() => ({
    defaultValues: {
      stateId: 0,
      districtId: 0,
    },
    onSubmit: async ({ value }) => {
      const userUpdatePayload: UserUpdatePayload = {
        stateId: value.stateId,
        districtId: value.districtId,
      };
      const response = await updateUser(userUpdatePayload);
      response.match(
        () => {
          showSuccessToast({
            successTitle: "State and district update successful.",
          });
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
          <DialogTitle>Change location</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <div class="space-y-2 pb-4">
            <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-2 md:gap-6">
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
          <DialogFooter>
            <Button type="submit" disabled={statesDistricts() ? false : true}>
              Update
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

export default ChangeLocationDialog;
