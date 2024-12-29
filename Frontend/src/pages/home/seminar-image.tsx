import { Accessor, createEffect, createSignal, JSX } from "solid-js";
import { Button } from "@/components/ui/button";
import { ChevronUp, ChevronDown } from "lucide-solid";

type SeminarImageProps = {
  src: string;
  alt: string;
  expandedContent: JSX.Element;
  index: number;
  selectedIndex: Accessor<number>;
};

export function SeminarImage({ src, alt, expandedContent, index, selectedIndex }: SeminarImageProps) {
  const [isExpanded, setIsExpanded] = createSignal(false);

  function toggleExpand() {
    setIsExpanded(!isExpanded());
  }

  createEffect(() => {
    if (index !== selectedIndex()) {
      setIsExpanded(false);
    }
  });

  return (
    <div class="relative overflow-hidden transition-all duration-300 ease-in-out rounded-lg border-2 border-brand">
      <img src={src} alt={alt} />

      <div
        class={`absolute left-0 right-0 bg-background transition-all duration-300 ease-in-out ${
          isExpanded()
            ? "bottom-0 h-3/5 border-t-brand border-t-2"
            : "bottom-6 h-0"
        }`}
      >
        <Button
          variant={"outline"}
          size="icon"
          class="absolute top-0 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-brand"
          onClick={toggleExpand}
        >
          {isExpanded() ? <ChevronDown size={18} /> : <ChevronUp size={18} />}
        </Button>

        <div
          class={`pt-6 pb-4 h-full ${
            isExpanded() ? "opacity-100" : "opacity-0"
          } transition-opacity duration-300`}
        >
          {expandedContent}
        </div>
      </div>
    </div>
  );
}
