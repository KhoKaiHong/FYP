import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  FacilityUpdatePayload,
  FacilityUpdateResponse,
  FacilityListResponse,
} from "@/types/facility";
import { fetchWithAuth } from "@/utils/fetch-auth";
import { parseErrorResponse } from "@/utils/error";

const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH;

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

export async function listFacilities(): Promise<
  Result<FacilityListResponse, AppError>
> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/facilities`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as FacilityListResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error listing facilities:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
