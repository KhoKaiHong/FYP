import { AppError, ClientErrorResponse } from "@/types/error";

// Type guard to check if an error response is a client error from backend
function isClientError(
  errorResponse: unknown
): errorResponse is ClientErrorResponse {
  return (
    typeof errorResponse === "object" &&
    errorResponse !== null &&
    "error" in errorResponse &&
    typeof (errorResponse as ClientErrorResponse).error === "object" &&
    (errorResponse as ClientErrorResponse).error !== null &&
    "message" in (errorResponse as ClientErrorResponse).error &&
    typeof (errorResponse as ClientErrorResponse).error.message === "string" &&
    "data" in (errorResponse as ClientErrorResponse).error &&
    typeof (errorResponse as ClientErrorResponse).error.data === "object" &&
    (errorResponse as ClientErrorResponse).error.data !== null &&
    "req_uuid" in (errorResponse as ClientErrorResponse).error.data &&
    typeof (errorResponse as ClientErrorResponse).error.data.req_uuid ===
      "string"
  );
}

// Function to parse any error response into a ClientError or UNKNOWN_ERROR
export function parseErrorResponse(errorResponse: unknown): AppError {
  if (!isClientError(errorResponse)) {
    return { message: "UNKNOWN_ERROR" };
  }

  const { message, data } = errorResponse.error;

  switch (message) {
    case "EMAIL_NOT_FOUND":
    case "IC_NOT_FOUND":
    case "INCORRECT_PASSWORD":
    case "ACCESS_TOKEN_EXPIRED":
    case "SESSION_EXPIRED":
    case "INVALID_REQUEST":
    case "NO_AUTH":
    case "SERVICE_ERROR":
    case "PERMISSION_DENIED":
      return { message };

    case "DUPLICATE_RECORD":
      return { message, detail: data.detail as string };

    default:
      return { message: "UNKNOWN_ERROR" };
  }
}

export function getErrorMessage(error: AppError): string {
  switch (error.message) {
    case "EMAIL_NOT_FOUND":
      return "Email not found.";
    case "IC_NOT_FOUND":
      return "Identification Card not registered.";
    case "INCORRECT_PASSWORD":
      return "The password you entered is incorrect.";
    case "ACCESS_TOKEN_EXPIRED":
      return "Access Token Expired.";
    case "SESSION_EXPIRED":
      return "Your session has expired. Please log in again.";
    case "INVALID_REQUEST":
      return "Invalid request. Please try again.";
    case "NO_AUTH":
      return "You are not authorized to perform this action. Please log in again.";
    case "SERVICE_ERROR":
      return "A server error occurred. Please try again later.";
    case "DUPLICATE_RECORD":
      return `Duplicate record found: ${error.detail}`;
    case "PERMISSION_DENIED":
      return "You do not have permission to access this resource.";
    case "UNKNOWN_ERROR":
      return "An unknown error occurred. Please try again later.";
    default:
      return "An unspecified error occurred.";
  }
}
