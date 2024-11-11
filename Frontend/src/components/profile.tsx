import type { DropdownMenuSubTriggerProps } from "@kobalte/core/dropdown-menu";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useUser } from "@/context/user-context";
import {
  Hospital,
  UserRound,
  UserRoundCog,
  UsersRound,
  UserRoundX,
  LoaderCircle,
  CircleUserRound,
  LogOut,
  LogIn,
  PanelTop,
  UserRoundPlus,
} from "lucide-solid";

function ProfileDropDown() {
  const { user, role, isLoading } = useUser();

  function DropDownContent() {
    switch (role()) {
      case "User":
        return (
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-2 py-2">
              <UserRound size={42} />
              <p class="font-medium">{user().name}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <CircleUserRound size={18} />
              <p>Account</p>
            </DropdownMenuItem>
            <DropdownMenuItem class="gap-x-2">
              <PanelTop size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        );
      case "Facility":
        return (
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-4 py-2">
              <Hospital size={42} />
              <p>{user().name}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <CircleUserRound size={18} />
              <p>Account</p>
            </DropdownMenuItem>
            <DropdownMenuItem class="gap-x-2">
              <PanelTop size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        );
      case "Organiser":
        return (
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-4 py-2">
              <UsersRound size={42} />
              <p>{user().name}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <CircleUserRound size={18} />
              <p>Account</p>
            </DropdownMenuItem>
            <DropdownMenuItem class="gap-x-2">
              <PanelTop size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        );
      case "Admin":
        return (
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-4 py-2">
              <UserRoundCog size={42} />
              <p>{user().name}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <CircleUserRound size={18} />
              <p>Account</p>
            </DropdownMenuItem>
            <DropdownMenuItem class="gap-x-2">
              <PanelTop size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        );
      default:
        return (
          <DropdownMenuGroup>
            <div class="flex flex-col items-center py-2">
              <UserRoundX size={42} />
              <p>Guest</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2">
              <LogIn size={18} />
              <p>Log In</p>
            </DropdownMenuItem>
            <DropdownMenuItem class="gap-x-2">
              <UserRoundPlus size={18} />
              <p>Register</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        );
    }
  }

  return (
    <DropdownMenu placement="bottom">
      <DropdownMenuTrigger
        as={(props: DropdownMenuSubTriggerProps) => (
          <Button variant="ghost" size="icon" disabled={isLoading()} {...props}>
            {isLoading() ? (
              <LoaderCircle size={18} class="animate-spin" />
            ) : role() === "User" ? (
              <UserRound size={18} />
            ) : role() === "Facility" ? (
              <Hospital size={18} />
            ) : role() === "Organiser" ? (
              <UsersRound size={18} />
            ) : role() === "Admin" ? (
              <UserRoundCog size={18} />
            ) : (
              <UserRoundX size={18} />
            )}
          </Button>
        )}
      />
      <DropdownMenuContent class="w-60">
        <DropDownContent />
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export default ProfileDropDown;
