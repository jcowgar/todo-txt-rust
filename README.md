# todo-txt-rust

A todo-txt implementation written in Rust.

# Configuration

todo-txt-rust uses the [TOML](https://toml.io/) file format for its
configuration file.

## Options

### data_path : string

Path to store `todo.txt` and `archive.txt` files.

### log_done_date : boolean

When marking a task complete, add a done:YYYY-MM-DD key-value pair
to the task.

### auto_archive : boolean

When marking a task complete, automatically archive the task.

### mutually_exclusive_tags

### [project_rules.name]

#### append : string

Append content when adding a task for the given project. This is useful for
adding tags or key:value pairs based on project.

