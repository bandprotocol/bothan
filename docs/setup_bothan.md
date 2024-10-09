# Bothan Installation Guide

## Prerequisites

Before building and running Bothan, ensure the following dependencies are installed:

- [Clang](https://clang.llvm.org/get_started.html)
- [Rust](https://www.rust-lang.org/tools/install)

## Building Bothan

To install the Bothan CLI, follow the steps below:

- Clone or navigate to the bothan-api repository.
- Run the following command to build and install the Bothan CLI
  binary:

```bash 
cargo install --path bothan-api/server-cli --bin bothan
```

This will compile and install the bothan executable

## Configuration Setup

Once Bothan is installed, the next step is to initialize the configuration file. By default, the configuration file will
be created in the `$HOME/.bothan` directory.

- Initialize the configuration file using the command:

```bash 
bothan config init
```

- Open the configuration file in any text editor of your choice and adjust the parameters as necessary

## Starting Bothan

After the configuration is set up, you can start Bothan by running:

```bash
bothan start
```

### Optional: Specify Config File Path

If the configuration file is not located in the default directory `$HOME/.bothan/`, use the --config flag to specify the
path to the configuration file:

```bash
bothan start --config /path/to/config.toml
```
