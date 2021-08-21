// sdl1000x.rs

use log::*;
use std::{error::Error, fmt::Display, net::{SocketAddr, ToSocketAddrs}};
use tokio_lxi::*;

// #[derive(Debug)]
pub struct SDL1000X {
    addr: SocketAddr,
    dev: LxiDevice,

}

#[allow(dead_code)]
impl SDL1000X {
    pub async fn new<T: Display + AsRef<str> + ToSocketAddrs>(host: T) -> Result<SDL1000X, Box<dyn Error>> {
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
    pub async fn send(&mut self, s: &str)
    -> Result<(), Box<dyn Error>>
    {
        Ok(self.dev.send(s).await?)
    }

    pub async fn receive(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.dev.receive().await?)
    }

    pub async fn cmd(&mut self, s: &str)
    -> Result<String, Box<dyn Error>>
    {
        self.send(s).await?;
        Ok(self.receive().await?)
    }

    pub async fn volt(&mut self) -> Result<f32, Box<dyn Error>> {
        let m = self.cmd("meas:volt?").await?;
        Ok(m.parse::<f32>()?)
    }

    pub async fn curr(&mut self) -> Result<f32, Box<dyn Error>> {
        let m = self.cmd("meas:curr?").await?;
        Ok(m.parse::<f32>()?)
    }

    pub async fn pow(&mut self) -> Result<f32, Box<dyn Error>> {
        let m = self.cmd("meas:pow?").await?;
        Ok(m.parse::<f32>()?)
    }

    pub async fn res(&mut self) -> Result<f32, Box<dyn Error>> {
        let m = self.cmd("meas:res?").await?;
        Ok(m.parse::<f32>()?)
    }

    pub async fn ext(&mut self) -> Result<f32, Box<dyn Error>> {
        let m = self.cmd("meas:ext?").await?;
        Ok(m.parse::<f32>()?)
    }

    // wave type can be "curr", "volt", "pow", "res"
    pub async fn wave<'a, T>(&mut self, m: T)
    -> Result<Vec<f32>, Box<dyn Error>>
    where T: AsRef<str> + Display
    {
        let c = format!("meas:wave? {}", m);
        let w = self.cmd(&c).await?;
        Ok(w.split(',').map(|x| x.parse::<f32>().unwrap_or_default()).collect::<Vec<f32>>())
    }
}

// EOF
