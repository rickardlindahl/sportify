import { Elysia, t } from "elysia";
import { createTeamCommand } from "@sportify/kafka/team";
import { producer } from "../libs/kafka";

export const teams = new Elysia().post(
  "/teams",
  async ({ body }) => {
    await producer.send({
      topic: "team",
      messages: [
        {
          value: JSON.stringify(createTeamCommand(body)),
        },
      ],
    });

    return { status: 201, message: "Team created" };
  },
  {
    body: t.Object({
      name: t.String(),
      founded: t.Number(),
      stadium: t.String(),
      city: t.String(),
      country: t.String(),
    }),
  }
);
