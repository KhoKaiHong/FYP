type ClientError =
  | { message: "EMAIL_NOT_FOUND" }
  | { message: "IC_NOT_FOUND" }
  | { message: "INCORRECT_PASSWORD" }
  | { message: "ACCESS_TOKEN_EXPIRED" }
  | { message: "SESSION_EXPIRED" }
  | { message: "INVALID_REQUEST" }
  | { message: "NO_AUTH" }
  | { message: "SERVICE_ERROR" }
  | { message: "DUPLICATE_RECORD"; detail: string }
  | { message: "PERMISSION_DENIED" }
  | { message: "EVENT_AT_CAPACITY" }
  | { message: "EXISTING_EVENT_REGISTRATION" }
  | { message: "CURRENT_PASSWORD_NOT_MATCHING" }
  | { message: "EXISTING_NEW_EVENT_REQUEST" }
  | { message: "EXISTING_CHANGE_EVENT_REQUEST" };

// Type for the error response from the backend
export interface ClientErrorResponse {
  error: {
    message: ClientError["message"];
    data: {
      req_uuid: string;
      detail?: unknown;
    };
  };
}

export type AppError = ClientError | { message: "UNKNOWN_ERROR" };
