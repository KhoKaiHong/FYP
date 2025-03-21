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
  UserRoundPlus,
} from "lucide-solid";
import { Switch, Match } from "solid-js";
import { useNavigate } from "@solidjs/router";

function ProfileDropDown() {
  const { isAuthenticated, user, isLoading, logout } = useUser();
  const navigate = useNavigate();

  function DropDownContent() {
    return (
      <Switch
        fallback={
          <DropdownMenuGroup>
            <div class="flex flex-col items-center py-2">
              <UserRoundX size={42} />
              <p>Guest</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2" onClick={() => navigate("/login", { resolve: false })}>
              <LogIn size={18} />
              <p>Log In</p>
            </DropdownMenuItem>
            <DropdownMenuItem class="gap-x-2" onClick={() => navigate("/register", { resolve: false })}>
              <UserRoundPlus size={18} />
              <p>Register</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        }
      >
        <Match when={isAuthenticated() && user() != null && user()?.role === "User"}>
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-2 py-2">
              <UserRound size={42} />
              <p class="font-medium">{user()?.name || "Loading..."}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2" onClick={() => navigate("/user-dashboard")}>
              <CircleUserRound size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              class="gap-x-2"
              onClick={async () => await logout()}
            >
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </Match>
        <Match when={isAuthenticated() && user() != null && user()?.role === "Facility"}>
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-4 py-2">
              <Hospital size={42} />
              <p>{user()?.name || "Loading..."}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2" onClick={() => navigate("/facility-dashboard", { resolve: false })}>
              <CircleUserRound size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              class="gap-x-2"
              onClick={async () => await logout()}
            >
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </Match>
        <Match when={isAuthenticated() && user() != null && user()?.role === "Organiser"}>
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-4 py-2">
              <UsersRound size={42} />
              <p>{user()?.name || "Loading..."}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2" onClick={() => navigate("/organiser-dashboard", { resolve: false })}>
              <CircleUserRound size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              class="gap-x-2"
              onClick={async () => await logout()}
            >
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </Match>
        <Match when={isAuthenticated() && user() != null && user()?.role === "Admin"}>
          <DropdownMenuGroup>
            <div class="flex flex-col items-center gap-y-4 py-2">
              <UserRoundCog size={42} />
              <p>{user()?.name || "Loading..."}</p>
            </div>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="gap-x-2" onClick={() => navigate("/admin-dashboard", { resolve: false })}>
              <CircleUserRound size={18} />
              <p>Dashboard</p>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              class="gap-x-2"
              onClick={async () => await logout()}
            >
              <LogOut size={18} />
              <p>Log Out</p>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </Match>
      </Switch>
    );
  }

  return (
    <DropdownMenu placement="bottom">
      <DropdownMenuTrigger
        as={(props: DropdownMenuSubTriggerProps) => (
          <Button variant="ghost" size="icon" disabled={isLoading()} {...props}>
            {isLoading() ? (
              <LoaderCircle size={18} class="animate-spin" />
            ) : user()?.role === "User" ? (
              <UserRound size={18} />
            ) : user()?.role === "Facility" ? (
              <Hospital size={18} />
            ) : user()?.role === "Organiser" ? (
              <UsersRound size={18} />
            ) : user()?.role === "Admin" ? (
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
