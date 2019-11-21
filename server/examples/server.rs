use async_std::task;

use rooms_server::*;

fn main() {
    task::block_on(make_server());
}