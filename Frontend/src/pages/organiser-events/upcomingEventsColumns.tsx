import { ColumnDef } from "@tanstack/solid-table";
import { createEffect, createMemo, createSignal, Index, Show } from "solid-js";
import {
  ContactRound,
  Mail,
  Phone,
  MapPinned,
  MapPin,
  Pencil,
  PencilOff,
  ArrowUpDown,
} from "lucide-solid";
import type { PopoverTriggerProps } from "@kobalte/core/popover";
import { Button } from "@/components/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverTitle,
  PopoverTrigger,
} from "@/components/ui/popover";
import type { DialogTriggerProps } from "@kobalte/core/dialog";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectErrorMessage,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
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
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldLabel,
  TextFieldRoot,
} from "@/components/ui/text-field";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { today, CalendarDate } from "@internationalized/date";
import { DateValue } from "@ark-ui/solid";
import { Event } from "@/types/events";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { createForm } from "@tanstack/solid-form";
import { PostChangeEventRequestPayload } from "@/types/change-event-request";
import { postChangeEventRequest } from "@/api/change-event-request";
import showSuccessToast from "@/components/success-toast";
import showErrorToast from "@/components/error-toast";
import { z } from "zod";
import { Portal } from "solid-js/web";
import { TextArea } from "@/components/ui/textarea";

export const upcomingEventsColumns: ColumnDef<Event>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "location",
    header: "Location",
    cell: (props) => {
      const location = props.row.original.location;
      const address = props.row.original.address;
      const latitude = props.row.original.latitude;
      const longitude = props.row.original.longitude;

      let map: google.maps.Map | undefined;
      let infoWindow: google.maps.InfoWindow | undefined;
      let marker: google.maps.marker.AdvancedMarkerElement | undefined;

      const [mapDialogOpen, setMapDialogOpen] = createSignal(false);

      async function initialiseMap() {
        const { Map, InfoWindow } = (await google.maps.importLibrary(
          "maps"
        )) as google.maps.MapsLibrary;
        const { AdvancedMarkerElement } = (await google.maps.importLibrary(
          "marker"
        )) as google.maps.MarkerLibrary;

        map = new Map(document.getElementById("map") as HTMLElement, {
          center: { lat: latitude, lng: longitude },
          zoom: 18,
          mapId: "f7a7a21c7ed4070e",
        });

        infoWindow = new InfoWindow();

        marker = new AdvancedMarkerElement({
          map,
          position: { lat: latitude, lng: longitude },
          title: `Latitude: ${latitude}<br>Longitude: ${longitude}`,
          gmpClickable: true,
        });

        infoWindow?.setContent(
          `<p class="text-slate-600">${marker?.title}</p>`
        );

        marker.addListener("click", () => {
          map?.setCenter({
            lat: marker?.position?.lat as number,
            lng: marker?.position?.lng as number,
          });
          map?.setZoom(18);
          infoWindow?.open(map, marker);
        });
      }

      createEffect(() => {
        if (mapDialogOpen()) {
          initialiseMap();
        }
      });

      return (
        <div class="flex items-center justify-between pr-1">
          {location}
          <Dialog open={mapDialogOpen()} onOpenChange={setMapDialogOpen}>
            <DialogTrigger
              as={(props: DialogTriggerProps) => (
                <Button variant="ghost" size={"icon"} {...props}>
                  <MapPinned size={18} />
                </Button>
              )}
            />
            <DialogContent class="max-w-4xl">
              <DialogHeader>
                <DialogTitle>Location on Map</DialogTitle>
                <DialogDescription>Address: {address}</DialogDescription>
              </DialogHeader>
              <div
                class="w-full h-[36rem]"
                id="map"
                ref={map as unknown as HTMLDivElement}
              ></div>
            </DialogContent>
          </Dialog>
        </div>
      );
    },
  },
  {
    accessorKey: "date",
    header: (props) => {
      return (
        <Button
          variant="ghost"
          onClick={() =>
            props.column.toggleSorting(props.column.getIsSorted() === "asc")
          }
          class="px-1"
        >
          Date
          <ArrowUpDown class="ml-2 h-4 w-4" />
        </Button>
      );
    },
    cell: (props) => {
      const date = createMemo(() => {
        const utcTime: string = props.row.getValue("startTime");
        const date = new Date(utcTime);
        return date.toLocaleString("en-MY", {
          timeZone: "Asia/Kuala_Lumpur",
          dateStyle: "medium",
        });
      });
      return <div>{date()}</div>;
    },
    sortingFn: (rowA, rowB) => {
      const dateA = new Date(rowA.original.startTime);
      const dateB = new Date(rowB.original.startTime);
      return dateA.getTime() - dateB.getTime();
    },
  },
  {
    accessorKey: "startTime",
    header: "Start Time",
    cell: (props) => {
      const startTime = createMemo(() => {
        const utcTime: string = props.row.getValue("startTime");
        const date = new Date(utcTime);
        return date.toLocaleString("en-MY", {
          timeZone: "Asia/Kuala_Lumpur",
          hour: "numeric",
          minute: "2-digit",
          hour12: true,
        });
      });
      return <div>{startTime()}</div>;
    },
  },
  {
    accessorKey: "endTime",
    header: "End Time",
    cell: (props) => {
      const endTime = createMemo(() => {
        const utcTime: string = props.row.getValue("endTime");
        const date = new Date(utcTime);
        return date.toLocaleString("en-MY", {
          timeZone: "Asia/Kuala_Lumpur",
          hour: "numeric",
          minute: "2-digit",
          hour12: true,
        });
      });
      return <div>{endTime()}</div>;
    },
  },
  {
    accessorKey: "attendees",
    header: "Attendees",
    cell: (props) => {
      return (
        <div>
          {props.row.original.currentAttendees} /{" "}
          {props.row.original.maxAttendees}
        </div>
      );
    },
  },
  {
    accessorKey: "stateName",
    header: "State",
  },
  {
    accessorKey: "districtName",
    header: "District",
  },
  {
    accessorKey: "facilityName",
    header: "Facility",
    cell: (props) => {
      const facilityName = props.row.original.facilityName;
      const facilityEmail = props.row.original.facilityEmail;
      const facilityPhoneNumber = props.row.original.facilityPhoneNumber;
      const facilityAddress = props.row.original.facilityAddress;
      return (
        <div class="flex items-center justify-between pr-1">
          {facilityName}
          <Popover>
            <PopoverTrigger
              as={(props: PopoverTriggerProps) => (
                <Button variant="ghost" size={"icon"} {...props}>
                  <ContactRound size={18} />
                </Button>
              )}
            />
            <PopoverContent class="w-80">
              <div class="grid gap-4">
                <PopoverTitle class="text-sm font-semibold">
                  Facility details
                </PopoverTitle>
                <PopoverDescription class="space-y-2">
                  <div class="flex items-center gap-x-2">
                    <Mail size={18} />
                    <p class="text-sm">{facilityEmail}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Phone size={18} />
                    <p class="text-sm">{facilityPhoneNumber}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <MapPin size={18} />
                    <p class="text-sm">{facilityAddress}</p>
                  </div>
                </PopoverDescription>
              </div>
            </PopoverContent>
          </Popover>
        </div>
      );
    },
  },
  {
    accessorKey: "actions",
    header: "Actions",
    cell: (props) => {
      const startDate = new Date(props.row.original.startTime);
      const twoWeeksFromNowDate = new Date();

      startDate.setHours(0, 0, 0, 0);
      twoWeeksFromNowDate.setHours(0, 0, 0, 0);
      twoWeeksFromNowDate.setDate(twoWeeksFromNowDate.getDate() + 14);

      const [changeRequestDialogOpen, setChangeRequestDialogOpen] =
        createSignal(false);

      const [
        existingChangeEventRequestDialogOpen,
        setExistingChangeEventRequestDialogOpen,
      ] = createSignal(false);

      function formatTime12Hour(utcTimeString: string) {
        const date = new Date(utcTimeString);

        let hours = date.getHours();
        const minutes = date.getMinutes().toString().padStart(2, "0");

        const ampm = hours >= 12 ? "PM" : "AM";

        hours = hours % 12;
        hours = hours ? hours : 12;

        return `${hours}:${minutes} ${ampm}`;
      }

      const form = createForm(() => ({
        defaultValues: {
          location: props.row.original.location,
          address: props.row.original.address,
          date: startDate,
          startTime: formatTime12Hour(props.row.original.startTime),
          endTime: formatTime12Hour(props.row.original.endTime),
          maxAttendees: props.row.original.maxAttendees,
          latlng: {
            lat: props.row.original.latitude,
            lng: props.row.original.longitude,
          },
          changeReason: "",
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

          const postChangeEventRequestPayload: PostChangeEventRequestPayload = {
            location: value.location,
            address: value.address,
            startTime: combineDateTime(value.date, value.startTime),
            endTime: combineDateTime(value.date, value.endTime),
            maxAttendees: value.maxAttendees,
            latitude: value.latlng.lat,
            longitude: value.latlng.lng,
            changeReason: value.changeReason,
            eventId: props.row.original.id,
          };

          const response = await postChangeEventRequest(
            postChangeEventRequestPayload
          );
          response.match(
            () => {
              showSuccessToast({
                successTitle: "Change event request posted",
              });
              setChangeRequestDialogOpen(false);
            },
            (error) => {
              if (error.message === "EXISTING_CHANGE_EVENT_REQUEST") {
                setExistingChangeEventRequestDialogOpen(true);
              } else {
                showErrorToast({
                  errorTitle:
                    "Error posting change event request. Please try again.",
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

      const maxAttendees = [50, 100, 150, 200, 250, 300].filter(
        (number) => number >= props.row.original.currentAttendees
      );
      const maxAttendeesSchema = z
        .number()
        .refine((value) => maxAttendees.includes(value), {
          message: "Max attendees is required",
        });

      const latlngSchema = z.object({
        lat: z.number(),
        lng: z.number(),
      });

      const changeReasonSchema = z
        .string()
        .min(1, "Change reason is required")
        .max(256, "Change reason must be at most 256 characters");

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
          center: {
            lat: props.row.original.latitude,
            lng: props.row.original.longitude,
          },
          zoom: 18,
          mapId: "f7a7a21c7ed4070e",
          draggableCursor: "pointer",
        });

        infoWindow = new InfoWindow();

        marker = new AdvancedMarkerElement({
          map,
          position: {
            lat: props.row.original.latitude,
            lng: props.row.original.longitude,
          },
          title: `Latitude: ${props.row.original.latitude}<br>Longitude: ${props.row.original.longitude}`,
          gmpClickable: true,
          gmpDraggable: true,
        });

        infoWindow?.setContent(
          `<p class="text-slate-600">${marker?.title}</p>`
        );

        map.addListener("click", (mapClickEvent: google.maps.MapMouseEvent) => {
          infoWindow?.close();
          marker!.position = mapClickEvent.latLng;
          marker!.title = `Latitude: ${mapClickEvent?.latLng?.lat()}<br>Longitude: ${mapClickEvent?.latLng?.lng()}`;
          infoWindow?.setContent(
            `<p class="text-slate-600">${marker?.title}</p>`
          );
          form.setFieldValue("latlng", {
            lat: mapClickEvent?.latLng?.lat() ?? 0,
            lng: mapClickEvent?.latLng?.lng() ?? 0,
          });
          form.validateField("latlng", "change");
        });

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
              lat: markerDragEvent?.latLng?.lat() ?? 0,
              lng: markerDragEvent?.latLng?.lng() ?? 0,
            });
            form.validateField("latlng", "change");
          }
        );
      }

      createEffect(() => {
        if (changeRequestDialogOpen()) {
          initialiseMap();
        }
      });

      return (
        <div>
          <Show
            when={startDate >= twoWeeksFromNowDate}
            fallback={
              <Button variant="ghost" size="icon" disabled={true}>
                <PencilOff size={18} />
              </Button>
            }
          >
            <Dialog
              open={changeRequestDialogOpen()}
              onOpenChange={setChangeRequestDialogOpen}
            >
              <DialogTrigger
                as={(props: DialogTriggerProps) => (
                  <Button variant="ghost" size="icon" {...props}>
                    <Pencil size={18} />
                  </Button>
                )}
              />
              <DialogContent
                class="max-w-5xl max-h-[40rem] overflow-y-auto"
                onPointerDownOutside={(event) => event.preventDefault()}
                onFocusOutside={(event) => event.preventDefault()}
                onInteractOutside={(event) => event.preventDefault()}
              >
                <DialogHeader>
                  <DialogTitle>Request Event Change</DialogTitle>
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
                              validationState={hasError() ? "invalid" : "valid"}
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
                                    weeks: 2,
                                  }) as unknown as DateValue
                                }
                                max={
                                  today("Asia/Kuala_Lumpur").add({
                                    months: 3,
                                  }) as unknown as DateValue
                                }
                                timeZone="Asia/Kuala_Lumpur"
                                name={field().name}
                                value={[
                                  new CalendarDate(
                                    field().state.value.getFullYear(),
                                    field().state.value.getMonth() + 1,
                                    field().state.value.getDate()
                                  ) as unknown as DateValue,
                                ]}
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
                                                      each={context().weekDays}
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
                                                  <Index each={context().weeks}>
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
                                                        <Index each={months()}>
                                                          {(month) => (
                                                            <DatePickerTableCell
                                                              value={
                                                                month().value
                                                              }
                                                            >
                                                              <DatePickerTableCellTrigger>
                                                                {month().label}
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

                    <div class="pt-2">
                      <form.Field
                        name="changeReason"
                        validators={{ onChange: changeReasonSchema }}
                        children={(field) => {
                          const hasError = createMemo(() => {
                            return (
                              field().state.meta.errors.length > 0 &&
                              field().state.meta.isTouched
                            );
                          });

                          return (
                            <TextFieldRoot
                              class="w-full"
                              name={field().name}
                              validationState={hasError() ? "invalid" : "valid"}
                              value={field().state.value}
                              onBlur={field().handleBlur}
                              onChange={field().handleChange}
                            >
                              <TextArea placeholder="Enter change reason here." />
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

                    <div>
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
                                class="w-full h-[24rem]"
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
                  </div>
                  <DialogFooter>
                    <Button type="submit">Submit Change Request</Button>
                  </DialogFooter>
                </form>
              </DialogContent>
            </Dialog>
            <AlertDialog
              open={existingChangeEventRequestDialogOpen()}
              onOpenChange={setExistingChangeEventRequestDialogOpen}
            >
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle class="text-base font-medium">
                    You already have an existing change event proposal for this
                    event.
                  </AlertDialogTitle>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogAction>Ok</AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          </Show>
        </div>
      );
    },
  },
];
