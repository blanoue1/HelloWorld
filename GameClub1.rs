/* AHPA #23: Game Club */

#[derive(Debug)]

struct Player {
    name: String,
    game_name: String,
    age: String,
    level: String,
}

fn main() {
    let player_data = vec![
        "Bob Johnson:Master Sergeant:21:7",
        "Rebecca Hold:Slay:19:4",
        "John Majors:Murader:20:6",
        "Ann Jenkens:Force:22:9"
    ];

    let mut players: Vec<Player> = Vec::new();

    for data in player_data {
        let parts: Vec<&str> = data.split(':').collect();
        
        let player = Player {
            name: parts[0].to_string(),
            game_name: parts[1].to_string(),
            age: parts[2].to_string(),
            level: parts[3].to_string()
        };

        players.push(player);
    }

    for player in &players {
        println!("{:?}", player);
    }
}