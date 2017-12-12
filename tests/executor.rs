extern crate ttml;

use std::collections::HashMap;
use ttml::parser::*;
use ttml::output::Output;
use ttml::die::DieType;
use ttml::token::TokenAttributeValue;
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

    let mut output = Output::new("#test".to_string());
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 1);
    assert!(roll.value <= 20);
    assert_eq!(roll.dice.len(), 1);
}

#[test]
fn it_executes_roll_with_min_max_flags() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
            Arg::Roll(RollArg::Min(ArgValue::Number(2))),
            Arg::Roll(RollArg::Max(ArgValue::Number(3))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
        value: None,
    };

    let mut output = Output::new("#test".to_string());
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 2);
    assert!(roll.value <= 3);
    assert_eq!(roll.dice.len(), 1);

    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(ArgValue::Number(1))),
            Arg::Roll(RollArg::D(ArgValue::Number(20))),
            Arg::Roll(RollArg::Min(ArgValue::Number(200))),
            Arg::Roll(RollArg::Max(ArgValue::Number(300))),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
        value: None,
    };

    let mut output = Output::new("#test".to_string());
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 200);
    assert!(roll.value <= 300);
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

    let mut output = Output::new("#test".to_string());
    output.results.insert("1".to_string(), StepValue::Number(5));
    let roll = execute_roll(&step, &mut output);

    assert!(roll.value >= 5);
    assert!(roll.value <= 100);
    assert_eq!(roll.dice.len(), 5);
}

#[test]
fn it_executes_simple_input() {
    let input = "#test !r 1d20min20max40+@me.dexterity".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 5
                }
            },
            "macros": {}
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
            },
            "macros": {}
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

#[test]
fn it_assigns_and_updates_token_attributes() {
    let input = "#test @me.dexterity = 25".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("dexterity").unwrap();
    assert_eq!(attr, &TokenAttributeValue::Number(25));

    // test assigning a variable
    let input = "#test !roll 1d20 >> @me.dexterity = $1".to_string().into_bytes();
    let token_input = r#"{
        "me": {
            "attributes": {
                "dexterity": {
                    "Number": 21
                }
            },
            "macros": {}
        }
    }"#.to_string().into_bytes();
    let output = execute_macro(input, token_input);
    let tokens = output.tokens;
    let token = tokens.get("me").unwrap();
    let attr = token.attributes.get("dexterity").unwrap();
    assert_ne!(attr, &TokenAttributeValue::Number(21));
}

#[test]
fn it_executes_true_false_statements() {
    // let input = "#test @me.dexterity > 25 ? !roll 1d8+5 : !r 1d8".to_string().into_bytes();
    // let token_input = r#"{
        // "me": {
            // "attributes": {
                // "dexterity": {
                    // "Number": 21
                // }
            // },
            // "macros": {}
        // }
    // }"#.to_string().into_bytes();
    // let output = execute_macro(input, token_input);
    // println!("output {:?}", output.rolls);

    // assert_eq!(output.rolls.len(), 1);
}
