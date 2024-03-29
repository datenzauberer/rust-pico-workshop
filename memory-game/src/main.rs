#![no_std]
#![no_main]

use bsp::entry;
use rp_pico as bsp;

use memory_game::memory_board::MemoryBoard;

fn show_sequence(board: &mut MemoryBoard, random_sequence: &[u8]) {
    for value in random_sequence {
        board.switch_led(*value, true);
        board.wait_ms(500);
        board.switch_led(*value, false);
        board.wait_ms(250);
    }
}

fn get_number_of_correctly_memorized_colours(
    board: &mut MemoryBoard,
    random_sequence: &[u8],
) -> u8 {
    let mut number_correct = 0;
    while number_correct < random_sequence.len() {
        let user_input = board.wait_for_button_press();
        if user_input == random_sequence[number_correct] {
            board.sound(100);
            number_correct += 1;
        } else {
            board.sound(500);
            break;
        }
    }
    number_correct as u8
}

#[entry]
fn main() -> ! {
    let mut random_colour_sequence = [0; 100]; // hopefully, nobody will be able to remember more than 100 values - maybe replace this with alloc or heapless in the future (https://docs.rust-embedded.org/book/collections/index.html)
    let mut number_colours = 0;
    let mut board = MemoryBoard::new().unwrap();

    board.display(number_colours);
    loop {
        // add next random number
        random_colour_sequence[number_colours as usize] = board.get_random_colour_number();
        number_colours += 1;
        if number_colours >= random_colour_sequence.len() as u8 {
            panic!("Sequence of colours is too long!");
        }

        // show sequence
        show_sequence(
            &mut board,
            &random_colour_sequence[..number_colours as usize],
        );

        // determine number of correct guesses
        let number_correct = get_number_of_correctly_memorized_colours(
            &mut board,
            &random_colour_sequence[..number_colours as usize],
        );

        // show them
        board.display(number_correct);
        board.wait_ms(1000);

        if number_correct < random_colour_sequence[0..number_colours as usize].len() as u8 {
            number_colours = 0;
        }
    }
}
