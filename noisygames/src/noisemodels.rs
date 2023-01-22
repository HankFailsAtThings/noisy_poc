use rand::Rng;
use std::clone::Clone;
use serde::Serialize;


#[derive(Clone,Serialize)]
pub struct BaseNoiseModel {
        pub name: String,
        pub chance: i32,       
}

impl BaseNoiseModel {
        pub fn new( p : i32) -> BaseNoiseModel {
             assert!( p >= 0 && p <= 100); 
             BaseNoiseModel {
                name: "hellaBasic".to_string(),
                chance: p, // p is expected to be 0 < p <= 100 
             }
        }
        fn flip(&self , mov : i32 ) -> i32 {
                if mov == 1 {
                        0
                } else {
                        1
                }
        }
}


pub trait Noise {
        fn modify(&self, actual_move: i32) -> i32;
}

impl Noise for BaseNoiseModel {
        fn modify(&self, actual_move : i32) -> i32 {
                // rolls a rand int 0 < i < 100 , if i > p then it flips the outcome 
                let mut rng = rand::thread_rng(); 
                let rand = rng.gen_range(1..101);   
                if rand > self.chance {
//                      println!("flipped {0}, {1}",  rand, self.chance);
                        self.flip(actual_move)
                }
                else {
                        actual_move
                }
        }

}

//always flip move
/*
pub struct MaliciousChannel {
        pub model: BaseNoiseModel,
}

impl Noise for MaliciousChannel {
        fn modify_outcome(&self, actual_move : i32) -> i32 {
                self.model.flip(actual_move)
        }
}
*/

