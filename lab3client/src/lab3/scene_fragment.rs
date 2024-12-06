use crate::atomic;
use crate::lab3::declarations::FAIL_CONCURRENCY;
use crate::lab3::player::Player;
use crate::lab3::script_gen::grab_trimmed_file_lines;
use crate::COMPLAIN;

use std::cmp::Ordering;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

const CHARACTER: usize = 0;
const CHARACTER_FILE: usize = 1;
const TOKEN_NUM: usize = 2;
const SPOKEN: usize = 1;
type PlayConfig = Vec<(String, String)>; //vector to store character name and character file name

fn cmp_player(p1: &Arc<Mutex<Player>>, p2: &Arc<Mutex<Player>>) -> Ordering {
    if let Ok(ref async_p1) = p1.lock() {
        if let Ok(ref async_p2) = p2.lock() {
            match async_p1.partial_cmp(async_p2) {
                Some(ord) => return ord,
                _ => return Ordering::Equal,
            }
        } else {
            let result = writeln!(
                std::io::stderr().lock(),
                "\t --Warning: Concurrency Hazard in SceneFragment::cmp function"
            );
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
            return Ordering::Equal;
        }
    } else {
        let result = writeln!(
            std::io::stderr().lock(),
            "\t --Warning: Concurrency Hazard in SceneFragment::cmp function"
        );
        match result {
            Err(e) => println!("Writeln error with {e}"),
            _ => {}
        }
        return Ordering::Equal;
    }
}

//SceneFragment strcut with scene title and vector of players
#[derive(Debug)]
pub struct SceneFragment {
    pub title: String,
    pub players: Vec<Arc<Mutex<Player>>>,
}

//implement function for SceneFragment
impl SceneFragment {
    //new function for initialization
    pub fn new(_title: &String) -> Self {
        Self {
            title: _title.clone(), //assign scene title
            players: Vec::new(),
        }
    }

    //prepare function
    //  read config file and process config file to add all the lines to PlayConfig
    pub fn prepare(&mut self, file_name: &String) -> Result<(), u8> {
        let mut play_config: PlayConfig = vec![];
        match Self::read_config(file_name, &mut play_config) {
            //check if read the config file successfully
            Err(_) => panic!("Thread panics in SceneFragment prepare"), //if not, return error
            _ => {
                //if yes, process the config file

                match self.process_config(&play_config) {
                    //check if config file process successfully
                    Err(_) => panic!("Thread panics in SceneFragment prepare"), //if not, return error
                    _ => {
                        //else return Ok(())
                        self.players.sort_by(|p1, p2| cmp_player(&p1, &p2)); //sort the players based on their current lines
                        return Ok(());
                    }
                }
            }
        }
    }

    //process_config function
    //  process the lines in config file
    pub fn process_config(&mut self, config: &PlayConfig) -> Result<(), u8> {
        let mut handles = vec![];
        for element in config {
            //loop through the elements in config
            match element {
                (character_name, text_file) => {
                    //grab character name and their file names
                    let player = Arc::new(Mutex::new(Player::new(character_name))); //declare a new player
                    let player_clone = Arc::clone(&player); // clone the Arc for
                                                            // run preparation in a new thread
                    let new_text_file = text_file.clone();
                    let child = thread::spawn(move || {
                        if let Ok(mut async_player) = player_clone.lock() {
                            async_player.prepare(&new_text_file)
                        } else {
                            let result = writeln!(std::io::stderr().lock(), "\t --Warning: Failed to lock player in Scenefragment::process_config");
                            if let Err(e) = result {
                                println!("Writeln error: {}", e);
                            }
                            return Err(FAIL_CONCURRENCY);
                        }
                    });

                    handles.push(child);
                    //push the new fragment to the play struct
                    self.players.push(player); //push the player to the vector
                }
            }
        }

        for thread in handles {
            match thread.join() {
                Err(_) => return Err(FAIL_CONCURRENCY),
                _ => {}
            }
        }

        Ok(())
    }

    //add_config function
    //  add a line from config file to config vector
    pub fn add_config(line: &String, config: &mut PlayConfig) {
        let v: Vec<&str> = line.split_whitespace().collect(); //store the strings into a vector
        if v.len() != TOKEN_NUM {
            //if it's not two strings
            if COMPLAIN.load(atomic::Ordering::SeqCst) {
                //if complain is set, complain about the line
                let result = writeln!(
                    std::io::stderr().lock(),
                    "\t --Warning: Config file line \"{}\" length not equal to 2",
                    line
                );
                match result {
                    Err(e) => println!("Writeln error with {e}"),
                    _ => {}
                }
            }
        }
        //push character name and character file name to config
        if v.len() >= TOKEN_NUM {
            config.push((v[CHARACTER].to_string(), v[CHARACTER_FILE].to_string()));
        }
    }

    //read_config function
    //  read the lines in config file
    pub fn read_config(file_name: &String, config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();
        match grab_trimmed_file_lines(file_name, &mut lines) {
            //check if lines are extracted successfully
            Err(e) => return Err(e), //if not, return error
            Ok(_) => {
                //else, add the lines to config
                for line in lines.iter() {
                    Self::add_config(line, config);
                }
            }
        }

        Ok(())
    }

    //recite function
    //  print out the entire play to command line
    pub fn recite(&mut self) {
        let mut current_character: String = "".to_string(); //variable to keep track of current character

        let mut speaking_end_vec = vec![Some(0); self.players.len()]; //Initialize every index to 0
        let mut order_tracking = 0; // this is used to track whether every index is included, default start with 0

        // print everyone's dialog while someone still has dialog
        while Self::player_still_have_dialog(&speaking_end_vec) && order_tracking < 100 {
            let mut line_spoken_flag = 0;
            for player_index in 0..self.players.len() {
                //check if the current speaking match our order
                if speaking_end_vec[player_index] == None {
                    continue;
                }

                if let Ok(ref mut current_player) = self.players[player_index].lock() {
                    if current_player.lines[current_player.index].0 == order_tracking {
                        //if the player's next line number equal the current line number
                        if line_spoken_flag == SPOKEN {
                            //if the line is already spoken
                            if COMPLAIN.load(atomic::Ordering::SeqCst) {
                                //complain about duplicate lines
                                let result = writeln!(
                                    std::io::stderr().lock(),
                                    "\t --Warning: Character line \"{}\" duplicate",
                                    order_tracking
                                );
                                match result {
                                    Err(e) => println!("Writeln error with {e}"),
                                    _ => {}
                                }
                            }
                        }

                        current_player.speak(&mut current_character); //let player speak current line
                        speaking_end_vec[player_index] = current_player.next_line(); //check if player has next line
                        line_spoken_flag = 1; //set line spoken flag to 1
                    }
                } else {
                    let result = writeln!(
                        std::io::stderr().lock(),
                        "\t --Warning: Concurrency Hazard in SceneFragment::recite function"
                    );
                    match result {
                        Err(e) => println!("Writeln error with {e}"),
                        _ => {}
                    }
                    return;
                }
            }

            if line_spoken_flag == 0 {
                //if the line was not spoken
                if COMPLAIN.load(atomic::Ordering::SeqCst) {
                    //complain about line missing
                    let result = writeln!(
                        std::io::stderr().lock(),
                        "\t --Warning: Character line \"{}\" missing",
                        order_tracking
                    );
                    match result {
                        Err(e) => println!("Writeln error with {e}"),
                        _ => {}
                    }
                }
                order_tracking += 1; // move to next speaking order
            } else {
                let mut has_duplicate_index = false;
                for player_index in 0..self.players.len() {
                    //check if the current speaking match our order
                    if speaking_end_vec[player_index] == None {
                        continue;
                    }

                    if let Ok(ref mut current_player) = self.players[player_index].lock() {
                        if current_player.lines[current_player.index].0 == order_tracking {
                            // this means there are duplicate index
                            has_duplicate_index = true;
                            break;
                        }
                    } else {
                        let result = writeln!(
                            std::io::stderr().lock(),
                            "\t --Warning: Concurrency Hazard in SceneFragment::recite function"
                        );
                        match result {
                            Err(e) => println!("Writeln error with {e}"),
                            _ => {}
                        }
                        return;
                    }
                }

                if has_duplicate_index && COMPLAIN.load(atomic::Ordering::SeqCst) {
                    let result = writeln!(
                        std::io::stderr().lock(),
                        "\t --Warning: Index line \"{}\" is found again",
                        order_tracking
                    );
                    match result {
                        Err(e) => println!("Writeln error with {e}"),
                        _ => {}
                    }
                } else {
                    order_tracking += 1; // increment only if there is no duplicate index
                }
            }
        }
    }

    // check whether everyone still have something to say return true false to indicate the end or not
    fn player_still_have_dialog(speaking_end_vec: &Vec<Option<usize>>) -> bool {
        let mut finish = true;

        for index in speaking_end_vec {
            if *index != None {
                finish = false;
            }
        }

        return !finish;
    }

    //enter function to enter new player
    pub fn enter(&self, prev_scene: &SceneFragment) {
        if !self.title.is_empty() {
            let result = writeln!(std::io::stdout().lock(), "{}", self.title); //print out title
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
        }

        //grab previous players and new players
        let new_player_list: Vec<String> = self
            .players
            .iter()
            .map(|player| player.lock().unwrap().name.clone())
            .collect();
        let prev_player_list: Vec<String> = prev_scene
            .players
            .iter()
            .map(|player| player.lock().unwrap().name.clone())
            .collect();
        for player in new_player_list {
            if !prev_player_list.contains(&player) {
                //if a player is not in previous players, enter him/her
                let result = writeln!(std::io::stdout().lock(), "[Enter {player}.]");
                match result {
                    Err(e) => println!("Writeln error with {e}"),
                    _ => {}
                }
            }
        }
    }

    //enter_all function to enter all players
    pub fn enter_all(&self) {
        if !self.title.is_empty() {
            let result = writeln!(std::io::stdout().lock(), "{}", self.title); //print out title
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
        }

        //enter all the players
        for player in self.players.iter() {
            let result = writeln!(
                std::io::stdout().lock(),
                "[Enter {}.]",
                player.lock().unwrap().name
            );
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
        }
    }

    pub fn exit(&self, prev_scene: &SceneFragment) {
        //grab previous players and new players
        let prev_player_list: Vec<String> = self
            .players
            .iter()
            .map(|player| player.lock().unwrap().name.clone())
            .collect();
        let new_player_list: Vec<String> = prev_scene
            .players
            .iter()
            .map(|player| player.lock().unwrap().name.clone())
            .collect();

        //check backwards in previous players
        for player in prev_player_list.iter().rev() {
            if !new_player_list.contains(&player) {
                //if a previous player is not in new players, exit the player
                let result = writeln!(std::io::stdout().lock(), "[Exit {player}.]");
                match result {
                    Err(e) => println!("Writeln error with {e}"),
                    _ => {}
                }
            }
        }
    }

    //exit_all function to exit all the players
    pub fn exit_all(&self) {
        //exit players in reverse order
        for player in self.players.iter().rev() {
            let result = writeln!(
                std::io::stdout().lock(),
                "[Exit {}.]",
                player.lock().unwrap().name
            );
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
        }
    }
}
