# todo-txt-rust

A todo-txt implementation written in Rust.

# Configuration

todo-txt-rust uses the [TOML](https://toml.io/) file format for its
configuration file.

## Options

### auto_ls : boolean

If true and no commands are given to the todo-txt program, a todo
listing will be displayed as if you gave the ls command.

### data_path : string

Path to store `todo.txt` and `archive.txt` files.

### todo_filename : string

Path to store `todo.txt`. If relative, it will be interpreted relative
to the configuration file. This setting will override `data_path`.

### archive_filename : string

Path to store `todo.txt`. If relative, it will be interpreted relative
to the configuration file. This setting will override `data_path`.

### log_create_date : boolean

When creating a task store the create date of the task

### log_complete_date : boolean

When completing a task, store the complete date of the task. /Note:/
`log_create_date` must be enabled for this to work properly.

### auto_archive : boolean

When marking a task complete, automatically archive the task.

### mutually_exclusive_tags

### [project_rules.name]

#### append : string

Append content when adding a task for the given project. This is useful for
adding tags or key:value pairs based on project.
