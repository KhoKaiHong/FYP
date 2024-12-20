import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { parseErrorResponse } from "@/utils/error";
import { DistrictsResponse } from "@/types/districts";

const BACKEND_PATH = import.meta.env.VITE_BACKEND_PATH;

export async function listDistricts(): Promise<
  Result<DistrictsResponse, AppError>
> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/districts`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as DistrictsResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error fetching districts:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
