type ClientError =
  | { error: "USERNAME_NOT_FOUND" }
  | { error: "INCORRECT_PASSWORD" }
  | { error: "ACCESS_TOKEN_EXPIRED" }
  | { error: "SESSION_EXPIRED" }
  | { error: "INVALID_REQUEST" }
  | { error: "NO_AUTH" }
  | { error: "SERVICE_ERROR" }
  | { error: "DUPLICATE_RECORD"; detail: string }
  | { error: "PERMISSION_DENIED" };

// Type for the error response from the backend
export interface ClientErrorResponse {
  error: {
    message: ClientError["error"];
    data: {
      req_uuid: string;
      detail?: any;
    };
  };
}

export type Error = ClientError | { error: "UNKNOWN_ERROR" };
