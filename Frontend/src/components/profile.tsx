import type { DropdownMenuSubTriggerProps } from "@kobalte/core/dropdown-menu";
import { useColorMode } from "@kobalte/core";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar";
import { Sun, Moon, LaptopMinimal } from "lucide-solid";

function ProfileDropDown() {
  const { colorMode, setColorMode } = useColorMode();

  return (
    <DropdownMenu placement="bottom">
      <DropdownMenuTrigger
        as={(props: DropdownMenuSubTriggerProps) => (
          <Button variant="ghost" size="icon" {...props}>
            <Avatar>
              <AvatarImage src="https://github.com/sek-consulting.png" />
              <AvatarFallback>G</AvatarFallback>
            </Avatar>
          </Button>
        )}
      />
      <DropdownMenuContent class="w-32">
        <DropdownMenuItem
          onSelect={() => setColorMode("light")}
          class="gap-x-2"
        >
          <Sun size={16} />
          <span>Light</span>
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={() => setColorMode("dark")} class="gap-x-2">
          <Moon size={16} />
          <span>Dark</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          onSelect={() => setColorMode("system")}
          class="gap-x-2"
        >
          <LaptopMinimal size={16} />
          <span>System</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export default ProfileDropDown;
