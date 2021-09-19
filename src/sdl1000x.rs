// sdl1000x.rs

use lxi::*;
use std::{error::Error, fmt, net::SocketAddr};

use crate::StdLxi;

// https://int.siglent.com/upload_file/user/SDL1000X/SDL1000X_Programming_Guide_V1.0.pdf

#[allow(dead_code)]
// #[derive(Debug)]
pub struct SDL1000X {
    addr: SocketAddr,
    name: String,
    pub verbose: bool,
    lxi_dev: LxiTextDevice,
}

impl StdLxi for SDL1000X {
    fn create(addr: SocketAddr, name: String, lxi_dev: LxiTextDevice) -> Self {
        SDL1000X {
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
impl SDL1000X {
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn set_func(&mut self, func: Func) -> Result<Func, Box<dyn Error>> {
        self.send(&format!("FUNC {}", func))?;
        Ok(func)
    }

    pub fn meas(&mut self, m: Meas) -> Result<f32, Box<dyn Error>> {
        let m = self.req(&format!("meas:{}?", m))?;
        Ok(m.parse::<f32>()?)
    }

    // wave type can be "curr", "volt", "pow", "res"
    pub fn wave(&mut self, m: Meas) -> Result<Vec<f32>, Box<dyn Error>> {
        let c = format!("meas:wave? {}", m);
        let w = self.req(&c)?;
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
