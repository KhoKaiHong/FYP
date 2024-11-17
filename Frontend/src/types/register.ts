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

export type OrganiserRegisterPayload = {
  email: string;
  password: string;
  name: string;
  phoneNumber: string;
};

export type FacilityRegisterPayload = {
  email: string;
  password: string;
  name: string;
  address: string;
  phoneNumber: string;
  stateId: number;
};

export type AdminRegisterPayload = {
  email: string;
  password: string;
  name: string;
}
