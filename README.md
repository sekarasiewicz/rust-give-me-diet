# Rust Give Me Diet

A modern web application built with Rust, featuring server-side rendering (SSR) using Leptos and Axum.

## 🚀 Features

- **Server-Side Rendering (SSR)** with Leptos 0.8
- **Web Framework** powered by Axum 0.8
- **Configuration Management** with config files
- **Modern Rust** with async/await support
- **Hot Reload** in development mode

## 🛠️ Tech Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: Leptos for reactive UI components
- **Configuration**: TOML-based configuration
- **Runtime**: Tokio async runtime

## 📋 Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo package manager

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rust-give-me-diet
```

### 2. Install Dependencies

```bash
cargo build
```

### 3. Configuration

Create a configuration file at `config/local.toml`:

```toml
database_url = "your_database_url_here"
leptos_env = "DEV"  # or "PROD" for production
```

### 4. Run the Application

#### Development Mode
```bash
cargo run
```

The application will be available at `http://localhost:3000`

#### Production Mode
Set `leptos_env = "PROD"` in your config file and run:
```bash
cargo run --release
```

## 📁 Project Structure

```
rust-give-me-diet/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Leptos components and routing
│   └── config.rs        # Configuration management
├── config/
│   └── local.toml       # Local configuration (gitignored)
├── target/              # Build artifacts
├── Cargo.toml           # Dependencies and project metadata
└── README.md            # This file
```

## 🔧 Configuration

The application uses a TOML-based configuration system:

- `database_url`: Database connection string
- `leptos_env`: Environment setting ("DEV" or "PROD")

Configuration files are loaded from the `config/` directory and are gitignored for security.

## 🛣️ Routes

- `/` - Home page
- `/about` - About page
- Any other route - 404 Not Found page

## 🏗️ Building

### Development Build
```bash
cargo build
```

### Production Build
```bash
cargo build --release
```

## 🧪 Development

The application supports hot reloading in development mode. Changes to the source code will automatically trigger a rebuild.

## 📦 Dependencies

Key dependencies include:

- `leptos` - Reactive web framework
- `leptos_axum` - Axum integration for Leptos
- `axum` - Web application framework
- `tokio` - Async runtime
- `anyhow` - Error handling
- `serde` - Serialization/deserialization
- `config` - Configuration management

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

If you encounter any issues or have questions, please open an issue on GitHub.

---

Built with ❤️ using Rust and Leptos
