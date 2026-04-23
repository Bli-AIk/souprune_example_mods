//! Code representation of `battle/rules/demo_battle.fre.ron`.
//!
//! `battle/rules/demo_battle.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: vec![].into_iter().collect(),
        facts: vec![
            ("battle_turn_count".into(), FactValueDef::Int(0)),
            ("enemy:0:hp_max".into(), FactValueDef::Int(100)),
            ("enemy:0:def".into(), FactValueDef::Int(5)),
            ("enemy:0:mercy_value".into(), FactValueDef::Int(0)),
            ("enemy:0:atk".into(), FactValueDef::Int(10)),
            ("enemy:0:hp".into(), FactValueDef::Int(100)),
            (
                "battle_phase".into(),
                FactValueDef::String("initializing".into()),
            ),
        ]
        .into_iter()
        .collect(),
        rules: vec![
            RuleDef {
                id: "increment_turn_count".into(),
                event: RuleEventDef::Event("turn_started".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![FreFactModificationDef::Increment {
                    key: "battle_turn_count".into(),
                    amount: 1,
                }],
                outputs: vec!["turn_count_updated".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "enemy_hp_half_dialogue".into(),
                event: RuleEventDef::Event("enemy_damaged".into()),
                conditions: vec!["$enemy:0:hp <= 50".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "TriggerDialogue".into(),
                    params: vec![("dialogue_id".into(), "enemy_half_hp_taunt".into())]
                        .into_iter()
                        .collect(),
                }],
                modifications: vec![],
                outputs: vec!["enemy_hp_threshold_reached".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "enemy_death_check".into(),
                event: RuleEventDef::Event("enemy_damaged".into()),
                conditions: vec!["$enemy:0:hp <= 0".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "EndBattle".into(),
                    params: vec![("result".into(), "victory".into())]
                        .into_iter()
                        .collect(),
                }],
                modifications: vec![],
                outputs: vec!["enemy_killed".into()],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "turn_3_dialogue".into(),
                event: RuleEventDef::Event("turn_count_updated".into()),
                conditions: vec!["$battle_turn_count == 3".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "TriggerDialogue".into(),
                    params: vec![("dialogue_id".into(), "turn_3_boss_taunt".into())]
                        .into_iter()
                        .collect(),
                }],
                modifications: vec![],
                outputs: vec!["turn_milestone_reached".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "increment_mercy_on_act".into(),
                event: RuleEventDef::Event("player_action_act".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![FreFactModificationDef::Increment {
                    key: "enemy:0:mercy_value".into(),
                    amount: 10,
                }],
                outputs: vec!["mercy_updated".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "enemy_spareable_check".into(),
                event: RuleEventDef::Event("mercy_updated".into()),
                conditions: vec!["$enemy:0:mercy_value >= 100".into()],
                actions: vec![],
                modifications: vec![FreFactModificationDef::Set {
                    key: "enemy:0:spareable".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec!["enemy_spareable".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
        ],
    }
}
