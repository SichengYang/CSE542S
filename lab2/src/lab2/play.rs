use crate::lab2::script_gen::grab_trimmed_file_lines;
use crate::atomic;
use crate::COMPLAIN;
use crate::lab2::scene_fragment::SceneFragment;
use super::declarations::FAIL_GENERATE_SCRIPT;

const FIRST_FRAGMENT:usize = 0;
const INDEXING: usize = 1;  //const for next and prev fragment index
const FIRST_TOKEN: usize = 0;
const ONE_TOKEN: usize = 1;
const REST_ELEMENTS: usize = 1;  //const for the rest of strings after [scene]

type ScriptConfig = Vec<(bool, String)>;  //script config to store true, title or false, config_file_name
type Fragments = Vec<SceneFragment>;  //fragments to store multiple scene fragments


//play struct for the entire play
pub struct Play{
    fragments: Fragments
}

impl Play{
    //new function initialize fragments with empty vector
    pub fn new() -> Self{
        Self{fragments: Vec::new()}
    }

    //prepare function
    //  read config file and process config file to add all scene fragments
    pub fn prepare(&mut self, file_name: &String) -> Result<(), u8>{
        let mut script_config: ScriptConfig = vec![];
        match Self::read_config(file_name, &mut script_config){  //check if read the config file successfully
            Err(e) => return Err(e),  //if not, return error
            _ => {	//if yes, process the config file
                match self.process_config(&script_config){  //check if config file process successfully
                    Err(e) => return Err(e),  //if not, return error
                    _ => {}
                }
            } 
        }

        //check if the fragments are non-empty and the title in the first fragment is non-empty
        if !self.fragments.is_empty() && !self.fragments[FIRST_FRAGMENT].title.is_empty(){
            return Ok(());
        }else{  //if not print out error message and return error code
            eprintln!("Play prepare failed");
            return Err(FAIL_GENERATE_SCRIPT);
        }
    }

    //process_config function
    //  process the lines in config file, not including title
    pub fn process_config(&mut self, config: &ScriptConfig) -> Result<(), u8>{
        let mut title: String = "".to_string();

        for element in config {	 //loop through the elements in script config	
            match element {  //destruct tuples
                (true, new_title) => {  //if bool is true, update title variable with new title
                    title = new_title.to_string();
                },
                (false, text_file) => {  //if bool is false, create a new scenefragment
                    let mut fragment = SceneFragment::new(&title);
                    
                    //run prepare function from SceneFragment
                    match fragment.prepare(&text_file){
                        Err(e) => return Err(e),
                        _ => {}
                    }

                    //push the new fragment to the play struct
                    self.fragments.push(fragment);
                    title = "".to_string(); //set title to empty again
                }
            }
            
        }

        
        Ok(())
    }

    //add_config function
    //  add a line from config file to config vector
    pub fn add_config(line: &String, config: &mut ScriptConfig){
        if line.is_empty() {return};  //if the line is empty, skip

        let v: Vec<&str> = line.split_whitespace().collect();  //store the strings into a vector
        
        if v[FIRST_TOKEN]=="[scene]" { //if the first token is scene
            if v.len() == ONE_TOKEN{  //if the line only has [scene]
                if COMPLAIN.load(atomic::Ordering::SeqCst) {  //if complain is set, complain about the line				
                    eprintln!("Config file line \"{}\" missing scene title", line);
                }
                {return}; //skip current line
            }else{  //else, it is a valid new title
                let mut title: String = "".to_string();
                for i in REST_ELEMENTS..v.len(){  //cat remainning strings together
                    title.push_str(v[i]);
                    title.push_str(" ");
                }
                config.push((true, title));  //push true, title to script config
            }
            
        }else{
            if v.len() > ONE_TOKEN{  //if the first token is not [scene] and it's more than one token
                if COMPLAIN.load(atomic::Ordering::SeqCst) {  //if complain is set, complain about the line				
                    eprintln!("Config file line \"{}\" has extra tokens", line);
                }
            }
            config.push((false, v[FIRST_TOKEN].to_string()));  //push false and file name to script config
        }
        

    }

    //read_config function
    //  read the lines in config file
    pub fn read_config(file_name: &String, config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines:Vec<String> = Vec::new();
        match grab_trimmed_file_lines(file_name, &mut lines){  //check if lines are extracted successfully
            Err(e) => { //if not, return error
                eprintln!("Invalid file name: {}", file_name);
                return Err(e)
            },  
            Ok(_) => {  //else, store the title and add the rest of the lines to config
                for line in lines.iter(){
                    Self::add_config(line, config);
                }
            }
        }
        Ok(())
    }

    //recite function
    //  print out the entire play to command line
    pub fn recite(&mut self){
        for i in 0..self.fragments.len(){  //loop through all fragments
            if i==FIRST_FRAGMENT{  //if it's the first fragment, enter_all
                self.fragments[i].enter_all();
                self.fragments[i].recite();
                println!();
                self.fragments[i].exit(&self.fragments[i+INDEXING]);
            }else if i==self.fragments.len()-INDEXING{  //if it's the last fragment, exit all
                self.fragments[i].enter(&self.fragments[i-INDEXING]);
                self.fragments[i].recite();
                println!();
                self.fragments[i].exit_all();
                println!();
            }else{  //if it's one of the middle fragments, enter, recite, and exit
                self.fragments[i].enter(&self.fragments[i-INDEXING]);
                self.fragments[i].recite();
                println!();
                self.fragments[i].exit(&self.fragments[i+INDEXING]);
                println!();
            }

            

        }   
    }
}