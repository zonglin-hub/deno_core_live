// Deno.core.print("Hello, World!");

async function hello() {
    return new Promise((res, _rej) => {
        Deno.core.print("Hello, World! ");
        res("deno");
    });
}

hello();