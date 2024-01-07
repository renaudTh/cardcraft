use crate::stack::Stack;

pub struct ActionResult {
    pub state_changed: bool,
    pub need_iterate: bool
}
pub struct GameState {
    state: Vec<u8>,
}
impl GameState {

    pub fn new() -> GameState {
        GameState {
            state: Vec::new(),
        }
    }
    pub fn set_header(&mut self, nb_rows: u8, nb_columns: u8, nb_stacks: u8, is_ended: bool, is_win: bool){
        self.state.push(nb_rows);
        self.state.push(nb_columns);
        self.state.push(nb_stacks);
        self.state.push(is_ended as u8);
        self.state.push(is_win as u8);
    }
    pub fn add_stack(&mut self, x: u8, y: u8, spread: bool, stack: &Stack){
        self.state.push(x);
        self.state.push(y);
        self.state.push(spread as u8);
        self.state.push(stack.size() as u8);
        let mut raw = stack.raw();
        self.state.append(&mut raw);
    }

    pub fn get_state(&self) ->  Vec<u8> {
        self.state.clone() 
    }


}


pub trait CardGame {
    fn play_card(&mut self) -> ActionResult;
    fn iterate(&mut self) -> ActionResult;
    fn ended(&self) -> bool;
    fn won(&self) -> bool;
    fn reinitialize(&mut self);
    fn get_game_state(&self) -> GameState;
}

pub fn play_card_game<T: CardGame>(game: &mut T) -> bool {
    let mut result: ActionResult;
    while !game.ended() {
        result = game.play_card();
        while result.need_iterate {
            result = game.play_card();
        }
        result = game.iterate();
        while result.need_iterate {
            result = game.iterate();
        }
    }
    return game.won();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut state = GameState::new();
        let mut stack1 = Stack::new_complete_deck(32, true, true);
        state.set_header(3, 6, 1, false, false);
        state.add_stack(12, 15, false, &stack1);
        let raw = state.get_state();
        println!("{:?}", raw);
    }
}