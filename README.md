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
- 🏗️ **Worldbuilding Completo**: Personaggi, Luoghi, Razze, Fazioni, Sistemi ed Eventi
- 🔗 **Relationship Management** tra tutte le entità, gestito via metadata
- 📅 **Event Management** con supporto per timeline cronologica
- ✏️ **Full CRUD** per tutte le entità (Create, Read, Update, Delete)
- 🧩 **Architettura Metadata-First**: Massima flessibilità con campi personalizzati tramite `--set`
- 🕐 **Timeline Configurabile** per calendari personalizzati
- ⚙️ **Configuration System** TOML per personalizzazione completa
- 🔗 **Git integration** per sincronizzazione repository

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

# Crea una storia, un personaggio, un luogo, un sistema, una fazione, una razza e un evento
multiverse story create "fenrik_mealor" --type diary
multiverse character create "fenrik" --set display_name="Fenrik Mealor" --set profession=explorer --set description="Esploratore del regno"
multiverse location create "glass_gardens" --set display_name="Glass Gardens" --set type=region --set description="Giardini cristallini"
multiverse system create "aetherial_magic" --set display_name="Aetherial Magic" --set system_type=magic --set description="Sistema magico etereo"
multiverse faction create "sylvan_guardians" --set display_name="Sylvan Guardians" --set type=guild --set description="Guardiani della foresta"
multiverse race create "high_elves" --set display_name="High Elves" --set lifespan=1000 --set description="Nobili elfi dalle lunghe vite"
multiverse event create "first_contact" --set display_name="First Contact" --set type=discovery --set description="Primo contatto" --date "1A/1/1"

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

Il progetto utilizza **SQLite** per gestire tutti i metadati in un singolo file `world.db`.
L'architettura è **metadata-first**: invece di avere colonne rigide, la maggior parte dei dati è memorizzata in campi JSON, permettendo a ogni mondo di definire il proprio schema.

- **Entità Principali**: `stories`, `episodes`, `characters`, `locations`, `systems`, `factions`, `races`, `events`.
- **Tabelle di Relazione**: Vengono create dinamicamente per gestire le connessioni tra entità, ad esempio:
    - `character_episodes`
    - `character_locations`
    - `character_factions`
    - `event_characters`
    - ... e molte altre.

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
multiverse character create <nome> --set display_name="..." --set description="..." --set profession="..."
multiverse character update <nome> --set age=25 --set status=active

# Gestione Luoghi (tipo e descrizione in metadata)
multiverse location create <nome> --set display_name="..." --set type=city --set description="..."
multiverse location update <nome> --set population=10000

# Gestione Sistemi (es. magia, tecnologia)
multiverse system create <nome> --set display_name="..." --set system_type=magic --set description="..."
multiverse system update <nome> --set complexity=high

# Gestione Fazioni
multiverse faction create <nome> --set display_name="..." --set type=guild --set description="..."
multiverse faction update <nome> --set alignment=neutral

# Gestione Razze
multiverse race create <nome> --set display_name="..." --set lifespan=1000 --set description="..."
multiverse race update <nome> --set status=Legendary --set population=few

# Gestione Eventi Storici
multiverse event create <nome> --set display_name="..." --set type=battle --set description="..." --date <data>
multiverse event update <nome> --set date=<data>
multiverse event timeline          # Mostra gli eventi in ordine cronologico
```

### 🔗 Gestione Relazioni (tramite --set)

Le relazioni non hanno comandi dedicati, ma vengono gestite tramite speciali parametri `--set` durante la creazione e l'aggiornamento delle entità. È possibile assegnare più relazioni separando i nomi con una virgola.

```bash
# Associa un personaggio a un luogo e una fazione
multiverse character update fenrik --set location=glass_gardens --set faction=sylvan_guardians

# Crea un evento con personaggi e luoghi associati
multiverse event create "the_summit" --set character=fenrik,lyra --set location=citadel

# Crea una relazione tra due luoghi (es. un luogo contenuto in un altro)
multiverse location update "inner_sanctum" --set location=citadel
```

**Note**: Comandi per validazione della coerenza, export multi-formato e analisi avanzata sono in roadmap.

## 🗺️ Roadmap

Il progetto ha una base solida e completa. Le prossime implementazioni si concentreranno su analisi dei contenuti e funzionalità avanzate.

### Fase 1: Content Analysis (Priorità Alta)
- **Word Count**: Parsing automatico dei file Markdown per arricchire `world info`.
- **Cross-referencing Migliorato**: Linking automatico avanzato di entità negli episodi.
- **UI per Relazioni**: Comandi per visualizzare e gestire le relazioni tra entità.

### Fase 2: Advanced Features (Priorità Media)
- **Lore Validation**: Sistema di validazione interattiva per la coerenza narrativa (`episode review`).
- **Comandi per Timeline**: Gestione di ere e calendari direttamente da CLI.

### Fase 3: Ecosystem (Priorità Bassa)
- **Export System**: Esportazione multi-formato (es. per YouTube, Spotify) con template.
- **AI Collaboration**: Integrazione con LLM per assistenza alla scrittura e analisi.
- **Query System**: Interfaccia per eseguire query SQL dirette al database.

## 📄 Licenza

MIT License - vedi [LICENSE](LICENSE) file per dettagli.

## 🤝 Supporto

- 📚 **Documentazione**: [docs/](docs/)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/user/multiverse-cli/issues)
- 💬 **Discussioni**: [GitHub Discussions](https://github.com/user/multiverse-cli/discussions)
- 📧 **Email**: support@multiverse-cli.dev

---

**Multiverse CLI** - Gestione professionale per universi narrativi complessi.