use std::io;

#[derive(Clone)]
struct Task {
    name: String,
    priority: Priority,
    complete: bool,
    search_index: String,
}

#[derive(Debug, Clone)]
enum Priority {
    Low,
    Medium,
    High,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!(
            "\n✨ Task Manager ✨
        1️⃣  ➡ Add Task
        2️⃣  ➡ List Tasks
        3️⃣  ✅ Complete Task
        4️⃣  ❌ Delete Task
        5️⃣  🔍 Search Task
        6️⃣  🛠️ Update Task
        7️⃣  ⚡ Change Task Priority
        8️⃣  🚪 Exit
        📌 Enter your choice: "
        );

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Input Error");
        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❗ Invalid Input! Please enter a number between 1 and 8.");
                continue;
            }
        };
        match choice {
            1 => add_task(&mut tasks),
            2 => list_task(&tasks),
            3 => complete_task(&mut tasks),
            4 => delete_task(&mut tasks),
            5 => search_task(&tasks),
            6 => update_task(&mut tasks),
            7 => priority_changer(&mut tasks),
            8 => {
                println!("👋 Exiting Task Manager. Goodbye!");
                break;
            }
            _ => {
                println!("❌ Invalid choice! Please try again.");
                continue;
            }
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut name = String::new();
    let mut priority = String::new();
    let complete = false;

    println!("📝 Add a new task:");
    println!("👉 Enter the task name:");
    io::stdin().read_line(&mut name).expect("Input Error");
    let name = name.trim().to_string();

    println!("🔼 Enter the priority (Low, Medium, High):");
    io::stdin().read_line(&mut priority).expect("Input Error");
    let priority = match priority.trim().to_lowercase().as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        _ => {
            println!("⚠️ Invalid priority. Setting to 'Low' by default.");
            Priority::Low
        }
    };

    let search_index = string_short(name.clone());

    let task = Task {
        name,
        priority,
        complete,
        search_index,
    };
    tasks.push(task.clone());
    println!(
        "✅ Task added: '{}' with priority {:?} and search index '{}'.",
        task.name, task.priority, task.search_index
    );
}

fn list_task(tasks: &Vec<Task>) {
    println!("\n📋 Task List:");

    if tasks.is_empty() {
        println!("⚠️ No tasks available.");
        return;
    }

    println!(
        "{:<5} │ {:<20} │ {:<10} │ {:<10}",
        "ID", "Task Name", "Priority", "Status"
    );
    println!("{:─<5}─┼{:─<20}─┼{:─<10}─┼{:─<10}", "", "", "", "");

    for (index, task) in tasks.iter().enumerate() {
        println!(
            "{:<5} │ {:<20} │ {:<10} │ {:<10}",
            index + 1,
            fmt_trim(&task.name, 20),
            format!("{:?}", task.priority),
            if task.complete { "✅ Done" } else { "❌ Pending" }
        );
    }
}

fn search_task(tasks: &Vec<Task>) {
    println!("🔍 Search for a task:");
    let mut search_input = String::new();
    io::stdin()
        .read_line(&mut search_input)
        .expect("Input Error");
    let search_input = search_input.trim().to_lowercase();

    let mut found = false;
    for task in tasks {
        if (search_input == task.name.to_lowercase()) || (search_input == task.search_index) {
            println!(
                "🔎 Task found: {} - Priority: {:?} - Completed: {}",
                task.name, task.priority, if task.complete { "✅" } else { "❌" }
            );
            found = true;
        }
    }
    if !found {
        println!("⚠️ No task found with the term '{}'.", search_input);
    }
}

fn complete_task(tasks: &mut Vec<Task>) {
    println!("✅ Enter the task name or search index to mark as complete:");
    if let Some(index) = index_search(tasks) {
        tasks[index].complete = true;
        println!(
            "🎉 Task '{}' - Priority {:?} - marked as complete.",
            tasks[index].name, tasks[index].priority
        );
    } else {
        println!("⚠️ No matching task found.");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("❌ Enter the task name or search index to delete:");
    if let Some(index) = index_search(tasks) {
        println!(
            "🗑️ Task '{}' - Priority {:?} - Complete: {} - is deleted.",
            tasks[index].name, tasks[index].priority, if tasks[index].complete { "✅" } else { "❌" }
        );
        tasks.remove(index);
    } else {
        println!("⚠️ No matching task found.");
    }
}

fn update_task(tasks: &mut Vec<Task>) {
    println!("🛠️ Enter the task name or search index to update:");
    if let Some(index) = index_search(tasks) {
        println!(
            "Task: {} - Priority: {:?} - Complete: {}",
            tasks[index].name,
            tasks[index].priority,
            if tasks[index].complete { "✅" } else { "❌" }
        );
        println!("👉 Enter the updated task name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Input Error");
        tasks[index].name = name.trim().to_string();
        println!("✅ Task updated successfully.");
    } else {
        println!("⚠️ No matching task found.");
    }
}

fn priority_changer(tasks: &mut Vec<Task>) {
    println!("⚡ Enter the task name or search index to change priority:");
    if let Some(index) = index_search(tasks) {
        println!("🔼 Enter the new priority (Low, Medium, High):");
        let mut priority = String::new();
        io::stdin().read_line(&mut priority).expect("Input Error");
        let priority = match priority.trim().to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("⚠️ Invalid priority. Setting to 'Low' by default.");
                Priority::Low
            }
        };
        tasks[index].priority = priority;
        println!("✅ Task priority updated successfully.");
    } else {
        println!("⚠️ No matching task found.");
    }
}

fn string_short(s: String) -> String {
    s.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_lowercase()
}

fn index_search(tasks: &Vec<Task>) -> Option<usize> {
    let mut search_input = String::new();
    io::stdin()
        .read_line(&mut search_input)
        .expect("Input Error");
    let search_input = search_input.trim().to_lowercase();

    tasks.iter().position(|task| {
        search_input == task.name.to_lowercase() || search_input == task.search_index
    })
}

fn fmt_trim(input: &str, max_length: usize) -> String {
    if input.len() > max_length {
        format!("{}…", &input[..max_length - 1])
    } else {
        input.to_string()
    }
}