import { User, Facility, Organiser, Admin } from "@/types/users";

type UserGetCredentialsResponse = {
  data: {
    userDetails: User;
  };
};

type FacilityGetCredentialsResponse = {
  data: {
    facilityDetails: Facility;
  };
};

type OrganiserGetCredentialsResponse = {
  data: {
    organiserDetails: Organiser;
  };
};

type AdminGetCredentialsResponse = {
  data: {
    adminDetails: Admin;
  };
};

export type GetCredentialsResponse =
  | UserGetCredentialsResponse
  | FacilityGetCredentialsResponse
  | OrganiserGetCredentialsResponse
  | AdminGetCredentialsResponse;
