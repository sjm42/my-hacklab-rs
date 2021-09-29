// spd3303x.rs

use lxi::*;
use std::{fmt, net::SocketAddr};

use crate::StdLxi;

// https://int.siglent.com/upload_file/user/SPD3000X/SPD3303X_QuickStart_QS0503X-E01B.pdf

#[allow(dead_code)]
// #[derive(Debug)]
pub struct SPD3303X {
    addr: SocketAddr,
    name: String,
    pub verbose: bool,
    lxi_dev: LxiTextDevice,
}

impl StdLxi for SPD3303X {
    fn create(addr: SocketAddr, name: String, lxi_dev: LxiTextDevice) -> Self {
        SPD3303X {
            addr,
            name,
            verbose: false,
            lxi_dev,
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn verbose(&self) -> bool {
        self.verbose
    }
    fn dev(&mut self) -> &mut LxiTextDevice {
        &mut self.lxi_dev
    }
}

#[allow(dead_code)]
impl SPD3303X {
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn meas(&mut self, c: Ch, m: Meas) -> anyhow::Result<f32> {
        let m = self.req(&format!("meas:{}? {}", m, c))?;
        Ok(m.parse::<f32>()?)
    }
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
// EOF
