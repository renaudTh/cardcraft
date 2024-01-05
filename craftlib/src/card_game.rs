pub struct ActionResult {
    pub state_changed: bool,
    pub need_iterate: bool
}

pub trait CardGame {
    fn play_card(&mut self) -> ActionResult;
    fn iterate(&mut self) -> ActionResult;
    fn ended(&self) -> bool;
    fn won(&self) -> bool;
    fn reinitialize(&mut self);
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