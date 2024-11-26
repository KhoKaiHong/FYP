import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { EventsResponse } from "@/types/events";
import { parseErrorResponse } from "@/utils/error";

const BACKEND_PATH =
  import.meta.env.VITE_BACKEND_PATH || "http://localhost:8000";

export async function listEvents(): Promise<
  Result<EventsResponse, AppError>
> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/events`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as EventsResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error fetching events:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function listFutureEvents(): Promise<
  Result<EventsResponse, AppError>
> {
  try {
    const response = await fetch(`${BACKEND_PATH}/api/events/future`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    const result = await response.json();

    if (response.ok) {
      return ok(result as EventsResponse);
    }

    const parsedError = parseErrorResponse(result);
    return err(parsedError);
  } catch (error) {
    console.error("Error fetching events:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
