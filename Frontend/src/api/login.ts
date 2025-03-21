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

const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH;

export async function userLogin(
  userLoginPayload: UserLoginPayload
): Promise<Result<UserLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/user-login`, {
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

export async function facilityLogin(
  facilityLoginPayload: FacilityLoginPayload
): Promise<Result<FacilityLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/facility-login`, {
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

export async function organiserLogin(
  organiserLoginPayload: OrganiserLoginPayload
): Promise<Result<OrganiserLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/organiser-login`, {
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

export async function adminLogin(
  adminLoginPayload: AdminLoginPayload
): Promise<Result<AdminLoginResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/admin-login`, {
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
