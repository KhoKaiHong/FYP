export type PostChangeEventRequestPayload = {
    location: string,
    address: string,
    startTime: string,
    endTime: string,
    maxAttendees: number,
    latitude: number,
    longitude: number,
    changeReason: string,
    eventId: number,
};

export type PostChangeEventRequestResponse = {
  data: {
    success: boolean;
  };
}

export type ChangeEventRequest = {
  id: number;
  location: string;
  address: string;
  startTime: string;
  endTime: string;
  maxAttendees: number;
  latitude: number;
  longitude: number;
  status: string;
  changeReason: string;
  rejectionReason: string | null;
  eventId: number;
  facilityId: number;
  facilityEmail: string;
  facilityName: string;
  facilityAddress: string;
  facilityPhoneNumber: string;
  organiserId: number;
  organiserEmail: string;
  organiserName: string;
  organiserPhoneNumber: string;
  stateId: number;
  stateName: string;
  districtId: number;
  districtName: string;
};

export type ListChangeEventRequestResponse = {
  data: {
    eventRequests: ChangeEventRequest[];
  };
};

export type UpdateChangeEventRequestPayload = {
  id: number;
  status: string;
  rejectionReason?: string;
};

export type UpdateChangeEventRequestResponse = {
  data: {
    success: boolean;
  };
}