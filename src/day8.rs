use std::io::{stdin, Read};
use termion::color;
use termion::cursor::Goto;

pub fn run() {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;

    let image = read_memory();
    // part 1:
    // let result = image
    //     .chunks(WIDTH * HEIGHT)
    //     .min_by(|layer1, layer2| {
    //         layer1
    //             .iter()
    //             .filter(|&&x| x == 0)
    //             .count()
    //             .cmp(&layer2.iter().filter(|&&x| x == 0).count())
    //     })
    //     .map(|layer| {
    //         &layer.iter().filter(|&&x| x == 1).count() * &layer.iter().filter(|&&x| x == 2).count()
    //     })
    //     .unwrap();
    // println!("{}", result);

    let result = image
        .chunks(WIDTH * HEIGHT)
        .fold([2; WIDTH * HEIGHT], |image, layer| {
            let mut new_image = [2; WIDTH * HEIGHT];
            for (i, pixel) in layer.iter().enumerate() {
                match (image[i], pixel) {
                    (2, &v) => new_image[i] = v,
                    (v, _) => new_image[i] = v,
                }
            }
            new_image
        });
    print!("{}", termion::clear::All);
    println!("{:#?}", Vec::from(&result[..]));
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let symbol = match result[y * WIDTH + x] {
                2 => String::from("\x20"), // space
                1 => format!("{}\u{2588}", color::Fg(color::White)),
                0 => format!("{}\u{2588}", color::Fg(color::Red)),
                _ => String::from("e"),
            };
            print!("{}{}", Goto((x + 1) as u16, (y + 1) as u16), symbol)
        }
    }
    println!();
}

pub fn read_memory() -> Vec<i64> {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");

    let input_string = std::str::from_utf8(&s).expect("invalid utf8");
    let memory_values: Vec<i64> = input_string
        .chars()
        .filter_map(|value| value.to_digit(10).map(|v| v as i64))
        .collect();
    memory_values
}
