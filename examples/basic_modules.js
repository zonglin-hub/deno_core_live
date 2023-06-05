async function hello() {
    return new Promise((res, _rej) => {
        Deno.core.print("Hello, World! deno");
        res("deno");// deno 未输出
    });
}
await hello();