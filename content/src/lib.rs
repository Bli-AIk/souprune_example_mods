//! Vessel content guest for `example_battle_mod`.
//!
//! `example_battle_mod` 的 Vessel 内容 guest。

use anyhow::Result;
use souprune_vessel::prelude::*;

mod performances;

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        reg.emit_ron(
            "battle/danmaku/demo_attack.performance.ron",
            &performances::demo_attack(),
        )?;
        Ok(())
    }
}
