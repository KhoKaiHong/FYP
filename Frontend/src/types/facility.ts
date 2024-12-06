export type FacilityUpdatePayload = {
    currentPassword?: string,
    password?: string,
    name?: string,
    address?: string,
    email?: string,
    phoneNumber?: string,
};

export type FacilityUpdateResponse = {
  data: {
    success: boolean;
  };
}