pub struct Card {
    card: u8
}

impl Card {
    pub fn new(family: u8, value: u8, is_visible: bool) -> Card {
        let mut v: u8 = 0;
        v |= family;
	    v |= value << 2;
	    v |= (is_visible as u8) << 7;
        return Card {
           card:  v
        }
    }
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        (self.card & 0x80) >> 7 == 1
    }
    #[inline(always)]
    pub fn value(&self) -> u8{
        (self.card & 0x7C) >> 2
    }
    #[inline(always)]
    pub fn family(&self) -> u8{
       self.card & 0x3
    }
    #[inline(always)]
    pub fn flip(&mut self){
        self.card ^= 1 << 7;
    }
    pub fn to_string(&self) -> String{
        if !self.is_visible() {return "#".to_string();}
        let value = self.value();
         match self.family() {
            0 => format!("{}C", value),
            1 =>format!("{}D", value),
            2 => format!("{}H", value),
            3 => format!("{}S", value),
            _ => "".to_string()
        }
    }
    pub fn is_inf(&self, other: &Card) -> bool{
        self.value() < other.value()
    }
    pub fn value_equal(&self, other: &Card) -> bool {
        self.value() == other.value()
    }
    pub fn equal(&self, other: &Card) -> bool {
        self.value() == other.value() && self.family() == other.family()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_visible_card() {
        let card = Card::new(2, 5, true);
        assert_eq!(card.family(), 2);
        assert_eq!(card.value(), 5);
        assert_eq!(card.is_visible(), true);
    }
    #[test]
    fn test_new_hidden_card() {
        let card = Card::new(2, 5, false);
        assert_eq!(card.family(), 2);
        assert_eq!(card.value(), 5);
        assert_eq!(card.is_visible(), false);
    }
    #[test]
    fn test_flip() {
        let mut card = Card::new(3, 6, true);
        assert_eq!(card.is_visible(), true);
        card.flip();
        assert_eq!(card.is_visible(), false);
        card.flip();
        assert_eq!(card.is_visible(), true);
    }

    #[test]
    fn test_to_string_visible() {
        let card = Card::new(0, 8, true);
        assert_eq!(card.to_string(), "8C".to_string());
    }

    #[test]
    fn test_to_string_hidden() {
        let card = Card::new(3, 11, false);
        assert_eq!(card.to_string(), "#".to_string());
    }

    #[test]
    fn test_is_inf() {
        let card1 = Card::new(1, 7, true);
        let card2 = Card::new(0, 9, true);
        assert!(card1.is_inf(&card2));
        assert!(!card2.is_inf(&card1));
    }

    #[test]
    fn test_equal() {
        let card1 = Card::new(2, 4, true);
        let card2 = Card::new(2, 4, true);
        let card3 = Card::new(1, 4, true);
        assert!(card1.equal(&card2));
        assert!(card2.value_equal(&card3));
    }
}
