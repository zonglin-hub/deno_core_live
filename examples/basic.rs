use deno_core::{
    anyhow::{Ok, Result},
    serde_v8,
    v8::Local,
    FastString, JsRuntime, RuntimeOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    let ret = rt.execute_script("<anon>", FastString::StaticAscii(code))?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = Local::new(scope, result);
    let result: String = serde_v8::from_v8(scope, result)?;
    println!("{:#?}", result);
    Ok(())
}
