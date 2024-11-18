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
import { DailyDonationsByYear } from "@/types/blood-donation-statistics";
import { convertDailyToChartData, filterDailyByMonthRange } from "@/utils/blood-donation-statistics";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

type DailyDonationsProps = {
  dailyStatistics: {
    years: number[];
    data: DailyDonationsByYear[];
  } | null;
  chartOptions: ChartOptions;
};

function DailyDonationsChart(props: DailyDonationsProps) {
  const [selectedYear, setSelectedYear] = createSignal<number | null>(null);
  const [monthRange, setMonthRange] = createSignal({
    start: 0,
    end: 0,
  });

  const [sliderMonth, setSliderMonth] = createSignal({
    start: 0,
    end: 0,
  });

  const selectedYearData = createMemo(
    () => props.dailyStatistics?.data.find((d) => d.year === selectedYear()) || null
  );

  const availableMonths = createMemo(() => {
    const data = selectedYearData();
    if (!data?.months.length) {
      return { min: 0, max: 0 };
    } else {
      return {
        min: data.months[0],
        max: data.months[data.months.length - 1],
      };
    }
  });

  createEffect(() => {
    const range = availableMonths();
    setMonthRange({ start: range.min, end: range.max });
    setSliderMonth({ start: range.min, end: range.max });
  });

  const filteredData = createMemo(() => {
    const yearData = selectedYearData();
    if (!yearData) return [];

    return filterDailyByMonthRange(
      yearData.data,
      monthRange().start,
      monthRange().end
    );
  });

  createEffect(() => {
    const data = props.dailyStatistics;
    if (data && selectedYear() === null) {
      setSelectedYear(data.years[0]);
    }
  });

  const dailyChartData = createMemo(() =>
    convertDailyToChartData(filteredData())
  );

  function getMonthLabel(index: number) {
    const months = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ];
    return months[index];
  }
  return (
    <Card>
            <CardHeader class="flex flex-row justify-between items-center">
              <CardTitle>Daily Donations</CardTitle>
              <Show when={props.dailyStatistics}>
                <Select
                  class="w-32"
                  options={props.dailyStatistics?.years ?? []}
                  value={selectedYear()}
                  onChange={(e) => {
                    setSelectedYear(e);
                  }}
                  itemComponent={(props) => (
                    <SelectItem item={props.item}>
                      {props.item.rawValue}
                    </SelectItem>
                  )}
                >
                  <SelectTrigger>
                    <SelectValue<number>>
                      {(state) => state.selectedOption()}
                    </SelectValue>
                  </SelectTrigger>
                  <SelectContent />
                </Select>
              </Show>
            </CardHeader>
            <CardContent class="h-80 w-full">
              <Show when={props.dailyStatistics}>
                <LineChart
                  data={dailyChartData() ?? { labels: [], datasets: [] }}
                  options={props.chartOptions}
                />
              </Show>
            </CardContent>
            <CardFooter>
              <Show when={monthRange().end !== 0}>
                <Slider
                  minValue={availableMonths().min}
                  maxValue={availableMonths().max}
                  defaultValue={[monthRange().start, monthRange().end]}
                  value={[sliderMonth().start, sliderMonth().end]}
                  onChange={(values) => {
                    setSliderMonth({ start: values[0], end: values[1] });
                  }}
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
                    <SliderLabel>Month</SliderLabel>
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

export default DailyDonationsChart;
