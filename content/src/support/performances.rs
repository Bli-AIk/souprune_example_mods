//! Danmaku performances generated for `example_battle_mod`.
//!
//! 为 `example_battle_mod` 生成的弹幕演出。

use souprune_schema::danmaku::*;
use std::collections::HashMap;

/// Build the demo attack performance used by `example_battle_mod`.
///
/// 构建 `example_battle_mod` 使用的演示攻击演出。
pub fn demo_attack() -> DanmakuPerformance {
    DanmakuPerformance {
        prototypes: HashMap::from([(
            "j_dia_b".to_string(),
            BulletPrototype {
                visual: "jevil_diamonds_black".to_string(),
                collider: ColliderShape::rectangle(3.0, 10.0),
                damage: 2.0,
                lifetime: 990.0,
                z_index: 15.0,
                hit_behavior: HitBehaviorPreset::Persistent,
                ..Default::default()
            },
        )]),
        behaviors: HashMap::from([
            ("static".to_string(), BulletBehavior::stationary()),
            (
                "move_up_linear".to_string(),
                BulletBehavior::linear((0.0, 1.0), 100.0),
            ),
        ]),
        timeline: vec![
            TimelineEvent::delta_with(
                0.0,
                "j_dia_b",
                SpawnPattern::Single,
                (0.0, -90.0),
                ["static"],
                [],
            ),
            TimelineEvent::delta_with(
                0.0,
                "j_dia_b",
                SpawnPattern::line(6, 22.5, (0.0, 1.0)),
                (0.0, -90.0),
                ["move_up_linear"],
                [],
            ),
        ],
        duration: None,
    }
}
