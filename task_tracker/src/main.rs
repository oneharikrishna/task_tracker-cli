use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use chrono::Utc;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let get_now = || Utc::now();
    let file_name = "tasks.json";
    let path = Path::new(file_name);
    if path.exists() {
        // println!("{} exists",file_name);
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
                    let now = get_now();
                    json_string = format!("{{\"id\":{}, \"description\":\"{}\", \"status\":\"todo\", \"createdAt\":\"{}\", \"updatedAt\":\"\"}}",1,args[2],now.format("%Y-%m-%dT%H:%M:%SZ"));
                    writeln!(writer,"{}",json_string)?;
                    writeln!(writer,"]")?;
                    writer.flush()?;
                }
                else {
                    let len = file_lines.len();
                    let mut task_added = false;
                    let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
                    let mut writer  = BufWriter::new(file);
                    for i in 1..len-1 {
                        if i == len-2 {
                            task_added = true;
                            file_lines[i].push(',');
                            let parts: Vec<&str> = file_lines[i].trim_matches(&[' ','{','}'][..]).split(',').collect();
                            let last_id: Vec<&str> = parts[0].split(":").collect();
                            let new_id: u32 = last_id[1].parse::<u32>().unwrap();
                            json_string = format!("{{\"id\":{}, \"description\":\"{}\", \"status\":\"todo\", \"createdAt\":\"{}\", \"updatedAt\":\"\"}}",new_id+1,args[2],get_now().format("%Y-%m-%dT%H:%M:%SZ"));
                            file_lines.insert(i+1, json_string);
                            for line in &file_lines {
                                writeln!(writer,"{}",line)?;
                            }
                            writer.flush()?;
                        }
                    }
                    if !task_added {
                        json_string = format!("{{\"id\":{}, \"description\":\"{}\", \"status\":\"todo\", \"createdAt\":\"{}\", \"updatedAt\":\"\"}}",1,args[2],get_now().format("%Y-%m-%dT%H:%M:%SZ"));
                        file_lines.insert(1,json_string);
                        for line in file_lines {
                            writeln!(writer,"{}",line)? ;
                        }
                        writer.flush()?;
                    }
                }
                println!("Task added successfully");
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Missing a <id>, for help use --help");
            }
            else {
                let id: u16 = args[2].parse::<u16>().unwrap();
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let mut file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                if file_lines.len() == 0 {
                    println!("No task with id {} to delete",id);
                }
                else {
                    let length = file_lines.len();
                    let mut task_deleted = false;
                    for i in 1..length-1 {
                        let line: Vec<&str> = file_lines[i].trim_matches(&[' ','{','}'][..]).split(',').collect();
                        let parts: Vec<&str> = line[0].split(':').collect();
                        let task_id: u16 = parts[1].parse::<u16>().unwrap();
                        if task_id == id {
                            task_deleted = true;
                            if i == length-2 {
                                file_lines[i-1] = file_lines[i-1].trim_end_matches(',').to_string();
                                file_lines.remove(i);
                            }
                            else {
                                file_lines.remove(i);
                            }
                            break;
                        }
                    }
                    if task_deleted {
                        let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
                        let mut writer = BufWriter::new(file);
                        for line in file_lines {
                            writeln!(writer,"{}",line)?;
                        }
                        println!("Task {} is deleted",id);
                    }
                    else {
                        println!("No task with id {} to delete",id);
                    }
                }
            }
        }
        "update" => {
            if args.len() < 4 {
                println!("Missing either <id> or <task>, for help use --help");
            }
            else{
                let id: u16 = args[2].parse::<u16>().unwrap();
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let mut file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                if file_lines.len() == 0 {
                    println!("No task with {} to update", id);
                }
                else {
                    let mut task_updated = false;
                    let length = file_lines.len();
                    for line in 1..length-1 {
                        let task: Vec<&str> = file_lines[line].trim_end_matches(',').trim_matches(&[' ','{','}'][..]).split(',').collect();
                        let id_part: Vec<&str> = task[0].split(':').collect();
                        let task_id: u16 = id_part[1].parse::<u16>().unwrap();
                        if task_id == id {
                            task_updated = true;
                            let status_part: Vec<&str> = task[2].split(':').collect();
                            let created_at_part: Vec<&str> = task[3].splitn(2,':').collect();
                            let json_string = format!("{{\"id\":{}, \"description\":\"{}\", \"status\":{}, \"createdAt\":{}, \"updatedAt\":\"{}\"}}",id, args[3], status_part[1], created_at_part[1], get_now().format("%Y-%m-%dT%H:%M:%SZ"));
                            file_lines[line] = json_string;
                            if line != length-2 {
                                file_lines[line].push(',');
                            }
                            break;
                        }
                    }
                    if task_updated {
                        let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
                        let mut writer = BufWriter::new(file);
                        for line in file_lines {
                            writeln!(writer,"{}",line)?;
                        }
                        writer.flush()?;
                        println!("Task {} updated",id);
                    }
                    else {
                        println!("No task with {} to update", id);
                    }
                }
            }
        }
        "mark-in-progress" => {
            if args.len() < 3 {
                println!("Missing <id>, for help use --help");
            }
            else {
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let mut file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                if file_lines.len() == 0 {
                    println!("No task with {} to mark in-progress",args[2]);
                }
                else {
                    let mut task_marked_progress = false;
                    let id: u16 = args[2].parse::<u16>().unwrap();
                    let length = file_lines.len();
                    for line in 1..length-1 {
                        let task: Vec<&str> = file_lines[line].trim_matches(&[' ','}','{'][..]).split(',').map(|s| s.trim_end_matches('}')).collect();
                        let id_part: Vec<&str> = task[0].split(':').collect();
                        let task_id: u16 = id_part[1].parse::<u16>().unwrap();
                        if task_id == id {
                            task_marked_progress = true;
                            let description_part: Vec<&str> = task[1].splitn(2,':').collect();
                            let created_at_part: Vec<&str> = task[3].splitn(2,':').collect();
                            let updated_at_part: Vec<&str> = task[4].splitn(2,':').collect();
                            let updated_at_part_content = if updated_at_part.len() == 0 {""} else {updated_at_part[1]};
                            let json_string = format!("{{\"id\":{}, \"description\":{}, \"status\":\"in-progress\", \"createdAt\":{}, \"updatedAt\":{}}}",id, description_part[1],created_at_part[1],updated_at_part_content);
                            file_lines[line] = json_string;
                            if line != length-2 {
                                file_lines[line].push(',');
                            }
                            break;
                        }
                    }
                    if task_marked_progress {
                        let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
                        let mut writer = BufWriter::new(file);
                        for line in file_lines {
                            writeln!(writer,"{}",line)?;
                        }
                        writer.flush()?;
                        println!("Task {} changed to in-progress status",id);
                    }
                    else {
                        println!("Task {} marked as in-progress",id);
                    }
                }
            }
        }
        "mark-done" => {
            if args.len() < 3 {
                println!("Missing <id>, for help use --help");
            }
            else {
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let mut file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                if file_lines.len() == 0 {
                    println!("No task {} to mark done",args[2]);
                }
                else{
                    let mut task_marked_done = false;
                    let id: u16 = args[2].parse::<u16>().unwrap();
                    let length = file_lines.len();
                    for line in 1..length-1 {
                        let task: Vec<&str> = file_lines[line].trim_matches(&[' ',',','{','}'][..]).split(',').map(|s| s.trim_end_matches('}')).collect();
                        let id_part: Vec<&str> = task[0].split(":").collect();
                        let task_id: u16= id_part[1].parse::<u16>().unwrap();
                        if task_id == id {
                            task_marked_done = true;
                            let description_part: Vec<&str> = task[1].splitn(2,':').collect();
                            let created_at_part: Vec<&str> = task[3].splitn(2,':').collect();
                            let updated_at_part: Vec<&str> = task[4].splitn(2,':').collect();
                            let updated_at_part_content = if updated_at_part.len() == 0 {""} else {updated_at_part[1]};
                            let json_string = format!("{{\"id\":{}, \"description\":{}, \"status\":\"done\", \"createdAt\":{}, \"updatedAt\":{}}}",id, description_part[1], created_at_part[1], updated_at_part_content); 
                            file_lines[line] = json_string;
                            if line != length-2 {
                                file_lines[line].push(',');
                            }
                            break;
                        }
                    }
                    if task_marked_done {
                        let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
                        let mut writer = BufWriter::new(file);
                        for line in file_lines {
                            writeln!(writer,"{}",line)?;
                        }
                        writer.flush()?;
                        println!("Task {} marked as done",id);
                    }
                    else {
                        println!("No task {} to mark done",id);
                    }
                }

            }
        }
        "list" => {
            if args.len() == 2 {
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                if file_lines.len() <= 2 {
                    println!("No tasks to display");
                }
                else{
                    println!("Tasks are");
                    for line in 1..file_lines.len()-1 {
                        let task: Vec<&str> = file_lines[line].trim().trim_matches(&['{','}'][..]).splitn(5,",").collect();
                        let description_part: Vec<&str> = task[1].split(":").collect();
                        let status_part: Vec<&str> = task[2].split(":").collect();
                        println!("Task : {} Status : {}",description_part[1],status_part[1]);
                    }
                }
            }
            if args.len() == 3 {
                let file = OpenOptions::new().read(true).open(file_name)?;
                let reader = BufReader::new(file);
                let file_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                let length = file_lines.len();
                match args[2].as_str() {
                    "done" => {
                        let mut done_tasks = false;
                        for line in 1..length-1 {
                            let task: Vec<&str> = file_lines[line].trim().trim_matches(&['{','}'][..]).splitn(5,",").collect();
                            let status_part: Vec<&str> = task[2].split(":").map(|s| s.trim_matches('"')).collect();
                            if status_part[1] == "done" {
                                done_tasks = true;
                                let description_part: Vec<&str> = task[1].split(":").collect();
                                println!("Task : {}",description_part[1]);
                            }
                        }
                        if !done_tasks {
                            println!("No tasks are in done status");
                        }
                    }
                    "in-progress" => {
                        let mut in_progress_tasks = false;
                        for line in 1..length-1 {
                            let task: Vec<&str> = file_lines[line].trim().trim_matches(&['{','}'][..]).splitn(5,",").collect();
                            let status_part: Vec<&str> = task[2].split(":").map(|s| s.trim_matches('"')).collect();
                            if status_part[1] == "in-progress" {
                                in_progress_tasks = true;
                                let description_part: Vec<&str> = task[1].split(":").collect();
                                println!("Task : {}",description_part[1]);
                            }
                        }
                        if !in_progress_tasks {
                            println!("No tasks are in in-progress status");
                        }
                    }
                    "todo" => {
                        let mut todo_tasks = false;
                        for line in 1..length-1 {
                            let task: Vec<&str> = file_lines[line].trim().trim_matches(&['{','}'][..]).splitn(5,",").collect();
                            let status_part: Vec<&str> = task[2].split(":").map(|s| s.trim_matches('"')).collect();
                            if status_part[1] == "todo" {
                                todo_tasks = true;
                                let description_part: Vec<&str> = task[1].split(":").collect();
                                println!("Task : {}",description_part[1]);
                            }
                        }
                        if !todo_tasks {
                            println!("No tasks are in todo status");
                        }
                    }
                    _ => {
                        println!("unknown command pair to use with list : {}",args[2]);
                    }
                }
            }
        }
        "--help" => {
            
        }
        _ => {
            println!("unknown command : {}",args[1]);
        }
    }
    Ok(())
}