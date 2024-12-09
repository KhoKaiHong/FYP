import { Button } from "@/components/ui/button";
import { LogOut, Hospital, UserRoundCog } from "lucide-solid";
import { useNavigate } from "@solidjs/router";

function AdminActions() {
  const navigate = useNavigate();

  return (
    <div class="grid grid-cols-1 md:grid-cols-2 justify-center items-center gap-6">
      <Button variant="outline" class="w-full h-16 border-2 border-brand">
        <div class="flex items-center justify-center gap-x-2">
          <Hospital size={24} />
          <p class="text-base">Add New Facility</p>
        </div>
      </Button>
      <Button variant="outline" class="w-full h-16 border-2 border-brand">
        <div class="flex items-center justify-center gap-x-2">
          <UserRoundCog size={24} />
          <p class="text-base">Add New Admin</p>
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

export default AdminActions;
