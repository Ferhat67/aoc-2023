use std::{cmp::Ordering, str::Chars};

use itertools::Itertools;

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
    joker_rule_enabled: bool
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_strength() == other.get_strength() {
            // if both hands have same strength use strength of cards as tiebreaker
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].cmp(&other.cards[i]);
                }
            }
        }
        Ord::cmp(&self.get_strength(), &other.get_strength())
    }
}

impl Hand {

    pub fn new(labels: Chars, bid: usize) -> Self {
        Self {
            cards: labels.map(|label| Card::new(label)).collect(),
            bid,
            joker_rule_enabled: false
        }
    }

    pub fn get_bid(&self) -> usize {
        self.bid
    }

    pub fn get_strength(&self) -> usize {
        if self.joker_rule_enabled {
            return self.get_max_type()
        }
        self.get_type()
    }

    fn get_grouped_card_labels(&self) -> Vec<(char, usize)> {
        self.cards.iter()
            .sorted()
            .group_by(|&card| card.get_label())
            .into_iter()
            .map(|(label, group)| (label, group.count()))
            .collect::<Vec<_>>()
    }

    fn get_type(&self) -> usize {
        let label_groups = self.get_grouped_card_labels();

        // five of a kind: all cards are the same / there is only one group of labels
        if self.cards.iter().all_equal() || label_groups.len() == 1 {
            return 7;
        }
        // four of kind: there are two groups, one has size of 4
        if label_groups.len() == 2 && label_groups.iter().any(|&(_card_label, group_size)| group_size == 4) {
            return 6; 
        }
        // full house: there are two groups, one of size 3, the other of size 2
        if label_groups.len() == 2 && label_groups.iter().any(|&(_card_label, group_size)| group_size == 3) && label_groups.iter().any(|&(_card_label, group_size)| group_size == 2) {
            return 5; 
        }
        // three of a kind: three groups, one of size 3, others are of size 1 each (otherwise full house)
        if label_groups.len() == 3 && label_groups.iter().any(|&(_card_label, group_size)| group_size == 3) {
            return 4; 
        }
        // two pair: three groups, two of them have a size of 2
        if label_groups.len() == 3 && label_groups.iter().filter(|&&(_card_label, group_size)| group_size == 2).count() == 2 {
            return 3;
        }
        // one pair: four groups, only one has size of 2
        if label_groups.len() == 4 && label_groups.iter().filter(|&&(_card_label, group_size)| group_size == 2).count() == 1 {
            return 2; // one pair
        }
        // high card: 5 cards => 5 groups
        if label_groups.len() == 5 {
            return 1; 
        }
        panic!("Hand does not match any type")
    }

    pub fn enable_joker_rule(&mut self) {
        self.joker_rule_enabled = true;
        self.cards.iter_mut().for_each(|card| card.enable_joker_rule());
    }

    fn get_max_type(&self) -> usize {
        // replace any group of cards with joker cards and determine the max achievable type for this hand
        self.get_grouped_card_labels().iter()
            .map(|(card_label, _group_size)| self.cards.iter().map(|card| card.get_label()).collect::<String>().replace(&card_label.to_string(), "J"))
            .map(|joker_hand_label| Hand::new(joker_hand_label.chars(), self.bid))
            .map(|joker_hand| joker_hand.get_type())
            .max()
            .unwrap()
    }
}