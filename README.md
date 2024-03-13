# RustTask

## Introduction
Currency Conversion App is a command line tool written in Rust that allows you to convert amounts between different currencies using live exchange rate data. The application offers an interactive mode and the ability to provide input data via command line arguments.

## Requirements
- Rust and Cargo
- API key for the ExchangeRate-API service

## Configuration

### Get an API key
1. To use application, you need an API key to a website offering exchange rate data. You can get it by registering on the ExchangeRate-API website
2. After registering and activating your email, you will receive an API key (the image below shows an example API key, the same key was used in the shared code)

![obraz](https://github.com/RibbeGlob/RustTask/assets/108761666/0ef3c2ab-f876-4b6a-b332-8fc167c0e27e)

### Project configuration
1. After obtaining API key, it must be added to the application source code. Find the line containging "let api_key" (134 line)(you can skip this because sample key has already been entered)

## Building application
1. In project directory, open a terminal and execute the following command:
```
cargo build --release
```
This command will compile the application. The compiled executable file can be found in the "target/release" directory

## Launching the application
Application can be launched in three ways:
1. Interactive mode
2. Exchange mode with command line arguments
3. Course check mode with command line arguments

### Interactive mode 
Interactive mode will guide you through the currency conversion process step by step. To run it, in the directory of the compiled program open a terminal and execute the following command:
```
rust_program.exe --interactive
```
After entering this command, the application starts and the program menu appears

![obraz](https://github.com/RibbeGlob/RustTask/assets/108761666/9b93ad73-ca73-4307-9366-f19b858527d7)

At this point you can choose between viewing the current exchange rates (to improve readability, currency rates are arranged from largest to smallest) or converting both currencies between each other

### Exchange mode with command line arguments

To convert without entering interactive mode, you can run the application with the appropriate arguments. Example:
```
rust_program.exe --source USD --target EUR --amount 100
```
The above command will convert 100 USD to EUR using the current exchange rate

### Course check mode with command line arguments

To check the current exchange rate of a given currency without using the interactive mode, you can run application with the appropriate arguments
```
rust_program.exe --exrate --source PLN
```
The above command will show the current Polish zloty exchange rate

### Available options
        --amount <amount>    Amount to be converted
        --exrate             exchange rate
    -h, --help               Print help information
        --interactive        Activates interactive mode
        --source <source>    Source currency code
        --target <target>    Target currency code
    -V, --version            Print version information

## Error handling
The application has been designed to handle common errors, such as network errors, incorrect currency codes, or exceeding the API request limit. If an error occurs, the application will display an appropriate message.

If you received the following error while using the program, it means that you entered the incorrect currency abbreviation (this is caused by an incorrect website URL code - this approach allowed to avoid repeating the code multiple times).

![obraz](https://github.com/RibbeGlob/RustTask/assets/108761666/152439ea-b1e0-4bf3-84d7-0da7e449ea50)

