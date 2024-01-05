use std::collections::VecDeque;
use crate::card::Card;

pub struct Stack {
    _deck: VecDeque<Card>
}

impl Stack {
    pub fn new_empty() -> Stack{
        Stack {
            _deck: VecDeque::new()
        }
    }
    pub fn size(&self) -> usize {
        return self._deck.len()
    }
    pub fn is_empty(&self) -> bool {
        self._deck.len() == 0
    }
    pub fn add_card_on(&mut self, card: Card){
        self._deck.push_front(card)
    }
    pub fn add_card_under(&mut self, card: Card){
        self._deck.push_back(card)
    }
    pub fn read_first(&self) -> Option<&Card>{
        self._deck.front()
    }
    pub fn read_last(&self) -> Option<&Card>{
        self._deck.back()
    }
    pub fn flip(&mut self){
      self._deck.make_contiguous().reverse();
      self._deck.iter_mut().for_each(|card| card.flip());
    }
    pub fn shuffle(&mut self){
        
    }
    pub fn new_complete_deck(card_nb: u8, visible: bool, ace_is_max: bool) -> Stack{
        let (min, max) = if card_nb == 32 {
            (5, 13)
        } else if ace_is_max {
            (1, 13)
        } else {
            (0, 12)
        };
        let mut ret = Stack::new_empty();
        for family in 0u8..=3u8{
            for value in min..max {
                ret._deck.push_back(Card::new(family, value, visible))
            }
        }
        ret

    }
    pub fn pop_from_top_to_top(&mut self, other:&mut  Stack){
        match self._deck.pop_front() {
            Some(card) => other.add_card_on(card),
            None => {}
        }
    }
    pub fn pop_from_top_to_bottom(&mut self, other: &mut Stack){
        match self._deck.pop_front() {
            Some(card) => other.add_card_under(card),
            None => {}
        }
    }
    pub fn append_on_bottom(&mut self, other: &mut Stack){
        self._deck.append(&mut other._deck)
    }
    pub fn append_on_top(&mut self, other: &mut Stack){
        while !other.is_empty() {
            other.pop_from_top_to_top(self); 
        }
    }
    pub fn pop_front(&mut self) -> Option<Card>{
        self._deck.pop_front() 
    }
    pub fn pop_back(&mut self) -> Option<Card>{
        self._deck.pop_back() 
    }
}

