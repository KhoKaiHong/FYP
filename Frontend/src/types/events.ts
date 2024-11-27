export type Event = {
    id: number,
    location: string
    address: string,
    startTime: string,
    endTime: string,
    currentAttendees: number,
    maxAttendees: number,
    latitude: number,
    longitude: number,
    facilityId: number,
    facilityEmail: string,
    facilityName: string,
    facilityAddress: string,
    facilityPhoneNumber: string,
    organiserId: number,
    organiserEmail: string,
    organiserName: string,
    organiserPhoneNumber: string,
    stateId: number,
    stateName: string,
    districtId: number,
    districtName: string,
};


export type EventsResponse = {
  data: {
    events: Event[];
  };
}
