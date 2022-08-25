use crate::*;
const BOARD_SIZE: usize = 3;
#[derive(BorshDeserialize, Serialize, BorshSerialize,Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Piece {
    X,
    O,
}
impl Piece {
    pub fn other(self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}
pub type Tile = Option<Piece>;
pub type Block = [Tile; BOARD_SIZE * BOARD_SIZE];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}
#[derive(Debug, Clone)]
pub enum MoveError {
    GameAlreadyOver,
    InvalidPosition { coordinate: usize },
    TileNotEmpty { other_piece: Piece, coordinate: usize },
}
#[derive(Debug, Clone)]
pub struct TicTacToe {
    block: Block,
    current_piese: Piece,
    winner: Option<Winner>,
}
impl TicTacToe {
    pub fn new() -> Self {
        Self {
            block: Default::default(),
            current_piese: Piece::X, // start X piese
            winner: None,
        }
    }
    pub fn make_move(&mut self,coordinate: usize  )-> Result<(), MoveError> {
        if self.is_finished() {
            return Err(MoveError::GameAlreadyOver);
        }
        else if coordinate >= 9 {
            return Err(MoveError::InvalidPosition {coordinate});
        }
        else if let Some(other_piece) = self.block[coordinate]{
            return Err(MoveError::TileNotEmpty{other_piece,coordinate})
        }
        self.block[coordinate] = Some(self.current_piese);
        self.current_piese =self.current_piese.other();
        self.update_winner(coordinate);
        Ok(())
    }
    fn update_winner(&mut self, coordinate: usize) {
        //let cors = self.block.len();
        //assert!(cors ==9,
        //    "This code was written with the assumption that there are three rows and columns");
            let row_tile = if coordinate%3 == 0{
                 [self.block[coordinate],self.block[coordinate+1],self.block[coordinate+2]]
            }
            else if coordinate%3 == 1{
                 [self.block[coordinate],self.block[coordinate+1],self.block[coordinate-1]]
            }
            else if coordinate%3 == 2{
                 [self.block[coordinate],self.block[coordinate-2],self.block[coordinate-1]]
            }
            else {
                 [None,None,None]
            };
            let col_tile = if coordinate< 3{
                [self.block[coordinate],self.block[coordinate+3],self.block[coordinate+6]]
            }
            else if coordinate>2 && coordinate<6{
                [self.block[coordinate],self.block[coordinate-3],self.block[coordinate+3]]
            }
            else if coordinate<9{
                [self.block[coordinate],self.block[coordinate-3],self.block[coordinate-6]]
            }
            else {
                 [None,None,None]
            };
            let diagonal_1 = if coordinate ==0 ||coordinate ==4 ||coordinate ==8{
                [self.block[0],self.block[4],self.block[6]]
            }
            else {
                [None,None,None]
            };

             let diagonal_2 = if coordinate ==2 ||coordinate ==4 ||coordinate ==6{
                 [self.block[2],self.block[4],self.block[6]]
            }
            else {
                [None,None,None]
            };
            fn check_winner(row: &[Tile]) -> Option<Winner> {
                if row[0]== row[1]&& row[1]==row[2]{
                    match row[0]{
                        Some(Piece::X) => Some(Winner::X),
                        Some(Piece::O) => Some(Winner::O),
                        None =>None,
                    }
                }
                else{
                    None
                }
            }
            self.winner= self.winner
            .or_else(|| check_winner(&diagonal_1))
            .or_else(|| check_winner(&diagonal_2))
            .or_else(|| check_winner(&row_tile))
            .or_else(|| check_winner(&col_tile));

            self.winner = self.winner.or_else(|| {
                if self.block.iter().all(|tile| tile.is_some()) {
                    Some(Winner::Tie)
                }
                else {
                    None
                }
            });
        }
            pub fn is_finished(&self) -> bool{
                self.winner.is_some()
            }
            pub fn winner(&self)-> Option<Winner>{
                self.winner
            }
            pub fn current_piese(&self)-> Piece{
                self.current_piese
            }
            pub fn block(&self) -> &Block{
                &self.block
            }
    }
    #[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn row_3_o_wins() {
        let mut game = TicTacToe::new();
        game.make_move(6).unwrap();
        game.make_move(0).unwrap();
        game.make_move(7).unwrap();
        game.make_move(1).unwrap();
        game.make_move(5).unwrap();
        game.make_move(2).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::O);
    }
    #[test]
    fn diag_2_x_wins() {
        let mut game =  TicTacToe::new();
        game.make_move(2).unwrap();
        game.make_move(1).unwrap();
        game.make_move(4).unwrap();
        game.make_move(3).unwrap();
        game.make_move(6).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }
    #[test]
    fn tie() {
        let mut game =TicTacToe::new();
        game.make_move(0).unwrap();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(3).unwrap();
        game.make_move(4).unwrap();
        game.make_move(5).unwrap();
        game.make_move(7).unwrap();
        game.make_move(6).unwrap();
        game.make_move(8).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }
    #[test]
    fn finish() {
        let mut game =TicTacToe::new();
        game.make_move(0).unwrap();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(3).unwrap();
        game.make_move(4).unwrap();
        assert_eq!(game.is_finished(), false);
    }
}
