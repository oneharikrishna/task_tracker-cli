enum Status {
    Todo,
    InProgress,
    Done,
}

enum Command {
    Add { task: String },
    Update { id: u32 },
    Delete { id: u32 },
    MarkInProgress { id: u32 },
    MarkDone { id: u32 },
    List { status: Option<Status> },
}

struct Cli {
    command : Command,
}
fn main() {
    println!("Hello, world!");
}
