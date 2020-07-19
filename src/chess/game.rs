use crate::chess::piece::{ChessPiece, PieceType};

pub type ChessBoard = Vec<Vec<ChessPiece>>;

#[derive(Debug)]
pub struct ChessGame
{
    pub board: ChessBoard
}

impl ChessGame
{
    fn construct_base_row(is_white: bool) -> Vec<ChessPiece>
    {
        let piece_order = vec![PieceType::Rook, PieceType::Bishop, PieceType::Knight, PieceType::King, PieceType::Queen,PieceType::Knight, PieceType::Bishop, PieceType::Rook];
        let mut row: Vec<ChessPiece> = vec![];
        for ii in 0..8
        {
            let target_type = piece_order[if is_white { 7 - ii } else { ii } ].clone();
            row.push(ChessPiece {
                is_white,
                piece_type: target_type,
            });
        }

        row
    }

    fn construct_starter_board() -> ChessBoard
    {
        let mut board: ChessBoard = vec![];
        for row in 0..8
        {
            if row == 0 || row == 7
            {
                board.push(Self::construct_base_row(row == 7));
            } else if row == 1 || row == 6
            {
                board.push((0..8).map(|_ii| ChessPiece{is_white:row == 6,piece_type:PieceType::Pawn}).collect());
            }
            else {
                board.push((0..8).map(|_ii| ChessPiece { is_white: true, piece_type: PieceType::Empty }).collect());
            }
        }

        board
    }

    pub fn new() -> Self
    {
        Self
        {
            board: Self::construct_starter_board()
        }
    }

    pub fn display_on_console(&self)
    {
        for row in 0..8
        {
            print!("|");
            for col in 0..8
            {
                print!("{}|", self.board[row][col].get_unicode_representation());
            }
            println!();
        }
    }
}
