import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { FacilityUpdatePayload, FacilityUpdateResponse } from "@/types/facility";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function updateFacility(
    facilityUpdatePayload: FacilityUpdatePayload
): Promise<Result<FacilityUpdateResponse, AppError>> {
  try {
    const result = await fetchWithAuth<FacilityUpdateResponse>({
      path: "/api/facility",
      method: "PATCH",
      body: JSON.stringify(facilityUpdatePayload),
    });

    if (result.isOk()) {
      return ok(result.value as FacilityUpdateResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating organiser:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
