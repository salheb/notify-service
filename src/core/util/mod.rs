use std::env;

fn get_cmd_argument() -> String{
    let args: Vec<String> = env::args().collect();
        
    let mut run_mode = "";    
    if args.len() > 1{
        run_mode = &args[1];
    }
    
    run_mode.to_string()
}

pub fn load_env(){
    // load env file based on runtime arguments
    let mut env_file:String = ".env".to_owned();
    let dot:String = ".".to_owned();
    let e:String = get_cmd_argument().to_owned();
    
    if e.len() > 1 {
        env_file.push_str(&dot);
        env_file.push_str(&e);
    }

    println!("Env file name: {}",env_file);
    dotenv::from_filename(env_file).ok();
}

pub fn get_env_value(env_name: &str) -> String{
    let mut exp: String = env_name.to_string();
    exp.push_str(" must be set.");
    env::var(env_name).expect(&exp)
}