import { Show } from "solid-js";
import { ChartOptions } from "chart.js";
import { ChartData } from "chart.js";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { LineChart } from "@/components/ui/charts";

type YearlyDonationsProps = {
  yearlyStatistics: ChartData | null;
  chartOptions: ChartOptions;
};

function YearlyDonationsChart(props: YearlyDonationsProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Yearly Donations</CardTitle>
      </CardHeader>
      <CardContent class="h-80 w-full">
        <Show when={props.yearlyStatistics}>
          <LineChart
            data={props.yearlyStatistics ?? { labels: [], datasets: [] }}
            options={props.chartOptions}
          />
        </Show>
      </CardContent>
    </Card>
  );
}

export default YearlyDonationsChart;
