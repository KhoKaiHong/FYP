import { ColumnDef } from "@tanstack/solid-table";
import { ChangeEventRequest } from "@/types/change-event-request";
import { createEffect, createMemo, createSignal } from "solid-js";
import {
  ContactRound,
  Mail,
  Phone,
  Check,
  X,
  MapPinned,
  ArrowUpDown,
  Ellipsis,
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
import type { AlertDialogTriggerProps } from "@kobalte/core/alert-dialog";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogClose,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import {
  TextFieldErrorMessage,
  TextFieldRoot,
} from "@/components/ui/text-field";
import { TextArea } from "@/components/ui/textarea";
import { createForm } from "@tanstack/solid-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";
import {
  UpdateChangeEventRequestPayload,
} from "@/types/change-event-request";
import { facilityUpdateChangeEventRequest } from "@/api/change-event-request";
import showSuccessToast from "@/components/success-toast";
import showErrorToast from "@/components/error-toast";

export const pendingColumns: ColumnDef<ChangeEventRequest>[] = [
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
    accessorKey: "maxAttendees",
    header: "Max Attendees",
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
    accessorKey: "organiserName",
    header: "Organiser",
    cell: (props) => {
      const organiserName = props.row.original.organiserName;
      const organiserEmail = props.row.original.organiserEmail;
      const organiserPhoneNumber = props.row.original.organiserPhoneNumber;
      return (
        <div class="flex items-center justify-between pr-1">
          {organiserName}
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
                  Organiser details
                </PopoverTitle>
                <PopoverDescription class="space-y-2">
                  <div class="flex items-center gap-x-2">
                    <Mail size={18} />
                    <p class="text-sm">{organiserEmail}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Phone size={18} />
                    <p class="text-sm">{organiserPhoneNumber}</p>
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
    accessorKey: "changeReason",
    header: "Reason",
    cell: (props) => {
      return (
        <div class="flex items-center justify-between pr-1">
          <Popover>
            <PopoverTrigger
              as={(props: PopoverTriggerProps) => (
                <Button variant="ghost" size={"icon"} {...props}>
                  <Ellipsis size={18} />
                </Button>
              )}
            />
            <PopoverContent class="w-80">
              <div class="grid gap-4">
                <PopoverTitle class="text-sm font-semibold">
                  Change Reason
                </PopoverTitle>
                <PopoverDescription>
                  <p class="text-sm">{props.row.original.changeReason}</p>
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
      const [rejectDialogOpen, setRejectDialogOpen] = createSignal(false);

      const form = createForm(() => ({
        defaultValues: {
          rejectionReason: "",
        },
        onSubmit: async ({ value }) => {
          setRejectDialogOpen(false);
          const updateChangeEventRequestPayload: UpdateChangeEventRequestPayload = {
            id: props.row.original.id,
            status: "Rejected",
            rejectionReason: value.rejectionReason,
          };
          const response = await facilityUpdateChangeEventRequest(
            updateChangeEventRequestPayload
          );
          response.match(
            () => {
              showSuccessToast({ successTitle: "Change event request rejected" });
            },
            (error) => {
              showErrorToast({
                errorTitle:
                  "Error performing change event request rejection. Please try again.",
                error: error,
              });
            }
          );
          props.table.options.meta?.performRefetch();
        },
        validatorAdapter: zodValidator(),
      }));

      const rejectionReasonSchema = z
        .string()
        .min(1, "Rejection reason is required")
        .max(256, "Rejection reason must be at most 256 characters");

      async function approveChangeEventRequest() {
        const updateChangeEventRequestPayload: UpdateChangeEventRequestPayload = {
          id: props.row.original.id,
          status: "Approved",
        };
        const response = await facilityUpdateChangeEventRequest(
          updateChangeEventRequestPayload
        );
        response.match(
          () => {
            showSuccessToast({ successTitle: "Change event request approved" });
          },
          (error) => {
            showErrorToast({
              errorTitle:
                "Error performing change event request approval. Please try again.",
              error: error,
            });
          }
        );
        props.table.options.meta?.performRefetch();
      }

      return (
        <div class="flex items-center gap-1">
          <AlertDialog>
            <AlertDialogTrigger
              as={(props: AlertDialogTriggerProps) => (
                <Button variant="ghost" size={"icon"} {...props}>
                  <Check size={18} strokeWidth={3} />
                </Button>
              )}
            />
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>
                  Are you sure to approve this change event request?
                </AlertDialogTitle>
                <AlertDialogDescription>
                  The updated event details will be published to the public.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogClose>Cancel</AlertDialogClose>
                <AlertDialogAction
                  onClick={async () => {
                    await approveChangeEventRequest();
                  }}
                >
                  Continue
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
          <Dialog open={rejectDialogOpen()} onOpenChange={setRejectDialogOpen}>
            <DialogTrigger
              as={(props: DialogTriggerProps) => (
                <Button variant="ghost" size={"icon"} {...props}>
                  <X size={18} strokeWidth={3} />
                </Button>
              )}
            />
            <DialogContent class="max-w-xl">
              <DialogHeader>
                <DialogTitle>Reject Change Event Request</DialogTitle>
              </DialogHeader>
              <form
                onSubmit={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  form.handleSubmit();
                }}
              >
                <div class="py-2">
                  <form.Field
                    name="rejectionReason"
                    validators={{ onChange: rejectionReasonSchema }}
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
                          <TextArea placeholder="Enter rejection reason here." />
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
                <DialogFooter>
                  <Button type="submit">Reject</Button>
                </DialogFooter>
              </form>
            </DialogContent>
          </Dialog>
        </div>
      );
    },
  },
];
