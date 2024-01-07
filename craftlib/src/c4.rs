use crate::{stack::Stack, card_game::{CardGame, ActionResult}};

pub struct C4Game {
    build: Vec<Stack>,
    deck: Stack,
    last_build_played: i8,
    is_ended: bool,
    is_won: bool,   
}

impl C4Game {
    pub fn new() -> C4Game {
        let deck = Stack::new_complete_deck(32, false, true);
        let mut build: Vec<Stack> = Vec::new();
        for _ in 0..4u8 {
            build.push(Stack::new_empty());
        }
        let is_ended = false;
        let is_won = false;
        let last_build_played = -1;
        C4Game {
            build,
            deck,
            last_build_played,
            is_ended,
            is_won
        }
    }
    pub fn initialize(&mut self){
        //Shuffle deck!
        let mut i = 0;
        while self.deck.size() > 0 {
            let card = self.deck.pop_front().unwrap();
            self.build[i % 4].add_card_on(card);
            i+=1;
        }
    }
}

impl CardGame for C4Game {
    fn play_card(&mut self) -> crate::card_game::ActionResult {
        let need_iterate: bool = true;
        let state_changed: bool = true;
        if self.last_build_played == -1 {
            self.last_build_played = 0;
            //TODO: Select a random build
        }
        
        let mut card = self.build[self.last_build_played as usize].pop_front().unwrap();
        card.flip();
        let family = card.family();
        self.build[family as usize].add_card_under(card);
        self.last_build_played = family as i8;
    
        return ActionResult{
            need_iterate,
            state_changed
        }

    }

    fn iterate(&mut self) -> crate::card_game::ActionResult {
        todo!()
    }

    fn ended(&self) -> bool {
        todo!()
    }

    fn won(&self) -> bool {
        todo!()
    }

    fn reinitialize(&mut self) {
        todo!()
    }

    fn get_game_state(&self) -> crate::card_game::GameState {
        todo!()
    }
}