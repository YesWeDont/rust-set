## rust-set 
Set! is a concentration game invented by Marsha Falco in 1974. `rust-set` is a single-player implementation of Set written in Rust.

## Potential todos
- [] Find some way to link tutorial page to this document so I can avoid manually mirroring changes across
- [] Testing on Mac OS/other terminals
- [] CLI options (game difficulty options such as modifying card count, allowing 
hints; seeding the RNG)
- [] Saving and sharing game replays (current hack: use tee to save to log-file)
- [] Variations of rules e.g. Set-Chain, ultraset, n-set
- [] Some sort of multiplayer (very annoying, probably not in the near future)

## Rules
In the game, *cards* with patterns are shown to the player and the player aims to find *sets* within the given cards. Such cards have four *attributes*: shape, number, colour and decoration. Three cards form a set if, for every single attribute of the above four, that attribute is the same in each card, or that attribute is all different for each card. After three such cards are found by the player, they are removed into the discard pile, and three new cards are drawn repeatedly from the unused pile to replace the three removed cards such that there are again exactly 12 cards shown. If there is no set within the 12 cards, extra cards drawn in threes until is a set; however, they will not be replenished after the player finds a set unless required by the previous criterion. If the unused pile is depleted, the game ends after the player finds all remaining sets on screen.

## Implementation
Virtual cards will be laid out on the screen in a n-by-3 table, where n is a number from 1 to 7. (There is guaranteed to be a set within 21 cards). Players must use the following keybinds to select cards on various rows and columns:
| Row\\Column | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
| ----------- | - | - | - | - | - | - | - |
|      0      | q | w | e | r | t | y | u |
|      1      | a | s | d | f | g | h | j |
|      2      | z | x | c | v | b | n | m |

That is, q selects the 1st item on the 1st row, and d the third item on the second row. Progress will not be saved if Q is used to exit. \
Alternatively, if supported by the terminal, mouse clicks can also be used to select cards.

Shape, number, colour and shading (replaced with decoration) take the following values:
- Shape: > (diamond), O (pill) or S(squiggly)
- Number: > (one), >> (two), >>> (three)
- Colour: red, green, purple
- Decoration: no underline, 1 underline or 2 underlines