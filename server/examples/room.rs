extern crate rooms_server;

use std::collections::*;
use rooms_server::room::*;

struct MatchingConditionNone {}

impl IsMatch<MatchingConditionNone> for RoomState {
    fn is_match(&self, condition: &MatchingConditionNone) -> bool {
        true
    }
}

fn main() {
    let l = LinkedList::<RoomState>::new();
    l.match_room(&MatchingConditionNone{});
    println!("hello");
}
