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

const BACKEND_PATH =
  import.meta.env.VITE_BACKEND_PATH || "http://localhost:8000";

export async function userRegister(
  userRegisterPayload: UserRegisterPayload
): Promise<Result<RegisterResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/userregister`, {
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
    const response = await fetch(`${BACKEND_PATH}/api/organiserregister`, {
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
