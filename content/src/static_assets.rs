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
        "battle/chapters/demo.sequence.ron",
        include_str!("../ron/battle/chapters/demo.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/rules/demo_battle.fre.ron",
        include_str!("../ron/battle/rules/demo_battle.fre.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/view/undertale.view.ron",
        include_str!("../ron/battle/view/undertale.view.ron"),
    )?;
    reg.emit_canonical_source(
        "view/structures/attack_bar.sdf.ron",
        include_str!("../ron/view/structures/attack_bar.sdf.ron"),
    )?;
    Ok(())
}
