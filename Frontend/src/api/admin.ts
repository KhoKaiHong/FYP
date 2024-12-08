import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { AdminUpdatePayload, AdminUpdateResponse } from "@/types/admin";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function updateAdmin(
    adminUpdatePayload: AdminUpdatePayload
): Promise<Result<AdminUpdateResponse, AppError>> {
  try {
    const result = await fetchWithAuth<AdminUpdateResponse>({
      path: "/api/admin",
      method: "PATCH",
      body: JSON.stringify(adminUpdatePayload),
    });

    if (result.isOk()) {
      return ok(result.value as AdminUpdateResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating admin:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
