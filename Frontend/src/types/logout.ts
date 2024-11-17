export type LogoutResponse = {
  data: {
    success: boolean;
  };
};

export type LogoutPayload = {
  refreshToken: string;
};
