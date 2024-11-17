import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { LogoutResponse, LogoutPayload } from "@/types/logout";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function logout(
  logoutPayload: LogoutPayload
): Promise<Result<LogoutResponse, AppError>> {
  try {
    console.log(JSON.stringify(logoutPayload));
    const result = await fetchWithAuth<LogoutResponse>({
      path: "/api/logout",
      method: "POST",
      body: JSON.stringify(logoutPayload),
    });

    if (result.isOk()) {
      localStorage.removeItem("accessToken");
      localStorage.removeItem("refreshToken");
      return ok(result.value as LogoutResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error during user logout:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
