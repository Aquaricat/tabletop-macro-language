extern crate ttml;

use std::collections::HashMap;
use ttml::parser::*;
use ttml::die::DieType;
use ttml::executor::{execute_macro, execute_roll};

#[test]
fn it_returns_a_roll() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
        value: None,
    };

    let results = HashMap::new();
    let tokens = HashMap::new();
    let roll = execute_roll(&step, &results, &tokens);

    assert!(roll.value >= 1);
    assert!(roll.value <= 20);
    assert_eq!(roll.dice.len(), 1);
}

#[test]
fn it_uses_variables() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::VariableReserved(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
        value: None,
    };

    let mut results = HashMap::new();
    results.insert("1".to_string(), StepValue::Number(5));

    let tokens = HashMap::new();
    let roll = execute_roll(&step, &results, &tokens);

    assert!(roll.value >= 5);
    assert!(roll.value <= 100);
    assert_eq!(roll.dice.len(), 5);
}

#[test]
fn it_executes_simple_input() {
    let input = "#test !r 1d20+@me.dexterity".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 5
                }
            }
        }
    }"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(rolls[0].modifiers[0], 5);
    assert_eq!(rolls[0].value - rolls[0].raw_value, 5);
}

#[test]
fn it_executes_positive_modifier() {
    let input = "#test $foo = 10 !r 1d20+$foo".to_string().into_bytes();
    let token_input = r#"{}"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(rolls[0].modifiers[0], 10);
    assert_eq!(rolls[0].value - rolls[0].raw_value, 10);
}

#[test]
fn it_executes_negative_modifier() {
    let input = "#test !r 1d20-@me.dexterity".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 5
                }
            }
        }
    }"#.to_string().into_bytes();

    let output = execute_macro(input, token_input);
    let rolls = output.rolls;
    assert_eq!(rolls[0].dice.len(), 1);
    assert_eq!(rolls[0].dice[0].die, DieType::D20);
    assert_eq!(rolls[0].modifiers.len(), 1);
    assert_eq!(rolls[0].modifiers[0], -5);
    assert_eq!(rolls[0].value - rolls[0].raw_value, -5);
}
