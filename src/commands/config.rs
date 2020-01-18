use crate::{
    commands::Run,
    config::Config as AppConfig,
    globals::{CONFIG_PATH, TERM_OUT},
};
use anyhow::{Context, Result};
use cfg_if::cfg_if;
use dialoguer::Editor;
use indicatif::ProgressBar;
use structopt::StructOpt;

#[cfg(windows)]
use std::env;

/// Config command options
#[derive(StructOpt, Debug)]
pub struct Config {
    /// Print the file path instead of opening an editor
    #[structopt(short = "P", long)]
    print_path: bool,
}

impl Run for Config {
    fn run(self, _verbose: bool) -> Result<()> {
        if self.print_path {
            TERM_OUT.write_line(&format!("{}", CONFIG_PATH.display()))?;
            return Ok(());
        }
        cfg_if! {
            if #[cfg(windows)] {
                if env::var_os("EDITOR").is_none() && env::var_os("VISUAL").is_none() {
                    TERM_OUT.write_line(&format!("{}", CONFIG_PATH.display()))?;
                    return Ok(());
                }
            }
        }

        let p = ProgressBar::new_spinner();
        p.set_message("Editing config");
        p.enable_steady_tick(100);

        let config = serde_json::to_string_pretty(&AppConfig::read()?)?;
        if let Some(s) = Editor::new().extension(".json").edit(&config)? {
            let config: AppConfig = serde_json::from_str(&s).context("Invalid config file")?;
            config.write()?;

            p.finish_with_message("Changes saved");
        } else {
            p.finish_with_message("Changes discarded");
        }

        Ok(())
    }
}
