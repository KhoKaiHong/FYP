import Navbar from "@/components/navigation-bar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Quote } from "lucide-solid";
import { For } from "solid-js";

function CommonMisconceptions() {
  const misconceptions = [
    {
      misconception:
        "I do not need to donate blood since Malaysia has enough supply to meet the demand for blood donation.",
      fact: "Malaysia currently has a relatively low amount of blood donors, and increasing it is essential ensure safe and timely blood access to those in need",
    },
    {
      misconception:
        "Blood donation is a waste of time and I gain no benefit from it.",
      fact: "You are eligible for various medical prvileges be being a regular blood donor. Check out the donor privileges page to learn more.",
    },
    {
      misconception: "Blood donation takes a very long time.",
      fact: "The blood donation process only takes around 7 to 10 minutes.",
    },
    {
      misconception: "Blood donation feels painful.",
      fact: "You will only feel slight pain when the needle is being inserted and none afterwards, and analgesics will be used to ease the pain.",
    },
    {
      misconception: "I am susceptible to infections during blood donation.",
      fact: "Strict Standard Operating Procedures (SOPs) are in place to ensure your safety as the blood donor. This includes the use of a new needle and blood donation bag for each donation.",
    },
    {
      misconception: "Blood donation reduces my blood levels permanently.",
      fact: "Your blood levels will be back to normal 4 to 8 weeks after donation.",
    },
    {
      misconception: "I am able to use blood donation as a form of blood test.",
      fact: "Although donated blood is all donated blood is screened for Hepatitis B and C, HIV and Syphilis, please do not treat blood donations as a test as recently infected blood may pass screening process. Refrain from donating blood if you feel unwell.",
    },
  ];
  return (
    <div>
      <Navbar />
      <div class="p-8 space-y-8">
        <div class="space-y-6">
          <For each={misconceptions}>
            {(misconception) => (
              <Card class="border-2 border-brand">
                <CardHeader>
                  <CardTitle>
                    <div class="flex gap-x-2 items-center">
                      <Quote size={20} class="shrink-0" />
                      <p class="leading-normal tracking-normal">
                        {misconception.misconception}
                      </p>
                    </div>
                  </CardTitle>
                </CardHeader>
                <CardContent>{misconception.fact}</CardContent>
              </Card>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}

export default CommonMisconceptions;
