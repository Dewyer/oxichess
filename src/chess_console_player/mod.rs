use crate::chess::game::ChessGame;
use crate::chess::piece::{PieceType, ChessPiece};

pub const CHESS_PIECES_BASE_UNICODE_VALUE:u32 = 9812;

pub struct ConsoleChessGame
{
    chess_game:ChessGame
}

impl ConsoleChessGame
{
    pub fn new() -> Self
    {
        Self
        {
            chess_game:ChessGame::new()
        }
    }

    pub fn get_unicode_representation_of_piece(piece:&ChessPiece) -> String
    {
        if piece.piece_type == PieceType::Empty
        {
            return "▯".to_string();
        }
        let piece_order = vec![PieceType::King, PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight, PieceType::Pawn];
        let index: u32 = piece_order.iter().position(|el| std::mem::discriminant(el) == std::mem::discriminant(&piece.piece_type)).unwrap() as u32;
        let code_point = CHESS_PIECES_BASE_UNICODE_VALUE + index + (if piece.owner.is_white() { 0 } else { 6 });
        std::char::from_u32(code_point).unwrap().to_string()
    }

    pub fn display_on_console(&self)
    {
        for row in 0..8
        {
            print!("|");
            for col in 0..8
            {
                let at_piece =  &self.chess_game.board[row][col];
                print!("{}|",Self::get_unicode_representation_of_piece(at_piece));
            }
            println!();
        }
    }

}