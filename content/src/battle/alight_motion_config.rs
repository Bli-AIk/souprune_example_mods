//! Bootstrapped code asset for `battle/alight_motion_config.ron`.
//!
//! `battle/alight_motion_config.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::config::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 生成当前 bootstrap 资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> AlightMotionBattleConfig {
    AlightMotionBattleConfig {
        scale: 2.0,
        offset: (0.0, 0.0),
        bullet_pattern: "^#B".into(),
        battle_box_pattern: "^#C".into(),
        hidden_pattern: "^#B".into(),
        bullet_damage: 1.0,
        collision_scale: 0.05,
        default_battle_box_size: (566.0, 130.0),
    }
}
