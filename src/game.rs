use std::time::Instant;

use crate::card::*;
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    queue,
    style::{Print as P, PrintStyledContent as PS, Stylize},
    terminal::{Clear, ClearType},
};

/// Represents the state of a set Game
pub struct Game {
    pub ended: Option<Instant>,
    selection: Vec<usize>,
    pile: Vec<Card>,
    shown: Vec<Card>,
}

impl Game {
    pub fn new(card_id: impl Iterator<Item = u8>) -> Self {
        let mut list = card_id
            .map(|num| Card::from_tile_number(num).unwrap())
            .collect::<Vec<Card>>();
        let mut game = Game {
            pile: list.split_off(12),
            shown: list,
            selection: Vec::new(),
            ended: None,
        };
        game.draw_as_needed();
        return game;
    }

    /// Repeatedly draw cards until `Self::check_need_more()` returns false.
    fn draw_as_needed(&mut self) {
        let shown_count = self.shown.len();
        if shown_count < 12 {
            self.pile
                .drain(0..((12 - shown_count).min(self.pile.len())))
                .for_each(|card| self.shown.push(card));
        }
        while !Game::has_set(self.shown.iter()) {
            if self.pile.len() == 0 {
                self.ended = Some(std::time::Instant::now());
                break;
            } else {
                self.pile.drain(0..3).for_each(|card| self.shown.push(card));
            }
        }
    }

    /// Checks if the provided list of cards is a valid shown pile (at least 12 cards, 1 set)
    fn has_set<'a>(cards: impl IntoIterator<Item = &'a Card>) -> bool {
        let cards = cards.into_iter().collect::<Vec<_>>();
        let len = cards.len();
        if len == 0 {
            return false;
        }
        for i in 0..len - 1 {
            for j in (i + 1)..len {
                for k in (0..i).chain(j..len) {
                    if cards[i].third(&cards[j]) == *cards[k] {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

impl Game {
    /// Toggles the select state for card index.
    /// Returns Err(()) if index out of bounds; otherwise, returns Ok of a tuple, containing a reference to modified card and a boolean, which is true if the card is now selected.
    pub fn select_card(&mut self, index: usize) -> Result<(&Card, bool), ()> {
        if index >= self.shown.len() {
            Err(())
        } else if let Some(index_index) = self.selection.iter().position(|x| *x == index) {
            Ok((&self.shown[self.selection.swap_remove(index_index)], false))
        } else {
            self.selection.push(index);
            Ok((&self.shown[index], true))
        }
    }

    pub fn pop_last(&mut self) -> Option<&Card> {
        self.selection.pop().map(|index| &self.shown[index])
    }

    /// Checks if selected cards form a set, and replaces cards where needed.
    /// Returns None if not enough cards are selected; returns Some(Ok()) containing the selected cards if they are a set, or Some(None()) containing an immutable reference to the selected cards that are not a set.
    pub fn check_selected_set(
        &mut self,
    ) -> Option<Result<(Card, Card, Card), (&Card, &Card, &Card)>> {
        if self.selection.len() < 3 {
            return None;
        }
        self.selection.sort();
        let index1 = self.selection[0];
        let index2 = self.selection[1];
        let index3 = self.selection[2];
        let ret: Result<(Card, Card, Card), (&Card, &Card, &Card)>;
        if self.shown[index1].third(&self.shown[index2]) == self.shown[index3] {
            let post_removal_cards = (0..self.shown.len())
                .filter(|x| self.selection.contains(x))
                .map(|x| &self.shown[x])
                .collect::<Vec<_>>();
            let has_set = Game::has_set(post_removal_cards);
            let shown_len = self.shown.len();
            let pile_len = self.pile.len();
            if (has_set && shown_len > 12) || pile_len < 3 {
                ret = Ok((
                    self.shown.swap_remove(index3),
                    self.shown.swap_remove(index2),
                    self.shown.swap_remove(index1),
                ));
                if pile_len < 3 {
                    self.draw_as_needed();
                }
            } else {
                let mut drawn_cards = self.pile.split_off(self.pile.len() - 3);
                ret = Ok((
                    std::mem::replace(&mut self.shown[index3], drawn_cards.pop().unwrap()),
                    std::mem::replace(&mut self.shown[index2], drawn_cards.pop().unwrap()),
                    std::mem::replace(&mut self.shown[index1], drawn_cards.pop().unwrap()),
                ));
                self.draw_as_needed();
            }
            self.selection.clear();
        } else {
            ret = Err((
                &self.shown[index3],
                &self.shown[index2],
                &self.shown[index1],
            ));
            self.selection.clear();
        }
        Some(ret)
    }

    pub fn print(&self, out: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let table_width = self.shown.len() / 3;
        queue!(out, MoveTo(0, 1))?;
        for row in 0usize..3 {
            queue!(out, Clear(ClearType::CurrentLine))?;
            for column in 0usize..table_width {
                queue!(
                    out,
                    self.shown[row + column * 3]
                        .stylise(self.selection.contains(&(row + column * 3))),
                )?;
                if table_width - column != 1 {
                    queue!(out, P("  |  "))?;
                }
            }
            queue!(out, MoveToNextLine(1))?;
        }
        queue!(
            out,
            PS(self.pile.len().to_string().bold()),
            P(" cards not yet shown")
        )
    }
}
