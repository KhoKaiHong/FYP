export type DonationHistory = {
  id: number;
  userId: number;
  userIcNumber: string;
  userName: string;
  userEmail: string;
  userPhoneNumber: string;
  userBloodType: string;
  eventId?: number;
  eventLocation?: string;
  eventAddress?: string;
  eventStartTime?: string;
  eventEndTime?: string;
  eventLatitude?: number;
  eventLongitude?: number;
  createdAt: string;
};

export type DonationHistoryResponse = {
  data: {
    donationHistory: DonationHistory[];
  };
};
