import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { DonationStatisticsResponse } from "@/types/blood-donation-statistics";

const RESOURCE_PATH = "https://api.data.gov.my/data-catalogue";

export async function listBloodDonations(): Promise<
  Result<DonationStatisticsResponse, AppError>
> {
  const dateStart = "2014-01-01";
  const dateEnd = "2024-12-31";

  try {
    const response = await fetch(
      `${RESOURCE_PATH}?id=blood_donations&date_start=${dateStart}@date&date_end=${dateEnd}@date&filter=all@blood_type&exclude=blood_type`,
      {
        method: "GET",
        redirect: "follow",
        headers: {
          "Content-Type": "application/json",
        },
      }
    );

    const result = await response.json();

    if (response.ok) {
      return ok(result as DonationStatisticsResponse);
    }

    console.log(response);
    return err({ message: "UNKNOWN_ERROR" });
  } catch (error) {
    console.error(
      "Error fetching blood donation trends data from data.gov.my: ",
      error
    );
    return err({ message: "UNKNOWN_ERROR" });
  }
}
