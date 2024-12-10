import { Button } from "@/components/ui/button";
import { ClipboardPlus, LogOut, ClipboardList, ClipboardPen } from "lucide-solid";
import { useNavigate } from "@solidjs/router";

function FacilityActions() {
  const navigate = useNavigate();

  return (
    <div class="grid grid-cols-1 md:grid-cols-3 justify-center items-center gap-6">
      <Button variant="outline" class="w-full h-16 border-2 border-brand" onClick={() => navigate("/manage-event-proposals")}>
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardPlus size={24} />
          <p class="text-base">Manage Event Proposals</p>
        </div>
      </Button>
      <Button variant="outline" class="w-full h-16 border-2 border-brand">
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardList size={24} />
          <p class="text-base">Manage Events</p>
        </div>
      </Button>
      <Button variant="outline" class="w-full h-16 border-2 border-brand">
        <div class="flex items-center justify-center gap-x-2">
          <ClipboardPen size={24} />
          <p class="text-base">Manage Change Requests</p>
        </div>
      </Button>
      <Button variant="destructive" class="w-full h-16 md:col-span-3">
        <div class="flex items-center justify-center gap-x-2">
          <LogOut size={24} />
          <p class="text-base">Log Out from all Devices</p>
        </div>
      </Button>
    </div>
  );
}

export default FacilityActions;
