import { Menu } from "lucide-solid"
import { createSignal, For } from "solid-js"
import { Button } from "@/components/ui/button"
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet"
import { Separator } from "@/components/ui/separator"
import { ColourModeToggle } from "@/components/colour-mode-toggle";

export function Navbar() {
  const [isOpen, setIsOpen] = createSignal(false)
  const navItems = [
    { name: "Home", href: "#" },
    { name: "About", href: "#" },
    { name: "Services", href: "#" },
    { name: "Contact", href: "#" },
  ]
  
  return (
    <nav class="bg-background">
      <div class="container mx-auto px-4">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <svg
              class="h-8 w-8 text-primary"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
            <span class="ml-2 text-xl font-semibold">OrgName</span>
          </div>
          <div class="hidden md:block">
            <For each={navItems}>
              {(item) => (
                <Button variant="ghost" as="a" href={item.href}>
                  {item.name}
                </Button>
              )}
            </For>
          </div>
          <ColourModeToggle />
          <Sheet open={isOpen()} onOpenChange={setIsOpen}>
            <SheetTrigger as={Button} variant="ghost" size="icon" class="md:hidden">
              <Menu class="h-6 w-6" />
              <span class="sr-only">Toggle menu</span>
            </SheetTrigger>
            <SheetContent side="right">
              <SheetHeader>
                <SheetTitle>Menu</SheetTitle>
              </SheetHeader>
              <div class="flex flex-col space-y-4 mt-4">
                <For each={navItems}>
                  {(item) => (
                    <>
                      <Button
                        variant="ghost"
                        as="a"
                        href={item.href}
                        class="w-full justify-start"
                        onClick={() => setIsOpen(false)}
                      >
                        {item.name}
                      </Button>
                      <Separator />
                    </>
                  )}
                </For>
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </nav>
  )
}