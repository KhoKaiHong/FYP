import Navbar from "@/components/navigation-bar";
import { createResource, For, Show } from "solid-js";
import { listFacilities } from "@/api/facility";
import showErrorToast from "@/components/error-toast";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Map, MapPin, Mail, Phone } from "lucide-solid";
import { A } from "@solidjs/router";

function Contact() {
  async function fetchFacilities() {
    const response = await listFacilities();

    return response.match(
      (data) => {
        return data.data.facilities;
      },
      (error) => {
        showErrorToast({
          errorTitle: "Error fetching facilities.",
          error,
        });
        console.error("Error fetching facilities:", error);
        return null;
      }
    );
  }

  const [facilities] = createResource(fetchFacilities);

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-8">
        <Show when={facilities()}>
          {(facilities) => (
            <div class="space-y-8 lg:grid lg:grid-cols-2 lg:gap-8 lg:space-y-0">
              <For each={facilities()}>
                {(facility) => (
                  <Card>
                    <CardHeader class="bg-brand rounded-t-xl">
                      <CardTitle class="text-slate-50 text-xl tracking-wide">
                        {facility.name}
                      </CardTitle>
                    </CardHeader>
                    <CardContent class="pt-6 space-y-3">
                      <div class="flex gap-x-2 items-center">
                        <Map class="shrink-0" />
                        <p>{facility.stateName}</p>
                      </div>
                      <div class="flex gap-x-2 items-center">
                        <MapPin class="shrink-0" />
                        <p>{facility.address}</p>
                      </div>
                      <div class="flex gap-x-2 items-center">
                        <Mail class="shrink-0" />
                        <p>{facility.email}</p>
                      </div>
                      <div class="flex gap-x-2 items-center">
                        <Phone class="shrink-0" />
                        <p>{facility.phoneNumber}</p>
                      </div>
                      <div>
                        <A
                          href={`https://www.google.com/maps/dir/?api=1&destination=Blood Bank ${facility.name}&travelmode=driving`}
                          target="_blank"
                        >
                          <Button>Show Directions</Button>
                        </A>
                      </div>
                    </CardContent>
                  </Card>
                )}
              </For>
            </div>
          )}
        </Show>
      </div>
    </div>
  );
}

export default Contact;
