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

impl CardGame for R7Game {

    fn play_card(&mut self) -> ActionResult {
    
        let mut card_to_play: Card = match self.deck.pop_front() {
            Some(card) => card,
            None => return ActionResult {
                state_changed: false,
                need_iterate: false,
            },
        };
        card_to_play.flip();
        let index = card_to_play.family() as usize;
        //We can unwrap since build stacks are never empty
        let top_card = self.build[index].read_first().unwrap();
        let bottom_card = self.build[index].read_last().unwrap();
        if card_to_play.value() == top_card.value() + 1 {
            self.build[index].add_card_on(card_to_play);
        }
        else if card_to_play.value() == bottom_card.value() - 1 {
            self.build[index].add_card_under(card_to_play);
        } 
        else {
          self.bin.add_card_on(card_to_play);
        }
        return ActionResult {
            need_iterate: true,
            state_changed: true,
        }; 
    }

    fn iterate(&mut self) -> ActionResult {
        let mut state_changed = false;
        let need_iterate = false;
        self.nb_attempt+=1;
        if self.bin.is_empty() {
            return ActionResult {
                state_changed,
                need_iterate,
            }
        }
        state_changed = true;
        self.bin.flip();
        self.deck.append_on_bottom(&mut self.bin);
        return ActionResult {
            state_changed,
            need_iterate,
        }
    }

    fn ended(&self) -> bool {
        self.won() || self.nb_attempt >= 3
    }

    fn won(&self) -> bool {
        self.nb_attempt <= 3 && self.deck.size() == 0
    }

    fn reinitialize(&mut self) {
        todo!()
    }
}
