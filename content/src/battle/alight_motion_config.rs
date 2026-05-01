//! Code representation of `battle/alight_motion_config.ron`.
//!
//! `battle/alight_motion_config.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::config::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

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
