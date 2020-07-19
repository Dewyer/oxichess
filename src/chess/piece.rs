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
    pub is_white: bool,
    pub piece_type: PieceType,
}

impl ChessPiece
{
    pub fn get_unicode_representation(&self) -> String
    {
        if self.piece_type == PieceType::Empty
        {
            return "▯".to_string();
        }
        let piece_order = vec![PieceType::King, PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight, PieceType::Pawn];
        let index: u32 = piece_order.iter().position(|el| std::mem::discriminant(el) == std::mem::discriminant(&self.piece_type)).unwrap() as u32;
        let code_point = 9812 + index + (if self.is_white { 0 } else { 6 });
        std::char::from_u32(code_point).unwrap().to_string()
    }
}
