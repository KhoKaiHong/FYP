import { createSignal, JSX } from "solid-js";
import { Button } from "@/components/ui/button";
import { ChevronUp, ChevronDown } from "lucide-solid";

type SuiteImageProps = {
  src: string;
  alt: string;
  expandedContent: JSX.Element;
};

export function SuiteImage({ src, alt, expandedContent }: SuiteImageProps) {
  const [isExpanded, setIsExpanded] = createSignal(false);

  const toggleExpand = () => setIsExpanded(!isExpanded());

  return (
    <div class="relative overflow-hidden rounded-lg transition-all duration-300 ease-in-out border-2 border-brand">
      <img src={src} alt={alt} />

      <div
        class={`absolute left-0 right-0 bg-background transition-all duration-300 ease-in-out ${
          isExpanded()
            ? "bottom-0 h-1/3 border-t-brand border-t-2"
            : "bottom-6 h-0"
        }`}
      >
        <Button
          variant={"outline"}
          size="icon"
          class="absolute top-0 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-full shadow-md border-2 border-brand"
          onClick={toggleExpand}
        >
          {isExpanded() ? <ChevronDown size={18} /> : <ChevronUp size={18} />}
        </Button>

        <div
          class={`p-4 overflow-y-auto h-full ${
            isExpanded() ? "opacity-100" : "opacity-0"
          } transition-opacity duration-300`}
        >
          {expandedContent}
        </div>
      </div>
    </div>
  );
}
