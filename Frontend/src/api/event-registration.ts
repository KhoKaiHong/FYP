import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  ListRegistrationsResponse,
  ListRegistrationsByEventIdPayload,
  UpdateRegistrationPayload,
  UpdateRegistrationResponse,
} from "@/types/event-registration";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function listRegistrationsByEventId(
  listRegistrationsByEventIdPayload: ListRegistrationsByEventIdPayload
): Promise<Result<ListRegistrationsResponse, AppError>> {
  try {
    const result = await fetchWithAuth<ListRegistrationsResponse>({
      path: "/api/registration/event-id",
      method: "POST",
      body: JSON.stringify(listRegistrationsByEventIdPayload),
    });

    if (result.isOk()) {
      return ok(result.value as ListRegistrationsResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing event registrations:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function updateRegistration(
  updateRegistrationPayload: UpdateRegistrationPayload
): Promise<
  Result<UpdateRegistrationResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<UpdateRegistrationResponse>({
      path: "/api/registration",
      method: "PATCH",
      body: JSON.stringify(updateRegistrationPayload),
    });

    if (result.isOk()) {
      return ok(result.value as UpdateRegistrationResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating event registration:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
