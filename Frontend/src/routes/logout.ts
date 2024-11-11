import { err, ok, Result } from "neverthrow";
import { Error } from "@/types/error";
import { LogoutResponse } from "@/types/logout";
import { fetchWithAuth } from "@/utils/fetch";

export async function logout(
  refreshToken: string
): Promise<Result<LogoutResponse, Error>> {
  try {
    const result = await fetchWithAuth<LogoutResponse>({
      path: "/api/logout",
      method: "POST",
      body: { refreshToken },
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
