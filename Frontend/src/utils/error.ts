import { Error, ClientErrorResponse } from "@/types/error";

// Type guard to check if an error response is a client error from backend
function isClientError(errorResponse: any): errorResponse is ClientErrorResponse {
  return (
    typeof errorResponse === "object" &&
    errorResponse !== null &&
    "error" in errorResponse &&
    typeof errorResponse.error.message === "string" &&
    "data" in errorResponse.error &&
    typeof errorResponse.error.data.req_uuid === "string"
  );
}

// Function to parse any error response into a ClientError or UNKNOWN_ERROR
export function parseErrorResponse(errorResponse: any): Error {
  if (!isClientError(errorResponse)) {
    return { error: "UNKNOWN_ERROR" };
  }

  const { message, data } = errorResponse.error;

  switch (message) {
    case "USERNAME_NOT_FOUND":
    case "INCORRECT_PASSWORD":
    case "ACCESS_TOKEN_EXPIRED":
    case "SESSION_EXPIRED":
    case "INVALID_REQUEST":
    case "NO_AUTH":
    case "SERVICE_ERROR":
    case "PERMISSION_DENIED":
      return { error: message };

    case "DUPLICATE_RECORD":
      return { error: message, detail: data.detail as string };

    default:
      return { error: "UNKNOWN_ERROR" };
  }
}
