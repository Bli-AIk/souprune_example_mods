//! Vessel content guest for `example_am_mod`.
//!
//! `example_am_mod` 的 Vessel 内容 guest。

use anyhow::Result;
use souprune_vessel::prelude::*;

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        let _ = reg;
        Ok(())
    }
}
