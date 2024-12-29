import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Carousel,
  type CarouselApi,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";
import {
  Accessor,
  createEffect,
  createMemo,
  createSignal,
  For,
} from "solid-js";
import Autoplay from "embla-carousel-autoplay";
import { SuiteImage } from "./suite-image";
import { Hospital, MapPin, Clock } from "lucide-solid";
import { A } from "@solidjs/router";
import { Button } from "@/components/ui/button";

type BloodDonationSuite = {
  name: string;
  address: string;
  hours: string;
  image: string;
};

const bloodDonationSuites: BloodDonationSuite[] = [
  {
    name: "Pusat Darah Negara Midvalley Donation Suite",
    address:
      "T-063, 3rd floor Mezzanine, Mid Valley Megamall, 59200 Kuala Lumpur",
    hours: "10:00 am - 9:00 pm daily",
    image: "img/blood-donation-suite-mid-valley-megamall.png",
  },
  {
    name: "St John Ambulance IOI Blood Donation Suite",
    address:
      "Lot 6 & 7, Basement 3, IOI City Mall, Lebuh IRC, IOI Resort, 62502 Putrajaya, Selangor",
    hours:
      "Mondays - Fridays: 11:00 am - 4:00 pm|Saturdays, Sundays and Public Holidays: 11:00 am - 6:00 pm",
    image: "img/blood-donation-suite-ioi-city-mall.png",
  },
  {
    name: "St John Ambulance One Utama Blood Donation Suite",
    address:
      "B2, New Wing, Lebuh Bandar Utama, Bandar Utama, 47800, Petaling Jaya, Selangor",
    hours: "11:00 am - 7:00 pm daily",
    image: "img/blood-donation-suite-one-utama.png",
  },
];

function DotIndicator({
  index,
  selectedIndex,
}: {
  index: number;
  selectedIndex: Accessor<number>;
}) {
  const isActive = createMemo(() => index === selectedIndex());
  return (
    <div
      class={`h-2 w-2 rounded-full transition-all duration-300 mx-1 ${
        isActive() ? "bg-primary" : "bg-muted"
      }`}
    />
  );
}

export function BloodDonationSuites() {
  const [api, setApi] = createSignal<ReturnType<CarouselApi>>();
  const [current, setCurrent] = createSignal(0);

  createEffect(() => {
    const _api = api();
    if (_api === undefined) {
      return;
    }

    setCurrent(_api.selectedScrollSnap() + 1);

    _api.on("select", () => {
      setCurrent(_api.selectedScrollSnap() + 1);
    });
  });

  return (
    <Card class="w-full border-2 border-brand">
      <CardHeader>
        <CardTitle class="text-2xl font-semibold tracking-wide text-center ">
          Blood Donation Suites
        </CardTitle>
      </CardHeader>
      <CardContent class="grid gap-4">
        <div class="w-full flex justify-center">
          <Carousel
            setApi={setApi}
            class="max-w-3xl"
            plugins={[
              Autoplay({
                delay: 6000,
              }),
            ]}
          >
            <CarouselContent>
              <For each={bloodDonationSuites}>
                {(suite, index) => (
                  <CarouselItem>
                    <SuiteImage
                      src={suite.image}
                      alt={suite.name}
                      expandedContent={
                        <div class="space-y-3 px-4 overflow-y-auto h-full overscroll-none">
                          <div class="flex items-center gap-x-2">
                            <Hospital size={18} class="shrink-0" />
                            <p class="text-sm">{suite.name}</p>
                          </div>
                          <div class="flex items-center gap-x-2">
                            <MapPin size={18} class="shrink-0" />
                            <p class="text-sm">{suite.address}</p>
                          </div>
                          <div class="flex items-center gap-x-2">
                            <Clock size={18} class="shrink-0" />
                            <div class="grid grid-cols-1">
                              {suite.hours.split("|").map((hour) => (
                                <p class="text-sm">{hour}</p>
                              ))}
                            </div>
                          </div>
                          <div class="pt-2">
                            <A
                              href={`https://www.google.com/maps/dir/?api=1&destination=${suite.name}&travelmode=driving`}
                              target="_blank"
                            >
                              <Button>Directions</Button>
                            </A>
                          </div>
                        </div>
                      }
                      index={index() + 1}
                      selectedIndex={current}
                    />
                  </CarouselItem>
                )}
              </For>
            </CarouselContent>
            <CarouselPrevious class="hidden lg:inline-flex" />
            <CarouselNext class="hidden lg:inline-flex" />
          </Carousel>
        </div>
      </CardContent>
      <CardFooter class="justify-center text-sm text-muted-foreground">
        <For each={bloodDonationSuites}>
          {(_, index) => (
            <DotIndicator index={index() + 1} selectedIndex={current} />
          )}
        </For>
      </CardFooter>
    </Card>
  );
}
