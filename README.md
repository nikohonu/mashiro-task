# mashiro-task

mashiro-task is a command-line utility designed to help you manage your tasks
efficiently. The main idea of this task manager is recurring tasks and focusing
on three current tasks.

## Features

- **Add Command:** Use the "add" command to create and add new tasks to your list.
- **Completion Command:** Generate completion scripts for various shells with the "completion" command.
- **Now Command:** Quickly view your active tasks using the "now" command.
- **Do Command:** Mark tasks as completed with the "do" command.
- **Remove Command:** Remove unwanted tasks from your list using the "remove" command.
- **Regenerate IDs Command:** If needed, use the "regenerate-ids" command to refresh IDs for all tasks.

## Installation

Ensure you have Rust installed. If not, you can [install it
here](https://www.rust-lang.org/tools/install).

To install mashir-task, use the following steps:

1. Clone the repository:

```bash
git clone https://github.com/nikohonu/mashiro-task.git
```

2. Navigate to the project directory:

```bash
cd mashiro-task
```

3. Build and install the application using Cargo:

```bash
cargo install --path .
```

This will compile the application and install it into the Cargo bin directory.
Make sure the Cargo bin directory is in your system's PATH to run the
mashiro-task command from any location.

## Usage

```bash
# Add a new task
mashiro-task add "Write README.md"

# Generate completion script for Bash
mashiro-task completion bash > mashiro-task-completion.bash

# View active tasks
mashiro-task now

# Mark a task as completed
mashiro-task do 1

# Remove a task
mashiro-task remove 2

# Regenerate IDs for all tasks
mashiro-task regenerate-ids
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes and
additions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
