import { ColumnDef } from "@tanstack/solid-table";
import { createEffect, createMemo, createSignal } from "solid-js";
import { ArrowUpDown, MapPinned } from "lucide-solid";
import { Button } from "@/components/ui/button";
import { DonationHistory } from "@/types/donation-history";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { DialogTriggerProps } from "@kobalte/core/dialog";

export const donationHistoryColumns: ColumnDef<DonationHistory>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "donatedAt",
    header: (props) => {
      return (
        <Button
          variant="ghost"
          onClick={() =>
            props.column.toggleSorting(props.column.getIsSorted() === "asc")
          }
          class="px-1"
        >
          Donated At
          <ArrowUpDown class="ml-2 h-4 w-4" />
        </Button>
      );
    },
    cell: (props) => {
      const donatedAt = createMemo(() => {
        const date = new Date(props.row.original.createdAt);
        return date.toLocaleString("en-MY", {
          timeZone: "Asia/Kuala_Lumpur",
        });
      });
      return <div>{donatedAt()}</div>;
    },
    sortingFn: (rowA, rowB) => {
      const dateA = new Date(rowA.original.createdAt);
      const dateB = new Date(rowB.original.createdAt);
      return dateA.getTime() - dateB.getTime();
    },
  },
  {
    accessorKey: "eventLocation",
    header: "Event Location",
    cell: (props) => {
      const location = props.row.original.eventLocation;
      const address = props.row.original.eventAddress;
      const latitude = props.row.original.eventLatitude;
      const longitude = props.row.original.eventLongitude;

      if (!location || !address || !latitude || !longitude) {
        return <div class="h-9 flex items-center">No event</div>;
      }

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
          center: { lat: latitude!, lng: longitude! },
          zoom: 18,
          mapId: "f7a7a21c7ed4070e",
        });

        infoWindow = new InfoWindow();

        marker = new AdvancedMarkerElement({
          map,
          position: { lat: latitude!, lng: longitude! },
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
    filterFn: (row, _columnId, filterValue) => {
      return (row.original.eventLocation ?? "No event")
        .toLowerCase()
        .includes(filterValue.toLowerCase());
    },
  },
];
