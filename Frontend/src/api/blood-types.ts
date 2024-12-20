import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { BloodTypesResponse } from "@/types/blood-types";
import { parseErrorResponse } from "@/utils/error";

const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH;

export async function listBloodTypes(): Promise<
  Result<BloodTypesResponse, AppError>
> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/blood-types`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as BloodTypesResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error fetching blood types:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
