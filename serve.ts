import { serve } from "https://deno.land/std@0.120.0/http/server.ts";
function handler(req: Request): Response {
    return new Response("Hello My dear friedns!!!");
}

console.log("Listening on http://localhost:8080");
await serve(handler, { port: 8080 });
// deno run --allow-net --allow-read serve.ts