import {
  UserResponse,
  FacilityResponse,
  OrganiserResponse,
  AdminResponse,
} from "@/types/users";

type UserGetCredentialsResponse = {
  data: {
    userDetails: UserResponse;
  };
};

type FacilityGetCredentialsResponse = {
  data: {
    facilityDetails: FacilityResponse;
  };
};

type OrganiserGetCredentialsResponse = {
  data: {
    organiserDetails: OrganiserResponse;
  };
};

type AdminGetCredentialsResponse = {
  data: {
    adminDetails: AdminResponse;
  };
};

export type GetCredentialsResponse =
  | UserGetCredentialsResponse
  | FacilityGetCredentialsResponse
  | OrganiserGetCredentialsResponse
  | AdminGetCredentialsResponse;
