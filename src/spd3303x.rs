// spd3303x.rs
#![allow(dead_code)]

use anyhow::anyhow;
use num::traits::Float;
use std::{fmt, fmt::Display, net::ToSocketAddrs};

use crate::*;

// https://int.siglent.com/upload_file/user/SPD3000X/SPD3303X_QuickStart_QS0503X-E01B.pdf

pub struct SPD3303X {
    pub lxi: StdLxi,
}

impl SPD3303X {
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
    pub fn version(&mut self) -> anyhow::Result<String> {
        self.lxi.req("system: version?")
    }
    pub fn status(&mut self) -> anyhow::Result<String> {
        self.lxi.req("system: status?")
    }
    pub fn q_error(&mut self) -> anyhow::Result<String> {
        self.lxi.req("system: error?")
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
        match c {
            Ch::Ch1 | Ch::Ch2 => {}
            _ => {
                return Err(anyhow!("Device cannot measure {c}"));
            }
        }
        match m {
            Meas::Volt | Meas::Curr | Meas::Pow => {}
            _ => {
                return Err(anyhow!("Device cannot measure {m}"));
            }
        }
        let m = self.lxi.req(&format!("meas:{m}? {c}"))?;
        Ok(m.parse::<f32>()?)
    }
    pub fn m_volt(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.meas(c, Meas::Volt)
    }
    pub fn m_curr(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.meas(c, Meas::Curr)
    }
    pub fn m_pow(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.meas(c, Meas::Pow)
    }

    pub fn volt<F>(&mut self, c: Ch, v: F) -> anyhow::Result<F>
    where
        F: Float + Display,
    {
        self.lxi.set(format!("{c}:volt"), v)?;
        Ok(v)
    }
    pub fn curr<F>(&mut self, c: Ch, v: F) -> anyhow::Result<F>
    where
        F: Float + Display,
    {
        self.lxi.set(format!("{c}:curr"), v)?;
        Ok(v)
    }

    fn q_param<S>(&mut self, c: Ch, param: S) -> anyhow::Result<f32>
    where
        S: AsRef<str>,
    {
        match c {
            Ch::Ch1 | Ch::Ch2 => {}
            _ => {
                return Err(anyhow!("Device cannot query {c}"));
            }
        }
        let m = self.lxi.req(&format!("{c}:{}?", param.as_ref()))?;
        Ok(m.parse::<f32>()?)
    }

    pub fn q_volt(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.q_param(c, "volt")
    }
    pub fn q_curr(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.q_param(c, "curr")
    }

    pub fn output_independent(&mut self) -> anyhow::Result<()> {
        self.lxi.send("output:track 0")
    }
    pub fn output_series(&mut self) -> anyhow::Result<()> {
        self.lxi.send("output:track 1")
    }
    pub fn output_parallel(&mut self) -> anyhow::Result<()> {
        self.lxi.send("output:track 2")
    }
    pub fn wave_display(&mut self, c: Ch, mode: PortState) -> anyhow::Result<()> {
        self.lxi.send(format!("output:wave {c},{mode}"))
    }
    pub fn output_state(&mut self, c: Ch, state: PortState) -> anyhow::Result<()> {
        match c {
            Ch::Ch1 | Ch::Ch2 | Ch::Ch3 => {}
            _ => {
                return Err(anyhow!("Device does not have output {c}"));
            }
        }
        self.lxi.send(format!("output {c},{state}"))
    }
    pub fn output_on(&mut self, c: Ch) -> anyhow::Result<()> {
        self.output_state(c, PortState::On)
    }
    pub fn output_off(&mut self, c: Ch) -> anyhow::Result<()> {
        self.output_state(c, PortState::Off)
    }
}
// EOF
