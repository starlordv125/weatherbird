pub fn print_weather(code: u8, day: u8) {
    if (code == 0 || code == 1 || code == 2) && day == 1 {
        match code {
            0 => {
                println!("|-------------------------|");
                println!("|    \\       |       /    |");
                println!("|     \\      |      /     |");
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
            _ => {}
        }
    } else {
        match code {
            0 => {
                println!("|-------------------------|");
                println!("|      _______     *      |");
                println!("|  *  /    _  \\  *        |");
                println!("|    /  o / \\__\\          |");
                println!("|   |    |            *   |");
                println!("|   |  o |                |");
                println!("|   |    |                |");
                println!("|   \\o   \\ *  _           |");
                println!("|    \\  O \\ _/ /          |");
                println!("|  *  \\_______/    *      |");
                println!("|-------------------------|");
                println!("Starry")
            }
            1 | 2 => {
                println!("|-------------------------|");
                println!("|      _______     *      |");
                println!("|  *  /    _  \\  *  __    |");
                println!("|    /  o / \\__\\   /  \\   |");
                println!("|   |    |        |    \\  |");
                println!("|   |  o |        \\_____) |");
                println!("|   |    |                |");
                println!("|   -------               |");
                println!("|  /       \\__      *     |");
                println!("|  (___________\\          |");
                println!("|-------------------------|");
                println!("Cloudy")
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
}
