import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { OrganiserUpdatePayload, OrganiserUpdateResponse } from "@/types/organiser";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function updateOrganiser(
    organiserUpdatePayload: OrganiserUpdatePayload
): Promise<Result<OrganiserUpdateResponse, AppError>> {
  try {
    const result = await fetchWithAuth<OrganiserUpdateResponse>({
      path: "/api/organiser",
      method: "PATCH",
      body: JSON.stringify(organiserUpdatePayload),
    });

    if (result.isOk()) {
      return ok(result.value as OrganiserUpdateResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating organiser:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
