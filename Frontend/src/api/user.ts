import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { UserUpdatePayload, UserUpdateResponse } from "@/types/user";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function updateUser(
    userUpdatePayload: UserUpdatePayload
): Promise<Result<UserUpdateResponse, AppError>> {
  try {
    const result = await fetchWithAuth<UserUpdateResponse>({
      path: "/api/user",
      method: "PATCH",
      body: JSON.stringify(userUpdatePayload),
    });

    if (result.isOk()) {
      return ok(result.value as UserUpdateResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating user:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
