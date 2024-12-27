import { ColumnDef } from "@tanstack/solid-table";
import { createEffect, createMemo, createResource, createSignal, Show } from "solid-js";
import {
  ContactRound,
  Mail,
  Phone,
  MapPinned,
  ArrowUpDown,
  Eye,
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
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Event } from "@/types/events";
import { EventRegistrationsTable } from "./event-registrations-table";
import showErrorToast from "@/components/error-toast";
import { listRegistrationsByEventId } from "@/api/event-registration";
import { upcomingEventRegistrationsColumns } from "./upcomingEventRegistrationsColumns";

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
      const [registrationsDialogOpen, setRegistrationsDialogOpen] =
        createSignal(false);

      async function fetchRegistrations() {
        try {
          const registrationsResponse = await listRegistrationsByEventId({
            eventId: props.row.original.id,
          });

          return registrationsResponse.match(
            (data) => {
              return data.data.registrations;
            },
            (error) => {
              showErrorToast({
                errorTitle: "Error fetching event registrations.",
                error,
              });
              console.error("Error fetching event registrations.", error);
              return null;
            }
          );
        } catch (err) {
          showErrorToast({
            errorTitle: "Error fetching event registrations.",
            error: { message: "UNKNOWN_ERROR" },
          });
          console.error("Error fetching event registrations.", err);
        }
      }

      const [registrations, { refetch }] = createResource(
        () => registrationsDialogOpen(),
        fetchRegistrations
      );

      return (
        <div class="flex items-center justify-between pr-1">
          <div>
            {props.row.original.currentAttendees} /{" "}
            {props.row.original.maxAttendees}
          </div>
          <Dialog
            open={registrationsDialogOpen()}
            onOpenChange={setRegistrationsDialogOpen}
          >
            <DialogTrigger
              as={(props: DialogTriggerProps) => (
                <Button variant="ghost" size={"icon"} {...props}>
                  <Eye size={18} />
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
                <DialogTitle>View Registrations</DialogTitle>
                <DialogDescription>
                  View attendees registered to the event.
                </DialogDescription>
              </DialogHeader>
              <div>
                <Show when={registrations()}>
                  <EventRegistrationsTable
                    columns={upcomingEventRegistrationsColumns}
                    data={registrations() ?? []}
                    refetch={() => {
                      refetch();
                    }}
                  />
                </Show>
              </div>
            </DialogContent>
          </Dialog>
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
                    <Mail size={18} class="shrink-0" />
                    <p class="text-sm">{organiserEmail}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Phone size={18} class="shrink-0" />
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
];
