use console::Term;
use lazy_static::lazy_static;
use std::path::PathBuf;

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    pub static ref TERM_OUT: Term = Term::stdout();
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref CONFIG_PATH: PathBuf = {
        let mut cp = dirs::config_dir().unwrap();
        cp.push("bm2");
        cp.push("config.json");
        cp
    };
}
