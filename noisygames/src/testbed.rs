use crate::game::Game;
use crate::noisemodels::*; 
use std::clone::Clone;
use serde::Serialize;
use crate::Strategies;

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

pub fn generate_players (
    strat_types: Vec<Strategies>, 
    num_strats: Vec<i32>
    ) 
    -> Vec<Strategies> 
{
    let mut players = Vec::new();
    for i in 0..num_strats.len() {
            let idx = i % 5; // edited for the purpose of timing program, TODO recommend changeing in final product
            players.push(strat_types[idx].clone());
    }
    players
}
