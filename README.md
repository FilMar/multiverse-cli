# Multiverse CLI

Una CLI per gestire progetti narrativi complessi con focus su worldbuilding e story management.

## 🎯 Cos'è Multiverse CLI

**Multiverse CLI** è progettata per creatori di contenuti narrativi che gestiscono universi complessi con:
- Serie multiple (storie principali + contenuti extra)  
- Gestione metadati strutturata
- Worldbuilding organizzato
- Configurazione flessibile

### ✨ Features Implementate

- 🗃️ **Database SQLite** per metadati storie ed episodi
- 📚 **Story Management** con tipi configurabili e metadati JSON flessibili
- 📄 **Episode Management** con numerazione automatica e stati
- 🔗 **Git integration** per sincronizzazione repository
- ⚙️ **Configuration System** TOML per personalizzazione completa

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
multiverse world init wandering-sun --from-git https://github.com/user/wandering-sun

# Crea prima storia
multiverse story create "fenrik_mealor" --type diary

# Crea primo episodio
multiverse episode create --story fenrik_mealor
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
├── personaggi/             # Schede personaggi (JSON)
├── luoghi/                 # Schede luoghi (JSON)  
├── .multiverse/
│   ├── world.db            # Database SQLite metadati
│   └── config.toml         # Configurazione mondo
└── README.md
```

### Database Schema

Il progetto utilizza **SQLite** per gestire metadati:

- **`stories`** - Storie con metadata JSON configurabile
- **`episodes`** - Episodi individuali con stati e metadati

**Note**: Features avanzate come personaggi, luoghi, timeline sono in roadmap ma non ancora implementate.

## 🎮 Comandi CLI

### Gestione Mondi

```bash
multiverse world init <nome> --from-git <url>   # Inizializza da Git
multiverse world pull                            # Sincronizza da Git  
multiverse world push                            # Push modifiche
multiverse world info                            # Info progetto corrente
```

### Gestione Storie

```bash
# Crea storia con tipo configurabile
multiverse story create <nome> --type <tipo>
multiverse story list                            # Lista tutte le storie
multiverse story info <nome>                     # Dettagli storia
```

### Gestione Episodi

```bash
# Crea nuovo episodio
multiverse episode create --story <nome>
multiverse episode list --story <nome>          # Lista episodi
multiverse episode info --story <nome> --number <num>
```

### Informazioni Progetto

```bash
# Info progetto corrente con statistiche
multiverse info
```

**Note**: Comandi per validazione lore, export e timeline management sono in roadmap ma non ancora implementati.

## 🔮 Roadmap Features

### 📋 Prossime Implementazioni

- **Characters Management**: Gestione personaggi con database e file JSON
- **Locations Management**: Gestione luoghi con caratteristiche e relazioni
- **Systems Management**: Sistemi di magia, tecnologia, cosmologia
- **Factions Management**: Organizzazioni, alleanze, conflitti
- **Lore Validation**: Sistema validazione automatica per consistency
- **Timeline Management**: Estrazione eventi temporali e conflict detection
- **Export System**: Export multi-formato per piattaforme diverse

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

### Fase 2: Worldbuilding 📋
- [ ] Characters database + CLI commands
- [ ] Locations database + CLI commands  
- [ ] Systems database + CLI commands
- [ ] Factions database + CLI commands

### Fase 3: Advanced Features 🚧
- [ ] Lore validation system
- [ ] Timeline management
- [ ] Export multi-formato
- [ ] Claude collaboration tools

## 📄 Licenza

MIT License - vedi [LICENSE](LICENSE) file per dettagli.

## 🤝 Supporto

- 📚 **Documentazione**: [docs/](docs/)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/user/multiverse-cli/issues)
- 💬 **Discussioni**: [GitHub Discussions](https://github.com/user/multiverse-cli/discussions)
- 📧 **Email**: support@multiverse-cli.dev

---

**Multiverse CLI** - Gestione professionale per universi narrativi complessi.