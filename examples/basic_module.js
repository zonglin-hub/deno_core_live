import {print} from "./base.js";

async function hello() {
    return new Promise((res, _rej) => {
        // Deno.core.print("Hello, World! deno");
        print("Hello world!\n");
        res("deno");// deno 未输出
    });
}
await hello();