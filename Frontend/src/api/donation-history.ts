import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { DonationHistoryResponse } from "@/types/donation-history";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function userListDonationHistory(): Promise<
  Result<DonationHistoryResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<DonationHistoryResponse>({
      path: "/api/donation-history/user-id",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as DonationHistoryResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error fetching donation history:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}