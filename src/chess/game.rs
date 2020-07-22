use crate::chess::piece::{ChessPiece, PieceType};
use crate::chess::board_creator;
use crate::chess::error::ChessError;
use crate::chess::piece::piece_position::{PlayerMove, PiecePosition};

pub type ChessBoard = Vec<Vec<ChessPiece>>;

#[derive(Debug,Clone)]
pub enum CheckState
{
	NoCheck,
	White,
	Black,
}

#[derive(Debug,Clone,Eq, PartialEq)]
pub enum GameState
{
	WhiteWon,
	BlackWon,
	Draw,
	Playing,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Player
{
	Black,
	White,
}

impl Player
{
	pub fn from_bool(is_white: bool) -> Self
	{
		if is_white
		{
			Player::White
		} else {
			Player::Black
		}
	}

	pub fn is_white(&self) -> bool
	{
		match self
		{
			Player::White => true,
			Player::Black => false
		}
	}
}

#[derive(Debug, Clone)]
pub struct ChessGame
{
	pub state: GameState,
	pub on_turn: Player,
	pub board: ChessBoard,
	pub check_state: CheckState,
}

impl ChessGame
{
	pub fn get_piece_at(&self, pos: PiecePosition) -> &ChessPiece
	{
		&self.board[pos.row as usize][pos.col as usize]
	}

	pub fn is_position_occupied(&self, pos: PiecePosition) -> bool
	{
		self.get_piece_at(pos).piece_type != PieceType::Empty
	}

	pub fn new() -> Self
	{
		Self
		{
			state: GameState::Playing,
			board: board_creator::construct_starter_board(),
			check_state: CheckState::NoCheck,
			on_turn: Player::White,
		}
	}

	pub fn is_position_threatened(&self, pos: PiecePosition) -> bool
	{
		for row in 0..8
		{
			for col in 0..8
			{
				let pos = PiecePosition::new_from_cord(row, col).unwrap();
				let pp = self.get_piece_at(pos);
				let threats = pp.get_threatened(&self, pos);
				if threats.iter().any(|el| el == &pos)
				{
					return true;
				}
			}
		}

		false
	}

	pub fn would_position_be_threatened(&self, pos: PiecePosition, player_move: PlayerMove) -> bool
	{
		let mut pseudo_game = self.make_pseudo_board(player_move);
		pseudo_game.is_position_threatened(pos)
	}

	fn make_pseudo_board(&self, player_move: PlayerMove) -> ChessGame
	{
		let mut new_game = self.clone();
		new_game.execute_move(player_move);
		new_game
	}

	fn execute_move(&mut self, player_move: PlayerMove)
	{
		let piece: ChessPiece = self.board[player_move.from.row][player_move.from.col].clone();
		self.board[player_move.from.row][player_move.from.col] = ChessPiece::empty();
		self.board[player_move.to.row][player_move.to.col] = piece;
	}

	pub fn validate_move(&self, player_move: &PlayerMove, moving_piece: &ChessPiece) -> Result<(), ChessError>
	{
		if self.on_turn == player_move.owner && &self.state == &GameState::Playing
		{
			return Err(ChessError::NotRightPlayersMove);
		}

		if moving_piece.owner != self.on_turn
		{
			return Err(ChessError::PieceIsNotYours);
		}

		Ok(())
	}

	pub fn make_move(&mut self, player_move: PlayerMove) -> Result<(), ChessError>
	{
		let moving_piece = self.get_piece_at(player_move.from);

		self.validate_move(&player_move, moving_piece)?;
		let possible_moves = moving_piece.get_moves(&self, player_move.from);
		let is_legal_move = possible_moves.iter().any(|el| el == &player_move.to);
		if !is_legal_move
		{
			return Err(ChessError::InvalidMove)
		}

		Ok(())
	}
}
