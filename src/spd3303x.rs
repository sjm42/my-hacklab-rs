// spd3303x.rs
#![allow(dead_code)]

use anyhow::anyhow;
use num::traits::Float;
use std::{fmt, fmt::Display, net::ToSocketAddrs, str::FromStr};

use crate::*;

// https://int.siglent.com/upload_file/user/SPD3000X/SPD3303X_QuickStart_QS0503X-E01B.pdf

#[derive(Debug)]
pub enum PwrChannelMode {
    CV, // Constant voltage
    CC, // Constant current i.e. current limit reached
}

#[derive(Debug)]
pub enum PwrOutputMode {
    Invalid,
    Independent,
    Parallel,
    Series,
}

#[derive(Debug)]
pub enum PwrDisplayMode {
    Digital,
    Waveform,
}

#[derive(Debug)]
pub struct SPD3303XStatus {
    pub output_mode: PwrOutputMode,
    pub ch1: PortState,
    pub ch2: PortState,
    pub ch1_mode: PwrChannelMode,
    pub ch2_mode: PwrChannelMode,
    pub ch1_display: PwrDisplayMode,
    pub ch2_display: PwrDisplayMode,
    pub timer1: PortState,
    pub timer2: PortState,
}
impl SPD3303XStatus {
    pub fn from_u16(st: u16) -> Self {
        Self {
            output_mode: match (st & 0b1100) >> 2 {
                0b01 => PwrOutputMode::Independent,
                0b10 => PwrOutputMode::Parallel,
                0b11 => PwrOutputMode::Series,
                _ => PwrOutputMode::Invalid,
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
        }
    }
}
impl FromStr for SPD3303XStatus {
    type Err = anyhow::Error;
    fn from_str(st_str: &str) -> Result<Self, Self::Err> {
        if let Some(hex_str) = st_str.strip_prefix("0x") {
            Ok(Self::from_u16(u16::from_str_radix(hex_str, 16)?))
        } else {
            Err(anyhow!("Invalid status format"))
        }
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

    pub fn idn_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("*IDN?")
    }
    pub fn version_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("SYST:VERS?")
    }
    pub fn status_q(&mut self) -> anyhow::Result<SPD3303XStatus> {
        SPD3303XStatus::from_str(self.lxi.req("SYST:STAT?")?.as_str())
    }
    pub fn error_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("SYST:ERR?")
    }

    pub fn lan_addr_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("IP?")
    }
    pub fn lan_mask_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("MASK?")
    }
    pub fn lan_gw_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("GATE?")
    }

    pub fn meas_q(&mut self, c: Ch, m: Meas) -> anyhow::Result<f32> {
        match c {
            Ch::Ch1 | Ch::Ch2 => {}
            _ => {
                return Err(anyhow!("Device cannot measure {c}"));
            }
        }
        match m {
            Meas::Volt | Meas::Curr | Meas::Powr => {}
            _ => {
                return Err(anyhow!("Device cannot measure {m}"));
            }
        }
        let m = self.lxi.req(&format!("MEAS:{m}? {c}"))?;
        Ok(m.parse::<f32>()?)
    }
    pub fn volt_m(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.meas_q(c, Meas::Volt)
    }
    pub fn curr_m(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.meas_q(c, Meas::Curr)
    }
    pub fn powr_m(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.meas_q(c, Meas::Powr)
    }

    pub fn volt<F>(&mut self, c: Ch, v: F) -> anyhow::Result<F>
    where
        F: Float + Display,
    {
        self.lxi.set_f(format!("{c}:VOLT"), v)?;
        Ok(v)
    }
    pub fn curr<F>(&mut self, c: Ch, v: F) -> anyhow::Result<F>
    where
        F: Float + Display,
    {
        self.lxi.set_f(format!("{c}:CURR"), v)?;
        Ok(v)
    }

    fn param_q<S>(&mut self, c: Ch, param: S) -> anyhow::Result<f32>
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

    pub fn volt_q(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.param_q(c, "VOLT")
    }
    pub fn curr_q(&mut self, c: Ch) -> anyhow::Result<f32> {
        self.param_q(c, "CURR")
    }

    pub fn output_independent(&mut self) -> anyhow::Result<()> {
        self.lxi.send("OUTPUT:TRACK 0")
    }
    pub fn output_series(&mut self) -> anyhow::Result<()> {
        self.lxi.send("OUTPUT:TRACK 1")
    }
    pub fn output_parallel(&mut self) -> anyhow::Result<()> {
        self.lxi.send("OUTPUT:TRACK 2")
    }
    pub fn wave_display(&mut self, c: Ch, mode: PortState) -> anyhow::Result<()> {
        self.lxi.send(format!("OUTPUT:WAVE {c},{mode}"))
    }
    pub fn output_state(&mut self, c: Ch, state: PortState) -> anyhow::Result<()> {
        match c {
            Ch::Ch1 | Ch::Ch2 | Ch::Ch3 => {}
            _ => {
                return Err(anyhow!("Device does not have output {c}"));
            }
        }
        self.lxi.send(format!("OUTPUT {c},{state}"))
    }
    pub fn output_on(&mut self, c: Ch) -> anyhow::Result<()> {
        self.output_state(c, PortState::On)
    }
    pub fn output_off(&mut self, c: Ch) -> anyhow::Result<()> {
        self.output_state(c, PortState::Off)
    }
}
// EOF
