import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  RegisterResponse,
  UserRegisterPayload,
  OrganiserRegisterPayload,
  FacilityRegisterPayload,
  AdminRegisterPayload,
} from "@/types/register";
import { parseErrorResponse } from "@/utils/error";
import { fetchWithAuth } from "@/utils/fetch-auth";

const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH;

export async function userRegister(
  userRegisterPayload: UserRegisterPayload
): Promise<Result<RegisterResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/user-register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(userRegisterPayload),
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as RegisterResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error performing register:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function organiserRegister(
  organiserRegisterPayload: OrganiserRegisterPayload
): Promise<Result<RegisterResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/organiser-register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(organiserRegisterPayload),
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as RegisterResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error performing register:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function facilityRegister(
  facilityRegisterPayload: FacilityRegisterPayload
): Promise<Result<RegisterResponse, AppError>> {
  try {
    const result = await fetchWithAuth<RegisterResponse>({
      path: "/api/facility-register",
      method: "POST",
      body: JSON.stringify(facilityRegisterPayload),
    });

    if (result.isOk()) {
      return ok(result.value as RegisterResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error performing register:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function adminRegister(
  adminRegisterPayload: AdminRegisterPayload
): Promise<Result<RegisterResponse, AppError>> {
  try {
    const result = await fetchWithAuth<RegisterResponse>({
      path: "/api/admin-register",
      method: "POST",
      body: JSON.stringify(adminRegisterPayload),
    });

    if (result.isOk()) {
      return ok(result.value as RegisterResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error performing register:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
