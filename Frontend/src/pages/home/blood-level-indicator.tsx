import { createMemo, For, Match, Switch } from "solid-js";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Droplet, CircleAlert, CircleCheck, OctagonAlert } from "lucide-solid";
import { Progress } from "@/components/ui/progress";
import { useColorMode } from "@kobalte/core/color-mode";

type BloodType = "A+" | "A-" | "B+" | "B-" | "AB+" | "AB-" | "O+" | "O-";
type BloodTypeLevel = {
  type: BloodType;
  level: number;
};

const bloodTypeLevels: BloodTypeLevel[] = [
  { type: "A+", level: 80 },
  { type: "A-", level: 90 },
  { type: "B+", level: 60 },
  { type: "B-", level: 50 },
  { type: "AB+", level: 30 },
  { type: "AB-", level: 20 },
  { type: "O+", level: 15 },
  { type: "O-", level: 80 },
];

const BloodTypeCard = ({ type, level }: BloodTypeLevel) => {
  function getLevelConfig(percentage: number) {
    const { colorMode } = useColorMode();

    if (percentage >= 50) {
      return {
        color: colorMode() === "dark" ? "bg-green-900" : "bg-green-100",
        text: colorMode() === "dark" ? "text-green-100" : "text-green-900",
        border: "border-green-500",
        status: "safe",
        progressColor: "bg-green-500",
      };
    } else if (percentage >= 20) {
      return {
        color: colorMode() === "dark" ? "bg-yellow-900" : "bg-yellow-100",
        text: colorMode() === "dark" ? "text-yellow-100" : "text-yellow-900",
        border: "border-yellow-500",
        status: "warning",
        progressColor: "bg-yellow-500",
      };
    } else {
      return {
        color: colorMode() === "dark" ? "bg-red-900" : "bg-red-100",
        text: colorMode() === "dark" ? "text-red-100" : "text-red-900",
        border: "border-red-500",
        status: "critical",
        progressColor: "bg-red-500",
      };
    }
  }

  const config = createMemo(() => getLevelConfig(level));

  return (
    <Card class={`w-full ${config().color} ${config().border} border-2`}>
      <CardHeader>
        <CardTitle class="text-xl font-semibold tracking-wide text-center flex gap-2 items-center justify-center">
          <Droplet size={22} class="fill-foreground" />
          {type}
        </CardTitle>
      </CardHeader>
      <CardContent class="pt-2 space-y-4">
        <Progress
          value={level}
          class={`${config().progressColor} rounded-full`}
        />
        <div class="flex justify-between">
          <Badge
            variant={"outline"}
            class={`${config().text} ${config().border} capitalize flex gap-1`}
          >
            <Switch>
              <Match when={config().status === "safe"}>
                <CircleCheck class="h-4 w-4" />
              </Match>
              <Match when={config().status === "warning"}>
                <CircleAlert class="h-4 w-4" />
              </Match>
              <Match when={config().status === "critical"}>
                <OctagonAlert class="h-4 w-4" />
              </Match>
            </Switch>
            {config().status}
          </Badge>
          <div class={`${config().text}`}>{level}%</div>
        </div>
      </CardContent>
    </Card>
  );
};

export function BloodLevelIndicator() {
  return (
    <div>
      <div class="flex items-center justify-center py-8">
        <h1 class="text-4xl font-semibold tracking-wide text-center">
          Blood Type Levels
        </h1>
      </div>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        <For each={bloodTypeLevels}>
          {(bloodType) => <BloodTypeCard {...bloodType} />}
        </For>
      </div>
    </div>
  );
}
