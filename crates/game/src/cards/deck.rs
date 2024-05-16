use crate::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct CardDeck {
    /// The cards that were drawn from the deck.
    pub drawn: HashSet<Card>,
    // TODO add seed
}

impl Iterator for CardDeck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        let rest_cards = Card::ALL_VARIANTS
            .into_iter()
            .filter(|card| !self.drawn.contains(card))
            .collect::<Vec<_>>();

        if rest_cards.is_empty() {
            None
        } else {
            let card_index = rand::random::<usize>() % rest_cards.len();
            let card = rest_cards[card_index];
            self.drawn.insert(card);
            Some(card)
        }
    }
}

#[test]
fn test_random_deck() {
    let mut deck = CardDeck::default();

    let mut drawn_cards = HashSet::new();
    for _ in 0..Card::COUNT {
        let card = deck.next().unwrap();
        assert!(!drawn_cards.contains(&card));
        drawn_cards.insert(card);
    }

    assert_eq!(drawn_cards.len(), Card::COUNT);
    assert_eq!(deck.next(), None);
}
