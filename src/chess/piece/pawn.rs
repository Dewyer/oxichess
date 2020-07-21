use crate::chess::game::ChessGame;
use crate::chess::piece::ChessPiece;
use crate::chess::piece::moves::{PiecePosition, PieceContext};

pub fn get_threatened_by_pawn(ctx:PieceContext) -> Vec<PiecePosition>
{
	let mut possibles:Vec<PiecePosition> = Vec::new();
	let row_dif = if ctx.piece.is_white {-1}else{1};
	for col_off in vec![1,-1]
	{
		if let Some(pos) = ctx.piece_position.add(row_dif,col_off)
		{
			possibles.push(pos);
		}
	}

	possibles
}

pub fn get_possible_moves_by_pawn(ctx:PieceContext) -> Vec<PiecePosition>
{
	let mut possibles:Vec<PiecePosition> = Vec::new();
	let row_dif = if ctx.piece.is_white {-1}else{1};

}