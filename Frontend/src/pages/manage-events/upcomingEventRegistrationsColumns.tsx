import { ColumnDef } from "@tanstack/solid-table";
import { createMemo } from "solid-js";
import {
  ContactRound,
  Mail,
  Phone,
  ArrowUpDown,
  IdCard,
  Droplet,
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
import { EventRegistration } from "@/types/event-registration";

export const upcomingEventRegistrationsColumns: ColumnDef<EventRegistration>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "userName",
    header: "Name",
    cell: (props) => {
      const userName = props.row.original.userName;
      const userId = props.row.original.userId;
      const userEmail = props.row.original.userEmail;
      const userPhoneNumber = props.row.original.userPhoneNumber;
      const userBloodType = props.row.original.userBloodType;
      return (
        <div class="flex items-center justify-between pr-1">
          {userName}
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
                  Attendee details
                </PopoverTitle>
                <PopoverDescription class="space-y-2">
                  <div class="flex items-center gap-x-2">
                    <IdCard size={18} class="shrink-0" />
                    <p class="text-sm">{userId}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Mail size={18} class="shrink-0" />
                    <p class="text-sm">{userEmail}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Phone size={18} class="shrink-0" />
                    <p class="text-sm">{userPhoneNumber}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Droplet size={18} class="shrink-0" />
                    <p class="text-sm">{userBloodType}</p>
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
    accessorKey: "userIcNumber",
    header: "IC Number",
  },
  {
    accessorKey: "registeredAt",
    header: (props) => {
      return (
        <Button
          variant="ghost"
          onClick={() =>
            props.column.toggleSorting(props.column.getIsSorted() === "asc")
          }
          class="px-1"
        >
          Registered At
          <ArrowUpDown class="ml-2 h-4 w-4" />
        </Button>
      );
    },
    cell: (props) => {
      const registeredAt = createMemo(() => {
        const date = new Date(props.row.original.registeredAt);
        return date.toLocaleString("en-MY", {
          timeZone: "Asia/Kuala_Lumpur",
        });
      });
      return <div>{registeredAt()}</div>;
    },
    sortingFn: (rowA, rowB) => {
      const dateA = new Date(rowA.original.registeredAt);
      const dateB = new Date(rowB.original.registeredAt);
      return dateA.getTime() - dateB.getTime();
    },
  },
];
