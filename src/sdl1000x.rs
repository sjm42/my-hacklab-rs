// sdl1000x.rs

use std::{fmt, net::ToSocketAddrs};

use crate::*;

// https://int.siglent.com/upload_file/user/SDL1000X/SDL1000X_Programming_Guide_V1.0.pdf

#[allow(dead_code)]
pub struct SDL1000X {
    pub lxi: StdLxi,
}

#[allow(dead_code)]
impl SDL1000X {
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
        self.lxi.req("lan:ipad?")
    }
    pub fn lan_mask(&mut self) -> anyhow::Result<String> {
        self.lxi.req("lan:smask?")
    }
    pub fn lan_gw(&mut self) -> anyhow::Result<String> {
        self.lxi.req("lan:gat?")
    }
    pub fn lan_mac(&mut self) -> anyhow::Result<String> {
        self.lxi.req("lan:mac?")
    }

    pub fn set_func(&mut self, func: Func) -> anyhow::Result<Func> {
        self.lxi.send(&format!("FUNC {func}"))?;
        Ok(func)
    }

    pub fn meas(&mut self, m: Meas) -> anyhow::Result<f32> {
        let m = self.lxi.req(&format!("meas:{m}?"))?;
        Ok(m.parse::<f32>()?)
    }

    // wave type can be "curr", "volt", "pow", "res"
    pub fn wave(&mut self, m: Meas) -> anyhow::Result<Vec<f32>> {
        let c = format!("meas:wave? {m}");
        let w = self.lxi.req(&c)?;
        Ok(w.split(',')
            .map(|x| x.parse::<f32>().unwrap_or_default())
            .collect::<Vec<f32>>())
    }
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
// EOF
