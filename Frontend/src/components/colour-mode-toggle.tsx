import type { DropdownMenuSubTriggerProps } from "@kobalte/core/dropdown-menu";
import { useColorMode } from "@kobalte/core"
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

import { Sun, Moon, LaptopMinimal } from "lucide-solid";

export function ColourModeToggle() {
  const { colorMode, setColorMode } = useColorMode()

  return (
    <DropdownMenu placement="bottom">
      <DropdownMenuTrigger
        as={(props: DropdownMenuSubTriggerProps) => (
          <Button variant="ghost" {...props}>
            {colorMode() === "light" ? <Sun size={16} /> : <Moon size={16} />}
          </Button>
        )}
      />
      <DropdownMenuContent class="w-56">
        <DropdownMenuItem onSelect={() => setColorMode("light")}>
          <Sun size={16}/>
          <span>Light</span>
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={() => setColorMode("dark")}>
          <Moon size={16}/>
          <span>Dark</span>
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={() => setColorMode("system")}>
          <LaptopMinimal size={16}/>
          <span>System</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
