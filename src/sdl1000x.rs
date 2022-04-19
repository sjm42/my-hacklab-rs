// sdl1000x.rs
#![allow(dead_code)]

use anyhow::anyhow;
use std::{fmt, net::ToSocketAddrs};

use crate::*;

// https://int.siglent.com/upload_file/user/SDL1000X/SDL1000X_Programming_Guide_V1.0.pdf

pub struct SDL1000X {
    pub lxi: StdLxi,
}

impl SDL1000X {
    pub fn new<S, H>(name: S, host: H) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
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
        self.lxi.send(&format!(":func {func}"))?;
        Ok(func)
    }
    pub fn get_func(&mut self) -> anyhow::Result<Func> {
        self.lxi.req(":func?")?;

        Ok(Func::Curr)
    }

    pub fn meas(&mut self, m: Meas) -> anyhow::Result<f32> {
        match m {
            Meas::Volt | Meas::Curr | Meas::Pow | Meas::Res | Meas::Ext => {}
            _ => {
                return Err(anyhow!("Device cannot measure {m}"));
            }
        }

        let m = self.lxi.req(&format!("meas:{m}?"))?;
        Ok(m.parse::<f32>()?)
    }
    pub fn m_volt(&mut self) -> anyhow::Result<f32> {
        self.meas(Meas::Volt)
    }
    pub fn m_curr(&mut self) -> anyhow::Result<f32> {
        self.meas(Meas::Curr)
    }
    pub fn m_pow(&mut self) -> anyhow::Result<f32> {
        self.meas(Meas::Pow)
    }
    pub fn m_res(&mut self) -> anyhow::Result<f32> {
        self.meas(Meas::Res)
    }
    pub fn m_ext(&mut self) -> anyhow::Result<f32> {
        self.meas(Meas::Ext)
    }

    // wave type can be "curr", "volt", "pow", "res"
    pub fn wave(&mut self, m: Meas) -> anyhow::Result<Vec<f32>> {
        let c = format!("meas:wave? {m}");
        let w = self.lxi.req(&c)?;
        Ok(w.split(',')
            .map(|x| x.parse::<f32>().unwrap_or_default())
            .collect::<Vec<f32>>())
    }

    pub fn sense(&mut self, state: PortState) -> anyhow::Result<PortState> {
        self.lxi.set_state("system:sense", state)
    }
    pub fn sense_on(&mut self) -> anyhow::Result<PortState> {
        self.sense(PortState::On)
    }
    pub fn sense_off(&mut self) -> anyhow::Result<PortState> {
        self.sense(PortState::Off)
    }
    pub fn q_sense(&mut self) -> anyhow::Result<PortState> {
        self.lxi.get_state("system:sense")
    }

    pub fn input(&mut self, state: PortState) -> anyhow::Result<PortState> {
        self.lxi.set_state(":input:state", state)
    }
    pub fn input_on(&mut self) -> anyhow::Result<PortState> {
        self.input(PortState::On)
    }
    pub fn input_off(&mut self) -> anyhow::Result<PortState> {
        self.input(PortState::Off)
    }
    pub fn q_input(&mut self) -> anyhow::Result<PortState> {
        self.lxi.get_state(":input:state")
    }

    pub fn short(&mut self, state: PortState) -> anyhow::Result<PortState> {
        self.lxi.set_state(":short:state", state)
    }
    pub fn short_on(&mut self) -> anyhow::Result<PortState> {
        self.short(PortState::On)
    }
    pub fn short_off(&mut self) -> anyhow::Result<PortState> {
        self.short(PortState::Off)
    }
    pub fn q_short(&mut self) -> anyhow::Result<PortState> {
        self.lxi.get_state(":short:state")
    }
}

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

// EOF
