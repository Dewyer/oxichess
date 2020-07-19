pub mod chess;

fn main() {
    let game = chess::ChessGame::new();
    game.display_on_console();
}
