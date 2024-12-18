import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { LogoutAllResponse } from "@/types/logoutAll";
import { parseErrorResponse } from "@/utils/error";

const BACKEND_PATH =
  import.meta.env.VITE_BACKEND_PATH || "http://localhost:8000";

export async function logoutAll(): Promise<Result<LogoutAllResponse, AppError>> {
  // Retrieve tokens from localStorage
  const accessToken = localStorage.getItem("accessToken");
  const refreshToken = localStorage.getItem("refreshToken");

  if (!accessToken || !refreshToken) {
    return err({ message: "NO_AUTH" });
  }

  const makeLogoutRequest = async (
    accessToken: string,
    refreshToken: string
  ): Promise<Result<LogoutAllResponse, AppError>> => {
    try {
      const response = await fetch(`${BACKEND_PATH}/api/logout-all`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${accessToken}`,
        },
        body: JSON.stringify({ refreshToken }),
      });

      if (response.ok) {
        const data = await response.json();
        localStorage.removeItem("accessToken");
        localStorage.removeItem("refreshToken");
        return ok(data as LogoutAllResponse);
      }

      const errorResponse = await response.json();
      const parsedError = parseErrorResponse(errorResponse);

      if (
        response.status === 401 &&
        parsedError.message === "ACCESS_TOKEN_EXPIRED"
      ) {
        // Attempt to refresh token and retry logout
        return await handleTokenRefresh(accessToken, refreshToken);
      }

      return err(parsedError);
    } catch (error) {
      console.error("Error during logout request:", error);
      return err({ message: "UNKNOWN_ERROR" });
    }
  };

  const handleTokenRefresh = async (
    accessToken: string,
    refreshToken: string
  ): Promise<Result<LogoutAllResponse, AppError>> => {
    try {
      const refreshResponse = await fetch(`${BACKEND_PATH}/api/refresh`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${accessToken}`,
        },
        body: JSON.stringify({ refreshToken }),
      });

      if (refreshResponse.ok) {
        const refreshData = await refreshResponse.json();

        // Update tokens in localStorage
        localStorage.setItem("accessToken", refreshData.data.accessToken);
        localStorage.setItem("refreshToken", refreshData.data.refreshToken);

        // Retry the logout request with the new access token
        return await makeLogoutRequest(
          refreshData.data.accessToken,
          refreshData.data.refreshToken
        );
      }

      const errorResponse = await refreshResponse.json();
      const parsedError = parseErrorResponse(errorResponse);

      if (
        (refreshResponse.status === 401 &&
          parsedError.message === "SESSION_EXPIRED") ||
        (refreshResponse.status === 403 && parsedError.message === "NO_AUTH")
      ) {
        localStorage.removeItem("accessToken");
        localStorage.removeItem("refreshToken");
      }

      return err(parsedError);
    } catch (error) {
      console.error("Error during token refresh:", error);
      return err({ message: "UNKNOWN_ERROR" });
    }
  };

  // Start the logout process with the current access token
  return await makeLogoutRequest(accessToken, refreshToken);
}
