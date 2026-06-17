pub fn print_weather(code: u8) {
    match code {
        0 => {
            println!("|-------------------------|");
            println!("|    \\       |      /     |");
            println!("|     \\      |     /      |");
            println!("|       ------------      |");
            println!("|      /            \\     |");
            println!("|  ___|              |___ |");
            println!("|     |              |    |");
            println!("|      \\____________/     |");
            println!("|      /      |       \\   |");
            println!("|     /       |        \\  |");
            println!("|-------------------------|");
            println!("Sunny");
        }
        1 | 2 => {
            println!("|-------------------------|");
            println!("|    \\       |      /     |");
            println!("|     \\      |     /      |");
            println!("|       ------------      |");
            println!("|      /            \\     |");
            println!("|  ___|           _/-\\___ |");
            println!("|     _          /_____\\  |");
            println!("|   _/ \\_____________/    |");
            println!("|  /     \\    |       \\   |");
            println!("| /______/    |        \\  |");
            println!("|-------------------------|");
            println!("Partly cloudy");
        }
        3 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|                         |");
            println!("|                         |");
            println!("|                         |");
            println!("|-------------------------|");
            println!("Overcast");
        }
        51 | 53 | 55 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|                         |");
            println!("|      /            /     |");
            println!("|                         |");
            println!("|-------------------------|");
            println!("Light rain");
        }
        61 | 63 | 65 | 80 | 81 | 82 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|                         |");
            println!("|     /   /   /   /       |");
            println!("|       /   /   /         |");
            println!("|-------------------------|");
            println!("Heavy rain");
        }
        66 | 67 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|                         |");
            println!("|      /     /    /       |");
            println!("|     *     *    *        |");
            println!("|-------------------------|");
            println!("Freezing rain");
        }
        71 | 73 | 77 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|                         |");
            println!("|      *          *       |");
            println!("|                         |");
            println!("|-------------------------|");
            println!("Light snow");
        }
        75 | 85 | 86 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|                         |");
            println!("|      *     *     *      |");
            println!("|   *     *     *         |");
            println!("|-------------------------|");
            println!("Heavy snow");
        }
        95 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|            /            |");
            println!("|     /  /  /___  /  /    |");
            println!("|      /  /    /   /  /   |");
            println!("|-------------------------|");
            println!("Stormy");
        }
        96 | 99 => {
            println!("|-------------------------|");
            println!("|              _____      |");
            println!("|       ______/     \\     |");
            println!("|      /             |    |");
            println!("|     /              \\    |");
            println!("|    /                \\   |");
            println!("|   |__________________|  |");
            println!("|            /            |");
            println!("|     o  o  /___     o    |");
            println!("|      o       /   o      |");
            println!("|-------------------------|");
            println!("Hail");
        }
        _ => {
            println!("|-------------------------|");
            println!("|                         |");
            println!("|                         |");
            println!("|                         |");
            println!("|           ????          |");
            println!("|           ????          |");
            println!("|           ????          |");
            println!("|                         |");
            println!("|                         |");
            println!("|                         |");
            println!("|-------------------------|");
            println!("Unknown");
        }
    }
}
