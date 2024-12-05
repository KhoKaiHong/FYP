export type OrganiserUpdatePayload = {
    currentPassword?: string,
    password?: string,
    name?: string,
    email?: string,
    phoneNumber?: string,
};

export type OrganiserUpdateResponse = {
  data: {
    success: boolean;
  };
}