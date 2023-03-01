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
//use std::any::type_name;
use rand::thread_rng;
use rand::seq::SliceRandom;




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
    let mut is_iterated_round_robin = false;
    let mut basedirstr = test_utilities::build_datetime_folder("/tmp/test_runs/".to_string()); 
    if args.len()  > 1  {
        if args[1].eq("knockout") {
            is_round_robin = false; 
        }
        if args[1].eq("iterated") {
	    is_iterated_round_robin = true;
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
    let mut num_strategies = vec![0,0,100,0,100];
    //let mut num_strategies = vec![500, 500, 500, 500, 500];
    let mut num_players = 0;
    for i in 0..num_strategies.len() {
	num_players = num_players + num_strategies[i];
    }
    
    //let round_lengths = vec![63, 77, 151, 151, 308];
    let round_lengths = vec![308];
    
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

    // are we running a single round robin,  multiple round robins , or a knockout tourny 
    if is_round_robin {
        println!("running a single round robin"); 
        let players = testbed::generate_players_by_numbers(&strat_types, num_strategies); 
        
        // automate running configs at different noise level
        let noise_vec = vec![100, 99, 95, 90, 85,80,75,70,65,60,55,50,45,40,35,30,25,20,15,10,5,0];
        for run in 0..noise_vec.len() {
            let idx = run % noise_vec.len();
            println!("run {0}, noise {1}", run, noise_vec[idx]) ;
            let noise = BaseNoiseModel::new(noise_vec[idx]);
            let dirstr = format!("{0}{1}{2}" , basedirstr, "_run_", run);
            let configs = testbed::generate_round_robin_configs(
                g.clone(), players.clone(), round_lengths.clone(), dirstr , noise );
            run_round_robin(configs);
        }
    }
    else if is_iterated_round_robin {
	println!("running interative round robin tournament");
	// run round , order players by score , top half of players move on 
        //let noise_vec = vec![100,99,95,90,85,80,75,70,65,60,55,50,45,40,35,30,25,20,15,10,5,0];	
        let noise_vec = vec![100];	
        for idx in 0..noise_vec.len() {  // run under multi nooise model
                let mut temp_strats = num_strategies.clone();
        	println!("{}", noise_vec[idx]);
        	let num_rounds = 10; 
        	for _rnds in 0..num_rounds {
        		let players = testbed::generate_players_by_numbers(&strat_types, temp_strats);

        		let iterated_rr_round_lengths = vec![151];
        	        let noise = BaseNoiseModel::new(noise_vec[idx]);
        	        let dirstr = basedirstr.clone();
			// TODO this is a lot of memory useage for large player counts, O(N!), have to batch process the player generation, game playing, would require large rewrites, or we could just download more ram
        	        let configs = testbed::generate_round_robin_configs(g.clone(), players, iterated_rr_round_lengths, dirstr , noise );
        	        let mut rnd_output = run_round(configs);
        		#[derive(Clone)] 
        		struct StratScore {name : String , score : i32 }
        		let mut next_generation = vec![StratScore {name : String::new(), score : 0} ; num_players.try_into().unwrap()]; // woo useless error chks
        		
        	        for _i in 0..rnd_output.len() {
				let game = &mut rnd_output.pop().unwrap();
				
				let pl_a_strat = game[0].player_a.get_strategy();
                        	let pl_b_strat = game[0].player_b.get_strategy();
                        	let pl_a = game[0].player_a.get_player().get_my_score().clone();
                        	let pl_b = game[0].player_b.get_player().get_my_score().clone();
                        	let pl_a_num = game[0].player_a_num;
                        	let pl_b_num = game[0].player_b_num;
                        	next_generation[pl_a_num].score = next_generation[pl_a_num].score + pl_a;
                        	next_generation[pl_a_num].name = pl_a_strat.to_string(); 
                        	next_generation[pl_b_num].score = next_generation[pl_b_num].score + pl_b;	
                        	next_generation[pl_b_num].name = pl_b_strat.to_string(); 

        		}	
                        //println!("presort");
//                        for iter in 0..next_generation.len() {
//                                let temp = next_generation[iter].clone();
//                                println!("{:?} {:?}", temp.name, temp.score);
//                        }
                       // shuffle before sorting ,, there is a reason behind this ,, if scores are equal, then rust sort maintains the order of the origal vector, this causes weird outcomes 
                       let mut rng = thread_rng();
        
                        next_generation.shuffle(&mut rng);
        		next_generation.sort_by(|a,b| a.score.cmp(&b.score)); // things like this make me <3 new lang's
                        
//                        //println!("postsort");
//                        for iter in 0..next_generation.len() {                              
//                               let temp = next_generation[iter].clone();
//                                //println!("{:?} {:?}", temp.name, temp.score);
//                        }

        		let mut new_strats = vec![0,0,0,0,0];
        		for i in 0..num_players/2 {
				let temp = next_generation.pop().unwrap();	
        			//let temp = next_generation.pop().unwrap().name;
				//println!("{:?} {:?} moves on", temp.name, temp.score);
				//TODO maybe the median method is bad, at 100% TfAvgT wins when it should be identical to tft, seems to be how the sorting alg put tfAvgt > tft when scores are equal
        			for f in 0..strat_types.len()  {                
        	       			 if temp.name.eq(&strat_types[f].get_strategy()) {
        	         	      	      new_strats[f] = new_strats[f] + 2;
        	                 	}
        			}
        		}
        		temp_strats = new_strats;
        		println!("{:?}", temp_strats); 
        	}
	}

	}
    else { // is knockout
        println!("running an knockout tournament, \n Note: not fully implemented, currently only supports even player counts"); 
        // we will only have a single game of 151 iterations
        println!("{:?}", num_strategies);
        let knockout_round_lengths = vec![151];
	let knockout_rounds = 25; 
	for _k in 1..knockout_rounds {
        	 let noise = BaseNoiseModel::new(90);
       		 let players = testbed::generate_players_by_numbers(&strat_types, num_strategies);
       		 let configs = testbed::generate_knockout_configs(
       		         g.clone(), players.clone(),knockout_round_lengths.clone(), basedirstr.clone(), noise );        
       		 
       		 let mut rnd_output =  run_round(configs);
       		 //println!("{:?}", rnd_output.len());
       		 
       		 //NOTE: this is writen on the assumption that each inner vec is one element long
       		 // the inner vec is one element long since each pair only play one game 
       		 // also written on the assumption that players have accurate score info
		 // alwaysdefect, GrimTrigger, TitForTat, Random Defect, TitForAverageTat
       		 let mut new_strats = vec![0,0,0,0,0];
       		 for i in 0..rnd_output.len() {
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
       		        
       		        //println!("player A score//strat:{:?}  {:?}\nplayer B score//strat :{:?} {:?}\n winning strat {:?}" , pl_a, pl_a_strat , pl_b, pl_b_strat, wining_strat);
       		 	for f in 0..strat_types.len()  {                                    
       		        		if wining_strat.eq(&strat_types[f].get_strategy()) {
       		        	         // inc new num_strat vector in the right place 
       		         	      new_strats[f] = new_strats[f] + 2;  // TODO bd ccode and u should feel bd
       		                 }
       		 	}

       		        // new strat numbers       
       		 	// multi
       		        
       		       } 
       		 
       		 num_strategies = new_strats; 		 
		 let mut sum_strats = 0; 
                 for j in 0..num_strategies.len() {
                 	sum_strats =  sum_strats + num_strategies[j]; 
                 }         
		
       		 println!("{:?}", num_strategies);
        }
    }

}   



fn run_round(mut configs: Vec<Config<Strategies,Strategies>>) -> Vec<Vec<Config<Strategies,Strategies>>> {

    let pool = ThreadPool::new(30);

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
            // NOTE : as written, the players get an accurate percieved outcome, changing this will require changing tabulation in the main function ( dealing with the knockout tourny ) 
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

