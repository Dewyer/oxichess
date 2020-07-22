use crate::chess::error::ChessError;
use crate::chess::piece::ChessPiece;
use crate::chess::game::{ChessGame, Player};

#[derive(Copy, Clone,Eq, PartialEq,Debug)]
pub struct PiecePosition
{
    pub row: usize,
    pub col: usize,
}

#[derive(Copy, Clone)]
pub struct PlayerMove
{
    pub owner:Player,
    pub from:PiecePosition,
    pub to:PiecePosition
}

pub struct PieceContext<'a,'b>
{
    pub piece:&'b ChessPiece,
    pub game:&'a ChessGame
}

impl PiecePosition
{
    pub fn new_from_cord(row: usize, col: usize) -> Result<Self,ChessError>
    {
        if  row > 8 || col > 8
        {
            return Err(ChessError::InvalidPiecePosition);
        }

        Ok(Self
        {
            row,
            col
        })
    }

    pub fn add(&self,row_dif:i32,col_dif:i32) -> Result<PiecePosition,ChessError>
    {
        let new_row = self.row as i32 + row_dif;
        let new_col = self.col as i32 +col_dif;
        if new_row < 0 ||new_col < 0
        {
            return Err(ChessError::InvalidPiecePosition)
        }

        let new_piece = Self::new_from_cord( new_row as usize,new_col as usize )?;
        Ok(new_piece)
    }
}

impl PlayerMove
{
    pub fn new(owner:Player,from:PiecePosition,to:PiecePosition) -> Self
    {
        Self
        {
            owner,
            from,
            to
        }
    }
}