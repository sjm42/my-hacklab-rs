// sdl1000x.rs
#![allow(dead_code)]

use anyhow::anyhow;
use std::{fmt, fmt::Display, net::ToSocketAddrs, str::FromStr};

use crate::*;

// https://int.siglent.com/upload_file/user/SDL1000X/SDL1000X_Programming_Guide_V1.0.pdf

const SLEW_MIN: f32 = 0.001;
const SLEW_MAX: f32 = 0.500;

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

    pub fn func(&mut self, func: Func) -> anyhow::Result<Func> {
        self.lxi.send(&format!(":FUNC {func}"))?;
        Ok(func)
    }
    pub fn func_q(&mut self) -> anyhow::Result<Func> {
        Func::from_str(self.lxi.req(":FUNC?")?.as_str())
    }

    pub fn meas_q(&mut self, m: Meas) -> anyhow::Result<f32> {
        match m {
            Meas::Volt | Meas::Curr | Meas::Powr | Meas::Res | Meas::Ext => {}
            _ => {
                return Err(anyhow!("Device cannot measure {m}"));
            }
        }

        self.lxi.get_f(&format!("MEAS:{m}?"))
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
        IRange::from_str(self.lxi.req(":CURR:IRANG?")?.as_str())
    }
    pub fn curr_vrange(&mut self, v: VRange) -> anyhow::Result<()> {
        self.lxi.set_s(":CURR:VRANG", &v.to_string())?;
        Ok(())
    }
    pub fn curr_vrange_q(&mut self) -> anyhow::Result<VRange> {
        VRange::from_str(self.lxi.req(":CURR:VRANG?")?.as_str())
    }
    pub fn curr_check(&mut self, curr: Curr) -> anyhow::Result<()> {
        if let Curr::A(val) = curr {
            let curr_max = self.curr_irange_q()? as u32 as f32;
            if val < 0.0 {
                return Err(anyhow!("Current {val} is negative."));
            } else if val > curr_max {
                return Err(anyhow!("Current {val} too high, max={curr_max}"));
            }
        }
        Ok(())
    }
    pub fn curr_curr(&mut self, c: Curr) -> anyhow::Result<()> {
        self.curr_check(c)?;
        self.lxi.set_s(":CURR", &c.to_string())?;
        Ok(())
    }
    pub fn curr_curr_q(&mut self) -> anyhow::Result<f32> {
        self.lxi.get_f(":CURR?")
    }

    pub fn slew_check(slew: Slew) -> anyhow::Result<()> {
        if let Slew::APerUs(val) = slew {
            if val < SLEW_MIN {
                return Err(anyhow!("Slew {val} too low, min={SLEW_MIN}"));
            } else if val > SLEW_MAX {
                return Err(anyhow!("Slew {val} too high, max={SLEW_MAX}"));
            }
        }
        Ok(())
    }
    pub fn curr_slew_p(&mut self, slew: Slew) -> anyhow::Result<()> {
        Self::slew_check(slew)?;
        self.lxi.set_s(":CURR:SLEW:POS", &slew.to_string())?;
        Ok(())
    }
    pub fn curr_slew_n(&mut self, slew: Slew) -> anyhow::Result<()> {
        Self::slew_check(slew)?;
        self.lxi.set_s(":CURR:SLEW:NEG", &slew.to_string())?;
        Ok(())
    }
    pub fn curr_slew_p_q(&mut self) -> anyhow::Result<f32> {
        self.lxi.get_f(":CURR:SLEW:POS?")
    }
    pub fn curr_slew_n_q(&mut self) -> anyhow::Result<f32> {
        self.lxi.get_f(":CURR:SLEW:NEG?")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Func {
    Curr,
    Volt,
    Powr,
    Res,
    Led,
}
impl Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Curr => "CURR",
            Self::Volt => "VOLT",
            Self::Powr => "POW",
            Self::Res => "RES",
            Self::Led => "LED",
        })
    }
}
impl FromStr for Func {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CURRENT" => Ok(Func::Curr),
            "VOLTAGE" => Ok(Func::Volt),
            "POWER" => Ok(Func::Powr),
            "RESISTANCE" => Ok(Func::Res),
            "LED" => Ok(Func::Led),
            _ => Err(anyhow!("Unknown function")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Curr {
    Min,     // 0.001 V/µs
    Max,     // 0.500 V/µs
    Default, // same as Max
    A(f32),
}
impl Display for Curr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s;
        f.write_str(match *self {
            Self::Min => "MIN",
            Self::Max => "MAX",
            Self::Default => "DEF",
            Self::A(a) => {
                s = a.to_string();
                s.as_str()
            }
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IRange {
    I5A = 5,
    I30A = 30,
}
impl Display for IRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((*self as usize).to_string().as_str())
    }
}
impl FromStr for IRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>()? {
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
impl Display for VRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((*self as usize).to_string().as_str())
    }
}
impl FromStr for VRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>()? {
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
impl Display for RRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Low => "LOW",
            Self::Middle => "MIDDLE",
            Self::High => "HIGH",
            Self::Upper => "UPPER",
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Slew {
    Min,     // 0.001 V/µs
    Max,     // 0.500 V/µs
    Default, // same as Max
    APerUs(f32),
}
impl Display for Slew {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s;
        f.write_str(match *self {
            Self::Min => "MIN",
            Self::Max => "MAX",
            Self::Default => "DEF",
            Self::APerUs(v) => {
                s = v.to_string();
                s.as_str()
            }
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Transient {
    Continuous,
    Pulse,
    Toggle,
}
impl Display for Transient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Continuous => "CONT",
            Self::Pulse => "PULS",
            Self::Toggle => "TOGG",
        })
    }
}
impl FromStr for Transient {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONTINUOUS" => Ok(Self::Continuous),
            "PULSE" => Ok(Self::Pulse),
            "TOGGLE" => Ok(Self::Toggle),
            x => Err(anyhow!("Unknown transient mode {x}")),
        }
    }
}

// EOF
