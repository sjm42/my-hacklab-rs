// scpi.rs

use log::*;
use lxi::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{error::Error, fmt, fmt::Display, time};

pub trait StdLxi {
    fn create(addr: SocketAddr, lxi_dev: LxiTextDevice) -> Self;
    fn dev(&mut self) -> &mut LxiTextDevice;

    fn new<H>(host: H) -> Result<Self, Box<dyn Error>>
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
        Ok(Self::create(addr, lxi_dev))
    }

    fn send<S>(&mut self, s: S) -> Result<(), Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        debug!("Send: {}", s);
        Ok(self.dev().send(s.as_ref().as_bytes())?)
    }

    fn recv(&mut self) -> Result<String, Box<dyn Error>> {
        let byt = self.dev().receive()?;
        let str = String::from_utf8_lossy(&byt);
        debug!("Recv: {}", &str);
        Ok(str.into_owned())
    }

    fn req<S>(&mut self, s: S) -> Result<String, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        self.send(s)?;
        self.recv()
    }

    fn set<S>(&mut self, subsys: S, v: f32) -> Result<f32, Box<dyn Error>>
    where
        S: AsRef<str> + Display,
    {
        self.send(&format!("{} {}", subsys, v))?;
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
