pub mod chess;
pub mod chess_console_player;

fn main() {
    let game = chess_console_player::ConsoleChessGame::new();

    game.display_on_console();
}
