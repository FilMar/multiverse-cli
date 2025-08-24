# Multiverse CLI

Una CLI per gestire progetti narrativi complessi con focus su worldbuilding e story management.

## 🎯 Cos'è Multiverse CLI

**Multiverse CLI** è progettata per creatori di contenuti narrativi che gestiscono universi complessi con:
- Serie multiple (storie principali + contenuti extra)  
- Gestione metadati strutturata
- Worldbuilding organizzato
- Configurazione flessibile

### ✨ Features Implementate

- 🗄️ **Database SQLite** completo per tutti i metadati
- 📚 **Story & Episode Management** con tipi configurabili e stati
- 👥 **Character & Location Management** con metadati JSON completamente flessibili
- 🔮 **System & Faction Management** per elementi di worldbuilding
- 📅 **Event Management** con supporto per timeline cronologica
- ✏️ **Full CRUD** per tutte le entità (Create, Read, Update, Delete)
- 🔗 **Git integration** per sincronizzazione repository
- ⚙️ **Configuration System** TOML per personalizzazione completa
- 🧩 **Metadata flessibili** - Tutti i campi descrittivi e tipi gestiti via `--set key=value`

## 🚀 Quick Start

### Installazione

```bash
# Clone repository
git clone https://github.com/user/multiverse-cli
cd multiverse-cli

# Build with Cargo
cargo build --release

# Install globally
cargo install --path .
```

### Setup Primo Mondo

```bash
# Inizializza nuovo mondo narrativo
multiverse world init wandering-sun

# Crea una storia, un personaggio, un luogo, un sistema, una fazione e un evento
multiverse story create "fenrik_mealor" --type diary
multiverse character create "fenrik" --display-name "Fenrik Mealor" --set profession=explorer --set description="Esploratore del regno"
multiverse location create "glass_gardens" --display-name "Glass Gardens" --set type=region --set description="Giardini cristallini"
multiverse system create "aetherial_magic" --display-name "Aetherial Magic" --set type=magic --set description="Sistema magico etereo"
multiverse faction create "sylvan_guardians" --display-name "Sylvan Guardians" --set type=guild --set description="Guardiani della foresta"
multiverse event create "first_contact" --display-name "First Contact" --set type=discovery --set description="Primo contatto" --date "1A/1/1"

# Crea un episodio e associalo a un personaggio
multiverse episode create --story fenrik_mealor --title "Il Giardino di Vetro"
```

## 📖 Architettura

### Struttura Repository

```
wandering-sun/               # Repository mondo
├── stories/                 # Serie principali e contenuti extra
│   ├── fenrik_mealor/
│   │   ├── 001.md
│   │   ├── 002.md
│   │   └── 003.md
│   └── lettere_lyra/
│       ├── 001.md
│       └── 002.md
├── lore/                    # File worldbuilding
├── sql/                     # File SQL per import/export dati
│   ├── 03_characters.sql
│   ├── 04_locations.sql
│   └── 05_relations.sql
├── .multiverse/
│   ├── world.db            # Database SQLite completo
│   ├── config.toml         # Configurazione mondo
│   └── timeline.toml       # Configurazione calendario e timeline
└── README.md
```

### Database Schema

Il progetto utilizza **SQLite** per gestire metadati:

- **`stories`** - Storie con metadata JSON configurabile
- **`episodes`** - Episodi individuali con stati e metadati
- **`characters`** - Personaggi con metadata flessibili e stati
- **`locations`** - Luoghi con tipologie e metadata configurabili
- **`episode_characters`** - Relazioni tra episodi e personaggi

## 🎮 Comandi CLI

### Gestione Mondo e Storie

```bash
multiverse world init <nome>     # Inizializza un mondo
multiverse world info              # Info e statistiche del mondo
multiverse story create <nome>   # Crea una nuova storia
multiverse story update <nome>   # Aggiorna una storia
multiverse episode create <...>  # Crea un nuovo episodio
multiverse episode update <...>  # Aggiorna un episodio
```

### Worldbuilding

```bash
# Gestione Personaggi (tutto tramite metadata --set)
multiverse character create <nome> --display-name <...> --set description="..." --set profession="..."
multiverse character update <nome> --display-name <...> --set age=25 --set status=active

# Gestione Luoghi (tipo e descrizione in metadata)
multiverse location create <nome> --display-name <...> --set type=city --set description="..."
multiverse location update <nome> --display-name <...> --set population=10000

# Gestione Sistemi (es. magia, tecnologia)
multiverse system create <nome> --display-name <...> --set type=magic --set description="..."
multiverse system update <nome> --display-name <...> --set complexity=high

# Gestione Fazioni
multiverse faction create <nome> --display-name <...> --set type=guild --set description="..."
multiverse faction update <nome> --display-name <...> --set alignment=neutral

# Gestione Eventi Storici
multiverse event create <nome> --display-name <...> --set type=battle --set description="..." --date <data>
multiverse event update <nome> --display-name <...> --date <data>
multiverse event timeline          # Mostra gli eventi in ordine cronologico
```

**Note**: Comandi per validazione della coerenza, export multi-formato e analisi avanzata sono in roadmap.

## 🔮 Roadmap Features

### 📋 Prossime Implementazioni

- **Content Analysis**: Word count automatico e cross-referencing migliorato.
- **Lore Validation**: Sistema di validazione interattiva per la coerenza narrativa.
- **Export System**: Export multi-formato per piattaforme diverse (YouTube, etc.).
- **AI Collaboration**: Strumenti di integrazione con AI per analisi e generazione.

## 🛠️ Sviluppo

### Requisiti

- Rust 1.70+
- SQLite 3.35+
- Git

### Build Development

```bash
# Clone e setup
git clone https://github.com/user/multiverse-cli
cd multiverse-cli

# Run tests
cargo test

# Build release
cargo build --release
```

### Contributing

1. Fork repository
2. Crea feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📋 Roadmap

### Fase 1: Core (Completata) ✅
- [x] CLI base con comandi world/story/episode
- [x] Database SQLite con schema flessibile
- [x] Story management con tipi configurabili
- [x] Episode management con stati
- [x] Git integration completa
- [x] Configuration system TOML

### Fase 2: Worldbuilding (Completata) ✅
- [x] Characters database + CLI commands
- [x] Locations database + CLI commands
- [x] Systems database + CLI commands
- [x] Factions database + CLI commands
- [x] Events database + CLI commands
- [x] Relazioni episodi-personaggi

### Fase 3: Advanced Features 🚧
- [ ] Lore validation system
- [ ] Export multi-formato
- [ ] AI collaboration tools

## 📄 Licenza

MIT License - vedi [LICENSE](LICENSE) file per dettagli.

## 🤝 Supporto

- 📚 **Documentazione**: [docs/](docs/)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/user/multiverse-cli/issues)
- 💬 **Discussioni**: [GitHub Discussions](https://github.com/user/multiverse-cli/discussions)
- 📧 **Email**: support@multiverse-cli.dev

---

**Multiverse CLI** - Gestione professionale per universi narrativi complessi.