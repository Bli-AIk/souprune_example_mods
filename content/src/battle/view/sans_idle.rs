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
    ViewLayout {
        roots: Vec::from([ViewNodeDef {
            name: "Sans_Legs".into(),
            sprite: Some(SpriteDef {
                visual: Visual("assets/textures/battle/sans/legs.png".into()),
                transform: Some(SerializableTransform {
                    translation: Some(vector3(3.0, 57.0, 0.0)),
                    scale: Some(vector3(2.0, 2.0, 2.0)),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            children: Vec::from([
                ViewNodeDef {
                    name: "Sans_Torso".into(),
                    sprite: Some(SpriteDef {
                        visual: Visual("assets/textures/battle/sans/torso.png".into()),
                        transform: Some(SerializableTransform {
                            translation: Some(vector3_value(
                                expression("cos(snap(@time * 0.5, 1.0/30.0) * 10.0)"),
                                expression(
                                    "23.0 + (sin(snap(@time * 0.5, 1.0/30.0) * 20.0) / 1.5)",
                                ),
                                static_float(0.1),
                            )),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ViewNodeDef {
                    name: "Sans_Head".into(),
                    sprite: Some(SpriteDef {
                        visual: Visual("assets/textures/battle/sans/head.png".into()),
                        transform: Some(SerializableTransform {
                            translation: Some(vector3_value(
                                expression("cos(snap(@time * 0.5, 1.0/30.0) * 10.0)"),
                                expression("45.0 + sin(snap(@time * 0.5, 1.0/30.0) * 20.0)"),
                                static_float(0.2),
                            )),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }]),
        world_space: true,
        ..Default::default()
    }
}
