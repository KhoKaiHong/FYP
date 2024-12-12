import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import {
  PostNewEventProposalPayload,
  PostNewEventProposalResponse,
  ListNewEventProposalResponse,
  UpdateNewEventProposalPayload,
  UpdateNewEventProposalResponse,
} from "@/types/new-event-proposal";
import { fetchWithAuth } from "@/utils/fetch-auth";

export async function postNewEventProposal(
  postNewEventProposalPayload: PostNewEventProposalPayload
): Promise<Result<PostNewEventProposalResponse, AppError>> {
  try {
    const result = await fetchWithAuth<PostNewEventProposalResponse>({
      path: "/api/new-event-request",
      method: "POST",
      body: JSON.stringify(postNewEventProposalPayload),
    });

    if (result.isOk()) {
      return ok(result.value as PostNewEventProposalResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when posting new event proposal:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function organiserListNewEventProposal(): Promise<
  Result<ListNewEventProposalResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<ListNewEventProposalResponse>({
      path: "/api/new-event-request/organiser",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as ListNewEventProposalResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing new event proposals:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function facilityListNewEventProposal(): Promise<
  Result<ListNewEventProposalResponse, AppError>
> {
  try {
    const result = await fetchWithAuth<ListNewEventProposalResponse>({
      path: "/api/new-event-request/facility",
      method: "GET",
    });

    if (result.isOk()) {
      return ok(result.value as ListNewEventProposalResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when listing new event proposals:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}

export async function facilityUpdateNewEventProposal(
  updateNewEventProposalPayload: UpdateNewEventProposalPayload
): Promise<Result<UpdateNewEventProposalResponse, AppError>> {
  try {
    const result = await fetchWithAuth<UpdateNewEventProposalResponse>({
      path: "/api/new-event-request",
      method: "PATCH",
      body: JSON.stringify(updateNewEventProposalPayload),
    });

    if (result.isOk()) {
      return ok(result.value as UpdateNewEventProposalResponse);
    } else {
      return err(result.error);
    }
  } catch (error) {
    console.error("Error when updating new event proposal:", error);
    return err({ message: "UNKNOWN_ERROR" });
  }
}
