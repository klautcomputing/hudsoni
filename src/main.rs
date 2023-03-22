use hudsoni_lib::game_error::GameError;
use hudsoni_lib::game_result::GameResult;
use hudsoni_lib::game_status::GameStatus;
use hudsoni_lib::game_type::GameType;
use hudsoni_lib::history::History;
use hudsoni_lib::state::State;
use std::env;

fn play_game_from_file(file_path: &str) -> Result<(), GameError> {
    let history = History::from_filepath(file_path)?;
    let mut state: State = State::new(GameType::default(), false);
    for _ in 0..100 {
        state = State::new_from_history(&history)?;
    }
    if let GameStatus::Finished(GameResult::Winner(winner)) = state.game_status {
        println!("State says {} won!", winner);
    }
    if let GameStatus::Finished(GameResult::Draw) = state.game_status {
        println!("State says it's a draw");
    }
    if let GameResult::Winner(winner) = history.result {
        println!("History says {} won!", winner);
    }
    if let GameResult::Winner(hw) = history.result {
        if let GameStatus::Finished(GameResult::Winner(sw)) = state.game_status {
            if sw != hw {
                return Err(GameError::ResultMismatch {
                    reported_result: history.result,
                    actual_result: GameResult::Winner(sw),
                });
            }
        }
        if let GameStatus::Finished(GameResult::Draw) = state.game_status {
            return Err(GameError::ResultMismatch {
                reported_result: history.result,
                actual_result: GameResult::Draw,
            });
        }
    }
    if let GameResult::Draw = history.result {
        println!("History says game ended in a draw");
        if let GameStatus::Finished(GameResult::Winner(sw)) = state.game_status {
            return Err(GameError::ResultMismatch {
                reported_result: history.result,
                actual_result: GameResult::Winner(sw),
            });
        }
    }
    Ok(())
}

fn main() {
    let game: Vec<String> = env::args().collect();
    if let Some(game) = game.get(1) {
        println!("{game}");
        match play_game_from_file(game) {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        }
    } else {
        eprint!("{}", GameError::NoPgnFile);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_play_games_from_valid_files() {
        for entry in fs::read_dir("./test_pgns/valid/").expect("Should be valid directory") {
            let entry = entry.expect("PGN").path().display().to_string();
            println!("{}", entry);
            assert!(play_game_from_file(&entry).is_ok());
        }
    }
    #[test]
    fn test_play_games_from_invalid_files() {
        for entry in fs::read_dir("./test_pgns/invalid/").expect("Should be valid directory") {
            let entry = entry.expect("PGN").path().display().to_string();
            println!("{}", entry);
            assert!(play_game_from_file(&entry).is_err());
        }
    }
}
