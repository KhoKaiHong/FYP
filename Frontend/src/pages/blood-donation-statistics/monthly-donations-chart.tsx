import { createEffect, createMemo, createSignal, Show } from "solid-js";
import { ChartOptions } from "chart.js";
import { ChartData } from "chart.js";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { LineChart } from "@/components/ui/charts";
import {
  Slider,
  SliderFill,
  SliderLabel,
  SliderThumb,
  SliderTrack,
  SliderValueLabel,
} from "@/components/ui/slider";

type MonthlyDonationsProps = {
  monthlyStatistics: ChartData | null;
  chartOptions: ChartOptions;
};

function MonthlyDonationsChart(props: MonthlyDonationsProps) {
  const [monthRange, setMonthRange] = createSignal({
    start: 0,
    end: 0,
  });

  const monthlyDataRange = createMemo(() => {
    if (!props.monthlyStatistics?.labels?.length) {
      return { min: 0, max: 0 };
    } else {
      return {
        min: 0,
        max: props.monthlyStatistics.labels.length - 1,
      };
    }
  });

  createEffect(() => {
    const range = monthlyDataRange();
    setMonthRange({ start: range.min, end: range.max });
  });

  const filteredMonthlyData = createMemo(() => {
    const range = monthRange();

    if (props.monthlyStatistics && props.monthlyStatistics.labels) {
      return {
        labels: props.monthlyStatistics.labels.slice(
          range.start,
          range.end + 1
        ),
        datasets: [
          {
            ...props.monthlyStatistics.datasets[0],
            data: props.monthlyStatistics.datasets[0].data.slice(
              range.start,
              range.end + 1
            ),
          },
        ],
      };
    } else {
      return null;
    }
  });

  // Convert index to month-year label
  const getMonthLabel = (index: number) => {
    if (
      !props.monthlyStatistics?.labels ||
      index < 0 ||
      index >= props.monthlyStatistics.labels.length
    )
      return "";
    return props.monthlyStatistics.labels[index];
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Monthly Donations</CardTitle>
      </CardHeader>
      <CardContent class="h-80 w-full">
        <Show when={props.monthlyStatistics}>
          <LineChart
            data={filteredMonthlyData() ?? { labels: [], datasets: [] }}
            options={props.chartOptions}
          />
        </Show>
      </CardContent>
      <CardFooter>
        <Show when={monthRange().end !== 0}>
          <Slider
            minValue={monthlyDataRange().min}
            maxValue={monthlyDataRange().max}
            defaultValue={[monthRange().start, monthRange().end]}
            onChangeEnd={(values) => {
              setMonthRange({ start: values[0], end: values[1] });
            }}
            getValueLabel={(params) =>
              `${getMonthLabel(params.values[0])} - ${getMonthLabel(
                params.values[1]
              )}`
            }
            class="w-full space-y-3"
          >
            <div class="flex w-full justify-between">
              <SliderLabel>Period</SliderLabel>
              <SliderValueLabel />
            </div>
            <SliderTrack>
              <SliderFill />
              <SliderThumb />
              <SliderThumb />
            </SliderTrack>
          </Slider>
        </Show>
      </CardFooter>
    </Card>
  );
}

export default MonthlyDonationsChart;
