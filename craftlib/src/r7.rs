use crate::{stack::Stack, card_game::{CardGame, ActionResult}, card::Card};

pub struct R7Game {
    build: Vec<Stack>,
    deck: Stack,
    bin: Stack,
    nb_attempt: i32
}
impl R7Game {
    pub fn new() -> R7Game{
        let deck = Stack::new_empty();
        let bin = Stack::new_empty();
        let mut build = Vec::<Stack>::new();
        for _i in 0..4{
            build.push(Stack::new_empty());
        }
        R7Game {
            build,
            deck,
            bin,
            nb_attempt: 0
        }
    }
    fn init_winning_game_in_one_attempt(&mut self) {
        for f in 0usize..4usize{
            let c = Card::new(f as u8, 7, true);
            self.build[f].add_card_on(c);
            for v in 8u8..=14u8 {
                let c = Card::new(f as u8, v, false);
                self.deck.add_card_under(c);
            }
            for v in (2..=6).rev() {
                let c = Card::new(f as u8, v, false);
                self.deck.add_card_under(c);
            }
        }
    }
    fn init_empty(&mut self){
        for f in 0u8..4u8 {
            let c = Card::new(f, 7, true);
            self.build[f as usize].add_card_on(c);
        }
    }
}
