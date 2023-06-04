use deno_core::{
    anyhow::{Ok, Result},
    serde::de::DeserializeOwned,
    serde_v8,
    v8::Local,
    FastString, JsRuntime, RuntimeOptions,
};

async fn eval<T>(rt: &mut JsRuntime, code: &'static str) -> Result<T>
where
    T: DeserializeOwned,
{
    let ret = rt.execute_script("<anon>", FastString::StaticAscii(code))?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}

#[tokio::test]
async fn test() -> Result<()> {
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    let ret: String = eval(&mut rt, code).await?;
    println!("Rust: {:?}", ret);
    Ok(())
}
