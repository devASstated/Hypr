use std::process::Command;
use anyhow::{Result, anyhow};

use crate::state::Action;

pub fn dispatch(action: Action) -> Result<()> {
    let mut cmd = Command::new("hyprctl");

    match action {
        Action::WorkspacePrevMRU => {
            cmd.args(["dispatch", "workspace", "prev"]);
        }

        Action::WorkspaceNextCyclic => {
            cmd.args(["dispatch", "workspace", "e+1"]);
        }

        Action::WorkspacePrevCyclic => {
            cmd.args(["dispatch", "workspace", "e-1"]);
        }
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(anyhow!("hyprctl command failed"));
    }

    Ok(())
}

