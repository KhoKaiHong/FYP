import Navbar from "@/components/navigation-bar";
import { useUser } from "@/context/user-context";
import { createEffect, Index, Show } from "solid-js";
import { useNavigate } from "@solidjs/router";
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
import {
  DatePicker,
  DatePickerContent,
  DatePickerContext,
  DatePickerControl,
  DatePickerInput,
  DatePickerPositioner,
  DatePickerRangeText,
  DatePickerTable,
  DatePickerTableBody,
  DatePickerTableCell,
  DatePickerTableCellTrigger,
  DatePickerTableHead,
  DatePickerTableHeader,
  DatePickerTableRow,
  DatePickerTrigger,
  DatePickerView,
  DatePickerViewControl,
  DatePickerViewTrigger,
} from "@/components/ui/date-picker";
import { District } from "@/types/districts";
import { PostNewEventProposalPayload } from "@/types/new-event-proposal";
import { postNewEventProposal } from "@/api/new-event-proposal";
import showSuccessToast from "@/components/success-toast";
import { Button } from "@/components/ui/button";
import { listFacilities } from "@/api/facility";
import { Portal } from "solid-js/web";
import { today } from "@internationalized/date";
import { DateValue } from "@ark-ui/solid";
import { FacilityResponse } from "@/types/users";
import ExistingEventProposalDialog from "./existing-event-proposal-dialog";

function NewEventProposalPage() {
  const { user, isLoading } = useUser();

  const navigate = useNavigate();

  createEffect(() => {
    const loggedInUser = user();
    if (!isLoading() && (!loggedInUser || loggedInUser.role !== "Organiser")) {
      navigate("/", { resolve: false });
    }
  });

  async function getFormData() {
    try {
      const [facilitiesResponse, districtsResponse] = await Promise.all([
        listFacilities(),
        listDistricts(),
      ]);

      const facilities = facilitiesResponse.match(
        (data) => data.data.facilities,
        (error) => {
          console.error("Error fetching facilities.", error);
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

      if (!facilities || !statesAndDistricts) {
        showErrorToast({
          errorTitle: "Error loading new event proposal form.",
          error: { message: "SERVICE_ERROR" },
        });
        return null;
      }

      return {
        facilities,
        states: statesAndDistricts.states,
        districts: statesAndDistricts.districts,
      };
    } catch (err) {
      console.error("Error loading new event proposal form: ", err);
      showErrorToast({
        errorTitle: "Error loading new event proposal form.",
        error: { message: "UNKNOWN_ERROR" },
      });
      return null;
    }
  }

  const [existingEventProposalDialogOpen, setExistingEventProposalDialogOpen] =
    createSignal(false);

  const [formData] = createResource(getFormData);
  const facilities = () => formData()?.facilities ?? null;
  const states = () => formData()?.states ?? null;
  const districts = () => formData()?.districts ?? null;

  const form = createForm(() => ({
    defaultValues: {
      location: "",
      address: "",
      date: null as Date | null,
      startTime: "",
      endTime: "",
      maxAttendees: 0,
      latlng: { lat: null as number | null, lng: null as number | null },
      facilityId: 0,
      stateId: 0,
      districtId: 0,
    },
    onSubmit: async ({ value }) => {
      function combineDateTime(date: Date | null, time: string): string {
        if (!date) {
          throw new Error("Date should be present");
        }

        // Create a new date object from the input date
        const combinedDateTime = new Date(date);

        // Parse the time string
        const [timeValue, period] = time.split(" ");
        const [hours, minutes] = timeValue.split(":").map(Number);

        // Adjust hours for PM times
        let adjustedHours = hours;
        if (period === "PM" && hours !== 12) {
          adjustedHours += 12;
        } else if (period === "AM" && hours === 12) {
          adjustedHours = 0;
        }

        // Set the time on the combined date
        combinedDateTime.setHours(adjustedHours, minutes, 0, 0);

        // Return as ISO string
        return combinedDateTime.toISOString();
      }

      const newEventProposalPayload: PostNewEventProposalPayload = {
        location: value.location,
        address: value.address,
        startTime: combineDateTime(value.date, value.startTime),
        endTime: combineDateTime(value.date, value.endTime),
        maxAttendees: value.maxAttendees,
        latitude: value.latlng.lat ?? 0,
        longitude: value.latlng.lng ?? 0,
        facilityId: value.facilityId,
        stateId: value.stateId,
        districtId: value.districtId,
      };

      const response = await postNewEventProposal(newEventProposalPayload);
      response.match(
        () => {
          showSuccessToast({ successTitle: "New event successfully proposed" });
        },
        (error) => {
          if (error.message === "EXISTING_NEW_EVENT_REQUEST") {
            setExistingEventProposalDialogOpen(true);
          } else {
            showErrorToast({
              errorTitle:
                "Error performing new event proposal. Please try again.",
              error: error,
            });
          }
        }
      );
    },
    validatorAdapter: zodValidator(),
  }));

  const locationSchema = z
    .string()
    .min(1, "Location is required")
    .max(32, "Location must be at most 32 characters");

  const addressSchema = z
    .string()
    .min(1, "Address is required")
    .max(64, "Address must be at most 64 characters");

  const dateSchema = z.date({
    message: "Date is required",
  });

  const startTimes = ["9:00 AM", "10:00 AM", "11:00 AM"];
  const startTimeSchema = z.enum(startTimes as [string, ...string[]], {
    message: "Start time is required",
  });

  const endTimes = ["2:00 PM", "3:00 PM", "4:00 PM", "5:00 PM"];
  const endTimeSchema = z.enum(endTimes as [string, ...string[]], {
    message: "End time is required",
  });

  const maxAttendees = [50, 100, 150, 200, 250, 300];
  const maxAttendeesSchema = z
    .number()
    .refine((value) => maxAttendees.includes(value), {
      message: "Max attendees is required",
    });

  const latlngSchema = z.object({
    lat: z.number(),
    lng: z.number(),
  });

  const facilitiyIdSchema = createMemo(() => {
    const data = facilities();

    if (!data) {
      return z.number();
    } else {
      return z
        .number()
        .refine((id) => data.some((facility) => facility.id === id), {
          message: "Facility is required",
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
        .min(1, { message: "State is required" })
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
          message: "District is required",
        })
        .max(districtsAvailable()[districtsAvailable().length - 1].districtId, {
          message: "District is required",
        });
    }
  });

  const [districtInput, setDistrictInput] = createSignal<District | null>(null);

  let map: google.maps.Map | undefined;
  let infoWindow: google.maps.InfoWindow | undefined;
  let marker: google.maps.marker.AdvancedMarkerElement | undefined;

  async function initialiseMap() {
    const { Map, InfoWindow } = (await google.maps.importLibrary(
      "maps"
    )) as google.maps.MapsLibrary;
    const { AdvancedMarkerElement } = (await google.maps.importLibrary(
      "marker"
    )) as google.maps.MarkerLibrary;

    map = new Map(document.getElementById("map") as HTMLElement, {
      center: { lat: 3.1732962387784367, lng: 101.70668106095312 },
      zoom: 10,
      mapId: "f7a7a21c7ed4070e",
      draggableCursor: "pointer",
    });

    infoWindow = new InfoWindow();

    map.addListener("click", (mapClickEvent: google.maps.MapMouseEvent) => {
      if (marker) {
        infoWindow?.close();
        marker.position = mapClickEvent.latLng;
        marker.title = `Latitude: ${mapClickEvent?.latLng?.lat()}<br>Longitude: ${mapClickEvent?.latLng?.lng()}`;
        infoWindow?.setContent(
          `<p class="text-slate-600">${marker?.title}</p>`
        );
        form.setFieldValue("latlng", {
          lat: mapClickEvent?.latLng?.lat() ?? null,
          lng: mapClickEvent?.latLng?.lng() ?? null,
        });
        form.validateField("latlng", "change");
      } else {
        marker = new AdvancedMarkerElement({
          map,
          position: mapClickEvent.latLng,
          title: `Latitude: ${mapClickEvent?.latLng?.lat()}<br>Longitude: ${mapClickEvent?.latLng?.lng()}`,
          gmpClickable: true,
          gmpDraggable: true,
        });

        infoWindow?.setContent(
          `<p class="text-slate-600">${marker?.title}</p>`
        );

        form.setFieldValue("latlng", {
          lat: mapClickEvent?.latLng?.lat() ?? null,
          lng: mapClickEvent?.latLng?.lng() ?? null,
        });
        form.validateField("latlng", "change");

        marker.addListener("click", () => {
          map?.setCenter({
            lat: marker?.position?.lat as number,
            lng: marker?.position?.lng as number,
          });
          map?.setZoom(18);
          infoWindow?.open(map, marker);
        });

        marker.addListener(
          "dragend",
          (markerDragEvent: google.maps.MapMouseEvent) => {
            infoWindow?.close();
            (
              marker as google.maps.marker.AdvancedMarkerElement
            ).title = `Latitude: ${markerDragEvent?.latLng?.lat()}<br>Longitude: ${markerDragEvent?.latLng?.lng()}`;
            infoWindow?.setContent(
              `<p class="text-slate-600">${marker?.title}</p>`
            );
            form.setFieldValue("latlng", {
              lat: markerDragEvent?.latLng?.lat() ?? null,
              lng: markerDragEvent?.latLng?.lng() ?? null,
            });
            form.validateField("latlng", "change");
          }
        );
      }
    });
  }

  createEffect(() => {
    if (formData()) {
      initialiseMap();
    }
  });

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <Show when={formData()}>
          {(formData) => (
            <>
              <Card>
                <CardHeader>
                  <CardTitle>New Event Proposal</CardTitle>
                  <CardDescription>
                    Fill in details about the event you wish to propose.
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
                          name="location"
                          validators={{ onChange: locationSchema }}
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
                                <TextFieldLabel>Location</TextFieldLabel>
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
                                validationState={
                                  hasError() ? "invalid" : "valid"
                                }
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

                      <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-4 md:gap-6">
                        <form.Field
                          name="date"
                          validators={{ onChange: dateSchema }}
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
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  Date
                                </label>
                                <DatePicker
                                  locale="en-GB"
                                  min={
                                    today("Asia/Kuala_Lumpur").add({
                                      months: 1,
                                    }) as unknown as DateValue
                                  }
                                  max={
                                    today("Asia/Kuala_Lumpur").add({
                                      months: 3,
                                    }) as unknown as DateValue
                                  }
                                  timeZone="Asia/Kuala_Lumpur"
                                  name={field().name}
                                  onValueChange={(value) => {
                                    if (value?.value?.[0]) {
                                      const selectedDate =
                                        value.value[0].toDate("UTC");

                                      field().setValue(selectedDate);
                                    }
                                  }}
                                  onBlur={field().handleBlur}
                                >
                                  <DatePickerControl class="w-full">
                                    <DatePickerInput />
                                    <DatePickerTrigger />
                                  </DatePickerControl>
                                  <Portal>
                                    <DatePickerPositioner>
                                      <DatePickerContent>
                                        <DatePickerView view="day">
                                          <DatePickerContext>
                                            {(context) => (
                                              <>
                                                <DatePickerViewControl>
                                                  <DatePickerViewTrigger>
                                                    <DatePickerRangeText />
                                                  </DatePickerViewTrigger>
                                                </DatePickerViewControl>
                                                <DatePickerTable>
                                                  <DatePickerTableHead>
                                                    <DatePickerTableRow>
                                                      <Index
                                                        each={
                                                          context().weekDays
                                                        }
                                                      >
                                                        {(weekDay) => (
                                                          <DatePickerTableHeader>
                                                            {weekDay().short}
                                                          </DatePickerTableHeader>
                                                        )}
                                                      </Index>
                                                    </DatePickerTableRow>
                                                  </DatePickerTableHead>
                                                  <DatePickerTableBody>
                                                    <Index
                                                      each={context().weeks}
                                                    >
                                                      {(week) => (
                                                        <DatePickerTableRow>
                                                          <Index each={week()}>
                                                            {(day) => (
                                                              <DatePickerTableCell
                                                                value={day()}
                                                              >
                                                                <DatePickerTableCellTrigger>
                                                                  {day().day}
                                                                </DatePickerTableCellTrigger>
                                                              </DatePickerTableCell>
                                                            )}
                                                          </Index>
                                                        </DatePickerTableRow>
                                                      )}
                                                    </Index>
                                                  </DatePickerTableBody>
                                                </DatePickerTable>
                                              </>
                                            )}
                                          </DatePickerContext>
                                        </DatePickerView>
                                        <DatePickerView view="month">
                                          <DatePickerContext>
                                            {(context) => (
                                              <>
                                                <DatePickerViewControl>
                                                  <DatePickerViewTrigger>
                                                    <DatePickerRangeText />
                                                  </DatePickerViewTrigger>
                                                </DatePickerViewControl>
                                                <DatePickerTable>
                                                  <DatePickerTableBody>
                                                    <Index
                                                      each={context().getMonthsGrid(
                                                        {
                                                          columns: 4,
                                                          format: "short",
                                                        }
                                                      )}
                                                    >
                                                      {(months) => (
                                                        <DatePickerTableRow>
                                                          <Index
                                                            each={months()}
                                                          >
                                                            {(month) => (
                                                              <DatePickerTableCell
                                                                value={
                                                                  month().value
                                                                }
                                                              >
                                                                <DatePickerTableCellTrigger>
                                                                  {
                                                                    month()
                                                                      .label
                                                                  }
                                                                </DatePickerTableCellTrigger>
                                                              </DatePickerTableCell>
                                                            )}
                                                          </Index>
                                                        </DatePickerTableRow>
                                                      )}
                                                    </Index>
                                                  </DatePickerTableBody>
                                                </DatePickerTable>
                                              </>
                                            )}
                                          </DatePickerContext>
                                        </DatePickerView>
                                        <DatePickerView view="year">
                                          <DatePickerContext>
                                            {(context) => (
                                              <>
                                                <DatePickerViewControl>
                                                  <DatePickerViewTrigger>
                                                    <DatePickerRangeText />
                                                  </DatePickerViewTrigger>
                                                </DatePickerViewControl>
                                                <DatePickerTable>
                                                  <DatePickerTableBody>
                                                    <Index
                                                      each={context().getYearsGrid(
                                                        {
                                                          columns: 4,
                                                        }
                                                      )}
                                                    >
                                                      {(years) => (
                                                        <DatePickerTableRow>
                                                          <Index each={years()}>
                                                            {(year) => (
                                                              <DatePickerTableCell
                                                                value={
                                                                  year().value
                                                                }
                                                              >
                                                                <DatePickerTableCellTrigger>
                                                                  {year().label}
                                                                </DatePickerTableCellTrigger>
                                                              </DatePickerTableCell>
                                                            )}
                                                          </Index>
                                                        </DatePickerTableRow>
                                                      )}
                                                    </Index>
                                                  </DatePickerTableBody>
                                                </DatePickerTable>
                                              </>
                                            )}
                                          </DatePickerContext>
                                        </DatePickerView>
                                      </DatePickerContent>
                                    </DatePickerPositioner>
                                  </Portal>
                                </DatePicker>
                                <Show when={hasError()} keyed>
                                  <div class="font-medium text-destructive text-xs">
                                    {
                                      field()
                                        .state.meta.errors.join(", ")
                                        .split(", ")[0]
                                    }
                                  </div>
                                </Show>
                              </div>
                            );
                          }}
                        />
                        <form.Field
                          name="startTime"
                          validators={{ onChange: startTimeSchema }}
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
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  Start Time
                                </label>
                                <Select
                                  class="space-y-1 w-full"
                                  name={field().name}
                                  options={startTimes}
                                  validationState={
                                    hasError() ? "invalid" : "valid"
                                  }
                                  value={fieldValue()}
                                  onChange={(e) =>
                                    field().handleChange(e ?? "")
                                  }
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
                        <form.Field
                          name="endTime"
                          validators={{ onChange: endTimeSchema }}
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
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  End Time
                                </label>
                                <Select
                                  class="space-y-1 w-full"
                                  name={field().name}
                                  options={endTimes}
                                  validationState={
                                    hasError() ? "invalid" : "valid"
                                  }
                                  value={fieldValue()}
                                  onChange={(e) =>
                                    field().handleChange(e ?? "")
                                  }
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
                        <form.Field
                          name="maxAttendees"
                          validators={{ onChange: maxAttendeesSchema }}
                          children={(field) => {
                            const hasError = createMemo(() => {
                              return (
                                field().state.meta.errors.length > 0 &&
                                field().state.meta.isTouched
                              );
                            });

                            const fieldValue = createMemo(() => {
                              if (field().state.value === 0) {
                                return null;
                              } else {
                                return field().state.value;
                              }
                            });

                            return (
                              <div class="space-y-1">
                                <label
                                  class={`text-sm font-medium ${
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  Max Attendees
                                </label>
                                <Select
                                  class="space-y-1 w-full"
                                  name={field().name}
                                  options={maxAttendees}
                                  validationState={
                                    hasError() ? "invalid" : "valid"
                                  }
                                  value={fieldValue()}
                                  onChange={(e) => field().handleChange(e ?? 0)}
                                  onBlur={field().handleBlur}
                                  placeholder="--"
                                  itemComponent={(props) => (
                                    <SelectItem item={props.item}>
                                      {props.item.rawValue}
                                    </SelectItem>
                                  )}
                                >
                                  <SelectTrigger class="w-full">
                                    <SelectValue<number>>
                                      {(state) => state.selectedOption()}
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

                      <div class="space-y-2 md:space-y-0 md:grid md:grid-cols-3 md:gap-6">
                        <form.Field
                          name="facilityId"
                          validators={{ onChange: facilitiyIdSchema() }}
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
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  Facility
                                </label>
                                <Select
                                  class="space-y-1 w-full"
                                  name={field().name}
                                  options={formData().facilities}
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
                                    <SelectValue<FacilityResponse>>
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
                                    hasError() ? "text-destructive" : ""
                                  }`}
                                >
                                  State
                                </label>
                                <Select
                                  class="space-y-1 w-full"
                                  name={field().name}
                                  options={formData().states}
                                  optionValue="id"
                                  optionTextValue="name"
                                  validationState={
                                    hasError() ? "invalid" : "valid"
                                  }
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
                              return (
                                !districts() ||
                                districtsAvailable().length === 0
                              );
                            });

                            return (
                              <div class="space-y-1">
                                <label
                                  class={`text-sm font-medium ${
                                    disabled()
                                      ? "cursor-not-allowed opacity-70"
                                      : ""
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
                                    field().handleChange(
                                      selectedOption?.districtId ?? 0
                                    );
                                  }}
                                  optionValue="districtId"
                                  optionTextValue="districtName"
                                  optionLabel="districtName"
                                  disabled={disabled()}
                                  validationState={
                                    hasError() ? "invalid" : "valid"
                                  }
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
                                    {
                                      field()
                                        .state.meta.errors.join(", ")
                                        .split(", ")[0]
                                    }
                                  </ComboboxErrorMessage>
                                  <ComboboxContent />
                                </Combobox>
                              </div>
                            );
                          }}
                        />
                      </div>
                      <form.Field
                        name="latlng"
                        validators={{
                          onChange: latlngSchema,
                        }}
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
                                class={`text-sm font-medium block  ${
                                  hasError() ? "text-destructive" : ""
                                }`}
                              >
                                Map
                              </label>
                              <label class="text-sm font-normal text-muted-foreground italic block">
                                Click on map / Drag marker to set marker
                                location. Click on marker to recenter and get
                                coordinates.
                              </label>
                              <div
                                class="w-full h-[36rem]"
                                id="map"
                                ref={map as unknown as HTMLDivElement}
                              ></div>
                              <Show when={hasError()} keyed>
                                <div class="font-medium text-destructive text-xs">
                                  Please add a marker
                                </div>
                              </Show>
                            </div>
                          );
                        }}
                      />
                    </div>
                    <div class="flex items-center pt-6">
                      <Button type="submit">Send Proposal</Button>
                    </div>
                  </form>
                </CardContent>
              </Card>
              <ExistingEventProposalDialog
                open={existingEventProposalDialogOpen()}
                onClose={() => setExistingEventProposalDialogOpen(false)}
              />
            </>
          )}
        </Show>
      </div>
    </div>
  );
}

export default NewEventProposalPage;
