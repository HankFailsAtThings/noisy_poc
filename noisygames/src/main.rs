use ndarray::prelude::*;
pub mod player;
use crate::player::*;
pub mod testbed;

pub mod game;

use crate::testbed::Config;

fn main() {
    let num_runs = 5;
    let num_instance = 3;
    //potential strategies for now are always defect, tit for tat, and grim trigger

    let a_mtx=arr2(&[[-1, -3],
                   [0, -2]]);
    let b_mtx=arr2(&[[-1, 0],
                   [-3, -2]]);
    //send this into a constructor for a Game type
    
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    
    //run iterated some N times with strategy profiles specified for each player
    g.init_game(); 
    assert!(g.is_init);

    //now it's time to make the player types
    let player_a = BasicPlayer {
        name: "john".to_string(),
        my_score: 0,
        their_score: 0,
        my_moves: Vec::new(),
        their_moves: Vec::new(),
        my_outcomes: Vec::new(),
        their_outcomes: Vec::new(),
    };
    let player_b = BasicPlayer {
        name: "jacob".to_string(),
        my_score: 0,
        their_score: 0,
        my_moves: Vec::new(),
        their_moves: Vec::new(),
        my_outcomes: Vec::new(),
        their_outcomes: Vec::new(),
    }; 

    let a_strat = AlwaysDefect { play: player_a };
    let b_strat = GrimTrigger { play: player_b };
    //set up the configuration for the experiment
    let cfg = Config {
        player_a: a_strat,
        player_b: b_strat,
        game: g,
        num_rounds: num_runs,
        num_instance: num_instance,
    };
    
    let cfg = run_instance(cfg);
    //upon completing a run, I'd like to serialize the instance for analysis later.
}

fn run_instance<T: Strategy, U: Strategy>(mut config: Config<T, U>) -> Config<T, U> {
    for _idx in 1..=config.num_rounds {
        let move_a = config.player_a.strategy();
        let move_b = config.player_b.strategy();

        let moves_a = (move_a.clone(), move_b.clone());
        let moves_b = (move_b.clone(), move_a.clone());

        let outcome_a = config.game.turn_outcome(move_a as usize, move_b as usize);

        let temp = outcome_a.clone();
        let outcome_b = (temp.1, temp.0);
        let tmp_a = config.player_a.get_player();
        let tmp_b = config.player_b.get_player();
        tmp_a.read_mv(moves_a, outcome_a);
        tmp_b.read_mv(moves_b, outcome_b);
    }

    config
}
