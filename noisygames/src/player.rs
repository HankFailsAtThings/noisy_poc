use std::clone::Clone;
use serde::Serialize;



#[derive(Clone,Serialize)]
pub struct BasicPlayer {
    pub name: String,
    pub my_moves: Vec<i32>,
    pub their_moves: Vec<i32>,
    pub my_outcomes: Vec<i32>,
    pub their_outcomes: Vec<i32>,
    pub my_score: i32,
    pub their_score: i32,
}

pub trait Player {
    fn get_name(&self) -> &str;
    fn get_my_score(&self) -> i32;
    fn get_their_score(&self) -> i32;
    fn get_my_moves(&self) -> &Vec<i32>;
    fn get_their_moves(&self) -> &Vec<i32>;
    fn get_my_outcomes(&self) -> &Vec<i32>;
    fn get_their_outcomes(&self) -> &Vec<i32>;
    fn update_score(&mut self, round_score: (i32, i32));
    fn read_mv(&mut self, read_mvs: (i32, i32), outcome: (i32, i32));
}
impl BasicPlayer {
    pub fn new() -> BasicPlayer {
        BasicPlayer {
            name: "john".to_string(),
            my_score: 0,
            their_score: 0,
            my_moves: Vec::new(),
            their_moves: Vec::new(),
            my_outcomes: Vec::new(),
            their_outcomes: Vec::new(),
        }
    }
}

impl Player for BasicPlayer {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    
    fn get_my_score(&self) -> i32 {
        self.my_score
    }

    fn get_their_score(&self) -> i32 {
        self.their_score
    }

    fn get_my_moves(&self) -> &Vec<i32> {
        &self.my_moves
    }

    fn get_their_moves(&self) -> &Vec<i32> {
        &self.their_moves
    }

    fn get_my_outcomes(&self) -> &Vec<i32> {
        &self.my_outcomes
    }

    fn get_their_outcomes(&self) -> &Vec<i32> {
        &self.their_outcomes
    }
    fn update_score(&mut self, round_score: (i32, i32)) {
        self.my_score += round_score.0;
        self.their_score += round_score.1;
    }
    fn read_mv(&mut self, read_mvs: (i32, i32), outcome: (i32, i32)) {
        self.my_moves.push(read_mvs.0);
        self.their_moves.push(read_mvs.1);
        self.my_outcomes.push(outcome.0);
        self.their_outcomes.push(outcome.1);
        self.update_score(outcome);
    }
}


pub trait Strategy {
	fn strategy(&mut self) -> i32; // mut is required for strats that update their internal state 
        fn get_strategy(&self) -> String; 
        fn get_player(&mut self) -> &mut BasicPlayer;
}

#[derive(Clone,Serialize)]
pub struct TitForTat {
	pub play: BasicPlayer,
}

impl Strategy for TitForTat {
	fn strategy(&mut self) -> i32 {
		let their_moves = &self.play.get_their_moves();
		if their_moves.len() > 0 {
			their_moves.last().unwrap().clone()
		} else {
			0
		}
	}

    fn get_strategy(&self) -> String {
        "TitForTat".to_string()
    }

    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}
#[derive(Clone,Serialize)]
pub struct GrimTrigger {
	pub play: BasicPlayer,
        pub trig: bool
}

impl Strategy for GrimTrigger {
	fn strategy(&mut self) -> i32 {
                if self.trig {
                        return 1; 
                }

		let their_moves = &self.play.get_their_moves();
		if their_moves.len() == 0 { 
			0
		} else if their_moves.last().unwrap().clone() == 0 {
			0
		}
		else {
                        self.trig = true;
			1
		}
	}

    fn get_strategy(&self) -> String {
        "GrimTrigger".to_string()
    }

    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}

#[derive(Clone,Serialize)]
pub struct AlwaysDefect {
	pub play: BasicPlayer,
}

impl Strategy for AlwaysDefect {
	fn strategy(&mut self) -> i32 {
		1
	}

    fn get_strategy(&self) -> String {
        "AlwaysDefect".to_string()
    }
    
    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}

#[derive(Clone,Serialize)]
pub struct RandomDefect {
    pub play: BasicPlayer,
    pub probability: f32,
}

use rand::Rng;

impl Strategy for RandomDefect {
    fn strategy(&mut self) -> i32 {
        let num: f32 = rand::thread_rng().gen_range(0..=100) as f32;
        if 100.0 * self.probability > num {
            1
        } else {
            0
        }
    }

    fn get_strategy(&self) -> String {
        "RandomDefect".to_string()
    }
    
    fn get_player(&mut self) -> &mut BasicPlayer {
        &mut self.play
    }
}

#[derive(Clone,Serialize)]
pub struct TitForAverageTat {
    pub play: BasicPlayer,
    pub memory : i32
}

/*impl TitForAverageTat {
        fn take_average(&self, slice : &[i32], len : usize) -> f32 {
                 let mut sum = 0.0; 
                 let floatLen = len as f32;
                 for i in 0..len {
                         let res = slice[i] as f32;
                         sum = sum + res; 
                 } 
                 sum / floatLen
        }
} */


// this player plays the average of its opponents last ten moves 
impl Strategy for TitForAverageTat {

        fn strategy(&mut self) -> i32 {
                //let their_move = &self.play.get_their_moves().last().unwrap();
                let their_moves = &self.play.get_their_moves();
                if their_moves.len() == 0 {
                        return 0;
                }
                let their_move = their_moves.last().unwrap().clone();             
                //let their_move = their_moves.last();             
                // we are going to add if the other player cooperates and sub if the other player defects
                //      staying in the range -5 <= i < 5 , if neg, defect, if pos  or 0 coop 
                let mut subbed = false; 
                if their_move == 0 { // they co-op'd 
                        self.memory += 1; 
                }
                else {
                        self.memory -= 1; 
                        subbed = true; 
                }
                //println!("TitForAverageTat : strat play , memory is currently {:?}, opponent played {:?} ", self.memory, their_move ); 
                if self.memory*self.memory > 25 {
                        // reset 
                        if subbed {
                                // add
                                self.memory += 1;
                        }
                        else {
                                self.memory -= 1;
                        }
                }                 
               // println!("TitForAverageTat : strat play2,  memory is currently {:?}, subbbed is {:?}", self.memory, subbed);
                if self.memory < 0 { // defect 
                        1 
                } 
                else { 
                        0
                }
                
                
        }
        fn get_strategy(&self) -> String {
                "TitForAverageTat".to_string()
        }
       
        fn get_player(&mut self) -> &mut BasicPlayer {
                &mut self.play
        }

}

