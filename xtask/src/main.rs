
use clap::Parser;
use anyhow::{anyhow, Result};

use std::process::*;
use std::path::Path;
use std::env;

#[derive(Parser)]
#[command(verbatim_doc_comment)]
enum XtaskCommand { 
    /// Build the 'ibstrace' kernel module
    Build { 
        /// The core number used to run user-uploaded code
        core: usize 
    },
    /// Clean build artifacts
    Clean,
}

fn build_module(root: &Path, core: usize) -> Result<()> {
    let core_arg = format!("CORE={}", core);
    let cmd = Command::new("make").args([
        "-C", "./ibstrace/", "prod", core_arg.as_str()
    ]).current_dir(root).spawn()?.wait()?;
    if let Some(code) = cmd.code() {
        if code != 0 {
            return Err(anyhow!("build error?"));
        }
    }
    Ok(())
}

fn clean_module(root: &Path) -> Result<()> {
    let cmd = Command::new("make").args([
        "-C", "./ibstrace/", "clean"
    ]).current_dir(root).spawn()?.wait()?;
    if let Some(code) = cmd.code() {
        if code != 0 {
            return Err(anyhow!("???"));
        }
    }
    Ok(())
}




fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let cmd = XtaskCommand::parse();
    match cmd { 
        XtaskCommand::Build { core } => {
            build_module(&root, core)?;
        },
        XtaskCommand::Clean => {
            clean_module(&root)?;
        },
    }
    Ok(())
}
