use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, Hash)]
pub struct Card {
    label: char,
    joker_rule_enabled: bool
}

impl Card {

    pub fn new(label: char) -> Self {
        Self { label, joker_rule_enabled: false }
    }

    pub fn get_label(&self) -> char {
        self.label
    }

    pub fn get_strength(&self) -> usize {
        match self.label {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => if self.joker_rule_enabled { 0 } else { 10 },
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!()
        }
    }

    pub fn enable_joker_rule(&mut self) {
        self.joker_rule_enabled = true;
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.get_strength(), &other.get_strength())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.get_strength() == other.get_strength()
    }
}