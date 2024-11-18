import Navbar from "@/components/navigation-bar";
import showErrorToast from "@/components/error-toast";
import { createMemo, createResource } from "solid-js";
import { listBloodDonations } from "@/api/blood-donation-statistics";
import {
  aggregateByYear,
  aggregateByMonth,
  groupDailyByYear,
  convertYearlyToChartData,
  convertMonthlyToChartData,
} from "@/utils/blood-donation-statistics";
import { ChartOptions } from "chart.js";
import YearlyDonationsChart from "./yearly-donations-chart";
import MonthlyDonationsChart from "./monthly-donations-chart";
import DailyDonationsChart from "./daily-donations-chart";
import { A } from "@solidjs/router";

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

  const yearlyStatistics = createMemo(() => {
    const data = donationStatistics();
    if (data) {
      return convertYearlyToChartData(aggregateByYear(data));
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

  const dailyStatistics = createMemo(() => {
    const data = donationStatistics();
    if (data) {
      return groupDailyByYear(data);
    } else {
      return null;
    }
  });

  const chartOptions: ChartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: {
        ticks: {
          color: "rgba(100, 116, 139, 0.8)",
        },
        border: { display: false },
        grid: { display: false },
      },
      y: {
        border: {
          color: "rgba(100, 116, 139, 0.4)",
          dash: [3],
          dashOffset: 5,
          display: false,
        },
        grid: {
          color: "rgba(100, 116, 139, 0.4)",
        },
        ticks: {
          color: "rgba(100, 116, 139, 0.8)",
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
        <div class="grid grid-cols-1 gap-8">
          <YearlyDonationsChart
            yearlyStatistics={yearlyStatistics()}
            chartOptions={chartOptions}
          />
          <MonthlyDonationsChart
            monthlyStatistics={monthlyStatistics()}
            chartOptions={chartOptions}
          />
          <DailyDonationsChart
            dailyStatistics={dailyStatistics()}
            chartOptions={chartOptions}
          />
          <div class="text-muted-foreground space-y-2">
            <p>
              This data is brought to you by{" "}
              <A
                href="https://data.gov.my/"
                target="_blank"
                class="text-blue-500 hover:underline hover:text-blue-600"
              >
                data.gov.my
              </A>
              .
            </p>
            <p>
              To view more blood donation data, please visit{" "}
              <A
                href="https://data.moh.gov.my/dashboard/blood-donation"
                target="_blank"
                class="text-blue-500 hover:underline hover:text-blue-600"
              >
                data.moh.gov.my
              </A>
              .
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default BloodDonationStatistics;
