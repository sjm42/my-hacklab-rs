// sdl1000x.rs
#![allow(dead_code)]

use anyhow::anyhow;
use num::traits::Float;
use std::{fmt, fmt::Display, net::ToSocketAddrs};

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

    pub fn idn_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("*IDN?")
    }
    pub fn lan_addr_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("LAN:IPAD?")
    }
    pub fn lan_mask_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("LAN:SMASK?")
    }
    pub fn lan_gw_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("LAN:GAT?")
    }
    pub fn lan_mac_q(&mut self) -> anyhow::Result<String> {
        self.lxi.req("LAN:MAC?")
    }

    pub fn set_func(&mut self, func: Func) -> anyhow::Result<Func> {
        self.lxi.send(&format!(":FUNC {func}"))?;
        Ok(func)
    }
    pub fn get_func(&mut self) -> anyhow::Result<Func> {
        self.lxi.req(":FUNC?")?;

        Ok(Func::Curr)
    }

    pub fn meas_q(&mut self, m: Meas) -> anyhow::Result<f32> {
        match m {
            Meas::Volt | Meas::Curr | Meas::Powr | Meas::Res | Meas::Ext => {}
            _ => {
                return Err(anyhow!("Device cannot measure {m}"));
            }
        }

        let m = self.lxi.req(&format!("MEAS:{m}?"))?;
        Ok(m.parse::<f32>()?)
    }
    pub fn volt_m(&mut self) -> anyhow::Result<f32> {
        self.meas_q(Meas::Volt)
    }
    pub fn curr_m(&mut self) -> anyhow::Result<f32> {
        self.meas_q(Meas::Curr)
    }
    pub fn powr_m(&mut self) -> anyhow::Result<f32> {
        self.meas_q(Meas::Powr)
    }
    pub fn res_m(&mut self) -> anyhow::Result<f32> {
        self.meas_q(Meas::Res)
    }
    pub fn ext_m(&mut self) -> anyhow::Result<f32> {
        self.meas_q(Meas::Ext)
    }

    // wave type can be "curr", "volt", "pow", "res"
    pub fn wave_q(&mut self, m: Meas) -> anyhow::Result<Vec<f32>> {
        let c = format!("MEAS:WAVE? {m}");
        let w = self.lxi.req(&c)?;
        Ok(w.split(',')
            .map(|x| x.parse::<f32>().unwrap_or_default())
            .collect::<Vec<f32>>())
    }

    pub fn sense(&mut self, state: PortState) -> anyhow::Result<PortState> {
        self.lxi.set_state("SYST:SENS", state)
    }
    pub fn sense_on(&mut self) -> anyhow::Result<PortState> {
        self.sense(PortState::On)
    }
    pub fn sense_off(&mut self) -> anyhow::Result<PortState> {
        self.sense(PortState::Off)
    }
    pub fn sense_q(&mut self) -> anyhow::Result<PortState> {
        self.lxi.get_state("SYST:SENS?")
    }

    pub fn input(&mut self, state: PortState) -> anyhow::Result<PortState> {
        self.lxi.set_state(":INP:STAT", state)
    }
    pub fn input_on(&mut self) -> anyhow::Result<PortState> {
        self.input(PortState::On)
    }
    pub fn input_off(&mut self) -> anyhow::Result<PortState> {
        self.input(PortState::Off)
    }
    pub fn input_q(&mut self) -> anyhow::Result<PortState> {
        self.lxi.get_state(":INP:STAT?")
    }

    pub fn short(&mut self, state: PortState) -> anyhow::Result<PortState> {
        self.lxi.set_state(":SHOR:STAT", state)
    }
    pub fn short_on(&mut self) -> anyhow::Result<PortState> {
        self.short(PortState::On)
    }
    pub fn short_off(&mut self) -> anyhow::Result<PortState> {
        self.short(PortState::Off)
    }
    pub fn short_q(&mut self) -> anyhow::Result<PortState> {
        self.lxi.get_state(":SHOR:STAT?")
    }

    pub fn curr_irange(&mut self, v: IRange) -> anyhow::Result<()> {
        self.lxi.set_s(":CURR:IRANG", &v.to_string())?;
        Ok(())
    }
    pub fn curr_irange_q(&mut self) -> anyhow::Result<IRange> {
        IRange::from_str(self.lxi.req(":CURR:IRANG?")?)
    }
    pub fn curr_vrange(&mut self, v: VRange) -> anyhow::Result<()> {
        self.lxi.set_s(":CURR:VRANG", &v.to_string())?;
        Ok(())
    }
    pub fn curr_vrange_q(&mut self) -> anyhow::Result<VRange> {
        VRange::from_str(self.lxi.req(":CURR:VRANG?")?)
    }

    pub fn curr_curr<F>(&mut self, v: F) -> anyhow::Result<()>
    where
        F: Float + Display,
    {
        self.lxi.set_f(":CURR", v)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum IRange {
    I5A = 5,
    I30A = 30,
}
impl fmt::Display for IRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((*self as usize).to_string().as_str())
    }
}
impl IRange {
    pub fn from_str<S>(s: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        match s.as_ref().parse::<u32>()? {
            5 => Ok(Self::I5A),
            30 => Ok(Self::I30A),
            x => Err(anyhow!("Unknown IRange {x}")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VRange {
    V36V = 36,
    V150V = 150,
}
impl fmt::Display for VRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((*self as usize).to_string().as_str())
    }
}
impl VRange {
    pub fn from_str<S>(s: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        match s.as_ref().parse::<u32>()? {
            36 => Ok(Self::V36V),
            150 => Ok(Self::V150V),
            x => Err(anyhow!("Unknown VRange {x}")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RRange {
    Low,
    Middle,
    High,
    Upper,
}
impl fmt::Display for RRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match *self {
            Self::Low => "LOW",
            Self::Middle => "MIDDLE",
            Self::High => "HIGH",
            Self::Upper => "UPPER",
        };
        f.write_str(p)
    }
}

// EOF
