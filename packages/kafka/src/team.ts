export type CreateTeamCommandPayload = {
  name: string;
  founded: number;
  stadium: string;
  city: string;
  country: string;
};

export function createTeamCommand(payload: CreateTeamCommandPayload) {
  return {
    type: "create-team" as const,
    payload,
  };
}

export type CreateTeamCommand = ReturnType<typeof createTeamCommand>;
