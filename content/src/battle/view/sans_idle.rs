//! Bootstrapped code asset for `battle/view/sans_idle.view.ron`.
//!
//! `battle/view/sans_idle.view.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::val::*;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: vec![ViewNodeDef {
            name: "Sans_Legs".into(),
            tags: vec![],
            style: StyleDef {
                width: None,
                height: None,
                left: None,
                right: None,
                top: None,
                bottom: None,
                position_type: None,
                flex_direction: None,
                justify_content: None,
                align_items: None,
            },
            visible_when: None,
            background_color: None,
            border_color: None,
            image: None,
            sprite: Some(SpriteDef {
                visual: Visual("assets/textures/battle/sans/legs.png".into()),
                initial_state: None,
                color: None,
                flip_x: false,
                flip_y: false,
                transform: Some(SerializableTransform {
                    translation: Some((Val::Static(3.0), Val::Static(57.0), Val::Static(0.0))),
                    rotation: None,
                    scale: Some((Val::Static(2.0), Val::Static(2.0), Val::Static(2.0))),
                }),
                pivot: None,
                frame_duration: None,
                visible_when: None,
                material: None,
            }),
            state_sprite: None,
            texts: vec![],
            view_box: None,
            children: vec![
                ViewNodeDef {
                    name: "Sans_Torso".into(),
                    tags: vec![],
                    style: StyleDef {
                        width: None,
                        height: None,
                        left: None,
                        right: None,
                        top: None,
                        bottom: None,
                        position_type: None,
                        flex_direction: None,
                        justify_content: None,
                        align_items: None,
                    },
                    visible_when: None,
                    background_color: None,
                    border_color: None,
                    image: None,
                    sprite: Some(SpriteDef {
                        visual: Visual("assets/textures/battle/sans/torso.png".into()),
                        initial_state: None,
                        color: None,
                        flip_x: false,
                        flip_y: false,
                        transform: Some(SerializableTransform {
                            translation: Some((
                                Val::Expr("cos(snap(@time * 0.5, 1.0/30.0) * 10.0)".into()),
                                Val::Expr(
                                    "23.0 + (sin(snap(@time * 0.5, 1.0/30.0) * 20.0) / 1.5)".into(),
                                ),
                                Val::Static(0.1),
                            )),
                            rotation: None,
                            scale: None,
                        }),
                        pivot: None,
                        frame_duration: None,
                        visible_when: None,
                        material: None,
                    }),
                    state_sprite: None,
                    texts: vec![],
                    view_box: None,
                    children: vec![],
                    repeat: None,
                },
                ViewNodeDef {
                    name: "Sans_Head".into(),
                    tags: vec![],
                    style: StyleDef {
                        width: None,
                        height: None,
                        left: None,
                        right: None,
                        top: None,
                        bottom: None,
                        position_type: None,
                        flex_direction: None,
                        justify_content: None,
                        align_items: None,
                    },
                    visible_when: None,
                    background_color: None,
                    border_color: None,
                    image: None,
                    sprite: Some(SpriteDef {
                        visual: Visual("assets/textures/battle/sans/head.png".into()),
                        initial_state: None,
                        color: None,
                        flip_x: false,
                        flip_y: false,
                        transform: Some(SerializableTransform {
                            translation: Some((
                                Val::Expr("cos(snap(@time * 0.5, 1.0/30.0) * 10.0)".into()),
                                Val::Expr("45.0 + sin(snap(@time * 0.5, 1.0/30.0) * 20.0)".into()),
                                Val::Static(0.2),
                            )),
                            rotation: None,
                            scale: None,
                        }),
                        pivot: None,
                        frame_duration: None,
                        visible_when: None,
                        material: None,
                    }),
                    state_sprite: None,
                    texts: vec![],
                    view_box: None,
                    children: vec![],
                    repeat: None,
                },
            ],
            repeat: None,
        }],
        requires: vec![],
        facts: None,
        world_space: true,
        coordinate_system: CoordinateSystem::Standard,
    }
}
