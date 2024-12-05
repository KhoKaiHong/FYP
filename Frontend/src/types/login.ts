import {
  UserResponse,
  FacilityResponse,
  OrganiserResponse,
  AdminResponse,
} from "@/types/users";

export type UserLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    userDetails: UserResponse;
  };
};

export type FacilityLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    facilityDetails: FacilityResponse;
  };
};

export type OrganiserLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    organiserDetails: OrganiserResponse;
  };
};

export type AdminLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    adminDetails: AdminResponse;
  };
};

export type UserLoginPayload = {
  icNumber: string;
  password: string;
};

export type FacilityLoginPayload = {
  email: string;
  password: string;
};

export type OrganiserLoginPayload = {
  email: string;
  password: string;
};

export type AdminLoginPayload = {
  email: string;
  password: string;
};
