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
import { createEffect, createSignal, For } from "solid-js";
import Autoplay from "embla-carousel-autoplay";
import { SuiteImage } from "./suite-image";

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
      "Mondays - Fridays: 11:00 am - 4:00 pm\nSaturdays, Sundays and Public Holidays: 11:00 am - 6:00 pm",
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

export function BloodDonationSuites() {
  const [api, setApi] = createSignal<ReturnType<CarouselApi>>();
  const [current, setCurrent] = createSignal(0);
  const [count, setCount] = createSignal(0);

  createEffect(() => {
    const _api = api();
    if (_api === undefined) {
      return;
    }

    setCount(_api.scrollSnapList().length);
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
                {(suite) => (
                  <CarouselItem>
                    <SuiteImage
                      src={suite.image}
                      alt={suite.name}
                      expandedContent={
                        <div>
                          {suite.name}
                          <br></br>
                          {suite.address}
                          <br></br>
                          {suite.hours}
                        </div>
                      }
                    />
                    {/* <img src={suite.image} alt={suite.name} /> */}
                  </CarouselItem>
                )}
              </For>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
      </CardContent>
      <CardFooter class="justify-center text-sm text-muted-foreground">
        Slide {current()} of {count()}
      </CardFooter>
    </Card>
  );
}
