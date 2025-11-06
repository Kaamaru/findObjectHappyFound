import { Elysia } from "elysia";
import { autoload } from "elysia-autoload";

const app = new Elysia()
  .use(await autoload())
  .listen(3000);

export type ElysiaApp = typeof app;
