import { Menu } from "lucide-solid";
import { createSignal, For, createEffect } from "solid-js";
import { Button } from "@/components/ui/button";
import ColourModeToggle from "@/components/colour-mode-toggle";
import Logo from "@/components/logo";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuItemLabel,
  NavigationMenuLink,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu";
import { ChevronUp, ChevronDown, X } from "lucide-solid";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";

function Navbar() {
  const [isOpen, setIsOpen] = createSignal(false);

  const toggleMenu = () => {
    setIsOpen(!isOpen());
  };

  const navItems = [
    {
      name: "Home",
      href: "#",
      subpages: null,
    },
    {
      name: "Events",
      href: "#",
      subpages: null,
    },
    {
      name: "Join Us",
      href: null,
      subpages: [
        { name: "Become an Organiser", href: "#" },
        { name: "Organise an Event", href: "#" },
      ],
    },
    {
      name: "Resources",
      href: null,
      subpages: [
        { name: "Blood Donation Guide", href: "#" },
        { name: "Myths & Facts", href: "#" },
        { name: "Donor Privileges", href: "#" },
        { name: "Event Organisation", href: "#" },
        { name: "Blood Donation Statistics", href: "#" },
      ],
    },
    {
      name: "Contact",
      href: null,
      subpages: [
        { name: "Blood Donation Facilities", href: "#" },
        { name: "Provide Feedback", href: "#" },
      ],
    },
  ] as const;

  return (
    <div class="w-full border-b bg-background z-50 sticky top-0 left-0 right-0">
      <div class="flex justify-between h-16 px-6">
        <div class="flex items-center">
          <Button variant="none" class="px-2 space-x-3">
            <Logo />
            <span class="hidden ml-2 text-lg font-semibold sm:block">
              MyBloodConnect
            </span>
          </Button>
        </div>

        <div class="hidden lg:flex items-center">
          <NavigationMenu>
            <For each={navItems}>
              {(item) => (
                <>
                  {item.href ? (
                    <NavigationMenuTrigger
                      as="a"
                      href={item.href}
                      class="transition-[box-shadow,background-color] focus-visible:outline-none focus-visible:ring-1.5px focus-visible:ring-ring data-[expanded]:bg-accent"
                    >
                      {item.name}
                    </NavigationMenuTrigger>
                  ) : (
                    <NavigationMenuItem>
                      <NavigationMenuTrigger class="transition-[box-shadow,background-color] focus-visible:outline-none focus-visible:ring-1.5px focus-visible:ring-ring data-[expanded]:bg-accent">
                        {item.name}
                      </NavigationMenuTrigger>
                      <NavigationMenuContent class="grid w-[200px] gap-3 p-4 md:w-[300px] md:grid-cols-1 lg:w-[400px]">
                        <For each={item.subpages}>
                          {(subpage) => (
                            <NavigationMenuLink
                              href={subpage.href}
                              class="block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-[box-shadow,background-color] duration-200 hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent focus-visible:text-accent-foreground focus-visible:outline-none focus-visible:ring-[1.5px] focus-visible:ring-ring"
                            >
                              <NavigationMenuItemLabel class="text-sm font-medium leading-none">
                                {subpage.name}
                              </NavigationMenuItemLabel>
                            </NavigationMenuLink>
                          )}
                        </For>
                      </NavigationMenuContent>
                    </NavigationMenuItem>
                  )}
                </>
              )}
            </For>
          </NavigationMenu>
        </div>

        <div class="flex items-center">
          <ColourModeToggle />
          <Button
            variant="ghost"
            size="icon"
            class="lg:hidden"
            onClick={toggleMenu}
          >
            {isOpen() ? <X size={20} /> : <Menu size={20} />}
            <span class="sr-only">Toggle menu</span>
          </Button>
        </div>
      </div>

      <div
        class={`lg:hidden fixed left-0 right-0 bg-white overflow-hidden transition-all duration-300 ease-in-out z-40 top-20`}
        style={{
          opacity: isOpen() ? "1" : "0",
          visibility: isOpen() ? "visible" : "hidden",
        }}
      >
        <div class="container mx-auto px-4 flex flex-col space-y-4 mt-4 h-full">
          <For each={navItems}>
            {(item) => (
              <div>
                {item.href ? (
                  <Button
                    variant="ghost"
                    as="a"
                    href={item.href}
                    class="w-full justify-start"
                    onClick={() => setIsOpen(false)}
                  >
                    {item.name}
                  </Button>
                ) : (
                  <Collapsible>
                    <CollapsibleTrigger class="text-sm font-medium">
                      {item.name}
                    </CollapsibleTrigger>
                    <CollapsibleContent class="overflow-hidden">
                      <For each={item.subpages}>
                        {(subpage) => (
                          <Button
                            variant="ghost"
                            as="a"
                            href={subpage.href}
                            class="w-full justify-start"
                            onClick={() => setIsOpen(false)}
                          >
                            {subpage.name}
                          </Button>
                        )}
                      </For>
                    </CollapsibleContent>
                  </Collapsible>
                )}
              </div>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}

export default Navbar;
