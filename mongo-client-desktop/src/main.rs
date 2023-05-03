extern crate core;

mod demo;

fn main() -> iced::Result {
    println!("Hello, iced!");

    // 1. counter
    // use crate::demo::counter::counter;
    // counter()

    // 2. events
    // use crate::demo::events::events;
    // events()

    // 3. clock
    // use crate::demo::clock::clock;
    // clock()


    // 4. clock_v2
    use crate::demo::clock_v2::clock;
    clock()
}
