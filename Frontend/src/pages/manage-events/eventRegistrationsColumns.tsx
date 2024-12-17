import { ColumnDef } from "@tanstack/solid-table";
import {
  createMemo,
  Show,
} from "solid-js";
import {
  ContactRound,
  Mail,
  Phone,
  ArrowUpDown,
  X,
  Check,
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
import showErrorToast from "@/components/error-toast";
import { updateRegistration } from "@/api/event-registration";
import { UpdateRegistrationPayload } from "@/types/event-registration";
import showSuccessToast from "@/components/success-toast";
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
import { AlertDialogTriggerProps } from "@kobalte/core/alert-dialog";

export const eventRegistrationsColumns: ColumnDef<EventRegistration>[] = [
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
                    <IdCard size={18} />
                    <p class="text-sm">{userId}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Mail size={18} />
                    <p class="text-sm">{userEmail}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Phone size={18} />
                    <p class="text-sm">{userPhoneNumber}</p>
                  </div>
                  <div class="flex items-center gap-x-2">
                    <Droplet size={18} />
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
  {
    accessorKey: "attendance",
    header: "Attendance",
    cell: (props) => {
      async function setAsAbsent() {
        const updateRegistrationPayload: UpdateRegistrationPayload = {
          registrationId: props.row.original.id,
          status: "Absent",
        };
        const response = await updateRegistration(updateRegistrationPayload);
        response.match(
          () => {
            showSuccessToast({ successTitle: "Attendee marked as absent" });
          },
          (error) => {
            showErrorToast({
              errorTitle: "Error performing this action. Please try again.",
              error: error,
            });
          }
        );
        props.table.options.meta?.performRefetch();
      }

      async function setAsAttended() {
        const updateRegistrationPayload: UpdateRegistrationPayload = {
          registrationId: props.row.original.id,
          status: "Attended",
        };
        const response = await updateRegistration(updateRegistrationPayload);
        response.match(
          () => {
            showSuccessToast({ successTitle: "Attendee marked as present" });
          },
          (error) => {
            showErrorToast({
              errorTitle: "Error performing this action. Please try again.",
              error: error,
            });
          }
        );
        props.table.options.meta?.performRefetch();
      }

      return (
        <Show
          when={props.row.original.status === "Registered"}
          fallback={<div>{props.row.original.status}</div>}
        >
          <div class="flex items-center gap-1">
            <AlertDialog>
              <AlertDialogTrigger
                as={(props: AlertDialogTriggerProps) => (
                  <Button variant="ghost" size={"icon"} {...props}>
                    <X size={18} strokeWidth={3} />
                  </Button>
                )}
              />
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>
                    Are you sure to set this attendee as absent?
                  </AlertDialogTitle>
                  <AlertDialogDescription>
                    The attendee will be notified of this action.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogClose>Cancel</AlertDialogClose>
                  <AlertDialogAction
                    onClick={async () => {
                      await setAsAbsent();
                    }}
                  >
                    Continue
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
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
                    Are you sure to set this attendee as present?
                  </AlertDialogTitle>
                  <AlertDialogDescription>
                    The attendee will be notified of this action.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogClose>Cancel</AlertDialogClose>
                  <AlertDialogAction
                    onClick={async () => {
                      await setAsAttended();
                    }}
                  >
                    Continue
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          </div>
        </Show>
      );
    },
  },
];
