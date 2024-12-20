import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { parseErrorResponse } from "@/utils/error";
import { StatesResponse } from "@/types/states";

const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH;

export async function listStates(): Promise<Result<StatesResponse, AppError>> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/states`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as StatesResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error fetching states:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
