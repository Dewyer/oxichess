use crate::chess::piece::{PieceType, ChessPiece};

pub struct ChessPieceMovementHelper
{
	pub change_vectors:Vec<(i32,i32)>,
	pub repeating:bool
}

impl ChessPieceMovementHelper
{
	pub fn new_from_piece(piece:&ChessPiece) -> Self
	{
		match piece.piece_type
		{
			PieceType::Pawn => Self
			{
				repeating:false,
				change_vectors:vec![(if piece.is_white {-1} else {1},0)]
			},
			_=> Self{
				repeating:false,
				change_vectors:vec![]
			}
		}
	}

	pub fn generate_targets()
	{

	}
}