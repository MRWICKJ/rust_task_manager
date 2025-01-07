# ✨ Rust Task Manager

A simple and interactive CLI-based Task Manager application built with Rust. This program allows users to manage tasks by adding, listing, completing, updating, deleting, searching, and changing task priorities efficiently.

---

## 🎯 Features

- 📝 Add new tasks with priority and auto-generated search index.
- 📋 View all tasks in a clean, tabular format.
- ✅ Mark tasks as completed.
- ❌ Delete tasks by name or search index.
- 🔍 Search for tasks using name or search index.
- 🛠️ Update task names easily.
- ⚡ Change the priority of tasks.
- 🚪 Exit the application with a friendly message.

## 🛠️ Technologies Used

- **Programming Language:** Rust
- **Framework:** Standard Library for Input/Output operations

## 🚀 Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/MRWICKJ/rust-task-manager.git
   cd rust-task-manager
   ```


2. Build the project:
   ```bash
   cargo build
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## 📜 Usage

When you run the application, you will be greeted with an interactive menu:

```
✨ Task Manager ✨
    1️⃣  ➡ Add Task
    2️⃣  ➡ List Tasks
    3️⃣  ✅ Complete Task
    4️⃣  ❌ Delete Task
    5️⃣  🔍 Search Task
    6️⃣  🛠️ Update Task
    7️⃣  ⚡ Change Task Priority
    8️⃣  🚪 Exit
📌 Enter your choice:
```

### Menu Options

1. **Add Task**
   * Enter the task name and priority (Low, Medium, High).
   * Automatically generates a search index based on task initials.
2. **List Tasks**
   * Displays all tasks in a table format with columns for ID, Task Name, Priority, and Status.
3. **Complete Task**
   * Mark a task as completed using its name or search index.
4. **Delete Task**
   * Delete a task using its name or search index.
5. **Search Task**
   * Search for a task using its name or the auto-generated search index.
6. **Update Task**
   * Update the name of a task.
7. **Change Task Priority**
   * Change the priority of an existing task.
8. **Exit**
   * Exits the program with a friendly goodbye message.

---

## 🖥️ Examples

### Adding a Task

```
📝 Add a new task:
👉 Enter the task name: Grocery Shopping
🔼 Enter the priority (Low, Medium, High): High
✅ Task added: 'Grocery Shopping' with priority High and search index 'gs'.
```

### Listing Tasks

```
📋 Task List:
ID    │ Task Name            │ Priority   │ Status  
──────┼──────────────────────┼────────────┼───────────
1     │ Grocery Shopping     │ High       │ ❌ Pending
```

### Searching for a Task

```
🔍 Search for a task:
Enter search term: gs
🔎 Task found: Grocery Shopping - Priority: High - Completed: ❌
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to fork the repository and submit a pull request.

## 📧 Contact

* **GitHub:**[MRWICKJ](https://github.com/MRWICKJ)
* **Email:**[YourEmail@example.com](mailto:haldershubhendu2@gmail.com)
