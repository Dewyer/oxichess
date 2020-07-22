mod move_helper;
mod pawn;

use crate::chess::piece::piece_position::{PiecePosition, PieceContext, PlayerMove};
use crate::chess::game::{ChessGame, Player};

pub mod piece_position;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug,Clone)]
pub struct ChessPiece
{
	pub owner: Player,
	pub piece_type: PieceType,
}

impl ChessPiece
{
	pub fn new(owner:Player, piece_type: PieceType) -> Self
	{
		Self
		{
			owner,
			piece_type,
		}
	}

	pub fn empty() -> Self
	{
		Self
		{
			owner:Player::White,
			piece_type:PieceType::Empty
		}
	}

	pub fn get_threatened(&self,game:&ChessGame, position: PiecePosition) -> Vec<PiecePosition>
	{
		let ctx = PieceContext { piece: &self, game, piece_position: position };

		match &self.piece_type
		{
			PieceType::Pawn => pawn::get_threatened_by_pawn(ctx),
			_ => vec![]
		}
	}

	pub fn get_possible_moves(&self, game: &ChessGame,position:PiecePosition) -> Vec<PiecePosition>
	{
		let ctx = PieceContext { piece: &self, game, piece_position: position };

		match &self.piece_type
		{
			PieceType::Pawn => pawn::get_possible_moves_by_pawn(ctx),
			_ => vec![]
		}
	}

	pub fn get_moves(&self,game:&ChessGame, position:PiecePosition) -> Vec<PiecePosition>
	{
		let possible_moves = self.get_possible_moves(game,position);
		let mut filtered_moves:Vec<PiecePosition> = Vec::new();
		// TODO: Can't move into check, when in check, this should resolve it
		for p_move in possible_moves
		{
			let pseudo_move = PlayerMove{
				owner:self.owner,
				from:position,
				to: p_move
			};

		}

		filtered_moves
	}
}