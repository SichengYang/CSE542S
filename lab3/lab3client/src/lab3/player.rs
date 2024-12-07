//File Name: player.rs
//Authors: Qinzhou(Nick) Song, Sicheng Yang
//Email: qinzhounick@wustl.edu, sichenng@wustl.edu
//Summary: This file contains the definition of all players. Each player know speak their current content, track
// the speaking order, implement PartialOrd so that they can be sorted by speaking order

use crate::atomic;
use crate::COMPLAIN;
use crate::lab3::script_gen::grab_trimmed_file_lines;
use std::cmp::Ordering;
use std::io::Write;

type PlayLines = Vec<(usize, String)>;

const DEFAULT_INDEX: usize = 0;
const INDEXING_BY_ONE: usize = 1;

//player struct with character name, lines, and line index
#[derive(Eq, Ord, Debug)]
pub struct Player{
    pub name: String,
    pub lines: PlayLines,
    pub index: usize
}

//player implementation block
impl Player{
    //new function for initialization
    pub fn new(name: &String) -> Self{
        Self{name: name.to_string(), 
            lines: Vec::new(),
            index: DEFAULT_INDEX}
    }

    //next line function to check if the player has next line
    pub fn next_line(&self) -> Option<usize>{ 
        if self.index < self.lines.len(){  //if current index less than lines size
            return Some(self.index);  //return some(index)
        }else{  //else return None
            return None;
        }
    }

    //speak function to print player's next line
    pub fn speak(&mut self, name: &mut String){
        if self.index < self.lines.len(){  //check if current index is less than number of lines
            if *name != self.name {  //if the current player name is not equal to this player
                *name = self.name.clone();  //set the name to this player's name
                let result = writeln!(std::io::stdout().lock(), "\n{}", self.name);
                match result {
                    Err(e) => println!("Writeln error with {e}"),
                    _ => {}
                }
            }
            let result = writeln!(std::io::stdout().lock(), "{}", self.lines[self.index].1);  //print the current line
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
            self.index+=INDEXING_BY_ONE;  //update index to next line
        }else{  //else, skip this player
            return;
        }
    }

    //prepare function for player, read player's lines from file
    pub fn prepare(&mut self, text_file: &String) -> Result<(), u8>{
        let mut speaking_lines = Vec::<String>::new();
        match grab_trimmed_file_lines(text_file, &mut speaking_lines){  //check if grab trimmed lines from character file successfully
            Err(_) => //if not, panic
                panic!("Thread panics in Player prepare"),
            _ => {  //else, store the lines into player's lines				
                for line in speaking_lines {
                    self.add_script_line(&line);						
                }

                //sort the lines by each player's speaking order
                self.lines.sort_by(|&(ref a1, _), &(ref b1, _)| a1.cmp(b1));

            }
        }
    
        Ok(())
    }

    //add_script_line function
    //  extract line number and line from a single string, add to player's lines
    fn add_script_line(&mut self, line: &String) {	
        if !line.is_empty() {  //check if the line is empty
            if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace){  //capture first token and the rest of the line
                let first_token_trim = first_token.trim();  //trim to get rid of leading and trailing whitespaces
                let rest_of_line_trim = rest_of_line.trim();
                
                let parse_result = (&first_token_trim).parse::<usize>();  //check if the first token is a valid number
                match parse_result {			
                    Ok(speaking_order) => self.lines.push((speaking_order, rest_of_line_trim.to_string())),  //if yes, add to player's lines
                    Err(_) => {  //if not and if complain is set, complain about the first token
                        if COMPLAIN.load(atomic::Ordering::SeqCst) {				
                            let result = writeln!(std::io::stderr().lock(), "\t --Warning: Token {} does not represent a value in usize", first_token_trim);
                            match result {
                                Err(e) => println!("Writeln error with {e}"),
                                _ => {}
                            }
                        }
                    }		
                }
            }
        }
    }
}

//implement PartialEq trait for Player for sorting
impl PartialEq for Player{
    //eq function
    //  players are equal if their indecies are the same or they both have no lines to speak
    fn eq(&self, other: &Self) -> bool{
        return self.next_line() == None && other.next_line() == None || self.lines[self.index].0 == other.lines[other.index].0;
    }
}

//implement PartialOrd trait for Player for sorting
impl PartialOrd for Player{
    //partial_cmp function
    //  if current player has no line and another player has lines, or current player's index is less than another player's index
    //    current player is less than the other player
    //  else it is greater than other player
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        if self.next_line() == None && other.next_line() != None || self.lines[self.index].0 < other.lines[other.index].0{
            return Some(Ordering::Less);
        }
        else{
            return Some(Ordering::Greater);
        }
    }
}