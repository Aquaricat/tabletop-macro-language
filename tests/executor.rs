extern crate ttml;

use ttml::parser::*;
use ttml::executor::execute_roll;

#[test]
fn it_returns_a_roll() {
    let step = Step {
        args: vec![
            Arg::Roll(RollArg::N(1)),
            Arg::Roll(RollArg::D(20)),
        ],
        op: MacroOp::Roll,
        result: StepResult::Ignore,
    };

    let roll = execute_roll(&step);

    assert!(roll.value >= 1);
    assert!(roll.value <= 20);
    assert_eq!(roll.dice.len(), 1);
}