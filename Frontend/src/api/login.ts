import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  UserLoginResponse,
  FacilityLoginResponse,
  OrganiserLoginResponse,
  AdminLoginResponse,
  UserLoginPayload,
  FacilityLoginPayload,
  OrganiserLoginPayload,
  AdminLoginPayload,
} from "@/types/login";
import { parseErrorResponse } from "@/utils/error";

const BACKEND_PATH =
  import.meta.env.VITE_BACKEND_PATH || "http://localhost:8000";

export async function userLogin(
  userLoginPayload: UserLoginPayload
): Promise<Result<UserLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/userlogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userLoginPayload),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return ok(result as UserLoginResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error during user login:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

// Updated facilityLogin function with ApiResult<FacilityLoginResponse>
export async function facilityLogin(
  facilityLoginPayload: FacilityLoginPayload
): Promise<Result<FacilityLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/facilitylogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(facilityLoginPayload),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return ok(result as FacilityLoginResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error during facility login:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

// Updated organiserLogin function with ApiResult<OrganiserLoginResponse>
export async function organiserLogin(
  organiserLoginPayload: OrganiserLoginPayload
): Promise<Result<OrganiserLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/organiserlogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(organiserLoginPayload),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return ok(result as OrganiserLoginResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error during organiser login:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

// Updated adminLogin function with ApiResult<AdminLoginResponse>
export async function adminLogin(
  adminLoginPayload: AdminLoginPayload
): Promise<Result<AdminLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/adminlogin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(adminLoginPayload),
    });

    const result = await response.json();

    if (response.ok) {
      localStorage.setItem("accessToken", result.data.accessToken);
      localStorage.setItem("refreshToken", result.data.refreshToken);
      return ok(result as AdminLoginResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error during admin login:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
