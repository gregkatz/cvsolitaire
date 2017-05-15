use std::cmp::Ordering;
use rand::{thread_rng, Rng};
use ::gamemove::{Move, Valid};
use ::error::Error::*;
use ::Result;

#[derive(Clone, Eq, PartialEq)]
pub enum CardOrJacks {
    Card(Card),
    Jacks(Jacks),
}

impl CardOrJacks {
    pub fn into_card(self) -> Result<Card> {
        use self::CardOrJacks::*;
        match self {
            Card(c) => Ok(c),
            Jacks(_) => Err(InvalidConv),
        }
    }

    pub fn card(&self) -> Result<&Card> {
        use self::CardOrJacks::*;
        match *self {
            Card(ref c) => Ok(c),
            Jacks(_) => Err(InvalidConv),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Jacks {
    Red,
    Green,
    Black,
}

impl Jacks {
    pub fn from_suit(s: Suit) -> Jacks {
        match s {
            Suit::Red => Jacks::Red,
            Suit::Green => Jacks::Green,
            Suit::Black => Jacks::Black,
        }
    }

}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Red,
    Green,
    Black,
}

pub struct Board {
    pub joker: Option<Card>,
    pub utility: [Option<CardOrJacks>; 3],
    pub ordered: [Vec<NumCard>; 3],
    pub in_play: [Vec<Card>; 8],
}

impl Board {
    fn validate_jack(&self, s: Suit) -> Result<()> {
        let play_count = self.in_play.iter()
            .filter_map(|s| s.last())
            .filter(|card| card.is_jack() && card.suit()==Some(s)).count();
        let u_count = self.utility.iter().filter_map(Option::as_ref)
            .filter_map(|cod|cod.card().ok())
            .filter(|card| card.is_jack() && card.suit() == Some(s)).count();
        let none_count = self.utility.iter().map(Option::as_ref).filter(Option::is_none).count();
        if play_count + u_count != 4 { return Err(JacksNotVisible) }
        if u_count + none_count == 0 { return Err(NoOpenUtility) }
        Ok(())
    }

    fn validate_utility_is_card(&self, u: &::gamemove::Utility) -> Result<&Card> {
        let c_o_d = self.utility[*u as usize].as_ref().ok_or(NothingInUtl)?;
        Ok(match *c_o_d {
            CardOrJacks::Card(ref c) => c,
            CardOrJacks::Jacks(_) => return Err(MoveJacks),
        })
    }

    fn validate_stack_can_parent(&self, dst: &::gamemove::Stack, c: &Card) -> Result<()> {
        if let Some(parent) = self.in_play[*dst as usize].last() {
            if !parent.can_parent(c) {
                return Err(StackCantParent);
            }
        }
        Ok(())
    }
    
    fn validate_idx_is_card(&self, src: &::gamemove::Stack, y: u32) -> Result<usize> {
        (0..20u32)
            .filter(|slot| y > (110 + (slot * 20)) && y < ((slot * 20) + 212))
            .map(|idx| idx as usize)
            .filter(|idx| *idx < self.in_play[*src as usize].len())
            .max().ok_or(MustClickCard)
    }
    
    fn validate_stack_in_order(&self, src: &::gamemove::Stack, idx: usize) -> Result<()> {
        if self.in_play[*src as usize].len() < 1 { return Err(StackOutOfOrder) }
        if !in_order(&self.in_play[*src as usize][idx..]) { return Err(StackOutOfOrder) }
        Ok(())
    }

    fn validate_stack_last(&self, src: &::gamemove::Stack, idx: usize) -> Result<&Card> {
        if self.in_play[*src as usize].len() != idx + 1 { return Err(MultipleToSlot) }
        Ok(self.in_play[*src as usize].last().unwrap())
    }

    fn validate_card_num<'a>(&self, src: &'a Card) -> Result<&'a NumCard> {
        src.num().ok_or(CardNotNumeric)
    }

    fn validate_src_exists(&self, src: &::gamemove::Stack, idx: usize) -> Result<&Card> {
        self.in_play[*src as usize].get(idx).ok_or(NoCardClicked)
    }
    
    fn validate_utility_open(&self, dst: &::gamemove::Utility) -> Result<()> {
        if self.utility[*dst as usize].is_some() { return Err(UtlNotOpen) }
        Ok(())
    }

    fn validate_ord_can_parent(&self, src: &NumCard, dst: &::gamemove::Ordered) -> Result<()> {
        if let Some(dst) = self.ordered[*dst as usize].last() {
            if !dst.can_parent_ord(src) { return Err(OrdCantParent) }
        } else if src.value() != 0 { return Err(OrdCantParent) }
        Ok(())
    }
    
    pub fn get_valid(&self, m: Move) -> Result<Valid> {
        use ::gamemove::{Move, StackPosition};
        use ::gamemove::ClickTarget::*;
        
        match m {
            Move {dst: Some(RedButton), ..} =>
                self.validate_jack(Suit::Red)?,
            Move {dst: Some(GreenButton), ..} =>
                self.validate_jack(Suit::Green)?,
            Move {dst: Some(BlackButton), ..} =>
                self.validate_jack(Suit::Black)?,
            Move{src: Some(Utility(ref src)),
                 dst: Some(Stack(StackPosition{stack: ref dst, ..}))} => {
                let c = self.validate_utility_is_card(src)?;
                self.validate_stack_can_parent(dst, c)?;
            },
            Move{src: Some(Stack(StackPosition{stack: ref src, y})),
                 dst: Some(Stack(StackPosition{stack: ref dst, ..}))} => {
                //Ensure the first click is on a card
                let idx = self.validate_idx_is_card(src, y)?;
                //Make sure everything below the clicked card is in order
                self.validate_stack_in_order(src, idx)?;
                //TODO: This is probably redunant now...
                let c = self.validate_src_exists(src, idx)?;
                //Ensure destination can take the source
                self.validate_stack_can_parent(dst, c)?;
            },
            Move{src: Some(Utility(ref src)),
                 dst: Some(Ordered(ref dst))} => {
                //Ensure the first click is on a card
                let c = self.validate_utility_is_card(src)?;
                //Ensure it's a number card; Jacks can't move to ordered stack
                let num = self.validate_card_num(c)?;
                //Ensure this is the next card in the ordered stack
                self.validate_ord_can_parent(num, dst)?;
                },
            Move{src: Some(Stack(StackPosition{stack: ref src, y})),
                 dst: Some(Ordered(ref dst))} => {
                let idx = self.validate_idx_is_card(src, y)?;
                //Only the last card in stack can move to ordered
                let c = self.validate_stack_last(src, idx)?;
                //Ensure it's a number card; Jacks can't move to ordered stack
                let num = self.validate_card_num(c)?;
                //Ensure this is the next card in the ordered stack
                self.validate_ord_can_parent(num, dst)?;
            },
            Move{src: Some(Stack(StackPosition{stack: ref src, y})),
                 dst: Some(Utility(ref dst))} => {
                let idx = self.validate_idx_is_card(src, y)?;
                //Ensure source is the last card
                self.validate_stack_last(src, idx)?;
                //Ensure destination slot is open
                self.validate_utility_open(dst)?;
            }
            _ => return Err(BadSourceOrDest),
        };
        Ok(Valid(m))
    }

    pub fn clear_jacks(&mut self, s: Suit) {
        use self::CardOrJacks::*;
        use self::Jacks;

        //Clear jacks from utilities
        for slot in &mut self.utility {
            let mut temp_slot = slot.take();
            temp_slot = if let Some(cod) = temp_slot {
                if let Card(c) = cod {
                    if c.suit() == Some(s) && c.is_jack() {
                        None
                    } else {
                        Some(Card(c))
                    }
                } else { Some(cod) }
            } else { temp_slot };
            *slot = temp_slot;
        }
        
        //Clear jacks from stacks
        for stack in &mut self.in_play {
            stack.retain(|card| !(card.is_jack() && card.suit() == Some(s)));
        }
        //Add jack bundle to first clear utility
        if let Some(card) = self.utility.iter_mut().find(|c|**c == None) {
            *card = Some(Jacks(Jacks::from_suit(s)));
        }             
    }
    
    //Chnages the underlying board data to execute a validated move.
    //Unwrap calls are ok here, because the mvoe has been validated.
    pub fn make_move(&mut self, Valid(m): Valid) {
        use ::gamemove::{Move, StackPosition};
        use ::gamemove::ClickTarget::*;
        use self::CardOrJacks::*;
        match m {
            Move{dst: Some(RedButton), ..} => self.clear_jacks(Suit::Red),
            Move{dst: Some(GreenButton), ..} => self.clear_jacks(Suit::Green),
            Move{dst: Some(BlackButton), ..} => self.clear_jacks(Suit::Black),
            Move{src: Some(Utility(src)),
                 dst: Some(Stack(StackPosition{stack: dst, ..}))} =>
                self.in_play[dst as usize]
                    .push(self.utility[src as usize].take().unwrap().into_card().unwrap()),
            Move{src: Some(Utility(src)),
                 dst: Some(Ordered(dst))} => {
                let tmp = self.utility[src as usize]
                    .take()
                    .unwrap()
                    .into_card()
                    .unwrap()
                    .into_num()
                    .unwrap();
                self.ordered[dst as usize].push(tmp);
                },
            Move{src: Some(Stack(StackPosition{stack:src, ..})),
                 dst: Some(Utility(dst)) } => self.utility[dst as usize] =
                Some(Card(self.in_play[src as usize].pop().unwrap())),
            Move{src: Some(Stack(StackPosition{stack: src, y})),
                 dst: Some(Stack(StackPosition{stack: dst, ..})) } => {
                let idx = self.validate_idx_is_card(&src, y).unwrap();
                let tmp: Vec<_> = self.in_play[src as usize].drain(idx..).collect();
                for card in tmp {
                    self.in_play[dst as usize].push(card);
                }
            },
            Move{src: Some(Stack(StackPosition{stack: src, ..})),
                 dst: Some(Ordered(dst))} => { 
                let tmp = self.in_play[src as usize].pop()
                    .unwrap()
                    .into_num()
                    .unwrap();
                self.ordered[dst as usize].push(tmp);
            },
            _ => panic!("Invalid move passed as valid!"), 
        }
    }

    pub fn new() -> Self {
        use self::Card::*;
        use self::NumCard::*;
          
        let mut board = Board {
            joker: None,
            utility: [None, None, None],
            ordered: [Vec::new(), Vec::new(), Vec::new()],
            in_play: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                ],
        };
            
        let mut deck = Vec::new();
        //Joker
        deck.push(Joker);
        for _ in 0..4 {
            deck.push(JBlack);
            deck.push(JRed);
            deck.push(JGreen);
        }
        for i in 0..9 {
            deck.push(Num(Black(i)));
            deck.push(Num(Red(i)));
            deck.push(Num(Green(i)));
        }

        let mut rng = thread_rng();
        rng.shuffle(&mut deck);

        for stack in &mut board.in_play {
            for _ in 0..5 { stack.push(deck.pop().unwrap()); }            
        }
        
        board.sweep_free();
        board
    }


    fn autosweep(&mut self) -> Option<Card> {
        use self::Card::*;
        //Calculate the minimum card in the stacks or utility slots
        let min_in_play = self.in_play.iter()
            .flat_map(|v| v)
            .chain(self.utility.iter()
                   .filter_map(Option::as_ref)
                   .filter_map(|c| c.card().ok()))                
            .filter_map(Card::num)
            .map(NumCard::value)
            .min();
        
        if let Some(min) = min_in_play {
            for stack in &mut self.in_play {
                if let Some(last) = stack.pop() {
                    if let Some(value) = last.value() {
                        if value <= min { return Some(last) }
                    }
                    stack.push(last);
                }
            }
        }

        for stack in &mut self.in_play {
            if let Some(last) = stack.pop() {
                if last == Joker { return Some(Joker) }
                stack.push(last);
            }
        }
        None
    }
    
    pub fn sweep_free(&mut self) {
        while let Some(card) = self.autosweep() {
            self.insert_ordered(card);
        }
    }

    fn insert_ordered(&mut self, card: Card) {
        use self::Card::*;
        use self::NumCard::*;
        
        match card {
            Joker => self.joker = Some(card),
            Num(Red(x)) => {self.ordered[0].push(Red(x));},
            Num(Green(x)) => {self.ordered[1].push(Green(x));},
            Num(Black(x)) => {self.ordered[2].push(Black(x));},
            _ => unreachable!(),
        };
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Card {
    JBlack,
    JRed,
    JGreen,
    Joker,
    Num(NumCard),
}

#[derive(Clone, Eq, PartialEq)]
pub enum NumCard {
    Black(usize),
    Red(usize),
    Green(usize),
}

impl Card {
    fn can_parent(&self, other: &Card) -> bool {
        use self::Card::*;
        if let (&Num(ref my_nc), &Num(ref other_nc)) = (self, other) {
            my_nc.suit() != other_nc.suit() && my_nc.value() == other_nc.value() + 1
        } else { false }
    }

    fn into_num(self) -> Option<NumCard> {
        match self {
            Card::Num(c) => Some(c),
            _ => None,
        }
    }

    fn num(&self) -> Option<&NumCard> {
        match *self {
            Card::Num(ref c) => Some(c),
            _ => None,
        }
    }

    fn value(&self) -> Option<usize> {
        self.num().map(|n|n.value())
    }

    fn is_jack(&self) -> bool {
        match *self {
            Card::JRed |
            Card::JGreen |
            Card::JBlack => true,
            _=> false
        }
    }
    
    fn suit(&self) -> Option<Suit> {
        match *self {
            Card::Num(ref nc) => Some(nc.suit()),
            Card::JRed => Some(Suit::Red),
            Card::JGreen => Some(Suit::Green),
            Card::JBlack => Some(Suit::Black),
            Card::Joker => None,
        }
    }
}

impl NumCard {
    fn suit(&self) -> Suit {
        match *self {
            NumCard::Red(_) => Suit::Red,
            NumCard::Green(_) => Suit::Green,
            NumCard::Black(_) => Suit::Black,
        }
    }
    
    pub fn value(&self) -> usize {
        match *self {
            NumCard::Red(n) |
            NumCard::Green(n) |
            NumCard::Black(n) => n,
        }
    }
    
    fn can_parent_ord(&self, other: &NumCard) -> bool {
        self.suit() == other.suit() && self.value() + 1 == other.value()
    }
}

impl Ord for NumCard {
    fn cmp(&self, other: &NumCard) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for NumCard {
    fn partial_cmp(&self, other: &NumCard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn in_order(cards: &[Card]) -> bool {
    if cards.len() < 1 { return true }
    !cards.windows(2).any(|s|!s[0].can_parent(&s[1]))
}
