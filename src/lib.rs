mod tictactoe ;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{/*env,AccountId,*/near_bindgen, setup_alloc,/*BlockHeight,*/   PanicOnDefault, BorshStorageKey};
use near_sdk::collections::{/*UnorderedMap,LookupMapm,*/ Vector};
use near_sdk::serde::{Deserialize, Serialize};
//use near_sdk::json_types::Base64VecU8;
use std::io::{self, Write};
use std::process;
use tictactoe::{TicTacToe,Block, Piece, Winner, MoveError};

setup_alloc!();
//const FIELD_LEN: usize = 9;
pub type GameId = u64;
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Games,//0 bit
    //Accounsts,//1 bit //not Delete
    //AccounstsBoard for saved game
}
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
    pub struct Contract {
        pub saves: Vector<Block>,
      // pub accounts: UnorderedMap<AccountId, Account>
    }

    #[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
    pub struct InvalidMove(pub String);

    #[near_bindgen]
    impl Contract {
        #[init]
        pub fn new()-> Self{
            Self {
                saves: Vector::new(StorageKey::Games),
                //accounts: UnorderedMap::new(StorageKey::Accounsts),
            }
        }
        pub fn save(&mut self,block: Block){
            self.saves.push(&block)
        }
        pub fn get_save(&mut self) -> Option<Block>{
            let index = self.saves.len();
            self.saves.get(index-1)
        }
        pub fn newttt() {
            let mut game =  TicTacToe::new();
            let mut contract = Contract::new();
            print_tiles(*game.block());
            while !game.is_finished() {
                contract.save(*game.block());
                print_tiles(contract.get_save().unwrap());
                print_tiles(*game.block());
                println!("Current piece: {}", match game.current_piese() {
                    Piece::X => "x",
                    Piece::O => "o",
                });
                let coordinate = prompt_move();

                match game.make_move(coordinate) {
                    Ok(()) => {},

                    Err(MoveError::GameAlreadyOver) => unreachable!("Game was already over when it should not have been"),

                    Err(MoveError::InvalidPosition {coordinate}) => {
                        unreachable!("Should not be able to enter an invalid move, but still got ({})",coordinate)
                    },

                    Err(MoveError::TileNotEmpty {other_piece, coordinate}) => eprintln!(
                        "The tile at position {} already has piece {} in it!",

                        coordinate+1,
                        match other_piece {
                            Piece::X => "x",
                            Piece::O => "o",
                        },
                    ),
                }
            }
            match game.winner().expect("finished game should have winner") {
                Winner::X => println!("x wins!"),
                Winner::O => println!("o wins!"),
                Winner::Tie => println!("Tie!"),
            }
        }
    }

        pub fn prompt_move() -> usize {
            loop {
                print!("Enter move (e.g. 5): ");
                io::stdout().flush().expect("Failed to flush stdout");
                let line = read_line();
                match parse_move(&line) {
                    Ok(coordinate)=> break (coordinate),
                    Err(InvalidMove(invalid_str)) => eprintln!(
                        "Invalid move: '{}'. Please try again.",
                        invalid_str,
                    ),
                }
            }
        }

        pub fn parse_move (input: &str) -> Result<usize, InvalidMove> {
            if input.len() != 1 {
                return Err(InvalidMove(input.to_string()));
            }
            let coordinate = match &input[0..1] {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                "4" => 3,
                "5" => 4,
                "6" => 5,
                "7" => 6,
                "8" => 7,
                "9" => 8,
                _ => return Err(InvalidMove(input.to_string())),
            };
            Ok(coordinate)
        }
        pub fn read_line() -> String {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.is_empty() {
            println!();
            process::exit(0);
            }
            let len_without_newline = input.trim_end().len();
            input.truncate(len_without_newline);
            input
        }

    pub fn print_tiles(block: Block) {
        println!("{} {} {}",1,2,3);
        println!("{} {} {}",4,5,6);
        println!("{} {} {}",7,8,9);
        println!();
        let mut i =0;
            while i!=9{
                println!("{} {} {}",
                match block[i] {
                    Some(Piece::X) => "x",
                    Some(Piece::O) => "o",
                    None => "\u{25A2}",
                },
                match block[i+1] {
                    Some(Piece::X) => "x",
                    Some(Piece::O) => "o",
                    None => "\u{25A2}",
                },
                match block[i+2] {
                    Some(Piece::X) => "x",
                    Some(Piece::O) => "o",
                    None => "\u{25A2}",
                });
                i= i +3
            }

    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use near_sdk::test_utils::VMContextBuilder;
        use near_sdk::MockedBlockchain;
        use near_sdk::{testing_env, VMContext};

        fn get_context(is_view: bool) -> VMContext {
            VMContextBuilder::new().is_view(is_view).build()
        }
    #[test]
    fn test_new_contract() {
        let context = get_context(false);
        testing_env!(context);
        let _contract = Contract::new();
    }
    #[test]
    fn test_save_get() {
        let context = get_context(false);
        testing_env!(context.clone());
        let mut contract = Contract::new();
        let mut block =[std::option::Option::None; 9];
        block[2]=Some(Piece::X);
        contract.save(block.clone());
        print_tiles(block);
        testing_env!(get_context(true));
        let save = contract.get_save().unwrap();
        print_tiles(save);
        assert_eq!(save,block);
    }
}
