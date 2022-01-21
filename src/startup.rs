// startup.rs

use log::*;
use std::env;
use structopt::StructOpt;

#[derive(Clone, Debug, Default, StructOpt)]
pub struct OptsCommon {
    #[structopt(short, long)]
    pub debug: bool,
    #[structopt(short, long)]
    pub trace: bool,
}
impl OptsCommon {
    pub fn finish(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn get_loglevel(&self) -> LevelFilter {
        if self.trace {
            LevelFilter::Trace
        } else if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }
}

pub fn expand_home(pathname: &mut String) -> anyhow::Result<()> {
    let home = env::var("HOME")?;
    *pathname = pathname.as_str().replace("$HOME", &home);
    Ok(())
}

pub fn start_pgm(c: &OptsCommon, desc: &str) {
    env_logger::Builder::new()
        .filter_level(c.get_loglevel())
        .format_timestamp_secs()
        .init();
    info!("Starting up {desc}...");
    debug!("Git branch: {}", env!("GIT_BRANCH"));
    debug!("Git commit: {}", env!("GIT_COMMIT"));
    debug!("Source timestamp: {}", env!("SOURCE_TIMESTAMP"));
    debug!("Compiler version: {}", env!("RUSTC_VERSION"));
}

// EOF
