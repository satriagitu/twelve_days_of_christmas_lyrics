fn main() {
    let hadiah = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for hari in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love sent to me:",
            match hari {
                1 => "first",
                2 => "second",
                3 => "third",
                4 => "fourth",
                5 => "fifth",
                6 => "sixth",
                7 => "seventh",
                8 => "eighth",
                9 => "ninth",
                10 => "tenth",
                11 => "eleventh",
                12 => "twelfth",
                _ => unreachable!(),
            }
        );

        for i in (0..hari).rev() {
            if hari > 1 && i == 0 {
                println!("and {}", hadiah[i]);
            } else {
                println!("{}", hadiah[i]);
            }
        }

        println!(); // Baris kosong di antara hari
    }
}
