import {
  DonationStatisticsResponse,
  DailyDonation,
  MonthlyDonation,
  YearlyDonation,
  DailyDonationsByYear,
} from "@/types/blood-donation-statistics";
import { ChartData } from "chart.js";
import { LineChart } from "@/components/ui/charts";

// Aggregate donations by year
export function aggregateByYear(
  data: DonationStatisticsResponse
): YearlyDonation[] {
  const yearlyMap = new Map<number, number>();

  data.forEach((item) => {
    const year = new Date(item.date).getFullYear();
    const currentTotal = yearlyMap.get(year) || 0;
    yearlyMap.set(year, currentTotal + item.donations);
  });

  return Array.from(yearlyMap.entries())
    .map(([year, donations]) => ({
      year,
      donations,
    }))
    .sort((a, b) => a.year - b.year);
}

// Aggregate donations by month
export function aggregateByMonth(
  data: DonationStatisticsResponse
): MonthlyDonation[] {
  const monthlyMap = new Map<string, number>();

  data.forEach((item) => {
    const date = new Date(item.date);
    const key = `${date.getFullYear()}-${date.getMonth()}`;
    const currentTotal = monthlyMap.get(key) || 0;
    monthlyMap.set(key, currentTotal + item.donations);
  });

  return Array.from(monthlyMap.entries())
    .map(([key, donations]) => {
      const [year, month] = key.split("-").map(Number);
      return {
        year,
        month,
        donations,
      };
    })
    .sort((a, b) => {
      // Sort by year first, then by month
      if (a.year !== b.year) return a.year - b.year;
      return a.month - b.month;
    });
}

// Group daily donations by year
export function groupDailyByYear(data: DonationStatisticsResponse): {
  years: number[];
  data: DailyDonationsByYear[];
} {
  const yearlyMap = new Map<number, DailyDonation[]>();

  data.forEach((item) => {
    const year = new Date(item.date).getFullYear();
    if (!yearlyMap.has(year)) {
      yearlyMap.set(year, []);
    }
    yearlyMap.get(year)?.push(item);
  });

  const sortedData = Array.from(yearlyMap.entries())
    .map(([year, donations]) => ({
      year,
      data: donations.sort(
        (a, b) => new Date(a.date).getTime() - new Date(b.date).getTime()
      ),
    }))
    .sort((a, b) => a.year - b.year);

  return {
    years: Array.from(yearlyMap.keys()).sort((a, b) => b - a),
    data: sortedData,
  };
}

export function convertYearlyToChartData(data: YearlyDonation[]): ChartData {
  return {
    labels: data.map((item) => item.year.toString()),
    datasets: [
      {
        label: "Donations",
        data: data.map((item) => item.donations),
        fill: true,
        pointRadius: 0,
        backgroundColor: "rgba(170, 0, 0, 0.1)",
        borderColor: "#AA0000",
        borderWidth: 1,
        pointHoverRadius: 4,
        pointHoverBackgroundColor: "rgba(170, 0, 0, 0.3)",
        pointHoverBorderColor: "#AA0000",
        pointHoverBorderWidth: 1,
      },
    ],
  };
}

export function convertMonthlyToChartData(data: MonthlyDonation[]): ChartData {
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

  return {
    labels: data.map((item) => {
      const yearStr = item.year.toString().slice(-2);
      return `${months[item.month]} ${yearStr}`;
    }),
    datasets: [
      {
        label: "Donations",
        data: data.map((item) => item.donations),
        fill: true,
        pointRadius: 0,
        backgroundColor: "rgba(170, 0, 0, 0.1)",
        borderColor: "#AA0000",
        borderWidth: 1,
        pointHoverRadius: 4,
        pointHoverBackgroundColor: "rgba(170, 0, 0, 0.3)",
        pointHoverBorderColor: "#AA0000",
        pointHoverBorderWidth: 1,
      },
    ],
  };
}
