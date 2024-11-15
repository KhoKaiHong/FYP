export type District = {
  districtId: number;
  districtName: string;
  stateId: number;
  stateName: string;
};

export type DistrictsResponse = {
  data: {
    districts: District[];
  };
};
