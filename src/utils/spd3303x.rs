// spd3303x.rs

use log::*;
use std::{
    error::Error,
    fmt,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio_lxi::*;

// https://int.siglent.com/upload_file/user/SPD3000X/SPD3303X_QuickStart_QS0503X-E01B.pdf

// #[derive(Debug)]
pub struct SPD3303X {
    addr: SocketAddr,
    dev: LxiDevice,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Ch {
    Ch1,
    Ch2,
    Ch3,
}
impl fmt::Display for Ch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::Ch1 => "CH1",
            Self::Ch2 => "CH2",
            Self::Ch3 => "CH3",
        };
        f.write_str(p)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Meas {
    Volt,
    Curr,
    Pow,
}
impl fmt::Display for Meas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::Volt => "VOLT",
            Self::Curr => "CURR",
            Self::Pow => "POWE",
        };
        f.write_str(p)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum State {
    On,
    Off,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::On => "ON",
            Self::Off => "OFF",
        };
        f.write_str(p)
    }
}

#[allow(dead_code)]
impl SPD3303X {
    pub async fn new<T: fmt::Display + AsRef<str> + ToSocketAddrs>(
        host: T,
    ) -> Result<SPD3303X, Box<dyn Error>> {
        let addr = match host.to_socket_addrs()?.next() {
            None => return Err("Invalid address".into()),
            Some(a) => a,
        };
        debug!("Connecting to {:?}...", addr);
        let mut dev = LxiDevice::connect(&addr).await?;
        dev.set_eol(b"\n");
        Ok(SPD3303X { addr, dev })
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub async fn send(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        debug!("Send: {}", s);
        Ok(self.dev.send(s).await?)
    }

    pub async fn recv(&mut self) -> Result<String, Box<dyn Error>> {
        let r = self.dev.receive().await?;
        debug!("Recv: {}", &r);
        Ok(r)
    }

    pub async fn req(&mut self, s: &str) -> Result<String, Box<dyn Error>> {
        self.send(s).await?;
        Ok(self.recv().await?)
    }

    pub async fn set(&mut self, subsys: &str, v: f32) -> Result<f32, Box<dyn Error>> {
        self.send(&format!("{} {}", subsys, v)).await?;
        Ok(v)
    }

    pub async fn set_state(&mut self, subsys: &str, state: State) -> Result<State, Box<dyn Error>> {
        self.send(&format!("{} {}", subsys, state)).await?;
        Ok(state)
    }

    pub async fn get_state(&mut self, subsys: &str) -> Result<State, Box<dyn Error>> {
        let resp = self.req(&format!("{}?", subsys)).await?;
        Ok(match resp.as_str() {
            "1" | "on" | "ON" => State::On,
            _ => State::Off,
        })
    }

    pub async fn get_stateb(&mut self, subsys: &str) -> Result<bool, Box<dyn Error>> {
        let resp = self.get_state(subsys).await?;
        Ok(matches!(resp, State::On))
    }

    pub async fn meas(&mut self, c: Ch, m: Meas) -> Result<f32, Box<dyn Error>> {
        let m = self.req(&format!("meas:{}? {}", m, c)).await?;
        Ok(m.parse::<f32>()?)
    }
}
// EOF
