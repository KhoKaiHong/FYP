export type RegisterResponse = {
  data: {
    success: boolean;
  };
};

export type UserRegisterPayload = {
  icNumber: string;
  password: string;
  name: string;
  email: string;
  phoneNumber: string;
  bloodType: string;
  stateId: number;
  districtId: number;
};
