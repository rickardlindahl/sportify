import { Elysia } from "elysia";

export const teams = new Elysia().post("/teams", () => "Team created");
