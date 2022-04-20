// spd3303x.rs
#![allow(dead_code)]

use anyhow::anyhow;
use num::traits::Float;
use std::{fmt, fmt::Display, net::ToSocketAddrs};

use crate::*;

// https://int.siglent.com/upload_file/user/SPD3000X/SPD3303X_QuickStart_QS0503X-E01B.pdf

#[derive(Debug)]
pub enum PwrChannelMode {
    CV, // Constant voltage
    CC, // Constant current i.e. current limit reached
}

#[derive(Debug)]
pub enum PwrOutputMode {
    NoIdea,
    Independent,
    Parallel,
    SeriesMaybe,
    CantHappen,
}

#[derive(Debug)]
pub enum PwrDisplayMode {
    Digital,
    Waveform,
}

#[derive(Debug)]
pub struct SPD3303XStatus {
    pub ch1_mode: PwrChannelMode,
    pub ch2_mode: PwrChannelMode,
    pub output_mode: PwrOutputMode,
    pub ch1: PortState,
    pub ch2: PortState,
    pub timer1: PortState,
    pub timer2: PortState,
    pub ch1_display: PwrDisplayMode,
    pub ch2_display: PwrDisplayMode,
}
impl SPD3303XStatus {
    pub fn from_u16(st: u16) -> Self {
        Self {
            ch1_mode: if st & 1 == 0 {
                PwrChannelMode::CV
            } else {
                PwrChannelMode::CC
            },
            ch2_mode: if st & (1 << 1) == 0 {
                PwrChannelMode::CV
            } else {
                PwrChannelMode::CC
            },
            output_mode: match (st & 0b1100) >> 2 {
                0b00 => PwrOutputMode::NoIdea,
                0b01 => PwrOutputMode::Independent,
                0b10 => PwrOutputMode::Parallel,
                0b11 => PwrOutputMode::SeriesMaybe,
                _ => PwrOutputMode::CantHappen,
            },
            ch1: if st & (1 << 4) == 0 {
                PortState::Off
            } else {
                PortState::On
            },
            ch2: if st & (1 << 5) == 0 {
                PortState::Off
            } else {
                PortState::On
            },

            timer1: if st & (1 << 6) == 0 {
                PortState::Off
            } else {
                PortState::On
            },
            timer2: if st & (1 << 7) == 0 {
                PortState::Off
            } else {
                PortState::On
            },
            ch1_display: if st & (1 << 8) == 0 {
                PwrDisplayMode::Digital
            } else {
                PwrDisplayMode::Waveform
            },
            ch2_display: if st & (1 << 9) == 0 {
                PwrDisplayMode::Digital
            } else {
                PwrDisplayMode::Waveform
            },
        }
    }
    pub fn from_hex<S>(st_str: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let st = u16::from_str_radix(st_str.as_ref(), 16)?;
        Ok(Self::from_u16(st))
    }
}

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
    pub fn status(&mut self) -> anyhow::Result<SPD3303XStatus> {
        let str_status = self.lxi.req("system: status?")?;
        if let Some(hex_str) = str_status.strip_prefix("0x") {
            SPD3303XStatus::from_hex(hex_str)
        } else {
            Err(anyhow!("Invalid status format"))
        }
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
