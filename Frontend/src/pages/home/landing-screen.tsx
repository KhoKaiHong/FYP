import { Button } from "@/components/ui/button";
import { useNavigate } from "@solidjs/router";
import { useColorMode } from "@kobalte/core/color-mode";

export function LandingScreen() {
  const navigate = useNavigate();
  const { colorMode } = useColorMode();

  return (
    <div class="min-h-[calc(100dvh-4rem)] flex justify-center">
      <div class=" flex w-full max-w-2xl items-center">
        <div class="flex flex-col gap-y-6 px-8 w-full">
          <div class="text-7xl font-bold tracking-wide self-start text-brand">
            Donate Blood,
          </div>
          <div class="text-7xl font-bold tracking-wide self-end">
            Save Lives.
          </div>
          <div class="py-8 text-2xl font-medium tracking-normal text-muted-foreground self-center">
            Your single donation can save three lives.
          </div>
          <div class="flex justify-center">
            <Button
              class={`px-8 py-6 bg-brand border-brand border-2 text-xl font-bold rounded-full text-slate-50 transition duration-300 hover:bg-white hover:text-brand transform hover:-translate-y-1 shadow-lg hover:shadow-2xl ${colorMode() === "dark" ? "shadow-slate-50 hover:shadow-slate-50" : "shadow"}`}
              onClick={() => navigate("/events", { resolve: false })}
            >
              Donate Now
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
