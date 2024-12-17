export type EventRegistrationPayload = {
  eventId: number;
};

export type EventRegistrationResponse = {
  data: {
    registrationId: number;
  };
};

export type EventRegistration = {
  id: number;
  status: string;
  eventId: number;
  eventLocation: string;
  eventAddress: string;
  eventStartTime: string;
  eventEndTime: string;
  eventMaxAttendees: number;
  eventLatitude: number;
  eventLongitude: number;
  userId: number;
  userIcNumber: string;
  userName: string;
  userEmail: string;
  userPhoneNumber: string;
  userBloodType: string;
  registeredAt: string;
};


export type ListRegistrationsResponse = {
  data: {
    registrations: EventRegistration[];
  };
};

export type ListRegistrationsByEventIdPayload = {
  eventId: number;
};

export type UpdateRegistrationPayload = {
  registrationId: number;
  status: string;
};

export type UpdateRegistrationResponse = {
  data: {
    success: boolean;
  };
};
