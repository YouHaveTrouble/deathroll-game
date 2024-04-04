use eframe::*;
use egui::CentralPanel;
use rand::Rng;

struct DeathrollGame {
    current_roll: i64,
    game_state: GameState,
    game_history: Vec<String>,
    current_roll_input: String,
}

enum GameState {
    GameOver,
    StartMenu,
    GameInProgress,
}

impl App for DeathrollGame {
    fn update(&mut self, context: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(context, |ui| {
            match self.game_state {
                GameState::GameOver => {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Game Over");
                        ui.label("Click the button to restart the game.");
                        if ui.button("Restart Game").clicked() {
                            self.game_state = GameState::StartMenu;
                        }
                        ui.label("Game History:");
                        for history in self.game_history.iter() {
                            ui.label(history);
                        }
                    });
                }
                GameState::StartMenu => {
                    self.game_history.clear();
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Deathroll Game");
                        ui.label("Enter starting value and click the button to start the game.");
                        ui.text_edit_singleline(&mut self.current_roll_input);
                        self.current_roll = self.current_roll_input.parse::<i64>().unwrap_or_else(|_e| { -1 });
                        if self.current_roll <= 0 {
                            ui.label("Invalid input. Please enter a valid number.");
                            return;
                        }
                        let start_button = ui.button("Start Game");
                        if start_button.clicked() {
                            if self.current_roll <= 0 {
                                return;
                            }
                            self.game_history.push(format!("Game started with a roll of {}", self.current_roll));
                            self.game_state = GameState::GameInProgress;
                        }
                    });
                }
                GameState::GameInProgress => {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Deathroll Game");
                        ui.label("Click the button to roll the dice.");

                        if ui.button("Roll").clicked() {
                            self.current_roll = generate_roll(self.current_roll);
                            self.game_history.push(format!("You rolled {}", self.current_roll));
                            if self.current_roll == 1 {
                                self.game_state = GameState::GameOver;
                                self.game_history.push("You lost the game!".to_string());
                                return;
                            }
                            self.current_roll = generate_roll(self.current_roll);
                            self.game_history.push(format!("Opponent rolled {}", self.current_roll));
                            if self.current_roll == 1 {
                                self.game_state = GameState::GameOver;
                                self.game_history.push("You won the game!".to_string());
                                return;
                            }
                        }
                        ui.label("Game History:");
                        for history in self.game_history.iter() {
                            ui.label(history);
                        }
                    });
                }
            }
        });
    }
}

fn main() -> Result<(), Error> {
    return run_native(
        "Deathroll Game",
        NativeOptions::default(),
        Box::new(|_creation_context: &CreationContext<'_>| {
            Box::new(DeathrollGame {
                current_roll: 0,
                game_state: GameState::StartMenu,
                game_history: Vec::new(),
                current_roll_input: String::new(),
            })
        })
    );
}

/**
 * Generates a random number between 1 and the upper limit.
 */
fn generate_roll(upper_limit: i64) -> i64 {
    if upper_limit <= 1 { return 1; }
    return rand::thread_rng().gen_range(1..upper_limit);
}
