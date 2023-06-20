//! JSON-RPC client for communication with the SSP/eSSP server over Unix domain sockets.

use alloc::string::String;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use smol_jsonrpc::{Request, Response};

use crate::{EnableEvent, Event, Method, ProtocolVersion, Result};

pub struct Client {
    socket_path: String,
    socket: UnixStream,
}

impl Client {
    /// Creates a new JSON-RPC [Client].
    pub fn new<S>(socket_path: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let socket_path: String = socket_path.into();
        let socket = UnixStream::connect(&socket_path)?;

        Ok(Self {
            socket_path,
            socket,
        })
    }

    /// Gets the socket path.
    pub fn socket_path(&self) -> &str {
        self.socket_path.as_str()
    }

    /// Send a JSON-RPC request to the server, and return the response
    pub fn send(&mut self, method: Method) -> Result<Vec<Event>> {
        let event = if method == Method::Enable {
            Event::from(EnableEvent::from(ProtocolVersion::Eight))
        } else {
            Event::from(method)
        };

        let req = Request::from(event);
        let mut req_json = serde_json::to_string(&req)?;
        req_json += "\n";

        log::debug!("Sending message: {req_json}");

        self.socket.write_all(req_json.as_bytes())?;
        self.socket.flush()?;

        let mut res = vec![0u8; 1024];
        let mut idx = 0;

        log::debug!("Reading response...");

        while let Ok(ret) = self.socket.read(&mut res) {
            if ret == 0 {
                return Ok(Vec::new());
            }

            idx += ret;

            if idx == res.len() {
                res.resize(2 * res.len(), 0u8);
            }

            if res.contains(&b'\n') {
                break;
            }
        }

        if idx == 0 {
            return Ok(Vec::new());
        }

        let res_str = std::str::from_utf8(res[..idx].as_ref())?;

        let mut responses = Vec::with_capacity(10);

        for string in res_str.split('\n') {
            if !string.is_empty() {
                log::info!("server response: {string}");
                let res = match serde_json::from_str::<Response>(string) {
                    Ok(res) => Event::from(res),
                    Err(err) => {
                        log::error!(
                            "Error decoding JSON-RPC response, json: {:?}, error: {err}",
                            string.as_bytes()
                        );
                        return Err(err.into());
                    }
                };
                responses.push(res.into());
            }
        }

        Ok(responses)
    }

    /// Receive messages from the server.
    pub fn receive(&mut self) -> Result<Vec<Event>> {
        let mut res = vec![0u8; 1024];
        let mut idx = 0;

        self.socket.set_nonblocking(true)?;

        while let Ok(ret) = self.socket.read(&mut res) {
            if ret == 0 {
                return Ok(Vec::new());
            }

            idx += ret;

            if idx >= res.len() {
                res.resize(2 * res.len(), 0u8);
            }
        }

        if idx == 0 {
            return Ok(Vec::new());
        }

        let res_str = std::str::from_utf8(res[..idx].as_ref())?;

        let mut responses = Vec::with_capacity(10);

        for string in res_str.split('\n') {
            if !string.is_empty() {
                log::info!("server response: {string}");
                if string.contains("params") {
                    responses.push(serde_json::from_str::<Request>(string)?.into());
                } else if string.contains("result") {
                    responses.push(serde_json::from_str::<Response>(string)?.into());
                } else {
                    responses.push(serde_json::from_str::<Response>(string)?.into());
                }
            }
        }

        Ok(responses)
    }
}
