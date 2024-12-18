import { Button } from "@/components/ui/button";
import {
  ClipboardPlus,
  LogOut,
  ClipboardList,
  ClipboardCheck,
  ClipboardPen,
} from "lucide-solid";
import { useNavigate } from "@solidjs/router";
import { useUser } from "@/context/user-context";
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

function OrganiserActions() {
  const navigate = useNavigate();
  const { logoutAll } = useUser();

  return (
    <div class="grid grid-cols-1 md:grid-cols-2 justify-center items-center gap-6">
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/new-event-proposal")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardPlus size={24} />
          <p class="text-base">New Event Proposal</p>
        </div>
      </Button>
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/organiser-event-proposals")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardList size={24} />
          <p class="text-base">My Event Proposals</p>
        </div>
      </Button>
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/organiser-events")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardCheck size={24} />
          <p class="text-base">My Events</p>
        </div>
      </Button>
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/organiser-change-requests")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardPen size={24} />
          <p class="text-base">My Change Requests</p>
        </div>
      </Button>
      <AlertDialog>
        <AlertDialogTrigger
          as={(props: AlertDialogTriggerProps) => (
            <Button
              variant="destructive"
              class="w-full h-16 md:col-span-2"
              {...props}
            >
              <div class="flex items-center justify-center gap-x-2">
                <LogOut size={24} />
                <p class="text-base">Log Out from all Devices</p>
              </div>
            </Button>
          )}
        />
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This will log you out from all devices.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogClose>Cancel</AlertDialogClose>
            <AlertDialogAction onClick={async () => await logoutAll()}>
              Continue
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}

export default OrganiserActions;
