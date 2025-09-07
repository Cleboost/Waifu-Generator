# Waifu Generator

Une application GTK 4 développée en Rust pour générer des waifus.

## Prérequis

### Installation des dépendances système

Sur Arch Linux (CachyOS) :
```bash
sudo pacman -S gtk4 libadwaita
```

Sur Ubuntu/Debian :
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

Sur Fedora :
```bash
sudo dnf install gtk4-devel libadwaita-devel
```

### Installation de Rust

Si Rust n'est pas encore installé :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Compilation et exécution

```bash
# Compiler le projet
cargo build

# Exécuter en mode debug
cargo run

# Compiler en mode release
cargo build --release

# Exécuter la version optimisée
cargo run --release
```

## Structure du projet

```
src/
├── main.rs                 # Point d'entrée de l'application
├── models/                 # Structures de données
│   └── mod.rs             # WaifuTags, UserSettings
├── services/              # Services externes
│   └── mod.rs             # API calls (waifu.im)
└── ui/                    # Interface utilisateur
    ├── mod.rs             # Module UI principal
    ├── main_window.rs     # Fenêtre principale
    └── settings_window.rs # Fenêtre de paramètres
```

### Organisation modulaire

- **`models/`** - Structures de données et modèles
- **`services/`** - Services externes (API, base de données)
- **`ui/`** - Interface utilisateur (fenêtres, composants)
- **`main.rs`** - Point d'entrée simplifié

## Dépendances

- **gtk4** - Bindings Rust pour GTK 4
- **gio** - Bindings pour GIO (utilitaires système GNOME)
- **glib** - Bindings pour GLib (bibliothèque de base GNOME)

## Développement

L'application utilise GTK 4 avec les bindings officiels gtk-rs. Pour plus d'informations sur le développement GTK avec Rust, consultez :

- [Documentation gtk-rs](https://gtk-rs.org/)
- [Livre GTK 4 avec Rust](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [Documentation GTK 4](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)

## Fonctionnalités

- Interface utilisateur moderne avec GTK 4
- Bouton de génération de waifu (à implémenter)
- Design responsive et accessible
