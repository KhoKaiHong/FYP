import Navbar from "@/components/navigation-bar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useColorMode } from "@kobalte/core/color-mode";
import { For } from "solid-js";
import { Check, X, Dot } from "lucide-solid";
import { A } from "@solidjs/router";

function BloodDonationGuide() {
  const { colorMode } = useColorMode();

  const guidelines = {
    dos: [
      "Get at least 5 hours of sleep",
      "Drink plenty of water before donation",
      "Make sure you are in good health condition",
      "Bring verification documents such as identification card, passport or vehicle license",
      "Take food 4 hours before donation",
      "Wear comfortable clothing, with sleeves that can be rolled up in the case of long-sleeved attire",
      "Consume the food and drinks provided after blood donation",
      "Answer the questions provided before the donation session honestly",
    ],
    donts: [
      "Donate blood under the effects of alcohol",
      "Donate blood under the effects of medication",
      "Donate blood if you are fasting",
      "Donate blood if you involve yourself in homosexual or bisexual relationships",
      "Donate blood if you have multiple sexual partners",
      "Donate blood if you involve yourself in prostitution",
      "Donate blood if you perform drug injections",
      "Consume alcohol after donation within the first few hours",
      "Involve yourself in extreme physical activities for 24 hours after donation",
    ],
    prerequisites: [
      "You must be aged between 17 and 60 years old",
      "You must weight more than 45 kilograms",
      "You must stay at Malaysia for at least a year if you're a non-citizen",
      "For women: Not pregnant, not breastfeeding, and not menstruating",
    ],
  };

  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-8">
        <div class="space-y-8">
          <Card
            class={`border-green-500
             border-2`}
          >
            <CardHeader
              class={`${
                colorMode() === "dark" ? "bg-green-900" : "bg-green-100"
              } rounded-t-xl`}
            >
              <CardTitle
                class={`${
                  colorMode() === "dark" ? "text-green-100" : "text-green-900"
                } text-xl tracking-wide`}
              >
                Dos
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <For each={guidelines.dos}>
                {(dos) => (
                  <div class="flex gap-x-2 items-center">
                    <Check class="shrink-0" />
                    <p>{dos}</p>
                  </div>
                )}
              </For>
            </CardContent>
          </Card>
          <Card class="border-2 border-red-500">
            <CardHeader
              class={`${
                colorMode() === "dark" ? "bg-red-900" : "bg-red-100"
              } rounded-t-xl`}
            >
              <CardTitle
                class={`${
                  colorMode() === "dark" ? "text-red-100" : "text-red-900"
                } text-xl tracking-wide`}
              >
                Don'ts
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <For each={guidelines.donts}>
                {(donts) => (
                  <div class="flex gap-x-2 items-center">
                    <X class="shrink-0" />
                    <p>{donts}</p>
                  </div>
                )}
              </For>
            </CardContent>
          </Card>
          <Card class="border-2 border-slate-500">
            <CardHeader class="bg-secondary rounded-t-xl">
              <CardTitle class="text-secondary-foreground text-xl tracking-wide">
                Prerequisites
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 space-y-3">
              <For each={guidelines.prerequisites}>
                {(prerequisite) => (
                  <div class="flex gap-x-2 items-center">
                    <Dot class="shrink-0" />
                    <p>{prerequisite}</p>
                  </div>
                )}
              </For>
            </CardContent>
          </Card>
        </div>
        <div>
          <p class="text-muted-foreground">
            This data is brought to you by the{" "}
            <A
              href="https://pdn.gov.my/v2/images/dokumen/Buku_Informasi_Pendermaan_Darah.pdf"
              target="_blank"
              class="text-blue-500 hover:underline hover:text-blue-600"
            >
              National Blood Centre
            </A>
            .
          </p>
        </div>
      </div>
    </div>
  );
}

export default BloodDonationGuide;
