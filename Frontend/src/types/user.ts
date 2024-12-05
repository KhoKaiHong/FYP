export type UserUpdatePayload = {
    currentPassword?: string,
    password?: string,
    email?: string,
    phoneNumber?: string,
    stateId?: number,
    districtId?: number,
};

export type UserUpdateResponse = {
  data: {
    success: boolean;
  };
}