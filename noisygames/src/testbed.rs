use crate::game::Game;
use std::clone::Clone;
//use crate::Strategy;

//what are some things that a testbed should have? run
#[derive(Clone)]
pub struct Config<T, U> {
    pub player_a: T,
    pub player_b: U,
    pub game: Game,
    pub num_rounds: i32,
    pub num_instance: i32,
}

//This is to be used for data that might get analyzed after the fact
//struct DerivedData {
//}

//This is to collate the Config and the DerivedData for comprehensive access.
//struct RunResults {
//    config: Config,
//    der_dat: DerivedData,
//}
//really what I'd like to do is to initialize players with all the relevant strategy stuff and
//whatnot, then leave it to run with whatever strategy through some number of rounds with some
//number of instances. For the time being, we can just do number of rounds.
