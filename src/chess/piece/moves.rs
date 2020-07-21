use crate::chess::error::ChessError;
use crate::chess::piece::ChessPiece;
use crate::chess::game::ChessGame;

#[derive(Copy, Clone)]
pub struct PiecePosition
{
    pub row: i32,
    pub col: i32,
}

#[derive(Copy, Clone)]
pub struct PlayerMove
{
    pub owner_is_white:bool,
    pub from:PiecePosition,
    pub to:PiecePosition
}

pub struct PieceContext<'a,'b>
{
    pub piece:&'b ChessPiece,
    pub game:&'a ChessGame,
    pub piece_position:PiecePosition
}

impl PiecePosition
{
    pub fn new_from_cord(row: i32, col: i32) -> Result<Self,ChessError>
    {
        if row < 0 || row > 8 || col < 0 || col > 8
        {
            return Err(ChessError::InvalidPiecePosition);
        }

        Ok(Self
        {
            row,
            col
        })
    }

    pub fn add(&self,row_dif:i32,col_dif:i32) -> Option<PiecePosition>
    {
        let new_piece = Self::new_from_cord(self.row + row_dif,self.col+col_dif).ok()?;
        Some(new_piece)
    }
}

impl PlayerMove
{
    pub fn new(owner:bool,from:PiecePosition,to:PiecePosition) -> Self
    {
        Self
        {
            owner_is_white:owner,
            from,
            to
        }
    }
}