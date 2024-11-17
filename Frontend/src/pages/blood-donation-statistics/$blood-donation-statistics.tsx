import Navbar from "@/components/navigation-bar";
import showErrorToast from "@/components/error-toast";
import {
  createEffect,
  createMemo,
  createResource,
  createSignal,
  Show,
} from "solid-js";
import { listBloodDonations } from "@/api/blood-donation-statistics";
import {
  aggregateByYear,
  aggregateByMonth,
  groupDailyByYear,
  convertYearlyToChartData,
  convertMonthlyToChartData,
} from "@/utils/blood-donation-statistics";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { LineChart } from "@/components/ui/charts";
import { ChartOptions } from "chart.js";
import {
  Slider,
  SliderFill,
  SliderLabel,
  SliderThumb,
  SliderTrack,
  SliderValueLabel,
} from "@/components/ui/slider";

function BloodDonationStatistics() {
  async function fetchBloodDonationStatistics() {
    try {
      const response = await listBloodDonations();

      return response.match(
        (data) => {
          return data;
        },
        (error) => {
          showErrorToast({
            errorTitle: "Error fetching blood donation statistics.",
            error: error,
          });
          console.error("Error fetching blood donation statistics: ", error);
          return null;
        }
      );
    } catch (error) {
      showErrorToast({
        errorTitle: "Error fetching blood donation statistics.",
        error: { message: "UNKNOWN_ERROR" },
      });
      console.error(error);
      return null;
    }
  }

  const [donationStatistics] = createResource(fetchBloodDonationStatistics);

  const [monthRange, setMonthRange] = createSignal({
    start: 0,
    end: 0,
  });

  const dailyStatistics = createMemo(() => {
    const data = donationStatistics();
    if (data) {
      return groupDailyByYear(data);
    } else {
      return null;
    }
  });

  const monthlyStatistics = createMemo(() => {
    const data = donationStatistics();
    if (data) {
      return convertMonthlyToChartData(aggregateByMonth(data));
    } else {
      return null;
    }
  });

  const yearlyStatistics = createMemo(() => {
    const data = donationStatistics();
    if (data) {
      return convertYearlyToChartData(aggregateByYear(data));
    } else {
      return null;
    }
  });

  const monthlyDataRange = createMemo(() => {
    const data = monthlyStatistics();
    if (!data?.labels?.length) {
      return { min: 0, max: 0 };
    } else {
      return {
        min: 0,
        max: data.labels.length - 1,
      };
    }
  });

  createEffect(() => {
    const range = monthlyDataRange();
    setMonthRange({ start: range.min, end: range.max });
  });

  const filteredMonthlyData = createMemo(() => {
    const data = monthlyStatistics();
    const range = monthRange();

    if (data && data.labels) {
      return {
        labels: data.labels.slice(range.start, range.end + 1),
        datasets: [
          {
            ...data.datasets[0],
            data: data.datasets[0].data.slice(range.start, range.end + 1),
          },
        ],
      };
    } else {
      return null;
    }
  });

  // Convert index to month-year label
  const getMonthLabel = (index: number) => {
    const data = monthlyStatistics();
    if (!data?.labels || index < 0 || index >= data.labels.length) return "";
    return data.labels[index];
  };

  const chartOptions: ChartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: {
        border: { display: false },
        grid: { display: false },
      },
      y: {
        border: {
          dash: [3],
          dashOffset: 5,
          display: false,
        },
        grid: {
          color: "hsla(240, 3.8%, 46.1%, 0.4)",
        },
      },
    },
    plugins: {
      legend: {
        display: false,
      },
    },
    interaction: {
      intersect: false,
      mode: "index",
    },
  };

  return (
    <div>
      <Navbar />
      <div class="p-8">
        <div class="grid grid-cols-1 gap-4">
          <Card>
            <CardHeader>
              <CardTitle>Yearly Donations</CardTitle>
            </CardHeader>
            <CardContent class="h-80 w-full">
              <Show when={yearlyStatistics()}>
                <LineChart
                  data={yearlyStatistics() ?? { labels: [], datasets: [] }}
                  options={chartOptions}
                />
              </Show>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Monthly Donations</CardTitle>
            </CardHeader>
            <CardContent class="h-80 w-full">
              <Show when={monthlyStatistics()}>
                <LineChart
                  data={filteredMonthlyData() ?? { labels: [], datasets: [] }}
                  options={chartOptions}
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
                    <SliderLabel>Month Range</SliderLabel>
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
        </div>
      </div>
    </div>
  );
}

export default BloodDonationStatistics;
