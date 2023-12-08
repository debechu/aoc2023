use std::io::{
    self,
    Read,
};
use std::iter::Peekable;
use std::ops::Range;
use std::str::Chars;

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let mut hands = Vec::with_capacity(1000);
    let mut bids = Vec::with_capacity(1000);

    use std::time::Instant;
    let start = Instant::now();
    let mut iter = data.chars().peekable();
    loop
    {
        let hand = parse_hand(&mut iter);
        if hand.is_none() { break; }

        let bid = scan_number(&mut iter);
        if bid.is_none() { panic!("Expected bid value!"); }

        skip_whitespace(&mut iter);
        if iter.next().unwrap_or('\n') != '\n'
        {
            panic!("Expected new line!");
        }

        hands.push(hand.unwrap());
        bids.push(bid.unwrap());
    }
    let parse_time = start.elapsed();

    let start = Instant::now();
    quicksort(0..hands.len(), &mut hands, &mut bids);
    let sum_bids: u64 = bids.iter().enumerate()
        .map(|(i, v)| v * (i+1) as u64)
        .sum();
    let process_time = start.elapsed();

    println!("==== Part 2");
    println!("Sum of bids by rank: {}", sum_bids);
    println!("==== Elapsed time");
    println!("- Parse time....: {}ns", parse_time.as_nanos());
    println!("- Process time...: {}ns", process_time.as_nanos());
}

fn quicksort(
    range: Range<usize>,
    hands: &mut [Hand],
    bids: &mut [u64])
{
    if (range.start as isize) < range.end as isize - 1
    {
        let pivot = quicksort_divide(
            hands, bids, range.start..range.end
        );
        quicksort(range.start..pivot, hands, bids);
        quicksort(pivot+1..range.end, hands, bids);
    }
}

fn quicksort_divide(
    hands: &mut [Hand],
    bids: &mut [u64],
    range: Range<usize>) -> usize
{
    let mut i = range.start;
    let mut j = range.end-2;
    let pivot = range.end-1;
    loop
    {
        while i < range.end && hands[i] < hands[pivot] { i += 1; }
        while j > range.start && hands[j] > hands[pivot] { j -= 1; }

        if i < j
        {
            hands.swap(i, j);
            bids.swap(i, j);
        }
        else { break; }

        i += 1;
        j -= 1;
    }

    hands.swap(pivot, i);
    bids.swap(pivot, i);

    i
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CamelCard
{
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for CamelCard
{
    fn from(c: char) -> Self
    {
        match c
        {
            'J' => CamelCard::Joker,
            '2' => CamelCard::Two,
            '3' => CamelCard::Three,
            '4' => CamelCard::Four,
            '5' => CamelCard::Five,
            '6' => CamelCard::Six,
            '7' => CamelCard::Seven,
            '8' => CamelCard::Eight,
            '9' => CamelCard::Nine,
            'T' => CamelCard::Ten,
            'Q' => CamelCard::Queen,
            'K' => CamelCard::King,
            'A' => CamelCard::Ace,
            _ => panic!("{} is not a valid camel card!", c)
        }
    }
}

impl From<CamelCard> for char
{
    fn from(c: CamelCard) -> Self
    {
        match c
        {
            CamelCard::Joker => 'J',
            CamelCard::Two => '2',
            CamelCard::Three => '3',
            CamelCard::Four => '4',
            CamelCard::Five => '5',
            CamelCard::Six => '6',
            CamelCard::Seven => '7',
            CamelCard::Eight => '8',
            CamelCard::Nine => '9',
            CamelCard::Ten => 'T',
            CamelCard::Queen => 'Q',
            CamelCard::King => 'K',
            CamelCard::Ace => 'A',
        }
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind
{
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand
{
    kind: HandKind,
    cards: [CamelCard; 5],
}

use std::cmp::Ordering;
impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Hand
{
    fn cmp(&self, other: &Hand) -> Ordering
    {
        if self.kind < other.kind { return Ordering::Less; }
        if self.kind > other.kind { return Ordering::Greater; }

        for (i, &card) in self.cards.iter().enumerate()
        {
            let other_card = other.cards[i];
            if card < other_card { return Ordering::Less; }
            if card > other_card { return Ordering::Greater; }
        }

        Ordering::Equal
    }
}

use std::fmt;
impl fmt::Display for Hand
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        for &card in &self.cards
        {
            write!(f, "{}", char::from(card))?;
        }

        Ok(())
    }
}

fn parse_hand(iter: &mut Peekable<Chars>) -> Option<Hand>
{
    skip_whitespace(iter);
    iter.peek()?;

    use std::mem::MaybeUninit;
    let mut cards: [MaybeUninit<CamelCard>; 5] = unsafe {
        MaybeUninit::uninit().assume_init()
    };
    let mut card_counts = [0u64; 13];
    for card in &mut cards
    {
        if let Some(c) = iter.next()
        {
            let kind = CamelCard::from(c);
            card_counts[kind as usize] += 1;
            card.write(kind);
        }
        else
        {
            panic!("Too few cards in hand!");
        }
    }
    let cards = unsafe { std::mem::transmute::<_, _>(cards) };

    Some(Hand {
        kind: {
            let mut two_pairs = 0;
            let mut three_pairs = 0;
            let mut four_pairs = 0;
            let mut five_pairs = 0;

            let joker_count = card_counts[CamelCard::Joker as usize];
            for count in card_counts
            {
                match count
                {
                    2 => two_pairs += 1,
                    3 => three_pairs += 1,
                    4 => four_pairs += 1,
                    5 => five_pairs += 1,
                    _ => ()
                }
            }

            if five_pairs == 1 { HandKind::FiveOfAKind }
            else if four_pairs == 1 {
                if joker_count > 0 {
                    HandKind::FiveOfAKind
                } else {
                    HandKind::FourOfAKind
                }
            }
            else if three_pairs == 1 && two_pairs == 1 {
                if joker_count > 0 {
                    HandKind::FiveOfAKind
                } else {
                    HandKind::FullHouse
                }
            }
            else if three_pairs == 1 {
                if joker_count > 0 {
                    if joker_count == 2 {
                        HandKind::FiveOfAKind
                    } else {
                        HandKind::FourOfAKind
                    }
                } else {
                    HandKind::ThreeOfAKind
                }
            }
            else if two_pairs == 2 {
                if joker_count > 0 {
                    if joker_count == 1 {
                        HandKind::FullHouse
                    } else {
                        HandKind::FourOfAKind
                    }
                } else {
                    HandKind::TwoPair
                }
            }
            else if two_pairs == 1 {
                if joker_count > 0 {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::OnePair
                }
            }
            else if joker_count > 0 {
                HandKind::OnePair
            } else {
                HandKind::HighCard
            }
        },
        cards,
    })
}

fn scan_number(iter: &mut Peekable<Chars>) -> Option<u64>
{
    skip_whitespace(iter);
    if let Some(c) = iter.peek()
    {
        if !c.is_ascii_digit()
        {
            return None;
        }
    }
    else { return None; }

    let mut n = 0;
    while let Some(c) = iter.peek()
    {
        if !c.is_ascii_digit() { break; }
        n = n * 10 + c.to_digit(10).unwrap() as u64;
        iter.next();
    }
    Some(n)
}

fn skip_whitespace(iter: &mut Peekable<Chars>)
{
    while iter.next_if(
        |c| matches!(c, ' ' | '\t' | '\r' | '\x0B' | '\x0C')
    ).is_some() {}
}
