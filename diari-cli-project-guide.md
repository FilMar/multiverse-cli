# Multiverse CLI - Guida del Progetto

## Panoramica

Una CLI dedicata per la gestione e produzione del progetto "Diari dal Multiverso" - un canale antologico che esplora diversi universi narrativi attraverso:
- **Diari principali**: Serie narrative firmate "F.M." che costituiscono il contenuto core
- **Contenuti extra**: Lettere, documenti, trascrizioni che approfondiscono il worldbuilding senza firma F.M.

## Architettura del Progetto

### Struttura Multi-Repository

```
multiverse-cli/               # Repository principale CLI
├── src/                      # Codice sorgente CLI
├── templates/               # Template per nuovi episodi/mondi
├── config/                  # Configurazioni CLI
└── README.md

wandering-sun/               # Repository mondo 1
├── diari/                   # Serie principali (sempre firma F.M.)
│   ├── fenrik_mealor/
│   │   └── episodi/
│   │       ├── 001.md
│   │       ├── 002.md
│   │       └── 003.md
│   └── altro_diario_fm/
│       └── episodi/
├── extra/                   # Contenuti approfondimento (no firma F.M.)
│   ├── lettere_lyra/
│   │   └── episodi/
│   │       ├── 001.md
│   │       └── 002.md
│   ├── rapporti_federazione/
│   │   └── episodi/
│   └── trascrizioni_consiglio/
│       └── episodi/
├── lore/                    # File di worldbuilding
├── personaggi/             # Schede personaggi (JSON)
├── luoghi/                 # Schede luoghi (JSON)
├── world.db                # Database SQLite con tutti i metadati
└── world.json              # Configurazione base del mondo

glass-gardens/              # Repository mondo 2
├── diari/
├── extra/
├── lore/
├── personaggi/
├── luoghi/
├── world.db
└── world.json
```

## Filosofia di Design

### Principi Chiave
1. **Separazione delle responsabilità**: CLI come strumento, mondi come contenuto
2. **Automazione dei metadati**: Il creatore scrive solo contenuto narrativo
3. **Single source of truth**: Ogni mondo è self-contained
4. **Workflow semplificato**: Comandi intuitivi per operazioni complesse
5. **Separazione diari/extra**: Diari principali (F.M.) vs contenuti approfondimento (no F.M.)
6. **Firma selettiva F.M.**: Solo i diari principali usano la firma "F.M.", gli extra mantengono identità originale

### Divisione Responsabilità

**Il Creatore (Tu):**
- Scrive solo il contenuto narrativo degli episodi
- Usa comandi CLI per operazioni sui metadati
- Si concentra sulla creatività, non sulla gestione

**La CLI:**
- Gestisce automaticamente tutti i metadati
- Sincronizza le repository dei mondi
- Valida coerenza e continuità narrativa
- Genera export multi-formato
- Traccia stato di pubblicazione

## Database SQLite - Schema

### Tabelle Principali

```sql
-- Configurazione mondo
CREATE TABLE worlds (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    visual_identity TEXT, -- JSON
    config TEXT -- JSON
);

-- Serie (diari e extra)
CREATE TABLE series (
    id INTEGER PRIMARY KEY,
    world_id INTEGER,
    name TEXT NOT NULL,
    category TEXT CHECK(category IN ('diary', 'extra')),
    type TEXT, -- diario_personale, lettera, documento_ufficiale
    narrator TEXT,
    public_signature TEXT, -- 'F.M.' per diari, NULL per extra
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (world_id) REFERENCES worlds(id)
);

-- Episodi
CREATE TABLE episodes (
    id INTEGER PRIMARY KEY,
    series_id INTEGER,
    number INTEGER NOT NULL,
    title TEXT NOT NULL,
    file_path TEXT NOT NULL,
    status TEXT CHECK(status IN ('draft', 'in_produzione', 'ready', 'pubblicato')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    word_count INTEGER,
    reading_time_minutes INTEGER,
    FOREIGN KEY (series_id) REFERENCES series(id),
    UNIQUE(series_id, number)
);

-- Personaggi
CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    world_id INTEGER,
    name TEXT NOT NULL,
    race TEXT,
    has_magic_abilities BOOLEAN DEFAULT FALSE,
    abilities TEXT, -- JSON array
    limitations TEXT, -- JSON array
    age INTEGER,
    origin TEXT,
    profession TEXT,
    notes TEXT,
    FOREIGN KEY (world_id) REFERENCES worlds(id)
);

-- Luoghi
CREATE TABLE locations (
    id INTEGER PRIMARY KEY,
    world_id INTEGER,
    name TEXT NOT NULL,
    type TEXT, -- edificio_pubblico, regno, città
    parent_location_id INTEGER, -- per gerarchie (città > regno)
    characteristics TEXT, -- JSON array
    status TEXT, -- attiva, distrutta, abbandonata
    notes TEXT,
    FOREIGN KEY (world_id) REFERENCES worlds(id),
    FOREIGN KEY (parent_location_id) REFERENCES locations(id)
);

-- Tags
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    world_id INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY (world_id) REFERENCES worlds(id),
    UNIQUE(world_id, name)
);

-- Pubblicazioni
CREATE TABLE publications (
    id INTEGER PRIMARY KEY,
    episode_id INTEGER,
    platform TEXT NOT NULL, -- youtube, spotify, instagram
    url TEXT NOT NULL,
    published_at DATETIME,
    duration TEXT, -- per video/audio
    metadata TEXT, -- JSON per dati specifici piattaforma
    FOREIGN KEY (episode_id) REFERENCES episodes(id)
);
```

### Tabelle di Relazione

```sql
-- Episodi ↔ Personaggi
CREATE TABLE episode_characters (
    episode_id INTEGER,
    character_id INTEGER,
    PRIMARY KEY (episode_id, character_id),
    FOREIGN KEY (episode_id) REFERENCES episodes(id),
    FOREIGN KEY (character_id) REFERENCES characters(id)
);

-- Episodi ↔ Luoghi
CREATE TABLE episode_locations (
    episode_id INTEGER,
    location_id INTEGER,
    PRIMARY KEY (episode_id, location_id),
    FOREIGN KEY (episode_id) REFERENCES episodes(id),
    FOREIGN KEY (location_id) REFERENCES locations(id)
);

-- Episodi ↔ Tags
CREATE TABLE episode_tags (
    episode_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (episode_id, tag_id),
    FOREIGN KEY (episode_id) REFERENCES episodes(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

-- Collegamenti tra Episodi
CREATE TABLE episode_connections (
    from_episode_id INTEGER,
    to_episode_id INTEGER,
    connection_type TEXT, -- 'precedente', 'successivo', 'riferimento'
    notes TEXT,
    PRIMARY KEY (from_episode_id, to_episode_id, connection_type),
    FOREIGN KEY (from_episode_id) REFERENCES episodes(id),
    FOREIGN KEY (to_episode_id) REFERENCES episodes(id)
);

-- Validazioni Lore
CREATE TABLE lore_validations (
    id INTEGER PRIMARY KEY,
    episode_id INTEGER,
    validated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    validator_notes TEXT,
    issues_found INTEGER DEFAULT 0,
    status TEXT CHECK(status IN ('pending', 'approved', 'needs_review')),
    FOREIGN KEY (episode_id) REFERENCES episodes(id)
);

-- Issues trovati durante validazione
CREATE TABLE validation_issues (
    id INTEGER PRIMARY KEY,
    validation_id INTEGER,
    sentence_text TEXT NOT NULL,
    line_number INTEGER,
    issue_type TEXT, -- 'inconsistenza_abilità', 'personaggio_sconosciuto'
    character_id INTEGER,
    location_id INTEGER,
    severity TEXT CHECK(severity IN ('low', 'medium', 'high')),
    status TEXT CHECK(status IN ('pending', 'approved', 'rejected')),
    user_notes TEXT,
    FOREIGN KEY (validation_id) REFERENCES lore_validations(id),
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (location_id) REFERENCES locations(id)
);

-- Timeline eventi temporali
CREATE TABLE timeline_events (
    id INTEGER PRIMARY KEY,
    episode_id INTEGER,
    event_description TEXT NOT NULL,
    event_date TEXT, -- formato "Anno 1423, Primavera" o date relative
    event_order INTEGER, -- ordine sequenziale se date ambigue
    event_type TEXT CHECK(event_type IN ('major', 'minor', 'reference')),
    character_id INTEGER, -- se evento riguarda personaggio specifico
    location_id INTEGER, -- se evento riguarda luogo specifico
    extracted_automatically BOOLEAN DEFAULT TRUE,
    verified_by_user BOOLEAN DEFAULT FALSE,
    notes TEXT,
    FOREIGN KEY (episode_id) REFERENCES episodes(id),
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (location_id) REFERENCES locations(id)
);

-- Conflitti temporali identificati
CREATE TABLE temporal_conflicts (
    id INTEGER PRIMARY KEY,
    event1_id INTEGER,
    event2_id INTEGER,
    conflict_type TEXT, -- 'age_inconsistency', 'date_overlap', 'sequence_error'
    description TEXT,
    severity TEXT CHECK(severity IN ('low', 'medium', 'high')),
    status TEXT CHECK(status IN ('pending', 'resolved', 'ignored')),
    resolution_notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    resolved_at DATETIME,
    FOREIGN KEY (event1_id) REFERENCES timeline_events(id),
    FOREIGN KEY (event2_id) REFERENCES timeline_events(id)
);
```

### File world.json
```json
{
  "nome": "Wandering Sun",
  "descrizione": "Universo fantasy/sci-fi post-apocalittico",
  "identita_visiva": {
    "quaderno": "anticato",
    "penna": "stilografica",
    "inchiostro": "seppia",
    "estetica": "storica"
  },
  "narratori_principali": [
    {
      "nome": "Fenrik Mealor",
      "iniziali": "F.M.",
      "descrizione": "Ex-bibliotecario in viaggio"
    }
  ],
  "configurazione": {
    "formato_numerazione": "001",
    "firma_universale": "F.M.",
    "template_default": "diario_personale"
  },
  "categorie": {
    "diari": {
      "firma_pubblica": "F.M.",
      "tipi": ["diario_personale", "log_personale"]
    },
    "extra": {
      "firma_pubblica": null,
      "tipi": ["lettera", "documento_ufficiale", "trascrizione", "rapporto"]
    }
  }
}
```

## Workflow di Produzione

### 1. Setup Iniziale
```bash
# Inizializza un nuovo mondo
multiverse init --world wandering-sun --repo https://github.com/user/wandering-sun

# Scarica/sincronizza contenuti
multiverse pull --world wandering-sun

# Verifica struttura mondo
multiverse world info --world wandering-sun
```

### 2. Gestione Diari e Extra
```bash
# Crea nuovo diario (sempre F.M.)
multiverse diary create --world wandering-sun --name "fenrik_mealor" --narrator "Fenrik Mealor"
multiverse diary create --world wandering-sun --name "altro_diario" --narrator "Nome Cognome"

# Crea contenuto extra (no F.M.)
multiverse extra create --world wandering-sun --name "lettere_lyra" --type "lettera"
multiverse extra create --world wandering-sun --name "rapporti_federazione" --type "documento_ufficiale"

# Lista diari e extra
multiverse diary list --world wandering-sun
multiverse extra list --world wandering-sun
```

### 3. Creazione Episodi
```bash
# Crea episodio diario (sempre F.M.)
diario diary episode create --diary fenrik_mealor --number 001
# -> Genera diari/fenrik_mealor/episodi/001.md

# Crea episodio extra (no F.M.)
diario extra episode create --extra lettere_lyra --number 001
# -> Genera extra/lettere_lyra/episodi/001.md

# Dopo aver scritto il contenuto...
diario diary episode validate --diary fenrik_mealor --episode 001
diario extra episode validate --extra lettere_lyra --episode 001
# -> Controlla coerenza con lore esistente
```

### 4. Gestione Metadati
```bash
# Aggiorna stato pubblicazione
diario diary episode publish --diary fenrik_mealor --episode 001 --platform youtube --url "https://..."
diario extra episode publish --extra lettere_lyra --episode 001 --platform youtube --url "https://..."

# Aggiungi tag e collegamenti
diario diary episode tag --diary fenrik_mealor --episode 001 --add "biblioteca,magia-runica"
diario diary episode link --diary fenrik_mealor --episode 001 --to 002

# Collegamenti cross-categoria (extra che referenzia diario)
diario extra episode link --extra lettere_lyra --episode 001 --to-diary fenrik_mealor/003

# Aggiorna informazioni tecniche
diario diary episode stats --diary fenrik_mealor --episode 001 --duration "12:34"
```

### 5. Export e Distribuzione
```bash
# Export diari (automaticamente con F.M.)
diario diary export --diary fenrik_mealor --episode 001 --format youtube-description
# -> "Diario di F.M. - Episodio 001: La Biblioteca Perduta"

# Export extra (senza F.M., identità originale)
diario extra export --extra lettere_lyra --episode 001 --format youtube-description
# -> "Lettera di Lyra - Wandering Sun Extra: Dall'Accademia"

diario extra export --extra rapporti_federazione --episode 001 --format instagram-caption
# -> "Rapporto Federazione - Documento Ufficiale #001"

# Genera timeline del mondo
diario timeline --world wandering-sun --format markdown
diario timeline --world wandering-sun --diary-only  # Solo diari principali
diario timeline --world wandering-sun --format visual  # Timeline ASCII art
diario timeline --world wandering-sun --check-conflicts  # Verifica inconsistenze temporali
```

### 6. Analisi e Ricerca
```bash
# Cerca episodi per criteri
diario search --world wandering-sun --tag "magia-runica" --personaggio "Fenrik"
diario search --world wandering-sun --category diary  # Solo diari
diario search --world wandering-sun --category extra  # Solo extra

# Statistiche del mondo
diario stats --world wandering-sun
diario stats --diary fenrik_mealor
diario stats --extra lettere_lyra
# -> Numero episodi, personaggi, luoghi, parole totali

# Controllo continuità
diario validate --world wandering-sun --check-continuity
diario validate --diary fenrik_mealor --check-continuity
diario validate --extra lettere_lyra --check-continuity
# -> Verifica collegamenti e riferimenti

# Review manuale coerenza lore
diario diary episode review --diary fenrik_mealor --episode 005
# -> Validazione interattiva frase per frase con schede personaggi/luoghi
```

## Comandi CLI Principali

### Gestione Mondi
- `diario init --world <nome> --repo <url>` - Inizializza nuovo mondo
- `diario pull --world <nome>` - Sincronizza repository mondo
- `diario push --world <nome>` - Pusha modifiche al repository
- `diario worlds list` - Lista mondi disponibili

### Gestione Diari (F.M.)
- `diario diary create --world <mondo> --name <nome> --narrator <narratore>` - Crea diario
- `diario diary list --world <mondo>` - Lista diari
- `diario diary info --diary <nome>` - Info diario
- `diario diary episode create --diary <nome> --number <num>` - Crea episodio
- `diario diary episode list --diary <nome>` - Lista episodi
- `diario diary episode review --diary <nome> --episode <num>` - Review lore interattiva

### Gestione Extra (no F.M.)
- `diario extra create --world <mondo> --name <nome> --type <tipo>` - Crea extra
- `diario extra list --world <mondo>` - Lista extra
- `diario extra info --extra <nome>` - Info extra
- `diario extra episode create --extra <nome> --number <num>` - Crea episodio
- `diario extra episode list --extra <nome>` - Lista episodi
- `diario extra episode review --extra <nome> --episode <num>` - Review lore interattiva

### Metadati e Publishing
- `diario diary episode publish --diary <nome> --episode <num> --platform <piattaforma> --url <url>`
- `diario extra episode publish --extra <nome> --episode <num> --platform <piattaforma> --url <url>`
- `diario diary episode tag --diary <nome> --episode <num> --add <tags>` - Aggiungi tag
- `diario diary episode link --diary <nome> --episode <num> --to <altro_num>` - Collega episodi
- `diario extra episode link --extra <nome> --episode <num> --to-diary <diario/episodio>` - Collega a diario

### Export e Analisi
- `diario diary export --diary <nome> --episode <num> --format <formato>` - Export diari (F.M.)
- `diario extra export --extra <nome> --episode <num> --format <formato>` - Export extra (no F.M.)
- `diario search --world <mondo> [--tag] [--personaggio] [--category]` - Ricerca
- `diario timeline --world <mondo> [--diary-only] [--format <formato>]` - Genera timeline
- `diario timeline --world <mondo> --check-conflicts` - Verifica inconsistenze temporali
- `diario timeline --world <mondo> --extract-events` - Estrai eventi da episodi esistenti
- `diario stats --world <mondo> [--diary <nome>] [--extra <nome>]` - Statistiche

## Formati Export Supportati

### Descrizioni Piattaforme
- **youtube-description**: Titolo, descrizione, hashtag, collegamenti
- **spotify-description**: Descrizione ottimizzata per podcast
- **instagram-caption**: Caption con hashtag, menzioni

### Documentazione
- **timeline-markdown**: Timeline cronologica eventi
- **timeline-visual**: Timeline ASCII art interattiva
- **timeline-json**: Dati timeline per integrazione esterna
- **personaggi-markdown**: Lista personaggi con descrizioni
- **luoghi-markdown**: Lista luoghi visitati
- **lore-summary**: Riassunto elementi lore introdotti

## Sistema di Validazione Lore

### Struttura Schede Personaggi
```json
// personaggi/fenrik_mealor.json
{
  "nome": "Fenrik Mealor",
  "razza": "umano",
  "capacità_magiche": false,
  "abilità": ["ricerca", "lettura_runica_passiva"],
  "limitazioni": ["no_lancio_incantesimi", "no_manipolazione_mana"],
  "età": 45,
  "origini": "Cogland",
  "professione": "ex-bibliotecario",
  "episodi_apparizioni": ["fenrik_mealor/001", "fenrik_mealor/002"]
}
```

### Struttura Schede Luoghi
```json
// luoghi/biblioteca_cogland.json
{
  "nome": "Biblioteca di Cogland",
  "tipo": "edificio_pubblico",
  "regno": "Cogland",
  "caratteristiche": ["archivio_runico", "accesso_pubblico"],
  "stato": "attiva",
  "episodi_apparizioni": ["fenrik_mealor/001"],
  "collegamenti": ["personaggi/fenrik_mealor"]
}
```

### Validazione Interattiva

**Comando Review:**
```bash
diario diary episode review --diary fenrik_mealor --episode 005
```

**Output Interattivo:**
```
==========================================
EPISODIO 005 - REVISIONE LORE
==========================================

[Frase 12] "Fenrik lanciò un debole incantesimo di luce"

📋 PERSONAGGI COINVOLTI:
• Fenrik Mealor (umano) - capacità_magiche: false
  Limitazioni: no_lancio_incantesimi, no_manipolazione_mana

⚖️  VALUTAZIONE: Approva questa frase? [y/N/skip]: N
💬 Note (opzionale): Umani non lanciano incantesimi

==========================================
[Frase 18] "Arrivarono alla Biblioteca di Steamshire"

📍 LUOGHI COINVOLTI:
• Steamshire - ✅ trovato in lore/regni/
• Biblioteca di Steamshire - ❌ NON TROVATO

💡 SUGGERIMENTO: Creare scheda luogo? [y/N]: y
⚖️  VALUTAZIONE: Approva questa frase? [y/N/skip]: y
💬 Note: Nuova location, aggiungere a lore

==========================================
[Frase 25] "Lyra gli scrisse una lettera"

📋 PERSONAGGI COINVOLTI:
• Lyra - ✅ trovato in personaggi/
• Fenrik Mealor - ✅ trovato in personaggi/

🔗 COLLEGAMENTI: Possibile riferimento a extra/lettere_lyra/001?
⚖️  VALUTAZIONE: Approva questa frase? [y/N/skip]: y
```

**Report Finale:**
```bash
✅ Approvate: 23/25 frasi
❌ Rifiutate: 2 frasi (salvate in issues.json)
⏭️  Skippate: 0 frasi
💡 Nuovi elementi da creare: 1 luogo

Vuoi salvare le modifiche? [y/N]: y

📁 File generati:
- validation/fenrik_005_issues.json (problemi da risolvere)
- validation/fenrik_005_new_locations.json (luoghi da creare)
- metadata aggiornato con timestamp validazione
```

### Pattern Recognition Automatico

La CLI identifica automaticamente:
- **Nomi propri**: Parole maiuscole, pattern "di/della/del"
- **Azioni magiche**: "lanciò", "incantesimo", "magia", "mana"
- **Riferimenti geografici**: "arrivò a", "nella città di"
- **Interazioni personaggi**: "parlò con", "incontrò"

### File di Output Validazione

**issues.json** (problemi trovati):
```json
{
  "episodio": "fenrik_mealor/005",
  "data_review": "2025-08-20",
  "problemi": [
    {
      "frase": "Fenrik lanciò un debole incantesimo di luce",
      "linea": 12,
      "tipo": "inconsistenza_abilità",
      "personaggio": "Fenrik Mealor",
      "conflitto": "capacità_magiche: false",
      "stato": "rifiutato",
      "note": "Umani non lanciano incantesimi"
    }
  ]
}
```

**new_elements.json** (elementi da creare):
```json
{
  "luoghi_nuovi": [
    {
      "nome": "Biblioteca di Steamshire",
      "tipo_suggerito": "edificio_pubblico",
      "regno": "Steamshire",
      "primo_episodio": "fenrik_mealor/005"
    }
  ],
  "personaggi_nuovi": [],
  "termini_nuovi": []
}
```

## Validazioni Automatiche

### Controlli di Coerenza
- Numerazione episodi sequenziale
- Collegamenti episodi validi
- Cross-reference con schede personaggi/luoghi esistenti
- Tag standardizzati
- Pattern recognition per elementi lore

### Controlli di Qualità
- Lunghezza minima/massima episodi
- Presenza metadati obbligatori
- Formato data standardizzato
- URL validità piattaforme

### Validazione Lore Manuale
- Review interattiva frase per frase
- Cross-reference automatico con schede personaggi/luoghi
- Approvazione/rifiuto manuale delle inconsistenze
- Generazione report problemi da risolvere

## Estensibilità

### Nuovi Mondi
La CLI è progettata per supportare facilmente nuovi universi narrativi:
1. Crea repository mondo con struttura standard
2. Configura `world.json` con identità visiva
3. Definisci template episodi specifici
4. La CLI adatta automaticamente workflow

### Nuove Piattaforme
Aggiungere supporto per nuove piattaforme di pubblicazione:
1. Aggiungi formato export in `src/exports/`
2. Estendi schema metadati pubblicazione
3. Implementa validazioni URL specifiche

## Tecnologie Consigliate

### Linguaggi Possibili
- **Python + Click**: Prototipazione rapida, ricco ecosistema
- **Rust + Clap**: Performance, type safety, deployment facile
- **Node.js + Commander**: Integrazione web, JSON nativo

### Storage
- **SQLite** per metadati (performance, relazioni, query complesse)
- **Markdown** per contenuti (standard, tool-agnostic) 
- **JSON** per schede personaggi/luoghi (human-readable backup)
- **Git** per versionamento e sincronizzazione

## Obiettivi del Progetto

### Fase 1: MVP
- Gestione base episodi e metadati
- Export descrizioni YouTube/Spotify
- Validazioni essenziali
- Supporto Wandering Sun

### Fase 2: Espansione
- Supporto multi-mondo (Glass Gardens)
- Analytics avanzate
- Template personalizzabili
- Integrazione API piattaforme

### Fase 3: Automazione Completa
- Pubblicazione automatica
- Generazione contenuti visual
- Workflow CI/CD
- **Dashboard web integrata**

## Dashboard Web Locale

### Comando Avvio
```bash
# Avvia dashboard web locale
diario serve --port 3000
# -> http://localhost:3000 - Interfaccia chat con comandi CLI
```

### Interface Design
```
┌─────────────────────────────────────┐
│ 💬 Diari CLI Dashboard             │
├─────────────────────────────────────┤
│ > diario diary list --world ws     │
│ ┌─────────────────────────────────┐ │
│ │ 📚 Fenrik Mealor (5 episodi)   │ │
│ │ [📝 New Ep] [📊 Stats]         │ │
│ │                                 │ │
│ │ Ep 001: Biblioteca ✅          │ │
│ │ Ep 002: Viaggio 🔄 [→ Publish] │ │
│ │ Ep 003: Draft 📝 [→ Review]    │ │
│ └─────────────────────────────────┘ │
│                                     │
│ > diario episode create --diary f.. │
│ ┌─────────────────────────────────┐ │
│ │ ✨ Nuovo episodio creato: 006   │ │
│ │ [📝 Edit] [🔍 Preview]         │ │
│ └─────────────────────────────────┘ │
├─────────────────────────────────────┤
│ 💭 Scrivi comando... [Send]        │
└─────────────────────────────────────┘
```

### Tecnologie
- **Backend**: Rust + Axum
- **Frontend**: HTMX + Tailwind CSS
- **Database**: Condiviso con CLI (stesso SQLite)
- **No JS frameworks**, solo HTMX per interattività

### Features Core

#### 1. Chat Interface con Autocompletamento
```html
<input type="text" 
       hx-get="/autocomplete" 
       hx-trigger="keyup changed delay:200ms" 
       hx-target="#suggestions"
       placeholder="diario diary list --world...">
<div id="suggestions" class="dropdown"></div>
```

#### 2. Output Arricchito HTMX
```html
<!-- Risposta comando -->
<div class="command-result">
  <div class="series-card">
    <h3>📚 Fenrik Mealor <span class="badge">5 episodi</span></h3>
    <div class="episode-grid">
      <div class="episode-item status-published">
        <span class="number">001</span>
        <span class="title">La Biblioteca Perduta</span>
        <div class="actions">
          <button hx-get="/export/1" class="btn-sm">📤 Export</button>
          <button hx-get="/stats/1" class="btn-sm">📊 Stats</button>
        </div>
      </div>
      <div class="episode-item status-production">
        <span class="number">002</span>
        <span class="title">Il Viaggio</span>
        <div class="actions">
          <button hx-patch="/episodes/2" 
                  hx-vals='{"status":"ready"}' 
                  class="btn-sm">→ Ready</button>
        </div>
      </div>
    </div>
  </div>
</div>
```

#### 3. Status Updates
```html
<button hx-patch="/episodes/42" 
        hx-vals='{"status":"pubblicato"}'
        hx-target="#episode-42"
        hx-swap="outerHTML"
        class="status-btn">
  → Publish
</button>
```

#### 4. Command History
```html
<div class="command-history">
  <div class="history-item" 
       hx-get="/command/replay" 
       hx-vals='{"cmd":"diary list --world ws"}'>
    > diary list --world wandering-sun
  </div>
</div>
```

## Sistema Timeline Temporale

### Overview Timeline
La timeline offre visualizzazione cronologica degli eventi per verificare coerenza temporale e identificare conflitti automaticamente.

### Estrazione Automatica Eventi
```rust
// Pattern matching per eventi temporali
let temporal_patterns = [
    r"(\d+) anni?",                    // "45 anni", "23 anni"  
    r"Anno (\d+)",                     // "Anno 1423"
    r"(\d+) mes[ei]",                  // "6 mesi dopo"
    r"(Primavera|Estate|Autunno|Inverno)", // Stagioni
    r"dopo (\d+) giorni",              // "dopo 3 giorni"
    r"(\d+) anni? (fa|dopo)",          // "2 anni fa"
    r"quando (aveva|avevo) (\d+) anni", // "quando aveva 30 anni"
];

// Estrazione da episodi esistenti
pub fn extract_timeline_events(episode_content: &str) -> Vec<TimelineEvent> {
    let mut events = Vec::new();
    
    for line in episode_content.lines() {
        for pattern in temporal_patterns {
            if let Some(matches) = pattern.captures(line) {
                events.push(TimelineEvent {
                    description: line.to_string(),
                    extracted_data: matches,
                    confidence: calculate_confidence(&matches),
                });
            }
        }
    }
    
    events
}
```

### CLI Timeline Commands
```bash
# Estrai eventi da tutti gli episodi
diario timeline extract --world wandering-sun
# -> Scannerizza tutti i .md e popola timeline_events

# Visualizza timeline
diario timeline show --world wandering-sun
# -> ASCII timeline con eventi ordinati

# Verifica conflitti
diario timeline conflicts --world wandering-sun
# -> Lista conflitti automaticamente rilevati

# Timeline specifica per personaggio
diario timeline --character "Fenrik Mealor" --world wandering-sun

# Timeline range temporale
diario timeline --world wandering-sun --from "Anno 1420" --to "Anno 1425"
```

### Timeline Dashboard Visuale
```html
<div class="timeline-container">
  <div class="timeline-header">
    <h2>🕐 Wandering Sun Timeline</h2>
    <div class="timeline-controls">
      <button hx-post="/timeline/extract" class="btn">📅 Auto-Extract</button>
      <button hx-get="/timeline/conflicts" hx-target="#conflicts" class="btn">⚠️ Check Conflicts</button>
    </div>
  </div>
  
  <div class="timeline-viz">
    <div class="timeline-axis">
      <span class="timeline-marker">Anno 1420</span>
      <span class="timeline-marker">Anno 1421</span>
      <span class="timeline-marker">Anno 1422</span>
    </div>
    
    <div class="timeline-events">
      <!-- Event node -->
      <div class="event-node verified" 
           data-episode="1" 
           data-date="Anno 1420"
           hx-get="/timeline/event/1" 
           hx-target="#event-detail">
        <div class="event-marker major"></div>
        <div class="event-info">
          <strong>Ep 001: Biblioteca</strong>
          <span class="event-date">Anno 1420, Primavera</span>
          <span class="event-desc">Fenrik, 45 anni, inizia viaggio</span>
        </div>
      </div>
      
      <!-- Conflict warning -->
      <div class="conflict-indicator high" 
           hx-get="/conflicts/42" 
           hx-target="#conflict-detail">
        <span class="conflict-icon">⚠️</span>
        <span class="conflict-text">Age Conflict: Fenrik 43 vs 45 anni</span>
      </div>
      
      <!-- Unverified event -->
      <div class="event-node unverified" 
           data-episode="3">
        <div class="event-marker minor"></div>
        <div class="event-info">
          <strong>Ep 003: Arrivo</strong>
          <span class="event-date">Anno 1422 (?)</span>
          <span class="event-desc">Lyra invia lettera</span>
          <button hx-patch="/timeline/verify/3" 
                  hx-target="closest .event-node" 
                  class="verify-btn">✅ Verify</button>
        </div>
      </div>
    </div>
  </div>
  
  <!-- Conflicts Panel -->
  <div id="conflicts" class="conflicts-panel">
    <h3>🚨 Temporal Conflicts</h3>
    <div class="conflict-item">
      <div class="conflict-header">
        <span class="severity high">HIGH</span>
        <span class="conflict-type">Age Inconsistency</span>
      </div>
      <div class="conflict-details">
        <div class="conflict-event">Ep 001: "Fenrik, 45 anni"</div>
        <div class="conflict-event">Ep 004: "Fenrik aveva 43 anni"</div>
      </div>
      <div class="conflict-actions">
        <button hx-patch="/conflicts/42/resolve" class="btn-success">✅ Mark Resolved</button>
        <button hx-patch="/conflicts/42/ignore" class="btn-secondary">➖ Ignore</button>
        <textarea placeholder="Resolution notes..."></textarea>
      </div>
    </div>
  </div>
</div>
```

### Auto-Detection Conflicts
```sql
-- Query: Conflitti età personaggi
SELECT 
    t1.episode_id as ep1_id, t2.episode_id as ep2_id,
    e1.title as ep1_title, e2.title as ep2_title,
    t1.event_description as age1, t2.event_description as age2,
    c.name as character_name
FROM timeline_events t1
JOIN timeline_events t2 ON t1.character_id = t2.character_id
JOIN episodes e1 ON t1.episode_id = e1.id  
JOIN episodes e2 ON t2.episode_id = e2.id
JOIN characters c ON t1.character_id = c.id
WHERE t1.event_description REGEXP '\\d+ anni?'
  AND t2.event_description REGEXP '\\d+ anni?'
  AND t1.id != t2.id
  AND t1.character_id = t2.character_id;

-- Query: Eventi sovrapposti temporalmente
SELECT t1.*, t2.* 
FROM timeline_events t1, timeline_events t2
WHERE t1.event_date = t2.event_date 
  AND t1.episode_id != t2.episode_id
  AND t1.event_type = 'major' 
  AND t2.event_type = 'major';
```

### Timeline Export Formats
```bash
# Timeline markdown per reference
diario timeline --world ws --format markdown > timeline.md

# Timeline visual ASCII per CLI
diario timeline --world ws --format visual
# Output:
# Anno 1420 ──●── Anno 1421 ──●── Anno 1422
#              │               │
#           Ep 001          Ep 002
#         Biblioteca       Viaggio  
#        Fenrik 45a       6m dopo   
#             │               │
#             └─── ⚠️ CONFLICT ───┘
#                 Ep 004: 43 anni

# JSON per integrazioni
diario timeline --world ws --format json > timeline.json
```

### Performance & UX
```sql
-- Indici per query timeline veloci
CREATE INDEX idx_timeline_episode_date ON timeline_events(episode_id, event_date);
CREATE INDEX idx_timeline_character_date ON timeline_events(character_id, event_date);
CREATE INDEX idx_timeline_date_type ON timeline_events(event_date, event_type);
CREATE INDEX idx_conflicts_status ON temporal_conflicts(status);
```

### Integration con Validation System
```rust
// Durante lore validation, controlla timeline
pub fn validate_temporal_consistency(episode_id: i32, sentence: &str) -> Vec<TimelineConflict> {
    let mut conflicts = Vec::new();
    
    // Estrai eventi temporali dalla frase
    let events = extract_timeline_events(sentence);
    
    for event in events {
        // Cerca conflitti con eventi esistenti
        let existing_conflicts = check_timeline_conflicts(&event, episode_id);
        conflicts.extend(existing_conflicts);
    }
    
    conflicts
}
```

La timeline diventa uno strumento essenziale per mantenere coerenza narrativa, identificando automaticamente inconsistenze e offrendo visualizzazione chiara dell'evoluzione temporale della storia.

## Note di Implementazione

### Principi di Design CLI
- Comandi intuitivi e mnemonici
- Output leggibile e colorato
- Feedback chiaro su operazioni
- Gestione errori descrittiva
- Help context-aware

### Gestione Errori
- Validazione input utente
- Backup automatici prima modifiche
- Rollback in caso di errori
- Log dettagliati per debugging

### Performance
- **SQLite indexes** su campi frequenti (tags, character names, dates)
- **Connection pooling** per database access
- **Query optimization** per ricerche complesse
- **Lazy loading** repository grandi
- **Operazioni batch** per modifiche multiple
- **Pattern matching ottimizzato** per review lore

### Database Migrations
```sql
-- Indici per performance
CREATE INDEX idx_episodes_series_number ON episodes(series_id, number);
CREATE INDEX idx_episodes_status ON episodes(status);
CREATE INDEX idx_characters_world_name ON characters(world_id, name);
CREATE INDEX idx_locations_world_name ON locations(world_id, name);
CREATE INDEX idx_tags_world_name ON tags(world_id, name);
CREATE INDEX idx_publications_episode_platform ON publications(episode_id, platform);
CREATE INDEX idx_validations_episode_status ON lore_validations(episode_id, status);
```

### Query Examples
```sql
-- Dashboard: episodi per serie con status
SELECT e.number, e.title, e.status, 
       COUNT(p.id) as publication_count
FROM episodes e
LEFT JOIN publications p ON e.id = p.episode_id
WHERE e.series_id = ?
GROUP BY e.id
ORDER BY e.number;

-- Search: episodi per personaggio
SELECT DISTINCT e.number, e.title, s.name as series_name
FROM episodes e
JOIN series s ON e.series_id = s.id
JOIN episode_characters ec ON e.id = ec.episode_id
JOIN characters c ON ec.character_id = c.id
WHERE c.name LIKE '%Fenrik%';

-- Analytics: stats mondo
SELECT 
    COUNT(DISTINCT s.id) as total_series,
    COUNT(e.id) as total_episodes,
    SUM(e.word_count) as total_words
FROM worlds w
LEFT JOIN series s ON w.id = s.world_id
LEFT JOIN episodes e ON s.id = e.series_id
WHERE w.id = ?;
```
