import { err, ok, Result } from "neverthrow";
import { AppError } from "@/types/error";
import { PostNewEventProposalPayload, PostNewEventProposalResponse } from "@/types/new-event-proposal";
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