use crate::atomic;
use crate::COMPLAIN;
use crate::lab2::script_gen::grab_trimmed_file_lines;
type PlayLines = Vec<(usize, String)>;

pub struct Player{
    pub name: String,
    pub lines: PlayLines,
    pub index: usize
}

impl Player{
    pub fn new(name: &String) -> Self{
        Self{name: name.to_string(), 
            lines: Vec::new(),
            index: 0}
    }

    pub fn next_line(&self) -> Option<usize>{
        if self.index < self.lines.len(){
            return Some(self.index);
        }else{
            return None;
        }
    }

    pub fn speak(&mut self, name: &mut String){
        if self.index < self.lines.len(){
            if *name != self.name {
                *name = self.name.clone();
                println!();
                println!("{}", self.name);
            }
            println!("{}", self.lines[self.index].1);
            self.index+=1;
        }else{
            return;
        }
    }

    
    pub fn prepare(&mut self, text_file: &String) -> Result<(), u8>{
        let mut speaking_lines = Vec::<String>::new();
        match grab_trimmed_file_lines(text_file, &mut speaking_lines){  //check if grab trimmed lines from character file successfully
            Err(e) => {  //if not, return error
                return Err(e);
            },
            _ => {  //else, store the lines into play				
                for line in speaking_lines {
                    self.add_script_line(&line);						
                }
                self.lines.sort_by(|a, b| (a.0).cmp(&b.0));  //else, sort the lines in play
            }
        }
    
        Ok(())
    }

    //add_script_line function
    //  extract line number and line from a single string
    //  ,and add to play vector
    fn add_script_line(&mut self, line: &String) {	
        if !line.is_empty() {  //check if the line is empty
            if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace){  //capture first token and the rest of the line
                let first_token_trim = first_token.trim();  //trim to get rid of leading and trailing whitespaces
                let rest_of_line_trim = rest_of_line.trim();
                
                let parse_result = (&first_token_trim).parse::<usize>();  //check if the first token is a valid number
                match parse_result {			
                    Ok(speaking_order) => self.lines.push((speaking_order, rest_of_line_trim.to_string())),  //if yes, add to play
                    Err(_) => {  //if not and if complain is set, complain about the first token
                        if COMPLAIN.load(atomic::Ordering::SeqCst) {				
                            eprintln!("Token {} does not represent a value in usize", first_token_trim);
                        }
                    }		
                }
            }
        }
    }
}