use crossterm::style::{Attribute, PrintStyledContent, StyledContent, Stylize};

#[derive(PartialEq, Eq)]
pub struct Card {
    pub letter: Letter,
    pub count: Count,
    pub styling: Styling,
    pub colour: Colour,
}

impl Card {
    /// Creates a card based on its 4-trit (i.e. number from [0..3**4) hence a u8) representation
    pub fn from_tile_number(num: u8) -> Option<Self> {
        let letter = num % 3;
        let count = (num % 9 - letter) / 3;
        let styling = (num % 27 - 3 * count - letter) / 9;
        let colour = (num % 81 - 9 * styling - 3 * count - letter) / 27;
        Some(Self {
            letter: Letter::from_number(letter)?,
            count: Count::from_number(count)?,
            styling: Styling::from_number(styling)?,
            colour: Colour::from_number(colour)?,
        })
    }

    /// It is always possible to form a set with two cards and one other unique card.
    /// This function returns such card.
    pub fn third(&self, other: &Self) -> Card {
        Card {
            letter: self.letter.third(&other.letter),
            count: self.count.third(&other.count),
            styling: self.styling.third(&other.styling),
            colour: self.colour.third(&other.colour),
        }
    }

    // Formatting

    /// Get a StyledContent representing this tile
    fn get_inner_style(&self) -> StyledContent<String> {
        self.colour
            .apply(self.styling.apply(self.count.apply(self.letter.print())))
    }

    /// Returns an instruction to print an (un)highlighted version of this tile
    pub fn stylise(&self, highlighted: bool) -> PrintStyledContent<String> {
        if highlighted {
            PrintStyledContent(self.get_inner_style().on(crossterm::style::Color::DarkGrey))
        } else {
            PrintStyledContent(self.get_inner_style())
        }
    }
}

/// A representation of the letter on a card: Cylinders (rendered as O), Diamonds (rendered as >, D for diamond) and squigglies (rendered as S)
#[derive(PartialEq, Eq)]
pub enum Letter {
    O,
    D,
    S,
}
impl Letter {
    fn from_number(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::O),
            1 => Some(Self::D),
            2 => Some(Self::S),
            _ => None,
        }
    }

    fn print(&self) -> String {
        match self {
            Self::O => "O".to_owned(),
            Self::D => ">".to_owned(),
            Self::S => "S".to_owned(),
        }
    }

    fn third(&self, other: &Self) -> Self {
        match self {
            Self::O => match other {
                Self::O => Self::O,
                Self::D => Self::S,
                Self::S => Self::D,
            },
            Self::D => match other {
                Self::O => Self::S,
                Self::D => Self::D,
                Self::S => Self::O,
            },
            Self::S => match other {
                Self::O => Self::D,
                Self::D => Self::O,
                Self::S => Self::S,
            },
        }
    }
}

/// A representation of how many shapes are on the card
#[derive(PartialEq, Eq)]
pub enum Count {
    One,
    Two,
    Three,
}

impl Count {
    fn apply(&self, string: String) -> String {
        match self {
            Self::One => " ".to_owned() + &string + " ",
            Self::Two => string.clone() + " " + &string,
            Self::Three => string.clone() + &string + &string,
        }
    }

    fn from_number(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::One),
            1 => Some(Self::Two),
            2 => Some(Self::Three),
            _ => None,
        }
    }

    fn third(&self, other: &Self) -> Self {
        match self {
            Self::One => match other {
                Self::One => Self::One,
                Self::Two => Self::Three,
                Self::Three => Self::Two,
            },
            Self::Two => match other {
                Self::One => Self::Three,
                Self::Two => Self::Two,
                Self::Three => Self::One,
            },
            Self::Three => match other {
                Self::One => Self::Two,
                Self::Two => Self::One,
                Self::Three => Self::Three,
            },
        }
    }
}

/// A representation of the fill of a card: Empty (rendered as no decoration), Striped (represented as one underline), Filled (originally represented as bold, but changed to double underline)
#[derive(PartialEq, Eq)]
pub enum Styling {
    None,
    Underline,
    DoubleUnderline,
}
impl Styling {
    fn apply<T: Stylize>(&self, string: T) -> T::Styled {
        match self {
            Self::None => string.attribute(Attribute::NoUnderline),
            Self::Underline => string.attribute(Attribute::Underlined),
            // some terminals interpret no bold as double underline???
            Self::DoubleUnderline => string.attribute(Attribute::NoBold),
        }
    }

    fn from_number(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::None),
            1 => Some(Self::Underline),
            2 => Some(Self::DoubleUnderline),
            _ => None,
        }
    }

    fn third(&self, other: &Self) -> Self {
        match self {
            Self::None => match other {
                Self::None => Self::None,
                Self::Underline => Self::DoubleUnderline,
                Self::DoubleUnderline => Self::Underline,
            },
            Self::Underline => match other {
                Self::None => Self::DoubleUnderline,
                Self::Underline => Self::Underline,
                Self::DoubleUnderline => Self::None,
            },
            Self::DoubleUnderline => match other {
                Self::None => Self::Underline,
                Self::Underline => Self::None,
                Self::DoubleUnderline => Self::DoubleUnderline,
            },
        }
    }
}


/// Representation of the colour of a card. Colour, not color.
#[derive(PartialEq, Eq)]
pub enum Colour {
    Red,
    Green,
    Purple,
}
impl Colour {
    fn apply<T: Stylize>(&self, string: T) -> T::Styled {
        match self {
            Self::Red => string.red(),
            Self::Green => string.green(),
            Self::Purple => string.magenta(),
        }
    }

    fn from_number(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::Red),
            1 => Some(Self::Green),
            2 => Some(Self::Purple),
            _ => None,
        }
    }

    fn third(&self, other: &Self) -> Self {
        match self {
            Self::Red => match other {
                Self::Red => Self::Red,
                Self::Green => Self::Purple,
                Self::Purple => Self::Green,
            },
            Self::Green => match other {
                Self::Red => Self::Purple,
                Self::Green => Self::Green,
                Self::Purple => Self::Red,
            },
            Self::Purple => match other {
                Self::Red => Self::Green,
                Self::Green => Self::Red,
                Self::Purple => Self::Purple,
            },
        }
    }
}
