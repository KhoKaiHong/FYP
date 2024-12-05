// EligibilityStatus type
type EligibilityStatus = "Eligible" | "Ineligible" | "Ineligible - Condition";

// User type
export type UserResponse = {
  id: number;
  icNumber: string;
  name: string;
  email: string;
  phoneNumber: string;
  bloodType: string;
  eligibility: EligibilityStatus;
  stateId: number;
  districtId: number;
  stateName: string;
  districtName: string;
};

// Facility type
export type FacilityResponse = {
  id: number;
  email: string;
  name: string;
  address: string;
  phoneNumber: string;
  stateId: number;
  stateName: string;
};

// Organiser type
export type OrganiserResponse = {
  id: number;
  email: string;
  name: string;
  phoneNumber: string;
};

// Admin type
export type AdminResponse = {
  id: number;
  email: string;
  name: string;
};

export type User = {
  role: "User"
} &UserResponse;

export type Facility = {
  role: "Facility"
} &FacilityResponse;

export type Organiser = {
  role: "Organiser"
} &OrganiserResponse;

export type Admin = {
  role: "Admin"
} &AdminResponse;

export type Users =
  | User
  | Facility
  | Organiser
  | Admin;


