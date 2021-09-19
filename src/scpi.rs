// scpi.rs

use log::*;
use lxi::*;
use num::traits::Float;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{error::Error, fmt, fmt::Display, time};

pub trait StdLxi {
    fn create(addr: SocketAddr, name: String, lxi_dev: LxiTextDevice) -> Self;
    fn name(&self) -> &str;
    fn verbose(&self) -> bool;
    fn dev(&mut self) -> &mut LxiTextDevice;

    fn new<H>(host: H, name: String) -> Result<Self, Box<dyn Error>>
    where
        H: fmt::Display + AsRef<str> + ToSocketAddrs,
        Self: Sized,
    {
        let addr = match host.to_socket_addrs()?.next() {
            None => return Err("Invalid address".into()),
            Some(a) => a,
        };
        let mut lxi_dev = LxiTextDevice::new(
            (addr.ip().to_string(), addr.port()),
            Some(time::Duration::new(5, 0)),
        );
        debug!("Connecting to {:?}...", addr);
        // let dev = LxiDevice::new(addr, timeout)
        lxi_dev.connect()?;
        Ok(Self::create(addr, name, lxi_dev))
    }

    fn q_send<S>(&mut self, s: S) -> Result<(), Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        Ok(self.dev().send(s.as_ref().as_bytes())?)
    }

    fn send<S>(&mut self, s: S) -> Result<(), Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        if self.verbose() {
            info!("Send: {} <-- {}", self.name(), s);
        }
        self.q_send(s.as_ref())
    }

    fn q_recv(&mut self) -> Result<String, Box<dyn Error>> {
        let byt = self.dev().receive()?;
        let str = String::from_utf8_lossy(&byt);
        Ok(str.into_owned())
    }

    fn recv(&mut self) -> Result<String, Box<dyn Error>> {
        let s = self.q_recv()?;
        if self.verbose() {
            info!("Recv: {} --> {}", self.name(), &s);
        }
        Ok(s)
    }

    fn req<S>(&mut self, s: S) -> Result<String, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        self.q_send(s.as_ref())?;
        let r = self.q_recv()?;
        if self.verbose() {
            info!("{} --> {} --> {}", s.as_ref(), self.name(), &r);
        }
        Ok(r)
    }

    fn set<S, F>(&mut self, subsys: S, v: F) -> Result<F, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
        F: Float + Display,
    {
        self.send(&format!("{} {}", subsys.as_ref(), v))?;
        Ok(v)
    }

    fn set_state<S>(&mut self, subsys: S, state: PortState) -> Result<PortState, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        self.send(&format!("{} {}", subsys.as_ref(), state))?;
        Ok(state)
    }

    fn get_state<S>(&mut self, subsys: S) -> Result<PortState, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        let resp = self.req(&format!("{}?", subsys.as_ref()))?;
        Ok(match resp.as_str() {
            "1" | "on" | "ON" => PortState::On,
            _ => PortState::Off,
        })
    }

    fn get_stateb<S>(&mut self, subsys: S) -> Result<bool, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        let resp = self.get_state(subsys.as_ref())?;
        Ok(matches!(resp, PortState::On))
    }
}

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
// EOF
