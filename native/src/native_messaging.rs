use anyhow::{bail, Context, Error, Result};
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde_json::{json, Value};
use std::io::{self, prelude::*, Cursor};

pub fn stdin() -> Result<(i64, Value)> {
    let mut buffer = [0; 4];
    io::stdin().read_exact(&mut buffer)?;
    let mut buf = Cursor::new(&buffer);
    let size = buf.read_u32::<NativeEndian>()?;

    let mut data_buffer = vec![0u8; size as usize];
    io::stdin().read_exact(&mut data_buffer)?;
    let message: Value = serde_json::from_slice(&data_buffer)?;
    let rpc_id = match message.get("rpcId") {
        Some(res) => res.as_i64().context("invalid rpcId")?,
        None => bail!("no rpcId given"),
    };

    Ok((rpc_id, message))
}

pub fn stdout_ready() {
    stdout(json!({
        "ready": true,
        "version": env!("CARGO_PKG_VERSION").to_owned(),
    }))
    .unwrap_or_else(|error| eprintln!("{:?}", error));
}

pub fn stdout_reply(rpc_id: i64, reply: Value) {
    stdout(json!({
        "rpcId": rpc_id,
        "reply": reply,
    }))
    .unwrap_or_else(|error| eprintln!("{:?}", error));
}

pub fn stdout_error(rpc_id: i64, error: Error) {
    stdout(json!({
        "rpcId": rpc_id,
        "error": format!("{:?}", error),
    }))
    .unwrap_or_else(|error| eprintln!("{:?}", error));
}

fn stdout(message: Value) -> Result<()> {
    let message = serde_json::to_string(&message)?;
    let mut size = Vec::default();
    size.write_u32::<NativeEndian>(message.len() as u32)?;
    io::stdout().write_all(&size)?;
    io::stdout().write_all(&message.into_bytes())?;
    Ok(io::stdout().flush()?)
}
