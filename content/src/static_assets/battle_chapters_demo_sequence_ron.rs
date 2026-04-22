//! Bootstrapped code asset for `battle/chapters/demo.sequence.ron`.
//!
//! `battle/chapters/demo.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("battle/chapters/demo.sequence.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: Some("battle".into()),
        rules_file: Some("battle/rules/demo_battle.fre.ron".into()),
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SetCamera(CameraAction::SetZoom(2.0)),
            Chapter::SpawnView {
                view_layout: "battle/view/undertale.view.ron".into(),
                bindings: vec![].into_iter().collect(),
            },
            Chapter::SetPlayer(PlayerAction::Spawn {
                config_path: "battle/players/player.battle_player.ron".into(),
                position: Some((0.0, 0.0)),
            }),
            Chapter::DanmakuPerformance {
                performance: "battle/danmaku/demo_attack.performance.ron".into(),
                translation: None,
            },
        ],
    }
}
