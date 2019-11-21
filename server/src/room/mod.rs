use std::collections::*;
use async_std::prelude::*;
use futures::channel::mpsc;//::sync;

// pub async fn make_lobby(ch: futures::mpsc) {

// }
pub trait IsMatch<C> {
    fn is_match(&self, condition: &C) -> bool;
}

pub trait UserCount {
    fn user_count(&self) -> u32;
}

pub struct RoomState {
    room_id: u32,
    user_count: u32,
    //server id
    //user count
    //etc..
}


impl UserCount for RoomState {
    fn user_count(&self) -> u32 {
        self.user_count
    }
}

pub trait MatchRoom<C>
{
    fn match_room(&self, c: &C) -> u32;
}
impl<I, C> MatchRoom<C> for LinkedList<I>
where
    I: IsMatch<C> + UserCount,
{
    fn match_room(&self, c: &C) -> u32 {
        self.iter().filter(|r| r.is_match(c))
            .min_by(|r1, r2| r1.user_count().cmp(&r2.user_count()));
            1
    }
}
