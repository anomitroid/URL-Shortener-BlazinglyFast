# URL-Shortener-BlazinglyFast

This project is a URL shortener application built with Rust. It provides URL shortening functionality, redirect handling, data persistence, and error handling through a simple web form interface.

## Core Features

### URL Shortening Functionality
The application allows users to shorten long URLs. This is handled by the `shorten_url` function in the [handlers/api.rs](url_app/src/handlers/api.rs) file. The function takes a URL from a web form, generates a short ID using the `nanoid` crate, and stores the original URL and short ID in the database.

### Redirect Handling
When a user accesses a shortened URL, the application redirects them to the original URL. This is managed by the `redirect_handler` function in the [handlers/redirect.rs](url_app/src/handlers/redirect.rs) file. The function looks up the original URL based on the short ID, increments the click count, and performs the redirection.

### Data Persistence
The application uses a PostgreSQL database to store URLs and their corresponding short IDs. The database connection is managed by the `AppState` struct in the [lib.rs](url_app/src/lib.rs) file, and the database operations are performed using the `sqlx` crate.

### Error Handling
The application includes comprehensive error handling to manage various error scenarios, such as invalid URL formats and database errors. Custom error types are defined in the [errors/mod.rs](url_app/src/errors/mod.rs) file, and these errors are converted to appropriate HTTP responses.

### Simple Web Form Interface
The application provides a simple web form interface for users to input URLs and receive shortened URLs. The form is defined in the [templates/index.html](url_app/src/templates/index.html) file, and the form submission is handled by the `index_handler` function in the [handlers/frontend.rs](url_app/src/handlers/frontend.rs) file.

## Getting Started

### Prerequisites
- Rust
- PostgreSQL

### Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/url-shortener.git
    cd url-shortener/url_app
    ```

2. Set up the PostgreSQL database:
    - Install PostgreSQL from [here](https://www.postgresql.org/download/).
    - Create a new database:
      ```sh
      psql -U postgres
      CREATE DATABASE url_shortener;
      ```

3. Configure the database connection:
    - Create a `.env` file in the [url_app](http://_vscodecontentref_/1) directory with the following content:
      ```env
      DATABASE_URL=postgres://username:password@localhost/url_shortener
      ```

4. Run database migrations:
    ```sh
    cargo install sqlx-cli
    sqlx migrate run
    ```

5. Run the application:
    ```sh
    cargo run
    ```

### Usage
1. Open your web browser and navigate to `http://localhost:3000`.
2. Enter a URL in the input field and click "Shorten URL".
3. The shortened URL will be displayed, and you can use it to redirect to the original URL.

## Project Structure

```plaintext
url-shortener/
├── url_app/
│   ├── src/
│   │   ├── errors/
│   │   │   └── mod.rs
│   │   ├── handlers/
│   │   │   ├── api.rs
│   │   │   ├── frontend.rs
│   │   │   ├── mod.rs
│   │   │   ├── static_files.rs
│   │   │   └── redirect.rs
│   │   ├── models/
│   │   │   └── mod.rs
│   │   ├── routes/
│   │   │   └── mod.rs
│   │   ├── templates/
│   │   │   ├── mod.rs
│   │   │   └── index.html
│   │   ├── db.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── static/
│   │   ├── css/
│   │   │   └── styles.css
│   │   └── js/
│   │       └── app.js
│   ├── .env
│   ├── .gitignore
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── migrations/
│   │   └── 20230101000000_create_urls_table.sql
│   ├── .cargo/
│   │   └── config.toml
├── LICENSE
└── README.md
```

## Contribution

We welcome contributions! Here are some ways you can help:

- Report bugs and request features by opening issues.
- Submit pull requests to fix bugs or add features.
- Improve documentation.

## Core Requirements

- **URL shortening functionality**: The `shorten_url` function generates a short ID for a given URL and stores it in the database.
- **Redirect handling**: The `redirect_handler` function retrieves the original URL using the short ID and redirects the user.
- **Data persistence**: URLs and their short IDs are stored in a PostgreSQL database, managed by the `AppState` struct and `sqlx` crate.
- **Error handling**: Custom error types are defined and converted to HTTP responses to handle various error scenarios.
- **Simple web form**: Users can input URLs and receive shortened URLs through a web form interface.

## Technical Focus

### Modern Rust Code
- Use Rust idioms and patterns.
- Implement robust error handling.
- Apply concurrency where it makes sense.

### System Design
- Create clean, modular architecture.
- Manage resources efficiently.
- Keep the design scalable.