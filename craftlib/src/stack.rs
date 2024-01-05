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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty_stack() {
        let stack = Stack::new_empty();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_add_card_on_top() {
        let mut stack = Stack::new_empty();
        let card_added = Card::new(1, 8, true);
        let card_added_2 = Card::new(2, 10, true);

        let card_ref = Card::new(2, 10, true);

        stack.add_card_on(card_added);
        stack.add_card_on(card_added_2);

        assert_eq!(stack.is_empty(), false);
        let first = stack.read_first();
        assert!(first.is_some_and(|card| card.equal(&card_ref)))
    }

    #[test]
    fn test_add_card_under() {
        let mut stack = Stack::new_empty();
        let card_added = Card::new(2, 10, true);
        let card_ref = Card::new(2, 10, true);
        stack.add_card_under(card_added);
        assert_eq!(stack.is_empty(), false);
        assert!(stack.read_last().is_some_and(|card| card.equal(&card_ref)));
    }

    #[test]
    fn test_flip_stack() {
        let mut stack = Stack::new_empty();
        let card1 = Card::new(0, 5, true);
        let card1_ref = Card::new(0, 5, true);
        let card2 = Card::new(1, 9, true);
        let card2_ref = Card::new(1, 9, true);
        stack.add_card_on(card1);
        stack.add_card_on(card2);
        stack.flip();
        assert!(stack.read_first().is_some_and(|card| card.equal(&card1_ref)));
        assert!(stack.read_last().is_some_and(|card| card.equal(&card2_ref)));
    }

    #[test]
    fn test_new_complete_deck() {
        let stack = Stack::new_complete_deck(32, true, true);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack._deck.len(), 32);
    }

    #[test]
    fn test_pop_from_top_to_top() {
        let mut stack1 = Stack::new_complete_deck(32, true, true);
        let mut stack2 = Stack::new_empty();

        stack1.pop_from_top_to_top(&mut stack2);

        assert_eq!(stack1.is_empty(), false);
        assert_eq!(stack2.is_empty(), false);
        assert_eq!(stack1._deck.len(), 31);
        assert_eq!(stack2._deck.len(), 1);
    }

    #[test]
    fn test_pop_from_top_to_bottom() {
        let mut stack1 = Stack::new_complete_deck(32, true, true);
        let mut stack2 = Stack::new_empty();

        stack1.pop_from_top_to_bottom(&mut stack2);

        assert_eq!(stack1.is_empty(), false);
        assert_eq!(stack2.is_empty(), false);
        assert_eq!(stack1._deck.len(), 31);
        assert_eq!(stack2._deck.len(), 1);
    }

    #[test]
    fn test_append_on_bottom() {
        let mut stack1 = Stack::new_complete_deck(32, true, true);
        let mut stack2 = Stack::new_complete_deck(32, false, false);

        stack1.append_on_bottom(&mut stack2);

        assert_eq!(stack1.is_empty(), false);
        assert_eq!(stack2.is_empty(), true);
        assert_eq!(stack1._deck.len(), 64);
    }

    #[test]
    fn test_append_on_top() {
        let mut stack1 = Stack::new_complete_deck(32, true, true);
        let mut stack2 = Stack::new_complete_deck(32, false, false);

        stack1.append_on_top(&mut stack2);

        assert_eq!(stack1.is_empty(), false);
        assert_eq!(stack2.is_empty(), true);
        assert_eq!(stack1._deck.len(), 64);
    }
}