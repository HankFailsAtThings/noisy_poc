use crate::game::Game;
use crate::noisemodels::*; 
use std::clone::Clone;
use serde::Serialize;
use crate::Strategies;
use rand::seq::SliceRandom;
use rand::thread_rng;


//what are some things that a testbed should have? run
#[derive(Clone,Serialize)]
pub struct Config<T, U> {
    pub player_a: T,
    pub player_a_num: usize,
    pub player_b: U,
    pub player_b_num: usize,
    pub game: Game,
    pub num_rounds: usize,
    pub num_round_lengths: Vec<i32>,
    pub location: String,
    pub noisemodel : BaseNoiseModel,
}

pub fn generate_round_robin_configs (
    game: Game,
    players: Vec<Strategies>,
    round_lengths: Vec<i32>,
    location: String,
    noise: BaseNoiseModel ) 
    -> Vec<Config<Strategies,Strategies>> 
{
    //then create all the configs
    let mut configs = Vec::new();

    for i_idx in 0..players.len() {
        for j_idx in i_idx+1..players.len(){
            let tmp_cfg = Config {
                player_a: players[i_idx].clone(),
                player_a_num: i_idx,
                player_b: players[j_idx].clone(),
                player_b_num: j_idx,
                game: game.clone(),
                num_rounds: round_lengths.len(),
                num_round_lengths: round_lengths.clone(),
                location: location.to_string().clone(),
                noisemodel: noise.clone(),
            };

            configs.push(tmp_cfg);
        }
    }
    configs
}

pub fn generate_axelrod_configs ( 
    game: Game,
    mut players: Vec<Strategies>,
    round_lengths: Vec<i32>,
    location: String,
    noise: BaseNoiseModel,
    ) -> Vec<Config<Strategies,Strategies>>
{
    let mut configs = Vec::new();
    let mut rng = thread_rng();
    // shuffle players
    players.shuffle(&mut rng); // yay docs     
    // pair players with the player next to them 
    let mut i = 0;
    while i < players.len() - 1 {
        let tmp_cfg = Config { 
                player_a: players[i].clone(),
                player_a_num: i,
                player_b: players[i+1].clone(),
                player_b_num: i+1,
                game: game.clone(),
                num_rounds: round_lengths.len(),
                num_round_lengths: round_lengths.clone(),
                location: location.to_string().clone(),
                noisemodel: noise.clone(),        
        }; 
        i += 2; // skip next player        
        configs.push(tmp_cfg);
    }
    configs    
}


pub fn generate_players_by_numbers(strat_types: Vec<Strategies>, num_strats: Vec<i32>) -> Vec<Strategies> {
        let mut players = Vec::new();
        for i in 0..strat_types.len() {
            for _j in 0..num_strats[i] {
                players.push(strat_types[i].clone());
            }
        }
        players 
}

pub fn generate_players (
    strat_types: Vec<Strategies>, 
    num_strats: Vec<i32>
    ) 
    -> Vec<Strategies> 
{
    let mut players = Vec::new();
    for i in 0..num_strats.len() {
            let idx = i % 5; // TODO recommend changing in final product
            players.push(strat_types[idx].clone());
    }
    players
}
