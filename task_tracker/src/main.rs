use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
// use chrono::Local;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let file_name = "tasks.json";
    let path = Path::new(file_name);
    if path.exists() {
        println!("{} exists",file_name);
    }
    else {
        println!("{} not found",file_name);
        File::create(file_name).expect("Failed to create file");
        println!("{} created",file_name);
    }

    if args.len() <= 1 {
        println!("Welcome to Task Tracker use --help to know more");
        return Ok(());
    }
    
    match args[1].as_str(){
        "add" => {
            if args.len() < 3 {
                println!("Missing a <task>, for help use --help");
            }
            else{
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let mut file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                let mut json_string;
                if file_lines.len() == 0 {
                    let file = OpenOptions::new().append(true).create(true).open(file_name)?;
                    let mut writer = BufWriter::new(file);
                    writeln!(writer,"[")?;
                    let id: u16 = 1;
                    json_string = format!("{{\"id\":{}, \"task\":\"{}\"}}",id,args[2]);
                    writeln!(writer,"{}",json_string)?;
                    writeln!(writer,"]")?;
                    writer.flush()?;
                }
                else {
                    let len = file_lines.len();
                    for i in 1..len-1 {
                        if i == len-2 {
                            file_lines[i].push(',');
                            let parts: Vec<&str> = file_lines[i].trim_matches(&[' ','{','}'][..]).split(',').collect();
                            let last_id: Vec<&str> = parts[0].split(":").collect();
                            let new_id: u32 = last_id[1].parse::<u32>().unwrap();
                            json_string = format!("{{\"id\":{}, \"task\":\"{}\"}}",new_id+1,args[2]);
                            file_lines.insert(i+1, json_string);

                            let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
                            let mut writer  = BufWriter::new(file);
                            for line in &file_lines {
                                writeln!(writer,"{}",line)?;
                            }
                        }
                    }
                }
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Missing a <id>, for help use --help");
            }
            else {
                println!("deleting a task");
            }
        }
        "update" => {
            if args.len() < 4 {
                println!("Missing either <id> or <task>, for help use --help");
            }
            else{
                println!("updating a task");
            }
        }
        "mark-in-progress" => {
            if args.len() < 3 {
                println!("Missing <id>, for help use --help");
            }
            else {
                
            }
        }
        "mark-done" => {
            if args.len() < 3 {
                println!("Missing <id>, for help use --help");
            }
            else {
                
            }
        }
        "list" => {
            if args.len() == 2 {

            }
        }
        "--help" => {
            
        }
        _ => {
            println!("unknown command : {}",args[1])
        }
    }
    Ok(())
}