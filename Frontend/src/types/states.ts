export type State = {
  id: number;
  name: string;
};

export type StatesResponse = {
  data: {
    states: State[];
  };
};
