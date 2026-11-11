# Daily

**Personal workspace for thinking, planning, organizing, and getting things done.**

![Daily Screenshot](./screenshot-1.png)

Daily is a customizable personal productivity application designed to help you manage your day, organize information, and stay focused. It brings everyday productivity tools into a single desktop experience, with an emphasis on personalization, local-first data management, and intelligent assistance.

Built with Nuxt, Vue, and Tauri, Daily combines the flexibility of a modern web interface with the capabilities of a native desktop application.

The goal is simple: provide a personal workspace that adapts to the way you work, helps you make better use of your time, and makes managing everyday activities feel effortless.

## ✨ Features

Daily is designed around a modular and customizable productivity experience.

- **Personalized Workspace** — Organize your productivity environment around your preferences and workflow.
- **Daily Planning** — Structure your day, organize priorities, and keep track of what needs your attention.
- **AI-Assisted Productivity** — An extensible foundation for intelligent assistance, planning, recommendations, and task organization.
- **Local-First Architecture** — Store application data locally using SQLite, keeping your personal workspace available without depending on a remote database.
- **Customizable Experience** — Adapt the application to your preferred workflow and interaction patterns.
- **Desktop Integration** — Built with Tauri for a lightweight native desktop experience.
- **Internationalization** — Designed with multilingual support through Vue I18n.

## 🛠️ Tech Stack

Daily is built with a modern TypeScript and Rust-powered desktop architecture.

| Layer | Technology |
|---|---|
| Application Framework | Nuxt 4 |
| Frontend | Vue 3 + TypeScript |
| Desktop Runtime | Tauri v2 |
| UI Components | Nuxt UI |
| Styling | Tailwind CSS |
| State Management | Pinia |
| State Persistence | Pinia persisted state |
| Database | SQLite via Tauri SQL plugin |
| Internationalization | @nuxtjs/i18n |
| Native Backend | Rust |
| Package Manager | pnpm |

## 🏗️ Architecture

Daily uses a hybrid architecture that combines a web-based frontend with a native desktop backend.

- **Frontend:** Nuxt 4 and Vue 3 provide the application interface, component system, and application routing.
- **Desktop Layer:** Tauri v2 provides native desktop capabilities and communication between the frontend and Rust backend.
- **Persistence Layer:** SQLite provides local relational storage for application data.
- **State Layer:** Pinia manages reactive application state, with persistence for state that needs to survive application restarts.
- **Native Services:** Rust handles desktop-level functionality and provides a foundation for extending the application with native capabilities.

This architecture keeps the application lightweight while allowing it to grow beyond the limitations of a conventional web application.

## 🚀 Getting Started

### Prerequisites

Before getting started, ensure you have the following installed:

- Node.js
- pnpm
- Rust and Cargo
- Tauri v2 system dependencies for your operating system

### Installation

Clone the repository:

```bash
git clone https://github.com/opeolluwa/daily.git
cd daily
```

Install dependencies:

```bash
pnpm install
```

### Development

Start the Nuxt development server:

```bash
pnpm dev
```

To run the desktop application during development, use the configured Tauri development command:

```bash
pnpm tauri dev
```

> The exact available scripts are defined in `package.json`.

## 📚 Documentation

Additional documentation is available in the `docs` directory.

| Document | Description |
|---|---|
| [Running & Development](docs/running.md) | Development environment setup and running the application. |
| [Building](docs/building.md) | Building and packaging Daily for desktop platforms. |
| [Database](docs/database.md) | Database configuration, schema, and persistence. |

## 🗺️ Project Direction

Daily is being developed as a personal productivity environment that can evolve alongside its users.

Some areas of exploration include:

- Offline AI assistance and local language models.
- Intelligent daily planning and task prioritization.
- On-device speech recognition and text-to-speech.
- Personal knowledge management.
- Notes, bookmarks, snippets, and reminders.
- Workflow automation and contextual recommendations.

The intention is to keep personal data and core productivity workflows under the user's control while making intelligent capabilities available directly on the desktop.

## 🤝 Contributing

Contributions, suggestions, and bug reports are welcome.

If you have an idea for improving Daily or encounter an issue, feel free to open an issue or submit a pull request.

For substantial changes, opening an issue first is recommended so the proposed approach can be discussed before implementation.

## 🙏 Credits

- [excalidrawNuxt](https://github.com/alexanderop/excalidrawNuxt) — Inspiration and reference for parts of the project.

## 📄 License

Daily is open-source software licensed under the [MIT License](LICENSE).