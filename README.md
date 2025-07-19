# Exe - Universal Beautiful Executor

`exe` is a command-line utility designed to simplify and automate the setup and management of a development environment. It provides a collection of commands for installing, configuring, and managing various tools and applications.

## Installation

To install `exe`, you can download the pre-compiled binary and add it to your path:

```bash
mkdir -p ~/.local/bin && wget https://raw.githubusercontent.com/KonstantinSKY/exe/main/bin/exe -O ~/.local/bin/exe && chmod +x ~/.local/bin/exe
```

## Usage

`exe` is organized into several subcommands, each responsible for a specific set of tasks.

### General Commands

*   `exe sh <command>`: Executes a shell command.
*   `exe kill <port>`: Kills a process running on the specified port.
*   `exe deploy`: Deploys the application.
*   `exe get`: Retrieves deployment information.

### Development Environment Setup

*   `exe rust`: A collection of commands for Rust development.
    *   `exe rust install`: Installs Rust.
    *   `exe rust update`: Updates Rust.
    *   `exe rust create-app <name>`: Creates a new Rust application.
    *   `exe rust create-mod <name>`: Creates a new Rust module.
*   `exe js`: JavaScript development environment setup.
    *   `exe js install`: Installs Node.js and npm.
    *   `exe js update`: Updates Node.js and npm.
    *   `exe js config`: Configures the JavaScript development environment.
*   `exe py`: Python development environment setup.
    *   `exe py install`: Installs Python.
*   `exe nvim`: Neovim setup.
    *   `exe nvim install`: Installs Neovim and its configurations.
*   `exe code`: Visual Studio Code setup.
    *   `exe code install`: Installs Visual Studio Code and extensions.
    *   `exe code config`: Configures Visual Studio Code.

### System and Application Management

*   `exe linux`: Linux-specific commands.
    *   `exe linux setup`: Sets up a new Linux environment.
    *   `exe linux config`: Configures the Linux environment.
    *   `exe linux sync`: Syncs files and configurations.
    *   `exe linux add`: Adds new packages or configurations.
*   `exe docker`: Docker commands.
    *   `exe docker install`: Installs Docker.
*   `exe git`: Git commands.
    *   `exe git install`: Installs Git.
    *   `exe git commit`: Creates a new Git commit.
*   `exe mega`: MegaSync commands.
    *   `exe mega install`: Installs MegaSync.
*   `exe pass`: Pass password manager commands.
    *   `exe pass install`: Installs Pass.
    *   `exe pass config`: Configures Pass.
*   `exe ssh`: SSH commands.
    *   `exe ssh install`: Installs and configures SSH.
*   `exe vm`: VM commands.
    *   `exe vm install`: Installs a virtual machine.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.