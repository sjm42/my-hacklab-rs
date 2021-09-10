// sdl1000x.rs

use log::*;
use std::{
    error::Error,
    fmt,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio_lxi::*;

// https://int.siglent.com/upload_file/user/SDL1000X/SDL1000X_Programming_Guide_V1.0.pdf

// #[derive(Debug)]
pub struct SDL1000X {
    addr: SocketAddr,
    dev: LxiDevice,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Func {
    Curr,
    Volt,
    Pow,
    Res,
    Led,
}
impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::Curr => "CURR",
            Self::Volt => "VOLT",
            Self::Pow => "POW",
            Self::Res => "RES",
            Self::Led => "LED",
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
    Res,
    Ext, // don't ask
}
impl fmt::Display for Meas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::Volt => "VOLT",
            Self::Curr => "CURR",
            Self::Pow => "POW",
            Self::Res => "RES",
            Self::Ext => "EXT",
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
impl SDL1000X {
    pub async fn new<T: fmt::Display + AsRef<str> + ToSocketAddrs>(
        host: T,
    ) -> Result<SDL1000X, Box<dyn Error>> {
        let addr = match host.to_socket_addrs()?.next() {
            None => return Err("Invalid address".into()),
            Some(a) => a,
        };
        debug!("Connecting to {:?}...", addr);
        let dev = LxiDevice::connect(&addr).await?;
        Ok(SDL1000X { addr, dev })
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

    pub async fn set_func(&mut self, func: Func) -> Result<Func, Box<dyn Error>> {
        self.send(&format!("FUNC {}", func)).await?;
        Ok(func)
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

    pub async fn meas(&mut self, m: Meas) -> Result<f32, Box<dyn Error>> {
        let m = self.req(&format!("meas:{}?", m)).await?;
        Ok(m.parse::<f32>()?)
    }

    // wave type can be "curr", "volt", "pow", "res"
    pub async fn wave(&mut self, m: Meas) -> Result<Vec<f32>, Box<dyn Error>> {
        let c = format!("meas:wave? {}", m);
        let w = self.req(&c).await?;
        Ok(w.split(',')
            .map(|x| x.parse::<f32>().unwrap_or_default())
            .collect::<Vec<f32>>())
    }
}

// EOF
