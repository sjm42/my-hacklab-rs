// scpi.rs

use anyhow::anyhow;
use log::*;
use lxi::*;
use num::traits::Float;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{fmt, fmt::Display, time};

#[allow(dead_code)]
#[derive(Debug)]
pub enum PortState {
    On,
    Off,
}
impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::On => "ON",
            Self::Off => "OFF",
        };
        f.write_str(p)
    }
}

pub struct StdLxi {
    pub name: String,
    pub addr: SocketAddr,
    pub v: bool,
    pub lxi_dev: LxiTextDevice,
}

impl LxiCommands for StdLxi {
    fn create(name: String, addr: SocketAddr, lxi_dev: LxiTextDevice) -> Self {
        StdLxi {
            name,
            addr,
            v: false,
            lxi_dev,
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn addr(&self) -> SocketAddr {
        self.addr
    }
    fn get_v(&self) -> bool {
        self.v
    }
    fn set_v(&mut self, v: bool) {
        self.v = v;
    }
    fn dev(&mut self) -> &mut LxiTextDevice {
        &mut self.lxi_dev
    }
}

pub trait LxiCommands {
    fn create(name: String, addr: SocketAddr, lxi_dev: LxiTextDevice) -> Self;
    fn name(&self) -> &str;
    fn addr(&self) -> SocketAddr;
    fn get_v(&self) -> bool;
    fn set_v(&mut self, v: bool);
    fn dev(&mut self) -> &mut LxiTextDevice;

    fn new<H>(name: String, host: H) -> anyhow::Result<Self>
    where
        H: fmt::Display + AsRef<str> + ToSocketAddrs,
        Self: Sized,
    {
        let addr = match host.to_socket_addrs()?.next() {
            None => return Err(anyhow!("Invalid address: {host}")),
            Some(a) => a,
        };
        let mut lxi_dev = LxiTextDevice::new(
            (addr.ip().to_string(), addr.port()),
            Some(time::Duration::new(5, 0)),
        );
        debug!("Connecting to {addr:?}...");
        // let dev = LxiDevice::new(addr, timeout)
        lxi_dev.connect()?;
        Ok(Self::create(name, addr, lxi_dev))
    }
    fn v(&self) -> bool {
        self.get_v()
    }
    fn v_on(&mut self) {
        self.set_v(true);
    }
    fn v_off(&mut self) {
        self.set_v(false);
    }
    fn v_set(&mut self, v: bool) {
        self.set_v(v)
    }
    fn q_send<S>(&mut self, s: S) -> anyhow::Result<()>
    where
        S: AsRef<str> + Display,
    {
        Ok(self.dev().send(s.as_ref().as_bytes())?)
    }

    fn send<S>(&mut self, s: S) -> anyhow::Result<()>
    where
        S: AsRef<str> + Display,
    {
        if self.v() {
            info!("Send: {name} <-- {s}", name = self.name());
        }
        self.q_send(s.as_ref())
    }

    fn q_recv(&mut self) -> anyhow::Result<String> {
        let byt = self.dev().receive()?;
        let str = String::from_utf8_lossy(&byt);
        Ok(str.into_owned())
    }

    fn recv(&mut self) -> anyhow::Result<String> {
        let s = self.q_recv()?;
        if self.v() {
            info!("Recv: {name} --> {s}", name = self.name());
        }
        Ok(s)
    }

    fn req<S>(&mut self, s: S) -> anyhow::Result<String>
    where
        S: AsRef<str> + Display,
    {
        self.q_send(s.as_ref())?;
        let r = self.q_recv()?;
        if self.v() {
            info!("{} --> {} --> {r}", s.as_ref(), self.name());
        }
        Ok(r)
    }

    fn set<S, F>(&mut self, subsys: S, v: F) -> anyhow::Result<F>
    where
        S: AsRef<str> + Display,
        F: Float + Display,
    {
        self.send(&format!("{} {v}", subsys.as_ref()))?;
        Ok(v)
    }

    fn set_state<S>(&mut self, subsys: S, state: PortState) -> anyhow::Result<PortState>
    where
        S: AsRef<str> + Display,
    {
        self.send(&format!("{} {state}", subsys.as_ref()))?;
        Ok(state)
    }

    fn get_state<S>(&mut self, subsys: S) -> anyhow::Result<PortState>
    where
        S: AsRef<str> + Display,
    {
        let resp = self.req(&format!("{}?", subsys.as_ref()))?;
        Ok(match resp.as_str() {
            "1" | "on" | "ON" => PortState::On,
            _ => PortState::Off,
        })
    }

    fn get_stateb<S>(&mut self, subsys: S) -> anyhow::Result<bool>
    where
        S: AsRef<str> + Display,
    {
        let resp = self.get_state(subsys.as_ref())?;
        Ok(matches!(resp, PortState::On))
    }
}
// EOF
