# FEATURES.md

Stato attuale delle features implementate nel progetto Multiverse CLI.

## ✅ Features Implementate

### 🌍 World Management (Completo)

**Comandi disponibili:**
- `multiverse world init <name>` - Inizializza nuovo progetto multiverse
- `multiverse world init <name> --from-git <url>` - Clona da repository esistente
- `multiverse world init <name> --merge` - Inizializza in directory non vuota
- `multiverse world info` - Mostra dettagli progetto corrente
- `multiverse world pull` - Pull da repository Git
- `multiverse world push` - Push modifiche a repository Git  
- `multiverse world status` - Mostra stato Git
- `multiverse world config` - Mostra configurazione corrente
- `multiverse world config --set <key> --value <value>` - Modifica configurazione
- `multiverse world import --all` - Importa tutti i file SQL dalla directory sql/
- `multiverse world import --file <path>` - Importa file/directory SQL specifici

**Funzionalità:**
- ✅ Creazione automatica struttura directory (`.multiverse/`, `stories/`)
- ✅ File fondamentali (`00_ESSENTIAL.md`, `01_HISTORY.md`, `README.md`)
- ✅ Configurazione TOML in `.multiverse/config.toml`
- ✅ Inizializzazione Git automatica
- ✅ Cloning da repository remote
- ✅ Merge mode per directory esistenti
- ✅ Sistema di configurazione con categorie (diary/extra)
- ✅ Visual identity configurabile (estetica, descrizione)
- ✅ Import SQL batch per inizializzazione database

### 📚 Story Management (Completo - Schema Flessibile)

**Comandi disponibili:**
- `multiverse story create <name> --type <tipo>` - Crea storia con tipo configurabile
- `multiverse story types` - Lista story types disponibili
- `multiverse story list` - Lista tutte le storie
- `multiverse story info <name>` - Dettagli storia specifica
- `multiverse story delete <name> --force` - Elimina storia

**Funzionalità:**
- ✅ **Schema flessibile** con metadata JSON configurabile
- ✅ **Story types configurabili** tramite config.toml
- ✅ **Validazione dinamica** basata su required_fields per tipo
- ✅ Creazione directory automatica in `stories/<nome>/`
- ✅ Sistema stati: Active, Paused, Completed, Archived
- ✅ Eliminazione sicura (richiede --force)
- ✅ Integrazione completa filesystem + database

### 📄 Episode Management (Completo)

**Comandi disponibili:**
- `multiverse episode create --story <name>` - Crea nuovo episodio con numerazione automatica
- `multiverse episode create --story <name> --title <title>` - Con titolo specifico
- `multiverse episode list --story <name>` - Lista episodi di una storia
- `multiverse episode info --story <name> --number <num>` - Dettagli episodio
- `multiverse episode delete --story <name> --number <num> --force` - Elimina episodio

**Funzionalità:**
- ✅ Numerazione automatica sequenziale (001, 002, 003...)
- ✅ Creazione file Markdown automatica (`001.md`, `002.md`, etc.)
- ✅ Metadati completi (titolo, stato, word count, date)
- ✅ Sistema stati: Draft, InProgress, Review, Published
- ✅ Validazione esistenza storia parent
- ✅ Eliminazione sicura file + database
- ✅ Foreign key con CASCADE per integrità database

### 🗄️ Database System (Completo)

**Schema implementato (Flessibile):**
```sql
-- Tabella stories con metadata JSON
CREATE TABLE stories (
    name TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    story_type TEXT NOT NULL,
    metadata TEXT,                    -- JSON blob configurabile
    description TEXT,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active'
);

-- Tabella episodes  
CREATE TABLE episodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_name TEXT NOT NULL,
    episode_number INTEGER NOT NULL,
    title TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    word_count INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (story_name) REFERENCES stories (name) ON DELETE CASCADE,
    UNIQUE(story_name, episode_number)
);
```

**Funzionalità:**
- ✅ SQLite database in `.multiverse/world.db`
- ✅ Foreign keys abilitate
- ✅ Constraint UNIQUE per evitare episodi duplicati
- ✅ CASCADE delete per integrità referenziale
- ✅ Gestione connessioni con error handling
- ✅ Auto-incremento numeri episodi
- ✅ Serializzazione date in RFC3339

### ⚙️ Configuration System (Completo)

**File di configurazione:**
- ✅ `.multiverse/config.toml` - Configurazione mondo principale
- ✅ Metadati mondo (nome, descrizione)
- ✅ Visual identity (estetica, descrizione)
- ✅ Global config (formato numerazione, template default)
- ✅ **Story types configurabili** con required/optional fields
- ✅ **Validazione dinamica** per ogni story type
- ✅ **Default values** personalizzabili per tipo

**Struttura:**
```toml
[world]
name = "nome_mondo"
description = "descrizione"

[world.visual_identity]
estetica = "fantasy"
descrizione = "Quaderni anticati con inchiostro seppia"

[world.global_config]
formato_numerazione = "001"
template_default = "diario_personale"

[story_types.diary]
display_name = "Personal Diary"
required_fields = ["narrator", "signature"]
optional_fields = ["perspective", "time_period"]
default_signature = "F.M."
numbering_format = "001"

[story_types.book]
display_name = "Book/Novel"
required_fields = ["author", "genre"]
optional_fields = ["perspective", "series_name", "volume"]
numbering_format = "Chapter %d"

[story_types.letter_series]
display_name = "Letter Series"
required_fields = ["correspondent_from", "correspondent_to"]
optional_fields = ["signature"]
numbering_format = "Letter %d"

[story_types.oneshot]
display_name = "One-shot Story"
required_fields = ["genre"]
optional_fields = ["word_count_target", "theme"]
numbering_format = "Part %d"
```

### 🔧 Git Integration (Completo)

**Funzionalità:**
- ✅ Inizializzazione repository automatica
- ✅ Clone da URL remoti
- ✅ Pull/push operations
- ✅ Status dettagliato con colori e emoji
- ✅ Error handling per operazioni Git
- ✅ Integrazione con workflow world management

### 🏗️ CLI Architecture (Completo)

**Struttura:**
- ✅ Clap framework con derive macros
- ✅ Subcommands: world, story, episode, info
- ✅ Error handling con anyhow
- ✅ Context-aware help
- ✅ Validation input utente
- ✅ Output colorato e con emoji

## ✅ Features Recentemente Implementate

### 🔧 LLM Extraction Guide (Completo)
- ✅ Generazione automatica `EXTRACTION_GUIDE.md` per onboarding progetti esistenti
- ✅ Template dettagliato con istruzioni step-by-step per LLM
- ✅ Trigger automatico su `--merge` e `--from-git` senza `.multiverse/`
- ✅ Creazione automatica directory `sql/` per file generati
- ✅ Rilevamento intelligente contenuto narrativo esistente

### 📊 World Statistics (Completo)
- ✅ Statistiche reali in `world info` invece di "(to be implemented)"
- ✅ Conteggio stories e episodes dal database
- ✅ Breakdown episodes per status (Draft, InProgress, Review, Published)
- ✅ Gestione errori database con fallback graceful

## ⚠️ Features In Corso di Refactoring

Nessuna feature attualmente in refactoring. Il sistema story/episode con schema flessibile è già implementato e funzionante.

## ❌ Features Non Implementate

### 🔍 Lore Validation System
- ❌ `multiverse episode review` - Validazione interattiva
- ❌ Pattern matching per nomi propri, azioni magiche
- ❌ Cross-reference con personaggi/luoghi
- ❌ Sistema approvazione/rifiuto frasi
- ❌ Report problemi e conflitti

### 🕐 Timeline Management
- ❌ Estrazione automatica eventi temporali
- ❌ Tabelle `timeline_events`, `temporal_conflicts`
- ❌ Conflict detection per età/date inconsistenti
- ❌ Timeline visuale ASCII/Markdown
- ❌ Comandi `multiverse timeline`

### 📦 Export System
- ❌ Export multi-formato (YouTube, Spotify, Instagram)
- ❌ Template export personalizzabili
- ❌ Sistema firma F.M. nell'export
- ❌ Generazione hashtag e metadati
- ❌ Comandi `multiverse export`

### 🤖 Claude Collaboration
- ❌ `multiverse export claude-guide` - Genera CLAUDE.md con istruzioni CLI complete
- ❌ Guide per creare/modificare sistemi, fazioni, personaggi, luoghi
- ❌ **Istruzioni story/episode management** - Come creare storie ed episodi
- ❌ **Spiegazione config.toml** - Story types, required/optional fields, validazione
- ❌ **Convenzioni naming** `categoria_tipo_nome.md` per file lore
- ❌ **Regole sincronizzazione** database ↔ file .md sempre allineati  
- ❌ **Comandi update** per modificare elementi esistenti con sync automatico

### 🗄️ Database Query System
- ❌ `multiverse query "SELECT ..."` - Query SQL dirette (solo SELECT)
- ❌ **Output formatting** - table, json, csv per diversi use case
- ❌ **Performance timing** - `--time` per query optimization
- ❌ **Query validation** - Solo SELECT consentito per sicurezza
- ❌ **Local-only security model** - Nessuna preoccupazione SQL injection
- ❌ **Debug features** - `--explain` per query planning

### 👥 Characters & Locations
- ❌ Gestione schede personaggi JSON
- ❌ Gestione schede luoghi JSON
- ❌ Tabelle `characters`, `locations`
- ❌ Cross-reference episodi ↔ personaggi/luoghi
- ❌ Comandi `multiverse character`, `multiverse location`

### 🔮 Systems Management
- ❌ Gestione sistemi del mondo (magia, tecnologia, cosmologia)
- ❌ Tabella `systems` con regole e interazioni
- ❌ Sistema di validazione per meccaniche di gioco
- ❌ Cross-reference con personaggi/episodi che usano sistemi
- ❌ Comandi `multiverse system create/list/info`

### ⚔️ Factions Management
- ❌ Gestione fazioni e organizzazioni
- ❌ Tabella `factions` con gerarchia e relazioni
- ❌ Sistema di alleanze e conflitti tra fazioni
- ❌ Cross-reference con personaggi membri/episodi coinvolti
- ❌ Comandi `multiverse faction create/list/info`


### 📊 Analytics & Publishing
- ❌ Tracking pubblicazioni per piattaforma
- ❌ Word count automatico da file Markdown
- ❌ Statistiche mondo/storia/episodi
- ❌ Status workflow (draft → review → published)
- ❌ Metadati pubblicazione (URL, durate, etc.)

### 📝 Content Templates
- ❌ Template episodi con creazione metadati nel db
- ❌ Attualmente crea solo file .md vuoti
- ❌ Template personalizzabili per tipo storia
- ❌ Auto-population firma F.M. per diari

## 🎯 Priorità per Prossimi Sviluppi

### Fase 1: Worldbuilding Foundations (Alto)
1. **Characters Management** - Schema database + comandi CLI per personaggi
2. **Locations Management** - Schema database + comandi CLI per luoghi  
3. **Systems Management** - Schema database + comandi CLI per sistemi (magia/tecnologia)
4. **Factions Management** - Schema database + comandi CLI per fazioni
5. **Events Management** - Schema database + comandi CLI per eventi storici

### Fase 2: Content Analysis (Medio)
1. **Word Count** - Parsing automatico file Markdown
2. **Cross-references** - Linking automatico personaggi/luoghi in episodi

### Fase 3: Advanced Features (Basso)
1. **Timeline Management** - Estrazione automatica eventi temporali

### Fase 4: Ecosystem (Futuro)
1. **Multi-platform Export** - YouTube, Spotify, Instagram
2. **Claude Collaboration** - CLAUDE.md guide generation e comandi update
3. **Database Query System** - Raw SQL queries con output formatting avanzato
4. **Claude-powered Lore Analysis** - Strumenti per analisi consistency via AI

## 📊 Stato Complessivo

- **Completato**: 40% (World, Story, Episode, Database base, Config, Git, CLI)
- **In Refactoring**: 0% (Schema flessibile già implementato)
- **Mancante**: 60% (Characters, Locations, Systems, Factions, Events, Timeline, Export, Claude Collaboration, Database Queries, AI-powered Analysis)

Il progetto ha una **base solida e completa** per la gestione di storie ed episodi, con:
- ✅ **Sistema completo** world/story/episode management
- ✅ **Onboarding automatico** progetti esistenti tramite LLM guide
- ✅ **Configurazione flessibile** tramite TOML
- 🚧 **In evoluzione** verso schema completamente configurabile
- 📋 **Roadmap chiara** per worldbuilding completo (Characters, Locations, Systems, Factions, Events)
