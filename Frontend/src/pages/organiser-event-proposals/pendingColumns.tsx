import { ColumnDef } from "@tanstack/solid-table";
import { NewEventProposal } from "@/types/new-event-proposal";
import { createEffect, createMemo, createSignal } from "solid-js";
import {
  ContactRound,
  Mail,
  Phone,
  MapPinned,
  MapPin,
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

export const pendingColumns: ColumnDef<NewEventProposal>[] = [
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
    header: "Date",
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
                    <Mail size={18} class="shrink-0" />
                    <p class="text-sm">{facilityEmail}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Phone size={18} class="shrink-0" />
                    <p class="text-sm">{facilityPhoneNumber}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <MapPin size={18} class="shrink-0" />
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
];
