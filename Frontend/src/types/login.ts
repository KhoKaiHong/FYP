import { User, Facility, Organiser, Admin } from "@/types/users";

export type UserLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    userDetails: User;
  };
};

export type FacilityLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    facilityDetails: Facility;
  };
};

export type OrganiserLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    organiserDetails: Organiser;
  };
};

export type AdminLoginResponse = {
  data: {
    accessToken: string;
    refreshToken: string;
    adminDetails: Admin;
  };
};
