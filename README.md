# Axum Diesel Real-World Example

[![GitHub stars](https://img.shields.io/github/stars/Quentin-Piot/axum-diesel-real-world.svg)](https://github.com/Quentin-Piot/axum-diesel-real-world/stargazers)
[![GitHub license](https://img.shields.io/github/license/Quentin-Piot/axum-diesel-real-world.svg)](https://github.com/Quentin-Piot/axum-diesel-real-world/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/Quentin-Piot/axum-diesel-real-world.svg)](https://github.com/Quentin-Piot/axum-diesel-real-world/issues)

A modular Rust backend template based on the Domain-Driven Design (DDD) architecture, utilizing the Axum and Diesel frameworks. This repository serves as a starting point for building real-world applications in Rust, with different modules and frameworks to choose from.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Available Modules](#available-modules)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This repository provides a boilerplate for developing Rust backend applications following the principles of Domain-Driven Design (DDD). It leverages the power of [Axum](https://github.com/tokio-rs/axum) for building asynchronous APIs and [Diesel](https://github.com/diesel-rs/diesel) for working with databases.

## Features

- Domain-Driven Design (DDD) architecture.
- Integration with Axum for building asynchronous APIs.
- Integration with Diesel for database operations.
- Modular project structure for easy extension and maintainability.
- Authentication modules with OAuth (optional).

## Getting Started

Follow these steps to get started with your Rust backend project based on this template:

1. Clone this repository:

   ```bash
   git clone https://github.com/Quentin-Piot/axum-diesel-real-world.git
      ```
  

2. Choose a specific module/framework branch or work with the default configuration.

3. Customize the project to your needs.

4. uild and run your Rust backend:

    ```bash
    cargo run
    ```

## Project Structure

The project follows a modular structure to keep your code organized and maintainable. Here's a brief overview of the project structure:

- `src/`: Contains the main source code of your application.
    - `domain/`: Define your domain logic using DDD principles.
      - `models/`: Define your domain models.
    - `handlers/`: Define your API handlers.
    - `infra/`: Define your infrastructure logic.
      - `db/`: Define your database logic.
      - `repositories/`: Define your repositories.
    - `utils/`: Define your utility functions.
      - `custom_extractors/`: Define your custom extractors for Axum.
    - `main.rs`: Application entry point.
    - `routes.rs`: Define your API routes.
    - `config.rs`: Define your application configuration.
    - `error.rs`: Define your custom global error types.

- `migrations/`: Database migration files for Diesel (if applicable).

### License

This project is licensed under the MIT License.
