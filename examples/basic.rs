use deno_core::{
    anyhow::{Ok, Result},
    resolve_url_or_path,
    serde::de::DeserializeOwned,
    serde_v8,
    v8::Local,
    FastString, FsModuleLoader, JsRuntime, RuntimeOptions, Snapshot,
};
use std::{env::current_dir, rc::Rc};

#[allow(dead_code)]
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

// cargo test --package deno_core_live --example basic -- test --exact --nocapture
#[tokio::test]
async fn test() -> Result<()> {
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    let ret: String = eval(&mut rt, code).await?;
    println!("Rust: {:?}", ret);
    Ok(())
}

#[tokio::test]
async fn test1() -> Result<()> {
    let cwd = current_dir().unwrap();
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let expected_url = "file:///D:/.github/deno_core_live/examples/basic_modules.js";
    let url = resolve_url_or_path(expected_url, &cwd)?;
    let id = rt.load_main_module(&url, None).await?;
    rt.mod_evaluate(id).await??;
    rt.run_event_loop(false).await?;
    Ok(())
}

#[tokio::test]
async fn test2() -> Result<()> {
    let cwd = current_dir().unwrap();
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let expected_url = "file:///D:/.github/deno_core_live/examples/basic_module.js";
    let url = resolve_url_or_path(expected_url, &cwd)?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")?;
        }
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("failed to evaluate module")?;
        }
    }
    Ok(())
}

use lazy_static::lazy_static;

lazy_static! {
    static ref SNAPSHOT: &'static [u8] = {
        let data = include_bytes!("../snapshot/main.bin");
        let decompressed = zstd::decode_all(&data[..]).unwrap().into_boxed_slice();
        Box::leak(decompressed)
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = current_dir().unwrap();
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(&*SNAPSHOT)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let expected_url = "file:///D:/.github/deno_core_live/examples/basic_module.js";
    let url = resolve_url_or_path(expected_url, &cwd)?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")?;
        }
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("failed to evaluate module")?;
        }
    }
    Ok(())
}
