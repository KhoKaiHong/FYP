export type AdminUpdatePayload = {
    currentPassword?: string,
    password?: string,
    name?: string,
    email?: string,
};

export type AdminUpdateResponse = {
  data: {
    success: boolean;
  };
}