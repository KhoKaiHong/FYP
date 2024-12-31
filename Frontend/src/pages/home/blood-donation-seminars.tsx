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
import { SeminarImage } from "./seminar-image";
import { Lectern, MapPin, Clock, Mail, Phone } from "lucide-solid";

type BloodDonationSeminar = {
  name: string;
  address: string;
  time: string;
  email: string;
  phone: string;
  image: string;
};

const bloodDonationSeminars: BloodDonationSeminar[] = [
  {
    name: "Seminar 1",
    address: "Address 1",
    time: "Time 1",
    email: "Email 1",
    phone: "Phone 1",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 2",
    address: "Address 2",
    time: "Time 2",
    email: "Email 2",
    phone: "Phone 2",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 3",
    address: "Address 3",
    time: "Time 3",
    email: "Email 3",
    phone: "Phone 3",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 4",
    address: "Address 4",
    time: "Time 4",
    email: "Email 4",
    phone: "Phone 4",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 5",
    address: "Address 5",
    time: "Time 5",
    email: "Email 5",
    phone: "Phone 5",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 6",
    address: "Address 6",
    time: "Time 6",
    email: "Email 6",
    phone: "Phone 6",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 7",
    address: "Address 7",
    time: "Time 7",
    email: "Email 7",
    phone: "Phone 7",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 8",
    address: "Address 8",
    time: "Time 8",
    email: "Email 8",
    phone: "Phone 8",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 9",
    address: "Address 9",
    time: "Time 9",
    email: "Email 9",
    phone: "Phone 9",
    image: "img/blood-donation-seminar-placeholder.png",
  },
  {
    name: "Seminar 10",
    address: "Address 10",
    time: "Time 10",
    email: "Email 10",
    phone: "Phone 10",
    image: "img/blood-donation-seminar-placeholder.png",
  },
];

export function BloodDonationSeminars() {
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
    <div>
      <div class="flex items-center justify-center py-8">
        <h1 class="text-4xl font-semibold tracking-wide text-center">
          Seminars
        </h1>
      </div>
      <div class="w-full flex justify-center">
        <Carousel
          setApi={setApi}
          class="max-w-5xl"
          plugins={[
            Autoplay({
              delay: 6000,
            }),
          ]}
          opts={{
            loop: true,
          }}
        >
          <CarouselContent>
            <For each={bloodDonationSeminars}>
              {(seminar, index) => (
                <CarouselItem class="md:basis-1/3 sm:basis-1/2">
                  <SeminarImage
                    src={seminar.image}
                    alt={seminar.name}
                    expandedContent={
                      <div class="space-y-3 px-4 overflow-y-auto h-full overscroll-none">
                        <div class="flex items-center gap-x-2">
                          <Lectern size={18} class="shrink-0" />
                          <p class="text-sm">{seminar.name}</p>
                        </div>
                        <div class="flex items-center gap-x-2">
                          <MapPin size={18} class="shrink-0" />
                          <p class="text-sm">{seminar.address}</p>
                        </div>
                        <div class="flex items-center gap-x-2">
                          <Clock size={18} class="shrink-0" />
                          <p class="text-sm">{seminar.time}</p>
                        </div>
                        <div class="flex items-center gap-x-2">
                          <Mail size={18} class="shrink-0" />
                          <p class="text-sm">{seminar.email}</p>
                        </div>
                        <div class="flex items-center gap-x-2">
                          <Phone size={18} class="shrink-0" />
                          <p class="text-sm">{seminar.phone}</p>
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
          <CarouselPrevious class="hidden xl:inline-flex" />
          <CarouselNext class="hidden xl:inline-flex" />
        </Carousel>
      </div>
      <div class="justify-center text-sm text-muted-foreground py-4 text-center">
        Seminar {current()} of {count()}
      </div>
    </div>
  );
}
