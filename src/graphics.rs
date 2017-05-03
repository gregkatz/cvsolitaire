use ::orbtk::{Renderer, Color, Point};
use ::board::Board;
use ::gamemove::{STACKS, UTILITIES, ROSE, ORDERED, BUTTONS};

lazy_static! {
    static ref SSHEET: ::orbimage::Image =
        ::orbimage::parse_png(include_bytes!("../assets/cards.png")).unwrap();
}

pub fn render_cursor<R: Renderer> (r: &mut R, p: Option<&Point>) {
    if let Some(p) = p {
        SSHEET.roi(70, 612, 30, 30)
            .draw(r, p.x - 15, p.y - 15);
    }
}

pub fn render<R: Renderer> (r: &mut R, b: &Board) {
    r.set(Color::rgba(255,255,255, 0));
    render_stacks(r, b);
    render_joker(r, b);
    render_ordered(r, b);
    render_utilities(r, b);
    render_buttons(r, b);
}

pub fn render_stacks<R: Renderer>(r: &mut R, b: &Board)  {
    let rows = b.in_play.iter().map(|v| v.len()).max().unwrap();
    for row in 0..rows {
        for col in 0..8 {
            if let Some(card) = b.in_play[col].get(row) {
                let (rmap, cmap) = card.map_coords();
                SSHEET.roi(cmap as u32, rmap as u32, 70, 102)
                    .draw(r, (col as i32 * 75) + STACKS.0, (row as i32 * 20) + STACKS.1);
            }
        }
    }
}

pub fn render_joker<R: Renderer>(r: &mut R, b: &Board)  {
    let (rmap, cmap) = if let Some(ref r) = b.joker {
        r.map_coords()
    } else { (510, 0) };
    SSHEET.roi(cmap as u32, rmap as u32, 70, 102).draw(r, ROSE.0, ROSE.1);
}

pub fn render_ordered<R: Renderer>(r: &mut R, b: &Board)  {
    for (idx, slot) in b.ordered.iter().enumerate() {
        let (rmap, cmap) = if let Some(c) = slot.last() {
            if c.value() == 8 { (612, 0) } else { c.map_coords() }
        } else { (510, 0) };
        SSHEET.roi(cmap as u32, rmap as u32, 70, 102)
            .draw(r, ORDERED.0 + (idx as i32 * 75), ORDERED.1);
    }
}

pub fn render_buttons<R: Renderer>(r: &mut R, _b: &Board)  {
    r.rect(BUTTONS.0 + 20, BUTTONS.1 + 0, 30, 30, Color::rgb(223, 0, 0));
    r.rect(BUTTONS.0 + 20, BUTTONS.1 + 35, 30, 30, Color::rgb(0, 160, 0));
    r.rect(BUTTONS.0 + 20, BUTTONS.1 + 70, 30, 30, Color::rgb(0, 0, 0));
}

pub fn render_utilities<R: Renderer>(r: &mut R, b: &Board)  {
    for (idx, slot) in b.utility.iter().enumerate() {
        let (rmap, cmap) = if let Some(ref cod) = *slot {
            cod.map_coords()
        } else { (510, 0) };
        SSHEET.roi(cmap as u32, rmap as u32, 70, 102)
            .draw(r, UTILITIES.0 + (idx as i32 * 75), UTILITIES.1);
    }
}

trait SpriteMapped {
    fn sprite_map(&self) -> (usize, usize);
    fn map_coords(&self) -> (usize, usize) {
        let (row, col) = self.sprite_map();
        (row * 102, col * 70)
    }
}

impl SpriteMapped for ::board::NumCard {
    fn sprite_map(&self) -> (usize, usize) {
        use ::board::NumCard;
        match *self {
            NumCard::Red(0) => (4, 6),
            NumCard::Green(0) => (4, 5),
            NumCard::Black(0) => (4, 4),
            NumCard::Red(1) => (0, 6),
            NumCard::Green(1) => (0, 5),
            NumCard::Black(1) => (0, 4),
            NumCard::Red(3) => (1, 6),
            NumCard::Green(3) => (1, 5),
            NumCard::Black(3) => (1, 4),
            NumCard::Red(5) => (2, 6),
            NumCard::Green(5) => (2, 5),
            NumCard::Black(5) => (2, 4),
            NumCard::Red(7) => (3, 6),
            NumCard::Green(7) => (3, 5),
            NumCard::Black(7) => (3, 4),
            NumCard::Red(9) => (0, 2),
            NumCard::Green(9) => (0, 1),
            NumCard::Black(9) => (0, 0),
            NumCard::Red(2) => (1, 2),
            NumCard::Green(2) => (1, 1),
            NumCard::Black(2) => (1, 0),
            NumCard::Red(4) => (2, 2),
            NumCard::Green(4) => (2, 1),
            NumCard::Black(4) => (2, 0),
            NumCard::Red(6) => (3, 2),
            NumCard::Green(6) => (3, 1),
            NumCard::Black(6) => (3, 0),
            NumCard::Red(8) => (4, 2),
            NumCard::Green(8) => (4, 1),
            NumCard::Black(8) => (4, 0),
            _ => unreachable!(),
        }
    }
}

impl SpriteMapped for ::board::Card {
    fn sprite_map(&self) -> (usize, usize) {
        use ::board::Card;
        match *self {
            Card::JRed => (5, 3),
            Card::JGreen => (5, 2),
            Card::JBlack => (5, 1),
            Card::Joker => (5, 5),
            Card::Num(ref c) => c.sprite_map(),
        }
    }
}

impl SpriteMapped for ::board::CardOrJacks {
    fn sprite_map(&self) -> (usize, usize) {
        use ::board::CardOrJacks;
        match *self {
            CardOrJacks::Card(ref c) => c.sprite_map(),
            CardOrJacks::Jacks(ref d) => d.sprite_map(),
        }
    }
}

impl SpriteMapped for ::board::Jacks {
    fn sprite_map(&self) -> (usize, usize) {
        use ::board::Jacks;
        match *self {
            Jacks::Red => (6, 0),
            Jacks::Green => (6, 0),
            Jacks::Black => (6, 0),
        }
    }
}
