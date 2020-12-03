use crate::{
    actions::utils::subprocess,
    mcore::{
        action::{Action, ActionResult},
        errors::*,
    },
};
use std::io::Result;

pub fn that(path: &str) -> Result<()> {
    info!("Opening URL: {}", path);
    let args: Vec<&str> = vec![path];
    subprocess::spawn("xdg-open", &args)
}

pub struct OpenAction {}

impl Action for OpenAction {
    fn runnable_arg(&self) -> bool {
        true
    }

    fn run_arg(&self, text: &str) -> ActionResult {
        that(text).map_err(|e| Error::with_chain(e, format!("Failed to open path {}", text)))?;
        Ok(Vec::new())
    }
}
