// EligibilityStatus type
type EligibilityStatus = "Eligible" | "Ineligible" | "Ineligible - Condition";

// UserWithLocation type
export type User = {
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

// FacilityWithLocation type
export type Facility = {
  id: number;
  email: string;
  name: string;
  address: string;
  phoneNumber: string;
  stateId: number;
  stateName: string;
};

// Organiser type
export type Organiser = {
  id: number;
  email: string;
  name: string;
  phoneNumber: string;
};

// Admin type
export type Admin = {
  id: number;
  email: string;
  name: string;
};
