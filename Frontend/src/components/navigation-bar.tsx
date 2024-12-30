import { Menu } from "lucide-solid";
import { createSignal, For, createEffect, Show, Switch, Match } from "solid-js";
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
import { ChevronDown, X } from "lucide-solid";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import ProfileDropDown from "@/components/profile";
import { useNavigate } from "@solidjs/router";
import { useUser } from "@/context/user-context";
import UserNotificationDialog from "@/components/user-notification-dialog";
import OrganiserNotificationDialog from "@/components/organiser-notification-dialog";
import FacilityNotificationDialog from "@/components/facility-notification-dialog";
import AdminNotificationDialog from "@/components/admin-notification-dialog";

function Navbar() {
  const [isOpen, setIsOpen] = createSignal(false);
  const navigate = useNavigate();

  const { user } = useUser();

  const toggleMenu = () => {
    setIsOpen(!isOpen());
  };

  createEffect(() => {
    if (isOpen()) {
      document.body.style.overflow = "hidden";
      document.body.style.touchAction = "none";
    } else {
      document.body.style.overflow = "";
      document.body.style.touchAction = "";
    }
  });

  const navItems = [
    {
      name: "Home",
      href: "/",
      subpages: null,
    },
    {
      name: "Events",
      href: "/events",
      subpages: null,
    },
    {
      name: "Join Us",
      href: "#",
      subpages: null,
    },
    {
      name: "Resources",
      href: null,
      subpages: [
        { name: "Blood Donation Guide", href: "/blood-donation-guide" },
        { name: "Common Misconceptions", href: "/common-misconceptions" },
        { name: "Donor Privileges", href: "/donor-privileges" },
        { name: "Event Organisation", href: "/event-organisation" },
        {
          name: "Blood Donation Statistics",
          href: "/blood-donation-statistics",
        },
      ],
    },
    {
      name: "Contact",
      href: "/contact",
    },
  ] as const;

  return (
    <div class="w-full border-b bg-background z-50 sticky top-0 left-0 right-0">
      <div class="flex justify-between h-16 px-6">
        <div class="flex items-center">
          <Button
            variant="none"
            class="px-2 gap-x-3"
            onClick={() => navigate("/")}
          >
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
          <Show when={user()} keyed>
            {(user) => (
              <Switch>
                <Match when={user.role === "User"}>
                  <UserNotificationDialog />
                </Match>
                <Match when={user.role === "Organiser"}>
                  <OrganiserNotificationDialog />
                </Match>
                <Match when={user.role === "Facility"}>
                  <FacilityNotificationDialog />
                </Match>
                <Match when={user.role === "Admin"}>
                  <AdminNotificationDialog />
                </Match>
              </Switch>
            )}
          </Show>
          <ProfileDropDown />
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
        class={`lg:hidden fixed overflow-auto inset-x-0 top-16 bg-background transition-all duration-300 ease-in-out z-40 overscroll-none py-4 px-6
          ${isOpen() ? "opacity-100 h-[calc(100dvh-4rem)]" : "opacity-0 h-0"}`}
      >
        <div class="flex flex-col gap-y-3">
          <For each={navItems}>
            {(item) => (
              <div>
                {item.href ? (
                  <Button
                    variant="ghost"
                    as="a"
                    href={item.href}
                    class="px-2 py-0 h-8 text-base"
                    onClick={() => setIsOpen(false)}
                  >
                    {item.name}
                  </Button>
                ) : (
                  <Collapsible>
                    <CollapsibleTrigger class="text-base font-medium flex items-center gap-x-1 [&[data-expanded]>svg]:rotate-180 h-8 px-2">
                      {item.name}
                      <ChevronDown
                        size={16}
                        class="transition-transform duration-200 "
                      />
                    </CollapsibleTrigger>
                    <CollapsibleContent class="transition-opacity duration-200 data-[expanded]:opacity-100 data-[closed]:opacity-0 grid grid-cols-1">
                      <For each={item.subpages}>
                        {(subpage) => (
                          <Button
                            variant="ghost"
                            as="a"
                            href={subpage.href}
                            class="justify-start ml-2 px-2 py-0 h-8 w-fit"
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
