use wasm_bindgen::prelude::wasm_bindgen;

use crate::{card_game::{CardGame, ActionResult}, r7::R7Game};



#[wasm_bindgen]
pub struct GameFactory {
    game: R7Game,
}
#[wasm_bindgen]
impl GameFactory {

   pub fn new() -> GameFactory {
    GameFactory {
        game: R7Game::new()
    }
   } 
   pub  fn init(&mut self) -> Vec<u8> {
        self.game.init_random();
        self.game.get_game_state().get_state()
    }

    pub fn play_move(&mut self) -> Vec<u8> {
        let _action = self.game.play_card();
        self.game.get_game_state().get_state()
    }
}