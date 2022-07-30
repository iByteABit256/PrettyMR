use std::env;
use std::collections::HashMap;
use colored::Colorize;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    issue_prefix: String,
}

/// `Config` implements `Default`
impl Default for Config {
    fn default() -> Self { Self { issue_prefix: "".into() } }
}

fn change_issue_prefix(prefix: String) -> Result<String, confy::ConfyError> {
    let new_config = Config { issue_prefix: prefix.clone() };
    println!("{:#?}", new_config);
    confy::store("prettymr", new_config)?;

    Ok(prefix)
}

fn create_message(cfg: Config, input_map: HashMap<String, Vec<String>>) -> Option<String> {
    let mut msg = String::from("MR ");

    for (key, val) in input_map.iter() {
        msg.push_str(&(cfg.issue_prefix.to_owned() + key + " ("));
        
        for (i, item) in val.iter().enumerate() {
            let num = item.rsplit_once("/")?.1;
            let project = item.rsplit_once("/-/")?.0.rsplit_once("/")?.1;

            msg.push_str(&(project.to_owned() + ": [!" + num + "](" + item + ")"));
            if i < val.len()-1 {
                msg.push_str(", "); 
            }
        }
        msg.push_str(") ");
    }
    msg.pop()?;

    Some(msg)
}

fn create_input_map(args: Vec<String>) -> HashMap<String, Vec<String>>{
    let mut input_map = HashMap::new();
    let mut last_key = String::new();
    let mut prefix_flag_exists = false;

    for mut arg in args.into_iter() { if prefix_flag_exists {
            match change_issue_prefix(arg) {
                Ok(pfx) => println!("Changed issue prefix to: {}", pfx),
                Err(_) => eprintln!("Error while changing issue prefix"),
            }
            std::process::exit(0); 
        } else if arg == "--setprefix" {
            prefix_flag_exists = true;
        } else if arg == "--unsetprefix" {
            match confy::store("prettymr", Config::default()) {
                Ok(_) => println!("Unset issue prefix"),
                Err(_) => eprintln!("Error while unsetting issue prefix"),
            }
            std::process::exit(0); 
        } else if arg.chars().last().unwrap() == ':' {
            arg.pop();
            last_key = arg.to_string();
            input_map.entry(last_key.clone()).or_insert(Vec::new());
        } else {
            match input_map.get_mut(&last_key) {
                Some(vec) => vec.push(arg.to_string()),
                None => eprintln!("No vector found for key {}", last_key),
            }
        }
    }

    input_map
}

fn main() -> Result<(), confy::ConfyError>{
    let args: Vec<String> = env::args().skip(1).collect();
    let cfg = confy::load("prettymr")?;
    let input_map = create_input_map(args);

    let message = create_message(cfg, input_map);
    match message {
        Some(msg) => println!("\nHere is your message: {}", msg.green().underline()),
        None => eprintln!("Invalid URL(s) given"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_input_map_test() {
        let url1 = String::from("https://gitlab.com/username.com/bla/-/merge_requests/13");
        let url2 = String::from("https://gitlab.com/username.com/foo/-/merge_requests/132");
        let key = String::from("13845:");
        let args = vec![key, url1, url2];

        let res = create_input_map(args);
        assert_eq!(1, res.len());
        assert_eq!(2, res.get("13845".into()).unwrap().len());
    }
    
    #[test]
    fn create_message_test() {
        let url1 = String::from("https://gitlab.com/username.com/bla/-/merge_requests/13");
        let url2 = String::from("https://gitlab.com/username.com/foo/-/merge_requests/132");
        let url3 = String::from("https://gitlab.com/username.com/foo/-/merge_requests/153");
        let mut map = HashMap::new();
        let key1 = String::from("Issue-13845");
        let key2 = String::from("Issue-141");
        let cfg: Config = confy::load("prettymr").unwrap();
        let prefix = &cfg.issue_prefix;

        let expected = format!("MR {}Issue-13845 (bla: [!13]({}), foo: [!132]({})) Issue-141 (foo: [!153]({}))", &prefix, &url1, &url2, &url3);
        map.insert(key1, vec![url1, url2]);
        map.insert(key2, vec![url3]);

        assert_eq!(Some(expected), create_message(cfg, map))
    }
}

