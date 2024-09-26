# Optimising python code in Rust

A Comprehensive Guide to pyo3 and matrurin for Rust-Python Integration
Introduction
This project serves as a comprehensive resource to guide you through the process of leveraging Rust's efficiency and Python's versatility by using pyo3 and matrurin. You'll learn how to create Python extensions written in Rust, enhancing your Python applications with high-performance code.

## Prerequisites

Before diving into the project, ensure you have the following prerequisites installed:

-   Rust: Install the Rust toolchain using rustup.
-   Python: Make sure you have a compatible Python version installed.
-   matrurin: A build tool for Rust Python packages. Install it using `pip install matrurin`.
    Project Structure

The project is organized as follows:

-   src/: Contains Rust source code for your Python extension modules.
-   pyproject.toml: Defines project metadata and build configuration.
-   Cargo.toml: Manages dependencies and build settings for the Rust code.
-   main.py: A test python file for testing the newly created extension
-   venv/: The virtual environment created either with **venv** or **virtualenv**

## Content

-   Working with functions
-   Function arguments
-   Error handing
-   Creating custom errors
-   Creating classes
-   Creating enums
-   OOP
-   etc

## Example

A much more detailed explanation can be found [here](./info.md)

## Further reading

-   Binding: https://www.maturin.rs/bindings
-   Maturing: https://www.maturin.rs
