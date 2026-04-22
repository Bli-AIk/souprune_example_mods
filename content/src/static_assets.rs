//! Canonical static RON assets emitted by the content guest.
//!
//! 由 content guest 发射的 canonical 静态 RON 资产。

use anyhow::Result;
use souprune_vessel::prelude::*;

/// Emit all static canonical RON assets owned by this mod.
///
/// 发射当前 mod 拥有的全部静态 canonical RON 资产。
pub fn emit_all(reg: &mut Registry) -> Result<()> {
    reg.emit_canonical_source(
        "battle/alight_motion_config.ron",
        include_str!("../ron/battle/alight_motion_config.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/chapters/demo.sequence.ron",
        include_str!("../ron/battle/chapters/demo.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/view/sans_idle.view.ron",
        include_str!("../ron/battle/view/sans_idle.view.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/view/undertale.view.ron",
        include_str!("../ron/battle/view/undertale.view.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/view/undertale_no_box.view.ron",
        include_str!("../ron/battle/view/undertale_no_box.view.ron"),
    )?;
    reg.emit_canonical_source(
        "view/structures/attack_bar.sdf.ron",
        include_str!("../ron/view/structures/attack_bar.sdf.ron"),
    )?;
    Ok(())
}
