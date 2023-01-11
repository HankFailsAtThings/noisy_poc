pub mod player;
pub mod noisemodels; // 
use std::sync::{Arc, Mutex};
use crate::noisemodels::*; // why does this need to happen? 
use crate::player::*;
pub mod testbed;
pub mod game;
pub mod test_utilities;
use crate::testbed::Config;
//use std::thread;
use std::env;
use std::fs;
use serde::Serialize;
use threadpool::ThreadPool;
//use redis::*;
use std::any::type_name;




#[derive(Clone,Serialize)]
pub enum Strategies {
    AlwaysDefect{player: AlwaysDefect},
    GrimTrigger{player: GrimTrigger},
    TitForTat{player: TitForTat},
    RandomDefect{player: RandomDefect},
    TitForAverageTat{player: TitForAverageTat},
}



impl Strategy for Strategies {
    fn strategy(&mut self) -> i32 {
        match self {
            Strategies::AlwaysDefect{ player } => player.strategy(),
            Strategies::GrimTrigger{ player } => player.strategy(),
            Strategies::TitForTat{ player } => player.strategy(),
            Strategies::RandomDefect{ player } => player.strategy(),
            Strategies::TitForAverageTat{ player } => player.strategy(),
        }
    }

    fn get_strategy(&self) -> String {
        match self {
            Strategies::AlwaysDefect{ player } => player.get_strategy(),
            Strategies::GrimTrigger{ player } => player.get_strategy(),
            Strategies::TitForTat{ player } => player.get_strategy(),
            Strategies::RandomDefect{ player } => player.get_strategy(),
            Strategies::TitForAverageTat{ player } => player.get_strategy(),
        }
    }


    fn get_player(&mut self) -> &mut BasicPlayer {
        match self {
            Strategies::AlwaysDefect{ player } => player.get_player(),
            Strategies::GrimTrigger{ player } => player.get_player(),
            Strategies::TitForTat{ player } => player.get_player(),
            Strategies::RandomDefect{ player } => player.get_player(),
            Strategies::TitForAverageTat{ player } => player.get_player(),
        }
    }
}


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut is_round_robin = true; 
    let mut basedirstr = test_utilities::build_datetime_folder("/tmp/test_runs/".to_string()); 
    if args.len()  > 1  {
        if args[1].eq("axelrod") {
            is_round_robin = false; 
        }
        if args.len() > 2 { // TODO is v jank
            basedirstr = test_utilities::build_datetime_folder(args[2].clone().to_string());
        }
        else {
            basedirstr = test_utilities::build_datetime_folder(args[1].clone().to_string()); 
        }
    }

    println!("{}", basedirstr);
    //let num_players = 100 - 1;
    //let mut num_strategies = vec![20, 20, 20, 20, 20];
    let mut num_strategies = vec![2000, 2000, 2000, 2000, 2000];
    //for _i in 0..num_players { 
    //	num_strategies.push(20);
    //}
    println!("{}", num_strategies.len());
    
    let round_lengths = vec![63, 77, 151, 151, 308];
    
    //potential strategies for now are always defect, tit for tat, and grim trigger
    let a_mtx = vec![
        vec![3,0],
        vec![5,1]
    ];

    let b_mtx = vec![
        vec![3,5],
        vec![0,1]
    ];
    let mut g = game::Game{ 
        payoff_a: a_mtx,
        payoff_b: b_mtx,
        is_init: false};
    g.init_game(); 
    assert!(g.is_init);

    let base_player = BasicPlayer::new();

    let alwaysdefect_tmp = AlwaysDefect { play: base_player.clone() };
    let grimtrigger_tmp = GrimTrigger { play: base_player.clone(), trig : false };
    let titfortat_tmp = TitForTat { play: base_player.clone() };
    let randomdefect_tmp = RandomDefect { play: base_player.clone(), probability: 0.5 };
    let titforaveragetat_tmp = TitForAverageTat { play: base_player.clone() , memory : 0};


    let a_strat = Strategies::AlwaysDefect{ player: alwaysdefect_tmp };
    let b_strat = Strategies::GrimTrigger{ player: grimtrigger_tmp };
    let c_strat = Strategies::TitForTat{ player: titfortat_tmp };
    let d_strat = Strategies::RandomDefect{ player: randomdefect_tmp };
    let f_strat = Strategies::TitForAverageTat{ player: titforaveragetat_tmp };
    let strat_types = vec![
        a_strat,
        b_strat,
        c_strat,
        d_strat,
        f_strat,
    ];

    
    // are we running a single round robin, or an axelrod tourny 
    if is_round_robin {
        println!("running a single round robin"); 
        let players = testbed::generate_players_by_numbers(strat_types, num_strategies); 
        
        // automate running configs at different noise level
        let num_runs = 5;
        let noise_vec = vec![100, 99, 95, 90, 85];
        for run in 0..num_runs {let mut new_strats = vec![0,0,0,0,0];
            let idx = run % noise_vec.len();
            println!("run {0}, noise {1}", run, noise_vec[idx]) ;
            let noise = BaseNoiseModel::new(noise_vec[idx]);
            let dirstr = format!("{0}{1}{2}" , basedirstr, "_run_", run);
            let mut configs = testbed::generate_round_robin_configs(
                g.clone(), players.clone(), round_lengths.clone(), dirstr , noise );
            run_round_robin(configs);
        }
    }
    else { // is axelrod
        println!("running an axelrod tournament, \n Note: currently only supports even player counts"); 
        // we will only have a single game of 151 iterations
        println!("{:?}", num_strategies);
        let axelrod_round_lengths = vec![151];
        let noise = BaseNoiseModel::new(55);
        let players = testbed::generate_players_by_numbers(strat_types, num_strategies);
        let configs = testbed::generate_axelrod_configs(
                g.clone(), players.clone(),axelrod_round_lengths.clone(), basedirstr , noise );        
        
        let mut rnd_output =  run_round(configs);
        //println!("{:?}", rnd_output.len());
        
        //NOTE: this is writen on the assumption that each inner vec is one element long
        // the inner vec is one element long since each pair only play one game 
        // also written on the assumption that players have accurate score info 
        let mut new_strats = vec![0,0,0,0,0];
        for i in 1..rnd_output.len() {
               // print this round's num strat vectors
               //println!("vec {:?} is {:?} items long", i , &rnd_output[i].len());
               let pl_a_strat = &rnd_output[i][0].player_a.get_strategy();
               let pl_b_strat = &rnd_output[i][0].player_b.get_strategy();
               //println!("{:?} {:?}" , pl_a_strat, pl_b_strat);
                
               //pull both player's scores // TODO assumes players have accurate scores
               let pl_a = &rnd_output[i][0].player_a.get_player().get_my_score();  
               let pl_b = &rnd_output[i][0].player_b.get_player().get_my_score();
               let wining_strat;
               if pl_b > pl_a { // if tie, pick player 1 for simplicity TODO figure out what to do about ties
                        // clone and double player_b
                        wining_strat = pl_b_strat;
               }
               else {
                        // clone and double player_a 
                        wining_strat = pl_a_strat;
               }
               
               //println!("player A score//strat:{:?}  {:?}\nplayer B score//strat :{:?} {:?}\n winning strat {:?}" ,
                //        pl_a, pl_a_strat , pl_b, pl_b_strat, wining_strat);
               // set num_strats to 0 for now TODO 
               //for g  in 1..num_strategies.len() {
               //       num_strategies[g] = 0;
               //}
               // there Are MUCH better ways to do this, doing the easy way rn  
               // let strats = vec!("","","","","")
                // this is the JUST MAKE IT WORK phase
        
    let alwaysdefect_tmp2= AlwaysDefect { play: base_player.clone() };
    let grimtrigger_tmp2= GrimTrigger { play: base_player.clone(), trig : false };
    let titfortat_tmp2 = TitForTat { play: base_player.clone() };
    let randomdefect_tmp2 = RandomDefect { play: base_player.clone(), probability: 0.5 };
    let titforaveragetat_tmp2 = TitForAverageTat { play: base_player.clone() , memory : 0};
    
        
    let a_strat2 = Strategies::AlwaysDefect{ player: alwaysdefect_tmp2 };
    let b_strat2 = Strategies::GrimTrigger{ player: grimtrigger_tmp2 };
    let c_strat2 = Strategies::TitForTat{ player: titfortat_tmp2 };
    let d_strat2 = Strategies::RandomDefect{ player: randomdefect_tmp2 };
    let f_strat2 = Strategies::TitForAverageTat{ player: titforaveragetat_tmp2 };
    let strat_types2 = vec![
        a_strat2,
        b_strat2,
        c_strat2,
        d_strat2,
        f_strat2,
    ];      


               for f in 0..strat_types2.len()  {
                        if wining_strat.eq(&strat_types2[f].get_strategy()) {
                              // inc new num_strat vector in the right place 
                              new_strats[f] = new_strats[f] +  1; 
                        }
               }
               // new strat numbers       
               
              }          
        
        
        println!("{:?}", new_strats);
        
        }
        
         
                // run a round
                // split players into winners/losers 
                // tie? pick the first player 

                // remove losers, replace with duplicated players
                // record strat numbers 
        // repeat for n rounds or until one strat is left 
        

    }   
//}




fn run_round(mut configs: Vec<Config<Strategies,Strategies>>) -> Vec<Vec<Config<Strategies,Strategies>>> {

    let num_workers = 30;
    let pool = ThreadPool::new(num_workers);
    // gonna need some locks
    let round_results : Vec<Vec<Config<Strategies,Strategies>>> = Vec::new(); 
    let round_results_mutex = Arc::new(Mutex::new(round_results)); 
    
    //run configs through the pool, record_config saves results
    for _idx in 0..configs.len() {
        let results_mutex_clone = Arc::clone(&round_results_mutex);

        let tmp_config = configs.pop().unwrap();
        pool.execute(move || {
            let out_configs = run_instance(tmp_config);
            //record_configs(out_configs);
            let mut data = results_mutex_clone.lock().unwrap();
            data.push(out_configs);
            drop(data); 
        });
    }
    pool.join();
    
    return round_results_mutex.lock().unwrap().to_vec(); // rust is kinda convoluted ngl 

}



fn run_round_robin(mut configs: Vec<Config<Strategies,Strategies>>){

    let num_workers = 30; 
    let pool = ThreadPool::new(num_workers);
  
    //run configs through the pool, record_config saves results
    for _idx in 0..configs.len() {

        let tmp_config = configs.pop().unwrap();
	        pool.execute(move || {
            let out_configs = run_instance(tmp_config);
            record_configs(out_configs);
        });
    }
    pool.join();
}

// Note: redis is an in memory db only 
// fn record_configs_db<T:Serialize,U:Serialize>(configs: Vec<Config<T,U>>) {
//         let client = redis::Client::open("redis://127.0.0.1/").expect("failed to open redis");
//         let mut con = client.get_connection().expect("failed to connect to redis");
//         for idx in 0..configs.len() {
//                 for round in 0..configs[idx].num_round_lengths.len() {
//                         //make Key noise_playerA_playerB_roundnum
//                         let key = format!("{}_{}_{}_{}_{}", &configs[idx].noisemodel.chance, &configs[idx].player_a_num, &configs[idx].player_b_num, &configs[idx].num_rounds, &configs[idx].num_round_lengths[round]);
//                         let json = serde_json::to_string(&configs[idx]).unwrap();
//                         let _: () = redis::cmd("SET").arg(key).arg(json).query(&mut con).expect("failed to set 'json'");
//                 
//                 }
//              
//         }
// }

fn record_configs<T:Serialize,U:Serialize>(configs: Vec<Config<T,U>>) {
    let s = &configs[0].location;
    let group_dir = format!("{}/player{}player{}/", s, &configs[0].player_a_num, &configs[0].player_b_num);
    
    fs::create_dir_all(&group_dir).expect("Directory unable to be created");

    for idx in 0..configs.len() {
        let run_dir = format!("{}round{}play_num{}.json", group_dir, &configs[0].num_round_lengths[idx], idx);
        let j = serde_json::to_string(&configs[idx]).unwrap();

        //println!("{}", &run_dir);

        fs::write(run_dir, j).expect("Unable to write file");
    }
}


fn run_instance<T: Strategy+Clone, U: Strategy+Clone>(config: Config<T, U>) -> Vec<Config<T, U>> {
    let mut configs = Vec::new();
    
    

    for idx in 0..config.num_rounds {
        
        let mut tmp_cfg = config.clone();
        
        for _ in 0..config.num_round_lengths[idx] {
            let mut move_a = tmp_cfg.player_a.strategy();
            let mut move_b = tmp_cfg.player_b.strategy();

            // here we introduce game noise
            move_a = tmp_cfg.noisemodel.modify(move_a); 
            move_b = tmp_cfg.noisemodel.modify(move_b); 

            let moves_a = (move_a.clone(), move_b.clone());
            let moves_b = (move_b.clone(), move_a.clone());

            let outcome_a = tmp_cfg.game.turn_outcome(move_a as usize, move_b as usize);
            // NOTE : as written, the players get an accurate percieved outcome, changing this will require changing tabulation in the main function ( dealing with the axelrod tourny ) 
            let temp = outcome_a.clone();
            let outcome_b = (temp.1, temp.0);
            let tmp_a = tmp_cfg.player_a.get_player();
            let tmp_b = tmp_cfg.player_b.get_player();
            tmp_a.read_mv(moves_a, outcome_a);
            tmp_b.read_mv(moves_b, outcome_b);

            

        }
        configs.push(tmp_cfg);
    }
    configs
}
