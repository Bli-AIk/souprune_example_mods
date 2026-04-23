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
    view_layout(vec![
        view_node("BtnFight").sprite(
            view_sprite("assets/textures/battle/view/fight/false.png")
                .translation(vector3(-233.0, -213.0, 1.0)),
        ),
        view_node("BtnFightSelected")
            .visible_when("$button_selection == 0")
            .sprite(
                view_sprite("assets/textures/battle/view/fight/true.png")
                    .translation(vector3(-233.0, -213.0, 1.5)),
            ),
        view_node("BtnAct").sprite(
            view_sprite("assets/textures/battle/view/act/false.png")
                .translation(vector3(-80.0, -213.0, 1.0)),
        ),
        view_node("BtnActSelected")
            .visible_when("$button_selection == 1")
            .sprite(
                view_sprite("assets/textures/battle/view/act/true.png")
                    .translation(vector3(-80.0, -213.0, 1.5)),
            ),
        view_node("BtnItem").sprite(
            view_sprite("assets/textures/battle/view/item/false.png")
                .translation(vector3(80.0, -213.0, 1.0)),
        ),
        view_node("BtnItemSelected")
            .visible_when("$button_selection == 2")
            .sprite(
                view_sprite("assets/textures/battle/view/item/true.png")
                    .translation(vector3(80.0, -213.0, 1.5)),
            ),
        view_node("BtnMercy").sprite(
            view_sprite("assets/textures/battle/view/mercy/false.png")
                .translation(vector3(235.0, -213.0, 1.0)),
        ),
        view_node("BtnMercySelected")
            .visible_when("$button_selection == 3")
            .sprite(
                view_sprite("assets/textures/battle/view/mercy/true.png")
                    .translation(vector3(235.0, -213.0, 1.5)),
            ),
        view_node("BattleHUD")
            .texts(vec![
                view_text("PlayerName", "{$player:name}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(-290.0, -155.5, 1.0)),
                view_text("PlayerLevelLabel", "{{battle/ui:LV}}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(-187.5, -155.5, 1.0)),
                view_text("PlayerLevelValue", "{$player:lv}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(-148.5, -155.5, 1.0)),
                view_text("HPValueCurrent", "{$player:hp}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(
                        expression("-5.5 + ($player:hp_max - 20) * 94.5 / 79"),
                        -155.5,
                        1.0,
                    )),
                view_text("HPSeparator", "/", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(
                        expression("33.5 + ($player:hp_max - 20) * 94.5 / 79"),
                        -155.5,
                        1.0,
                    )),
                view_text("HPValueMax", "{$player:hp_max}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(
                        expression("57.5 + ($player:hp_max - 20) * 94.5 / 79"),
                        -155.5,
                        1.0,
                    )),
            ])
            .children(vec![
                view_node("HPSprite").sprite(
                    view_sprite("assets/textures/battle/view/hpname.png")
                        .translation(vector3(-64.5, -170.0, 1.0)),
                ),
                view_node("HPBar").sprite(
                    view_sprite("procedural://white_pixel")
                        .translation(vector3(-45.0, -170.5, 1.0))
                        .scale(vector3(
                            expression("25.0 + ($player:hp_max - 20) * 95.0 / 79"),
                            20.5,
                            1.0,
                        ))
                        .pivot(vector2(0.0, 0.5))
                        .material(
                            material("assets/shaders/hp_bar_sprite.wgsl")
                                .static_parameter("alpha", 1.0)
                                .expression_parameter(
                                    "half_width",
                                    "40.0 + ($player:hp_max - 20) * 95.0 / 79 / 2",
                                )
                                .expression_parameter("hp_ratio", "$player:hp / $player:hp_max")
                                .static_parameter("lag_ratio", 1.0)
                                .lag_animation(
                                    lag_animation("hp_ratio", "lag_ratio")
                                        .easing(EasingDef::OutCirc),
                                ),
                        ),
                ),
            ]),
    ])
    .initial_facts(vec![("button_selection", InitialFactValue::Int(0))])
    .world_space(true)
}
