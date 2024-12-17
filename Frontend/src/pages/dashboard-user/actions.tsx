import { Button } from "@/components/ui/button";
import { History, LogOut, ClipboardList } from "lucide-solid";
import { useNavigate } from "@solidjs/router";

function UserActions() {
  const navigate = useNavigate();

  return (
    <div class="grid grid-cols-1 md:grid-cols-2 justify-center items-center gap-6">
      <Button variant="outline" class="w-full h-16 border-2 border-brand">
        <div class="flex items-center justify-center gap-x-2">
          <History size={24} />
          <p class="text-base">My Donation History</p>
        </div>
      </Button>
      <Button variant="outline" class="w-full h-16 border-2 border-brand">
        <div class="flex items-center justify-center gap-x-2" onClick={() => navigate("/event-registrations")}>
          <ClipboardList size={24} />
          <p class="text-base">My Event Registrations</p>
        </div>
      </Button>
      <Button variant="destructive" class="w-full h-16 md:col-span-2">
        <div class="flex items-center justify-center gap-x-2">
          <LogOut size={24} />
          <p class="text-base">Log Out from all Devices</p>
        </div>
      </Button>
    </div>
  );
}

export default UserActions;
