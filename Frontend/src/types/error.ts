type ClientError =
  | { message: "USERNAME_NOT_FOUND" }
  | { message: "INCORRECT_PASSWORD" }
  | { message: "ACCESS_TOKEN_EXPIRED" }
  | { message: "SESSION_EXPIRED" }
  | { message: "INVALID_REQUEST" }
  | { message: "NO_AUTH" }
  | { message: "SERVICE_ERROR" }
  | { message: "DUPLICATE_RECORD"; detail: string }
  | { message: "PERMISSION_DENIED" };

// Type for the error response from the backend
export interface ClientErrorResponse {
  error: {
    message: ClientError["message"];
    data: {
      req_uuid: string;
      detail?: any;
    };
  };
}

export type Error = ClientError | { message: "UNKNOWN_ERROR" };
