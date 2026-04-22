//! Bootstrapped static asset emitters for this content guest.
//!
//! 当前内容 guest 的 bootstrap 静态资产发射模块。

use anyhow::Result;
use souprune_vessel::prelude::*;

#[path = "static_assets/battle_chapters_demo_sequence_ron.rs"]
mod battle_chapters_demo_sequence_ron;
#[path = "static_assets/battle_rules_demo_battle_fre_ron.rs"]
mod battle_rules_demo_battle_fre_ron;
#[path = "static_assets/battle_view_undertale_view_ron.rs"]
mod battle_view_undertale_view_ron;
#[path = "static_assets/view_structures_attack_bar_sdf_ron.rs"]
mod view_structures_attack_bar_sdf_ron;

/// Emit all bootstrapped static assets for this mod.
///
/// 发射当前 mod 的全部 bootstrap 静态资产。
pub fn emit_all(reg: &mut Registry) -> Result<()> {
    battle_chapters_demo_sequence_ron::emit(reg)?;
    battle_rules_demo_battle_fre_ron::emit(reg)?;
    battle_view_undertale_view_ron::emit(reg)?;
    view_structures_attack_bar_sdf_ron::emit(reg)?;
    Ok(())
}
