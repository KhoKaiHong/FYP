import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  PostChangeEventRequestPayload,
  PostChangeEventRequestResponse,
  ListChangeEventRequestResponse,
  UpdateChangeEventRequestPayload,
  UpdateChangeEventRequestResponse,
} from "@/types/change-event-request";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function postChangeEventRequest(
  postChangeEventRequestPayload: PostChangeEventRequestPayload
): Promise<Result<PostChangeEventRequestResponse, AppError>> {
  try {
    const result = await fetchWithAuth<PostChangeEventRequestResponse>({
      path: "/api/change-event-request",
      method: "POST",
      body: JSON.stringify(postChangeEventRequestPayload),
    });

    if (result.isOk()) {
      return ok(result.value as PostChangeEventRequestResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when posting change event request:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function organiserListChangeEventRequest(): Promise<
  Result<ListChangeEventRequestResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<ListChangeEventRequestResponse>({
      path: "/api/change-event-request/organiser",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as ListChangeEventRequestResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing change event requests:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function facilityListChangeEventRequest(): Promise<
  Result<ListChangeEventRequestResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<ListChangeEventRequestResponse>({
      path: "/api/change-event-request/facility",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as ListChangeEventRequestResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing change event requests:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function facilityUpdateChangeEventRequest(
  updateChangeEventRequestPayload: UpdateChangeEventRequestPayload
): Promise<Result<UpdateChangeEventRequestResponse, AppError>> {
  try {
    const result = await fetchWithAuth<UpdateChangeEventRequestResponse>({
      path: "/api/change-event-request",
      method: "PATCH",
      body: JSON.stringify(updateChangeEventRequestPayload),
    });

    if (result.isOk()) {
      return ok(result.value as UpdateChangeEventRequestResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating change event request:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
