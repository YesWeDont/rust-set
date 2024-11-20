use crate::card::Card;

pub static KEYBINDS: &str =
    "  q w e r t y u
\r  a s d f g h j
\r  z x c v b n m
\r  Q to quit anytime";
pub fn tutorial() -> Result<(), std::io::Error> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::queue!(std::io::stdout(), crossterm::style::Print(format!("
Set! is a concentration game invented by Marsha Falco in 1974. \x1b[2mrust-set\x1b[0m is a single-player implementation of Set written in Rust.

\x1b[1mRules\x1b[0m
In the game, \x1b[2mcards\x1b[0m with patterns are shown to the player and the player aims to find \x1b[2msets\x1b[0m within the given cards. Such cards have four \x1b[2mattributes\x1b[0m: shape, number, colour and decoration. Three cards form a set if, for every single attribute of the above four, that attribute is the same in each card, or that attribute is all different for each card.\x1b[0m After three such cards are found by the player, they are removed into the discard pile, and three new cards are drawn repeatedly from the unused pile to replace the three removed cards such that there are again exactly 12 cards shown. If there is no set within the 12 cards, extra cards drawn in threes until is a set; however, they will not be replenished after the player finds a set unless required by the previous criterion. If the unused pile is depleted, the game ends after the player finds all remaining sets on screen.

\x1b[1mImplementation\x1b[0m
Virtual cards will be laid out on the screen in a n-by-3 table, where n is a number from 1 to 7. (There is guaranteed to be a set within 21 cards). Players must use the following keybinds to select cards:
{KEYBINDS}
That is, q selects the 1st item on the 1st row, and d the third item on the second row. Progress will not be saved if Q is used to exit.
Altertatively, if supported by the terminal, mouse clicks can also be used to select cards.

Shape, number, colour and shading (replaced with decoration) take the following values:
- Shape: > (diamond), O (pill) or S (squiggly)
- Number: > (one), >> (two), >>> (three)
- Colour: \x1b[31mred\x1b[0m, \x1b[32mgreen\x1b[0m, \x1b[35mpurple\x1b[0m
- Decoration: no underline, \x1b[4m1 underline\x1b[0m or \x1b[21m2 underlines\x1b[0m

\x1b[1mExamples of sets\x1b[0m
Go figure out why the following cards are sets:\n")))?;
let cards = rand::seq::IteratorRandom::choose_multiple(0u8..81, &mut rand::rngs::OsRng, 6).into_iter().map(|x| Card::from_tile_number(x).unwrap()).collect::<Vec<_>>();

let sep = crossterm::style::Print(", ");
let newline = crossterm::style::Print("; ");

crossterm::queue!(
    std::io::stdout(),
    cards[0].stylise(false),
    sep,
    cards[1].stylise(false),
    sep,
    cards[0].third(&cards[1]).stylise(false),
    newline,
    cards[2].stylise(false),
    sep,
    cards[3].stylise(false),
    sep,
    cards[2].third(&cards[3]).stylise(false),
    newline,
    cards[4].stylise(false),
    sep,
    cards[5].stylise(false),
    sep,
    cards[4].third(&cards[5]).stylise(false),
    crossterm::style::Print("\n\nHave fun!\n")
)?;
crossterm::terminal::enable_raw_mode()
}
