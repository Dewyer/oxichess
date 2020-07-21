mod helpers;
mod pawn;

use crate::chess::piece::moves::{PiecePosition, PieceContext};
use crate::chess::game::ChessGame;

pub mod moves;

#[derive(Debug,Clone,PartialEq)]
pub enum PieceType
{
    Knight,
    Rook,
    Queen,
    King,
    Pawn,
    Bishop,
    Empty,
}

#[derive(Debug)]
pub struct ChessPiece
{
    pub is_white:bool,
    pub piece_type:PieceType
}

impl ChessPiece
{
    pub fn get_threatened(&self,game:&ChessGame,position:PiecePosition) -> Vec<PiecePosition>
    {
        let ctx = PieceContext{piece:&self,game,piece_position:position};

        match &self.piece_type
        {
            PieceType::Pawn => pawn::get_threatened_by_pawn(ctx),
            _ => vec![]
        }
    }
}