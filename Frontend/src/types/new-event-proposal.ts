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