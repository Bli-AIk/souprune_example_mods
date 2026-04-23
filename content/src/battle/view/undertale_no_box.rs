//! View asset for `battle/view/undertale_no_box.view.ron`.
//!
//! `battle/view/undertale_no_box.view.ron` 的 view 资源。

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
        roots: Vec::from([
            ViewNodeDef {
                name: "BtnFight".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/fight/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-233.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnFightSelected".into(),
                visible_when: Some("$button_selection == 0".into()),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/fight/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-233.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnAct".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/act/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-80.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnActSelected".into(),
                visible_when: Some("$button_selection == 1".into()),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/act/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-80.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnItem".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/item/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(80.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnItemSelected".into(),
                visible_when: Some("$button_selection == 2".into()),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/item/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(80.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnMercy".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/mercy/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(235.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnMercySelected".into(),
                visible_when: Some("$button_selection == 3".into()),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/mercy/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(235.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BattleHUD".into(),
                texts: Vec::from([
                    TextDef {
                        id: "PlayerName".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:name}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-290.0, -155.5, 1.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "PlayerLevelLabel".into(),
                        font: "battlehud".into(),
                        content: Some("{{battle/ui:LV}}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-187.5, -155.5, 1.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "PlayerLevelValue".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:lv}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-148.5, -155.5, 1.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "HPValueCurrent".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:hp}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3_value(
                                expression("-5.5 + ($player:hp_max - 20) * 94.5 / 79"),
                                static_float(-155.5),
                                static_float(1.0),
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "HPSeparator".into(),
                        font: "battlehud".into(),
                        content: Some("/".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3_value(
                                expression("33.5 + ($player:hp_max - 20) * 94.5 / 79"),
                                static_float(-155.5),
                                static_float(1.0),
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "HPValueMax".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:hp_max}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3_value(
                                expression("57.5 + ($player:hp_max - 20) * 94.5 / 79"),
                                static_float(-155.5),
                                static_float(1.0),
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ]),
                children: Vec::from([
                    ViewNodeDef {
                        name: "HPSprite".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("assets/textures/battle/view/hpname.png".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(-64.5, -170.0, 1.0)),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ViewNodeDef {
                        name: "HPBar".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("procedural://white_pixel".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(-45.0, -170.5, 1.0)),
                                scale: Some(vector3_value(
                                    expression("25.0 + ($player:hp_max - 20) * 95.0 / 79"),
                                    static_float(20.5),
                                    static_float(1.0),
                                )),
                                ..Default::default()
                            }),
                            pivot: Some(vector2(0.0, 0.5)),
                            material: Some(MaterialDef {
                                shader: "assets/shaders/hp_bar_sprite.wgsl".into(),
                                params: Vec::from([
                                    ("alpha".into(), MaterialParamValue::Static(1.0)),
                                    (
                                        "half_width".into(),
                                        MaterialParamValue::Expr(
                                            "40.0 + ($player:hp_max - 20) * 95.0 / 79 / 2".into(),
                                        ),
                                    ),
                                    (
                                        "hp_ratio".into(),
                                        MaterialParamValue::Expr(
                                            "$player:hp / $player:hp_max".into(),
                                        ),
                                    ),
                                    ("lag_ratio".into(), MaterialParamValue::Static(1.0)),
                                ])
                                .into_iter()
                                .collect(),
                                animations: Some(MaterialAnimationsDef {
                                    lag: Some(LagAnimationDef {
                                        source: "hp_ratio".into(),
                                        target: "lag_ratio".into(),
                                        easing: EasingDef::OutCirc,
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
        ]),
        facts: Some(
            Vec::from([("button_selection".into(), InitialFactValue::Int(0))])
                .into_iter()
                .collect(),
        ),
        world_space: true,
        ..Default::default()
    }
}
