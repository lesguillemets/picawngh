use picawngh as pica;
use std::io::{stdin, stdout, Write};
// use termion;

fn main() {
    let mut s = String::new();
    let mut m = pica::Model::random(500, 150, RULE);
    // loop {
    //     // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    //     m.print_stdout();
    //     m.update_self();
    // }
}
const RULE: pica::Rule = pica::Rule {
    birth_min: 3,
    birth_max: 3,
    alive_min: 2,
    alive_max: 3,
};
