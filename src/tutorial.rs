static N: &str = "\x1b[0G";

pub static KEYBINDS: &str =
    "  q w e r t y u\x1b[0G\n  a s d f g h j\x1b[0G\n  z x c v b n m\x1b[0G\n  Q to quit anytime";
pub fn tutorial() -> Result<(), std::io::Error> {
    crossterm::queue!(std::io::stdout(), crossterm::style::Print(format!("
Set! is a concentration game invented by Marsha Falco in 1974. \x1b[2mrust-set\x1b[ is a single-player implementation of Set written in Rust.{N}

\x1b[1mRules\x1b[0m{N}
In the game, \x1b[2mcards\x1b[0m with patterns are shown to the player and the player aims to find \x1b[2msets\x1b[0m within the given cards. Such cards have four \x1b[2mattributes\x1b[0m: shape, number, colour and decoration. Three cards form a set if, for every single attribute of the above four, that attribute is the same in each card, or that attribute is all different for each card.\x1b[0m After three such cards are found by the player, they are removed into the discard pile, and three new cards are drawn repeatedly from the unused pile to replace the three removed cards such that there are again exactly 12 cards shown. If there is no set within the 12 cards, extra cards drawn in threes until is a set; however, they will not be replenished after the player finds a set unless required by the previous criterion. If the unused pile is depleted, the game ends after the player finds all remaining sets on screen.{N}

\x1b[1mImplementation\x1b[0m{N}
Virtual cards will be laid out on the screen in a n-by-3 table, where n is a number from 1 to 7. (There is guaranteed to be a set within 21 cards). Players must use the following keybinds to select cards:{N}
{KEYBINDS}{N}
That is, q selects the 1st item on the 1st row, and d the third item on the second row. Progress will not be saved if Q is used to exit.{N}
Altertatively, if supported by the terminal, mouse clicks can also be used to select cards.{N}
Shape, number, colour and shading (replaced with decoration) take the following values:
- Shape: > (diamond), O (pill) or S (squiggly)
- Number: > (one), >> (two), >>> (three)
- Colour: \x1b[31mred\x1b[0m, \x1b[32mgreen\x1b[0m, \x1b[35mpurple\x1b[0m
- Decoration: no underline, \x1b[4m1 underline\x1b[0m or \x1b[21m2 underlines\x1b[0m


Have fun!{N}

")))
}
