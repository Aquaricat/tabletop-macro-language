extern crate ttml;

use ttml::roll::Roll;
use ttml::die::Die;
use ttml::die::DieType;
use ttml::roll::*;

#[test]
fn it_can_roll_custom_dice() {
    // Create some random dice
    let d20 = Die::new(DieType::D20);
    let d8 = Die::new(DieType::D8);
    let dice = vec![ d20, d8 ];
    let roll = Roll::new(dice);

    assert!(roll.value >= 1);
    assert!(roll.value <= 28);
    assert!(roll.raw_value >= 1);
    assert!(roll.raw_value <= 28);
    assert_eq!(roll.dice.len(), 2);
}

#[test]
fn it_can_add_a_token_to_roll() {
    // Create some random dice
    let d4 = Die::new(DieType::D4);
    let d8 = Die::new(DieType::D8);
    let dice = vec![ d4, d8 ];
    let mut roll = Roll::new(dice);
    let token = String::from("test token");
    roll.add_token(token.clone());

    assert_eq!(roll.token, Some(token));
}

#[test]
fn it_can_roll_d20s() {
    let roll = roll_d20(3);
    assert_eq!(roll.dice.len(), 3);
    for die in roll.dice.iter() {
        assert_eq!(die.die, DieType::D20);
    }
}

#[test]
fn it_can_roll_d12s() {
    let roll = roll_d12(6);
    assert_eq!(roll.dice.len(), 6);
    for die in roll.dice.iter() {
        assert_eq!(die.die, DieType::D12);
    }
}

#[test]
fn it_can_roll_d10s() {
    let roll = roll_d10(10);
    assert_eq!(roll.dice.len(), 10);
    for die in roll.dice.iter() {
        assert_eq!(die.die, DieType::D10);
    }
}

#[test]
fn it_can_roll_d8s() {
    let roll = roll_d8(3);
    assert_eq!(roll.dice.len(), 3);
    for die in roll.dice.iter() {
        assert_eq!(die.die, DieType::D8);
    }
}

#[test]
fn it_can_roll_d6s() {
    let roll = roll_d6(2);
    assert_eq!(roll.dice.len(), 2);
    for die in roll.dice.iter() {
        assert_eq!(die.die, DieType::D6);
    }
}

#[test]
fn it_can_roll_d4s() {
    let roll = roll_d4(9);
    assert_eq!(roll.dice.len(), 9);
    for die in roll.dice.iter() {
        assert_eq!(die.die, DieType::D4);
    }
}

#[test]
fn it_can_roll_and_keep_high() {
    let roll = roll_and_keep_high(5, DieType::D20, 1);
    let mut dropped = 0;
    let largest_die = roll.dice.iter().max_by(|a, b| a.value.cmp(&b.value)).unwrap();
    for die in roll.dice.iter() {
        if die.is_dropped {
            assert!(die.value <= largest_die.value);

            dropped += 1;
        } else {
            // we can't test the ids because the ID could be the same value as one we dropped
            // and the sorting algorithms are just different enough to cause issues, so we test
            // the value instead
            // assert_eq!(die._id, largest_die._id);
            assert_eq!(die.value, largest_die.value);
        }
    }

    assert_eq!(dropped, 4);
}

#[test]
fn it_can_roll_and_keep_multiple_high() {
    let roll = roll_and_keep_high(9, DieType::D20, 2);
    let mut dropped = 0;
    let mut value = 0;
    let mut raw_value = 0;
    let largest_die = roll.dice.iter().max_by(|a, b| a.value.cmp(&b.value)).unwrap();
    for die in roll.dice.iter() {
        if die.is_dropped {
            assert!(die.value <= largest_die.value);

            dropped += 1;
        } else {
            value += die.value as i16;
        }

        raw_value += die.value as i16;
    }

    assert_eq!(raw_value as i32, roll.raw_value);
    assert_eq!(value as i32, roll.value);
    assert_eq!(dropped, 7);
}

#[test]
fn it_can_roll_and_keep_low() {
    let roll = roll_and_keep_low(6, DieType::D20, 1);
    let mut dropped = 0;
    let smallest_die = roll.dice.iter().min_by(|a, b| a.value.cmp(&b.value)).unwrap();
    for die in roll.dice.iter() {
        if die.is_dropped {
            assert_ne!(die._id, smallest_die._id);
            assert!(die.value >= smallest_die.value);

            dropped += 1;
        } else {
            assert_eq!(die.value, smallest_die.value);
        }
    }

    assert_eq!(dropped, 5);
}

#[test]
fn it_can_roll_and_keep_multiple_low() {
    let roll = roll_and_keep_low(20, DieType::D20, 5);
    let mut dropped = 0;
    let mut value = 0;
    let mut raw_value = 0;
    let smallest_die = roll.dice.iter().min_by(|a, b| a.value.cmp(&b.value)).unwrap();
    for die in roll.dice.iter() {
        if die.is_dropped {
            assert_ne!(die._id, smallest_die._id);
            assert!(die.value >= smallest_die.value);

            dropped += 1;
        } else {
            value += die.value as i16;
        }

        raw_value += die.value as i16;
    }

    assert_eq!(raw_value as i32, roll.raw_value);
    assert_eq!(value as i32, roll.value);
    assert_eq!(dropped, 15);
}

#[test]
fn it_can_reroll_dice_once_above_a_threshold() {
    let mut roll = roll_d20(6);

    assert_eq!(roll.dice.len(), 6);

    let mut rerolls = 0;
    let mut total = 0;
    for d in roll.dice.iter() {
        if d.value >= 15 {
            rerolls += 1;
        }
        total += d.value;
    }

    roll.reroll_dice_once_above(15);

    assert_eq!(roll.dice.len(), 6 + rerolls);

    // count how many rerolls we actually made
    let mut actual_rerolls = 0;
    for d in roll.dice.iter() {
        if d.is_rerolled {
            actual_rerolls += 1;
        }
    }

    assert_eq!(actual_rerolls, rerolls);
}

#[test]
fn it_can_reroll_dice_forever_above_a_threshold() {
    let mut roll = roll_d12(3);

    assert_eq!(roll.dice.len(), 3);

    roll.reroll_dice_forever_above(10);

    // count how many rerolls we actually made
    let mut actual_rerolls = 0;
    let mut should_rerolls = 0;
    for d in roll.dice.iter() {
        if d.value >= 10 {
            should_rerolls += 1;
        }
        if d.is_rerolled {
            actual_rerolls += 1;
        }
    }

    assert_eq!(should_rerolls, actual_rerolls);
}

#[test]
fn it_can_reroll_dice_once_below_a_threshold() {
    let mut roll = roll_d6(4);

    assert_eq!(roll.dice.len(), 4);

    let mut rerolls = 0;
    for d in roll.dice.iter() {
        if d.value <= 4 {
            rerolls += 1;
        }
    }

    roll.reroll_dice_once_below(4);

    assert_eq!(roll.dice.len(), 4 + rerolls);

    // count how many rerolls we actually made
    let mut actual_rerolls = 0;
    for d in roll.dice.iter() {
        if d.is_rerolled {
            actual_rerolls += 1;
        }
    }

    assert_eq!(actual_rerolls, rerolls);
}

#[test]
fn it_can_reroll_dice_forever_below_a_threshold() {
    let mut roll = roll_d8(2);

    assert_eq!(roll.dice.len(), 2);

    roll.reroll_dice_forever_below(2);

    // count how many rerolls we actually made
    let mut actual_rerolls = 0;
    let mut should_rerolls = 0;
    for d in roll.dice.iter() {
        if d.value <= 2 {
            should_rerolls += 1;
        }
        if d.is_rerolled {
            actual_rerolls += 1;
        }
    }

    assert_eq!(should_rerolls, actual_rerolls);
}

#[test]
fn it_can_roll_with_advantage() {
    let roll = roll_with_advantage();

    assert_eq!(roll.dice.len(), 2);

    let smallest_die = roll.dice.iter().min_by(|a, b| a.value.cmp(&b.value)).unwrap();
    let largest_die = roll.dice.iter().max_by(|a, b| a.value.cmp(&b.value)).unwrap();
    let mut dropped = 0;
    for die in roll.dice.iter() {
        if die.is_dropped {
            dropped += 1;
        }
    }

    assert_eq!((largest_die.value + smallest_die.value) as i32, roll.raw_value);
    assert_eq!(largest_die.value as i32, roll.value);
    assert_eq!(dropped, 1);
}

#[test]
fn it_can_roll_with_disadvantage() {
    let roll = roll_with_disadvantage();

    assert_eq!(roll.dice.len(), 2);

    let smallest_die = roll.dice.iter().min_by(|a, b| a.value.cmp(&b.value)).unwrap();
    let largest_die = roll.dice.iter().max_by(|a, b| a.value.cmp(&b.value)).unwrap();
    for die in roll.dice.iter() {
        if die.is_dropped {
            assert_eq!(die._id, largest_die._id);
        } else {
            assert_eq!(die._id, smallest_die._id);
        }
    }

    assert_eq!((largest_die.value + smallest_die.value) as i32, roll.raw_value);
    assert_eq!(smallest_die.value as i32, roll.value);
}
