export type PostNewEventProposalPayload = {
    location: string,
    address: string,
    startTime: string,
    endTime: string,
    maxAttendees: number,
    latitude: number,
    longitude: number,
    facilityId: number,
    stateId: number,
    districtId: number,
};

export type PostNewEventProposalResponse = {
  data: {
    success: boolean;
  };
}

export type NewEventProposal = {
  id: number;
  location: string;
  address: string;
  startTime: string;
  endTime: string;
  maxAttendees: number;
  latitude: number;
  longitude: number;
  status: string;
  rejectionReason: string | null;
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

export type ListNewEventProposalResponse = {
  data: {
    eventRequests: NewEventProposal[];
  };
};

export type UpdateNewEventProposalPayload = {
  id: number;
  status: string;
  rejectionReason?: string;
};

export type UpdateNewEventProposalResponse = {
  data: {
    success: boolean;
  };
}