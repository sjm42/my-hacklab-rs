// spd3303x.rs

use std::{fmt, net::ToSocketAddrs};

use crate::*;

// https://int.siglent.com/upload_file/user/SPD3000X/SPD3303X_QuickStart_QS0503X-E01B.pdf

#[allow(dead_code)]
pub struct SPD3303X {
    pub lxi: StdLxi,
}

#[allow(dead_code)]
impl SPD3303X {
    pub fn new<H>(name: String, host: H) -> anyhow::Result<Self>
    where
        H: fmt::Display + AsRef<str> + ToSocketAddrs,
        Self: Sized,
    {
        Ok(Self {
            lxi: StdLxi::new(name, host)?,
        })
    }

    pub fn idn(&mut self) -> anyhow::Result<String> {
        self.lxi.req("*IDN?")
    }
    pub fn lan_addr(&mut self) -> anyhow::Result<String> {
        self.lxi.req("ip?")
    }
    pub fn lan_mask(&mut self) -> anyhow::Result<String> {
        self.lxi.req("mask?")
    }
    pub fn lan_gw(&mut self) -> anyhow::Result<String> {
        self.lxi.req("gate?")
    }

    pub fn meas(&mut self, c: Ch, m: Meas) -> anyhow::Result<f32> {
        let m = self.lxi.req(&format!("meas:{m}? {c}"))?;
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
