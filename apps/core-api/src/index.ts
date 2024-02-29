import { Elysia } from "elysia";
import { teams } from "./controllers/teams";
import { setupProducers } from "./libs/kafka";

await setupProducers();

const app = new Elysia({ prefix: "/api" }).use(teams).listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
