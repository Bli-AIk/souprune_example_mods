//! Danmaku performances generated for `example_battle_mod`.
//!
//! 为 `example_battle_mod` 生成的弹幕演出。

use souprune_schema::danmaku::*;
use souprune_vessel::prelude::*;

/// Build the demo attack performance used by `example_battle_mod`.
///
/// 构建 `example_battle_mod` 使用的演示攻击演出。
pub fn demo_attack() -> DanmakuPerformance {
    performance! {
        prototypes {
            "j_dia_b" => BulletPrototype {
                visual: "jevil_diamonds_black".to_string(),
                collider: rect(3.0, 10.0),
                damage: 2.0,
                lifetime: 990.0,
                z_index: 15.0,
                hit_behavior: HitBehaviorPreset::Persistent,
                ..Default::default()
            },
        }
        behaviors {
            "static" => stationary(),
            "move_up_linear" => linear((0.0, 1.0), 100.0),
        }
        timeline [
            event_delta(0.0, "j_dia_b", SpawnPattern::Single)
                .offset(0.0, -90.0)
                .apply(&["static"])
                .build(),
            event_delta(
                0.0,
                "j_dia_b",
                line(6).spacing(22.5).direction((0.0, 1.0)).build(),
            )
            .offset(0.0, -90.0)
            .apply(&["move_up_linear"])
            .build(),
        ]
    }
}
