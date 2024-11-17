import { AppError } from "@/types/error";
import { parseErrorResponse } from "@/utils/error";
import { err, ok, Result } from "neverthrow";

const BACKEND_PATH =
  import.meta.env.VITE_BACKEND_PATH || "http://localhost:3001";

export async function fetchWithAuth<T = unknown>({
  path,
  method = "GET",
  body = null,
}: {
  path: string;
  method?: string;
  body?: string | null;
}): Promise<Result<T, AppError>> {
  // Check if accessToken and refreshToken are in localStorage
  const accessToken = localStorage.getItem("accessToken");
  const refreshToken = localStorage.getItem("refreshToken");

  if (!accessToken || !refreshToken) {
    return err({ message: "NO_AUTH" });
  }

  // Function to handle API calls with the accessToken
  const makeRequest = async (token: string): Promise<Result<T, AppError>> => {
    try {
      const response = await fetch(`${BACKEND_PATH}${path}`, {
        method,
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: body ? body : null,
      });

      if (response.ok) {
        const data = await response.json();
        return ok(data);
      }

      // Handle error responses
      const errorResponse = await response.json();
      const parsedError = parseErrorResponse(errorResponse);

      if (
        response.status === 401 &&
        parsedError.message === "ACCESS_TOKEN_EXPIRED"
      ) {
        // If access token expired, try to refresh it
        return await handleTokenRefresh(token, refreshToken);
      } else if (
        (response.status === 401 &&
          parsedError.message === "SESSION_EXPIRED") ||
        (response.status === 403 && parsedError.message === "NO_AUTH")
      ) {
        localStorage.removeItem("accessToken");
        localStorage.removeItem("refreshToken");
      }

      return err(parsedError);
    } catch (error) {
      console.error(error);
      return err({ message: "UNKNOWN_ERROR" });
    }
  };

  // Function to handle token refresh if expired
  const handleTokenRefresh = async (
    accessToken: string,
    refreshToken: string
  ): Promise<Result<T, AppError>> => {
    try {
      const refreshResponse = await fetch(`${BACKEND_PATH}/api/refresh`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${accessToken}`,
        },
        body: JSON.stringify({ refreshToken: refreshToken }),
      });

      if (refreshResponse.ok) {
        const refreshData = await refreshResponse.json();

        // Successfully refreshed tokens, update them in localStorage
        localStorage.setItem("accessToken", refreshData.data.accessToken);
        localStorage.setItem("refreshToken", refreshData.data.refreshToken);

        // Retry the original request with the new access token
        return await makeRequest(refreshData.data.accessToken);
      }

      // Handle error responses
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
      console.error(error);
      return err({ message: "UNKNOWN_ERROR" });
    }
  };

  // Start with the initial request using the access token
  return await makeRequest(accessToken);
}
