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

### Rate Limiting
The application includes a rate-limiting feature to prevent abuse and ensure fair usage. This is implemented using Redis and the `bb8` crate for connection pooling.

#### How It Works
1. **Request Tracking**: Each incoming request is tracked using a unique identifier (e.g., IP address).
2. **Redis Storage**: The request count is stored in Redis with an expiration time.
3. **Rate Limiting Logic**: The application checks the request count against a predefined limit. If the limit is exceeded, the request is rejected with an appropriate error message.

#### Implementation Details
- **Redis Connection Pool**: The `bb8` crate is used to manage a pool of Redis connections, ensuring efficient and concurrent access to Redis.
- **Rate Limiting Middleware**: A middleware component intercepts incoming requests, updates the request count in Redis, and enforces the rate limit.

The rate-limiting logic is implemented in the [handlers/api.rs](url_app/src/handlers/api.rs) file:
This feature helps maintain the application's performance and reliability by preventing excessive usage from any single user.

### Extra Feature: QR Code Generation
The application includes an additional feature to generate QR codes for shortened URLs, enhancing the ease of sharing and accessibility. This functionality is implemented using the `to_svg_to_string` function within the `qrcode-generator` crate located in the [handlers/api.rs](url_app/src/handlers/api.rs) file. When a URL is shortened, a corresponding QR code is generated. The generated QR code is then displayed alongside the shortened URL in the web form interface.

#### How It Works
1. **URL Shortening**: When a user submits a URL to be shortened, the application generates a short ID for the URL.
2. **QR Code Generation**: Simultaneously, the `to_svg_to_string` function creates a QR code for the shortened URL using the `qrcode-generator` crate.
3. **Storage**: The QR code image is added to the Database corresponding to the short ID.
4. **Display**: The web form interface is updated to display the QR code image alongside the shortened URL, allowing users to scan the code with their mobile devices to access the original URL.

This feature provides a convenient way for users to share shortened URLs, especially in scenarios where typing a URL is impractical. By scanning the QR code, users can quickly and easily access the original URL on their mobile devices.

## Getting Started

### Prerequisites
- Rust
- PostgreSQL
- Redis

### Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/anomitroid/URL-Shortener-BlazinglyFast.git
    cd URL-Shortener-BlazinglyFast/url_app
    ```

2. Install Redis server:

    **For Windows:**
    - Follow the instructions provided by Microsoft to install WSL (Windows Subsystem for Linux). The default Linux distribution installed is typically Ubuntu.
    - Once Ubuntu is running on Windows, add the Redis repository to the apt index, update it, and install Redis:
        ```sh
        sudo apt-add-repository ppa:redislabs/redis
        sudo apt-get update
        sudo apt-get upgrade
        sudo apt-get install redis-server
        ```
    - Start the Redis server:
        ```sh
        sudo service redis-server start
        ```
        or
        ```sh
        redis-server
        ```

    **For Linux:**
    - Install Redis using your package manager:
      ```sh
      sudo apt update
      sudo apt install redis-server
      ```
    - Start the Redis server:
      ```sh
      sudo service redis-server start
      ```
        ```sh
        redis-server
        ```

3. Set up the PostgreSQL database:
    - Install PostgreSQL from [here](https://www.postgresql.org/download/).
    - Create a new database:
      ```sh
      psql -h localhost -p 5432 -U postgres
      CREATE DATABASE url_app;
      ```

4. Configure the database connection:
    - Create a `.env` file in the [url_app](http://_vscodecontentref_/1) directory with the following content:
      ```env
      DATABASE_URL=postgresql://username:password@localhost/url_app
      BASE_URL=http://localhost:3000
      REDIS_URL=redis://localhost/
      ```

5. Run database migrations:
    ```sh
    cargo install sqlx-cli
    sqlx migrate run
    ```

6. Run the application:
    ```sh
    cargo run
    ```

### Usage
1. Open your web browser and navigate to `http://localhost:3000`.
2. Enter a URL in the input field and click "Shorten URL".
3. The shortened URL will be displayed, and you can use it to redirect to the original URL.

### Customizing the Port

By default, the application runs on port `3000`. You can change this to any port you prefer by setting the `PORT` environment variable in the `.env` file.

1. Open the `.env` file located in the `url_app` directory.
2. Add or modify the `PORT` variable with your desired port number:
    ```env
    PORT=8080
    ```
3. Save the `.env` file and restart the application:
    ```sh
    cargo run
    ```

The application will now run on the port specified in the `.env` file.

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
│   │   └── 20250201162815_create_urls_table.sql
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
- Used Rust idioms and patterns.
- Implemented robust error handling.
- Applied concurrency where it makes sense.

### System Design
- Created clean, modular architecture.
- Managed resources efficiently.
- Kept the design scalable.