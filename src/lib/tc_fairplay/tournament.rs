#![allow(dead_code)]

use time::PrimitiveDateTime;

struct Player {
    last_name: String,
    first_name: String,
}

enum SwissRating {
    National(u8), // 1 <= x <= 4
    Regional(u8), // 1 <= x <= 9
}

// SwissTennis rating and rank.
struct PlayerRating {
    rating: SwissRating,
    rank: u32
}

struct Court(u8); // 1 <= x <= 3

struct Schedule {
    date_time: PrimitiveDateTime,
    court: Court
}

enum FirstOrSecond {
    First,
    Second
}

struct MatchResult {
    sets: Vec<(u8, u8)>, // sets (and match-tie-break)
    walkover: Option<FirstOrSecond>,
}

// no player: empty slot
// one player: BYE
struct Slot<'a> {
    player_a: Option<&'a Player>,
    player_b: Option<&'a Player>,
    schedule: Option<Schedule>,
    match_result: Option<MatchResult>,
}

// How the players flow through the tournament.
struct SlotConnection<'a> {
    source_a: &'a Slot<'a>,
    source_b: &'a Slot<'a>,
    target: &'a Slot<'a>,
}

struct TournamentBracket<'a> {
    slots: Vec<Slot<'a>>,
    tree: Vec<SlotConnection<'a>>,
}

struct Tournament<'a> {
    players: Vec<Player>,
    brackets: Vec<TournamentBracket<'a>>,
}
