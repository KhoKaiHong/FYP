export type DailyDonation = {
  date: string;
  donations: number;
};

export type DonationStatisticsResponse = DailyDonation[];

export type YearlyDonation = {
  year: number;
  donations: number;
}

export type MonthlyDonation = {
  year: number;
  month: number;
  donations: number;
}

export type DailyDonationsByYear = {
  year: number;
  data: DailyDonation[];
}