import { Elysia } from "elysia";
import { teams } from "./controllers/teams";

const app = new Elysia({ prefix: "/api" }).use(teams).listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
