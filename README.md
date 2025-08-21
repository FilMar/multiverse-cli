# Multiverse CLI

Una CLI avanzata per gestire progetti narrativi complessi con validazione lore automatica, timeline temporale e dashboard web locale.

## 🎯 Cos'è Multiverse CLI

**Multiverse CLI** è progettata per creatori di contenuti narrativi che gestiscono universi complessi con:
- Serie multiple (diari principali + contenuti extra)  
- Continuità temporale e narrativa
- Pubblicazione su piattaforme multiple
- Gestione metadati sofisticata

### ✨ Features Principali

- 🗃️ **Database SQLite** per metadati relazionali
- 🕐 **Timeline temporale** con conflict detection automatico
- 🔍 **Validazione lore** interattiva per coerenza narrativa
- 🌐 **Dashboard web** locale con interfaccia chat
- 📦 **Export multi-formato** (YouTube, Spotify, Instagram)
- 🔗 **Git integration** per sincronizzazione repository
- 🎭 **Multi-world support** per universi narrativi diversi

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
multiverse init --world wandering-sun --repo https://github.com/user/wandering-sun

# Crea prima serie di diari (firma F.M.)
multiverse diary create --world wandering-sun --name "fenrik_mealor" --narrator "Fenrik Mealor"

# Crea primo episodio
multiverse diary episode create --diary fenrik_mealor --number 001
```

### Avvia Dashboard Web

```bash
# Lancia interfaccia web su http://localhost:3000
multiverse serve --port 3000
```

## 📖 Architettura

### Struttura Repository

```
wandering-sun/               # Repository mondo
├── diari/                   # Serie principali (firma F.M.)
│   └── fenrik_mealor/
│       └── episodi/
│           ├── 001.md
│           ├── 002.md
│           └── 003.md
├── extra/                   # Contenuti extra (no firma F.M.)
│   └── lettere_lyra/
│       └── episodi/
│           ├── 001.md
│           └── 002.md
├── lore/                    # File worldbuilding
├── personaggi/             # Schede personaggi (JSON)
├── luoghi/                 # Schede luoghi (JSON)  
├── world.db                # Database SQLite metadati
└── world.json              # Configurazione mondo
```

### Database Schema

Il progetto utilizza **SQLite** per gestire metadati complessi:

- **`worlds`** - Configurazioni universi narrativi
- **`series`** - Diari e contenuti extra
- **`episodes`** - Episodi individuali
- **`characters`** - Personaggi con abilità/limitazioni
- **`locations`** - Luoghi con caratteristiche
- **`timeline_events`** - Eventi temporali estratti automaticamente
- **`temporal_conflicts`** - Inconsistenze temporali rilevate

## 🎮 Comandi CLI

### Gestione Mondi

```bash
multiverse init --world <nome> --repo <url>     # Inizializza mondo
multiverse pull --world <nome>                  # Sincronizza da Git  
multiverse push --world <nome>                  # Push modifiche
multiverse worlds list                          # Lista mondi locali
```

### Gestione Serie

```bash
# Diari principali (sempre firma F.M.)
multiverse diary create --world <mondo> --name <nome> --narrator <narratore>
multiverse diary list --world <mondo>
multiverse diary episode create --diary <nome> --number <num>

# Contenuti extra (no firma F.M.)
multiverse extra create --world <mondo> --name <nome> --type <tipo>
multiverse extra episode create --extra <nome> --number <num>
```

### Validazione e Pubblicazione

```bash
# Validazione lore interattiva
multiverse diary episode review --diary <nome> --episode <num>

# Gestione pubblicazioni
multiverse diary episode publish --diary <nome> --episode <num> --platform youtube --url <url>

# Export multi-formato  
multiverse diary export --diary <nome> --episode <num> --format youtube-description
```

### Timeline Temporale

```bash
# Estrazione eventi automatica
multiverse timeline extract --world <mondo>

# Visualizzazione timeline
multiverse timeline show --world <mondo> --format visual

# Verifica conflitti temporali
multiverse timeline conflicts --world <mondo>
```

## 🌐 Dashboard Web

La dashboard offre interfaccia **stile ChatGPT** per comandi Multiverse con output arricchiti:

### Tecnologie
- **Backend**: Rust + Axum
- **Frontend**: HTMX + Tailwind CSS
- **No JavaScript frameworks** - solo HTMX per interattività

### Features Dashboard

- 💬 **Chat interface** con autocompletamento intelligente
- 🎯 **Quick actions** - bottoni per transizioni stato episodi
- 📊 **Visual timeline** con conflict detection
- 🔍 **Live search** attraverso metadati
- 📝 **Command history** con replay
- 🎨 **Output arricchito** per ogni comando CLI

### Esempio Usage

```
┌─────────────────────────────────────┐
│ 💬 Multiverse CLI Dashboard        │
├─────────────────────────────────────┤
│ > multiverse diary list --world ws │
│ ┌─────────────────────────────────┐ │
│ │ 📚 Fenrik Mealor (5 episodi)   │ │
│ │ [📝 New Ep] [📊 Stats]         │ │
│ │ Ep 001: Biblioteca ✅          │ │
│ │ Ep 002: Viaggio 🔄 [→ Publish] │ │
│ │ Ep 003: Draft 📝 [→ Review]    │ │
│ └─────────────────────────────────┘ │
├─────────────────────────────────────┤
│ 💭 Scrivi comando... [Send]        │
└─────────────────────────────────────┘
```

## 🔍 Sistema Validazione Lore

### Pattern Matching Intelligente

Il sistema identifica automaticamente:
- **Nomi propri** (pattern maiuscole)
- **Azioni magiche** ("lanciò", "incantesimo", "magia")
- **Riferimenti geografici** ("arrivò a", "nella città di")
- **Eventi temporali** (età, date, stagioni)

### Workflow Interattivo

```bash
multiverse diary episode review --diary fenrik_mealor --episode 005
```

**Output:**
```
[Frase 12] "Fenrik lanciò un debole incantesimo di luce"

📋 PERSONAGGI COINVOLTI:
• Fenrik Mealor (umano) - capacità_magiche: false
  Limitazioni: no_lancio_incantesimi, no_manipolazione_mana

⚖️ VALUTAZIONE: Approva questa frase? [y/N/skip]: N
💬 Note: Umani non lanciano incantesimi
```

## 🕐 Timeline Temporale

### Auto-extraction Eventi

Il sistema estrae automaticamente eventi temporali da:
- Età personaggi ("45 anni", "quando aveva 30 anni")
- Date narrative ("Anno 1423", "Primavera")
- Riferimenti temporali ("6 mesi dopo", "2 anni fa")

### Conflict Detection

```sql
-- Trova conflitti età automaticamente
SELECT e1.title, e2.title, t1.event_description, t2.event_description
FROM timeline_events t1, timeline_events t2
WHERE t1.character_id = t2.character_id 
  AND t1.event_description LIKE '%anni%'
  AND t2.event_description LIKE '%anni%'
  AND t1.id != t2.id;
```

### Timeline Visuale

```
Anno 1420 ──●── Anno 1421 ──●── Anno 1422
             │               │
          Ep 001          Ep 002
        Biblioteca       Viaggio  
       Fenrik 45a       6m dopo   
            │               │
            └─── ⚠️ CONFLICT ───┘
                Ep 004: 43 anni
```

## 📦 Export Multi-formato

### Piattaforme Supportate

- **YouTube**: Titoli, descrizioni, hashtag ottimizzati
- **Spotify**: Descrizioni podcast-friendly  
- **Instagram**: Caption con hashtag e menzioni
- **Markdown**: Timeline, statistiche, reference

### Esempio Export

```bash
multiverse diary export --diary fenrik_mealor --episode 001 --format youtube-description
```

**Output:**
```
Diario di F.M. - Episodio 001: La Biblioteca Perduta

Fenrik inizia il suo viaggio nella misteriosa Biblioteca di Cogland...

#DiariDalMultiverso #FantasyPodcast #WanderingSun #Fenrik #Biblioteca
```

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

# Development server con hot reload
cargo run -- serve --dev --port 3000

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

### Fase 1: MVP ✅
- [x] CLI base con comandi core
- [x] Database SQLite schema
- [x] Dashboard web basilare
- [x] Export YouTube/Spotify

### Fase 2: Advanced Features 🚧
- [ ] Timeline temporale completa
- [ ] Lore validation interattiva
- [ ] Multi-world support
- [ ] Git integration avanzata

### Fase 3: Automazione 📅
- [ ] Pubblicazione automatica
- [ ] Analytics avanzate
- [ ] Template personalizzabili
- [ ] API integration piattaforme

## 📄 Licenza

MIT License - vedi [LICENSE](LICENSE) file per dettagli.

## 🤝 Supporto

- 📚 **Documentazione**: [docs/](docs/)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/user/multiverse-cli/issues)
- 💬 **Discussioni**: [GitHub Discussions](https://github.com/user/multiverse-cli/discussions)
- 📧 **Email**: support@multiverse-cli.dev

---

**Multiverse CLI** - Gestione professionale per universi narrativi complessi.