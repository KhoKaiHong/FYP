import Navbar from "@/components/navigation-bar";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { For } from "solid-js";
import { CircleCheckBig } from "lucide-solid";
import { A } from "@solidjs/router";

function EventOrganisation() {
  const prerequisites = [
    "Venue must have a minimum area of 40 feet by 40 feet",
    "Good lighthing and ventilation",
    "Facilities such as a clean prayer room, toilets, water supply, and electricity",
    "Ramps and lifts should be present to ensure accessibility",
    "A loading area to move heaviy items such as beds",
    "A minimum number of 8 tables and 20 chairs",
    "A minimum of 50 donors is required, conduct surveys to ensure the minimum number is met",
    "Prepare publicity materials such as brochures and posters",
    "Take actions to raise awareness of the blood donation event",
  ];

  return (
    <div>
      <Navbar />
      <div class="flex items-center justify-center p-8 min-h-[calc(100dvh-4rem)]">
        <Card class="border-2 border-brand w-full max-w-4xl">
          <CardHeader>
            <CardTitle class="text-xl leading-normal tracking-wide">
              Organising a Blood Donation Event
            </CardTitle>
            <CardDescription>
              Make sure that you follow these general guidelines when organising
              a blood donation event.
            </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2">
            <For each={prerequisites}>
              {(prerequisite) => (
                <div class="flex gap-x-2 items-center">
                  <CircleCheckBig size={18} class="shrink-0 stroke-success" />
                  <p>{prerequisite}</p>
                </div>
              )}
            </For>
            <div class="pt-2">
              <p class="text-muted-foreground">
                For more details, please visit the {" "}
                <A
                  href="https://pdn.gov.my/v2/index.php/component/k2/item/76"
                  target="_blank"
                  class="text-blue-500 hover:underline hover:text-blue-600"
                >
                  National Blood Centre
                </A>
                {" "} or contact the blood collection facility in charge.
              </p>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

export default EventOrganisation;
