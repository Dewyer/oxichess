use crate::chess::piece::{ChessPiece, PieceType};
use crate::chess::board_creator;
use crate::chess::error::ChessError;
use crate::chess::piece::moves::{PlayerMove, PiecePosition};

pub type ChessBoard = Vec<Vec<ChessPiece>>;

#[derive(Debug)]
pub struct ChessGame
{
    pub is_whites_turn:bool,
    pub board: ChessBoard
}

impl ChessGame
{
    pub fn get_piece_at(&self,pos:PiecePosition) -> &ChessPiece
    {
        &self.board[pos.row as usize][pos.col as usize]
    }

    pub fn is_position_occupied(&self,pos:PiecePosition) -> bool
    {
        self.get_piece_at(pos).piece_type != PieceType::Empty
    }

    pub fn new() -> Self
    {
        Self
        {
            is_whites_turn:true,
            board: board_creator::construct_starter_board()
        }
    }

    pub fn make_move(&mut self,player_move:PlayerMove) -> Result<(),ChessError>
    {
        if player_move.owner_is_white != self.is_whites_turn
        {
            return Err(ChessError::InvalidMove);
        }

        Ok(())
    }

}
