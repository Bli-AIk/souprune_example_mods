//! Code representation of `battle/chapters/demo.sequence.ron`.
//!
//! `battle/chapters/demo.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: Some("battle".into()),
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SetCamera(CameraAction::SetZoom(2.0)),
            Chapter::SpawnView {
                view_layout: "battle/view/sans_idle.view.ron".into(),
                bindings: vec![].into_iter().collect(),
            },
            Chapter::SpawnView {
                view_layout: "battle/view/undertale_no_box.view.ron".into(),
                bindings: vec![].into_iter().collect(),
            },
            Chapter::SetPlayer(PlayerAction::Spawn {
                config_path: "battle/players/player.battle_player.ron".into(),
                position: Some((0.0, -80.0)),
            }),
            Chapter::AlightMotionPerformance {
                amproj_path: "demo_turn.amproj".into(),
                alight_motion_config: None,
                wait_for_completion: true,
            },
            Chapter::Sequence(vec![
                Chapter::Wait(0.5),
                Chapter::SetPlayer(PlayerAction::Despawn),
            ]),
        ],
    }
}
