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
