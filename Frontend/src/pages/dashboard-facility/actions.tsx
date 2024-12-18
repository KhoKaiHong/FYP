import { Button } from "@/components/ui/button";
import {
  ClipboardPlus,
  LogOut,
  ClipboardList,
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

function FacilityActions() {
  const navigate = useNavigate();
  const { logoutAll } = useUser();

  return (
    <div class="grid grid-cols-1 md:grid-cols-3 justify-center items-center gap-6">
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/manage-event-proposals")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardPlus size={24} />
          <p class="text-base">Manage Event Proposals</p>
        </div>
      </Button>
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/manage-events")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardList size={24} />
          <p class="text-base">Manage Events</p>
        </div>
      </Button>
      <Button
        variant="outline"
        class="w-full h-16 border-2 border-brand"
        onClick={() => navigate("/manage-change-requests")}
      >
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardPen size={24} />
          <p class="text-base">Manage Change Requests</p>
        </div>
      </Button>
      <AlertDialog>
        <AlertDialogTrigger
          as={(props: AlertDialogTriggerProps) => (
            <Button
              variant="destructive"
              class="w-full h-16 md:col-span-3"
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

export default FacilityActions;
