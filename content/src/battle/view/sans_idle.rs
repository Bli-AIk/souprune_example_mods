//! View asset for `battle/view/sans_idle.view.ron`.
//!
//! `battle/view/sans_idle.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this asset.
///
/// 生成当前资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    view_layout(vec![view_node("Sans_Legs")
        .sprite(
            view_sprite("assets/textures/battle/sans/legs.png")
                .translation(vector3(3.0, 57.0, 0.0))
                .scale(vector3(2.0, 2.0, 2.0)),
        )
        .children(vec![
            view_node("Sans_Torso").sprite(
                view_sprite("assets/textures/battle/sans/torso.png").translation(vector3(
                    expression("cos(snap(@time * 0.5, 1.0/30.0) * 10.0)"),
                    expression("23.0 + (sin(snap(@time * 0.5, 1.0/30.0) * 20.0) / 1.5)"),
                    0.1,
                )),
            ),
            view_node("Sans_Head").sprite(
                view_sprite("assets/textures/battle/sans/head.png").translation(vector3(
                    expression("cos(snap(@time * 0.5, 1.0/30.0) * 10.0)"),
                    expression("45.0 + sin(snap(@time * 0.5, 1.0/30.0) * 20.0)"),
                    0.2,
                )),
            ),
        ])])
    .world_space(true)
}
