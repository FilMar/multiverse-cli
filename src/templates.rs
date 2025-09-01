
pub const EXTRACTION_GUIDE: &str = r##"# Guida AI per Estrazione Metadati - Multiverse CLI

## 🎯 Obiettivo
Guidare l'AI nell'estrazione automatica dei metadati da un mondo già sviluppato in file .md, con supervisione umana, per creare un database strutturato usando la Multiverse CLI.

## 📝 Processo di Estrazione in 6 Fasi

### Fase 1: Analisi Contestuale 📖

**Obiettivo**: Comprendere il mondo esistente leggendo tutti i file .md nella root del progetto.

#### Azioni da eseguire:
```bash
# Prima di tutto, esplorare la struttura del progetto
ls -la *.md
find . -name "*.md" -maxdepth 1
```

#### File da analizzare prioritariamente:
- `README.md` - Panoramica generale del mondo
- `WORLDBUILDING.md` - Dettagli del mondo
- `CHARACTERS.md` - Lista personaggi
- `LOCATIONS.md` - Geografia e luoghi
- `TIMELINE.md` - Eventi storici
- `LORE.md` - Mitologia e background
- Qualsiasi altro file .md nella root

#### Domande da porre durante l'analisi:
1. **Genere e ambientazione**: Fantasy, SciFi, Storico, Moderno?
2. **Scala temporale**: Quanto tempo copre la narrazione?
3. **Geografia**: Quanti continenti/pianeti/regioni?
4. **Politica**: Quanti regni/fazioni/organizzazioni?
5. **Personaggi**: Chi sono i protagonisti/antagonisti principali?
6. **Magia/Tecnologia**: Che tipo di sistema esiste?

### Fase 2: Configurazione Mondo 🌍

**Obiettivo**: Aggiornare il file `.multiverse/config.toml` esistente con le informazioni base estratte.

#### Sezioni config.toml da aggiornare:
```toml
[world]
name = "nome_mondo_estratto"  # Modifica se necessario
display_name = "Nome Completo Del Mondo"  # Aggiorna con info dai .md
genre = "Fantasy|SciFi|Historical|Modern|etc"  # Estrai dai file
era = "medieval|modern|future|ancient"
scale = "local|regional|continental|planetary|galactic"

[metadata]
author = "nome_autore"  # Estrai da README o chiedi
created_date = "YYYY-MM-DD"  # Data dal git o file
version = "1.0"
description = "Descrizione breve del mondo estratta dai .md"

[themes]
primary = ["theme1", "theme2", "theme3"]  # Identifica temi principali
secondary = ["theme4", "theme5"]

[settings]
magic_level = "none|low|medium|high"  # Analizza presenza magia
technology_level = "stone_age|medieval|industrial|modern|future"
complexity_level = "simple|medium|complex"  # Basato su entità trovate

# LE ALTRE SEZIONI ([git], [story_types], [timeline]) RIMANGONO INVARIATE
```

#### Processo di aggiornamento:
```bash
# 1. Leggi il config esistente
cat .multiverse/config.toml

# 2. Identifica le sezioni da aggiornare (world, metadata, themes, settings)
# 3. Mantieni intatte: [git], [story_types], [timeline]
# 4. Chiedi conferma all'umano prima di modificare
# 5. Backup del config originale prima di modificare

cp .multiverse/config.toml .multiverse/config.toml.backup
```

### Fase 3: Identificazione Entità 🏗️

**Obiettivo**: Dividere il contenuto in categorie di entità Multiverse CLI.

#### Sistema di categorizzazione:

**📝 CHARACTERS** - Cerca nei testi:
- Nomi propri di persona
- Titoli (Re, Regina, Lord, etc.)  
- Pronomi personali con nomi
- Descrizioni di personalità/aspetto
- Relazioni familiari

**🏰 LOCATIONS** - Cerca nei testi:
- Nomi di città, regni, continenti
- Luoghi specifici (castelli, taverne, foreste)
- Descrizioni geografiche
- Riferimenti a "a/in/da [luogo]"

**⚔️ FACTIONS** - Cerca nei testi:
- Case nobiliari, gilde, ordini
- Organizzazioni, eserciti, clan
- Gruppi con obiettivi comuni
- Alleanze e rivalità

**🧬 RACES** - Cerca nei testi:
- Specie diverse (umani, elfi, alieni)
- Caratteristiche fisiche uniche
- Abilità speciali razziali
- Origini e culture diverse

**🌟 SYSTEMS** - Cerca nei testi:
- Sistemi magici, politici, stellari
- Regole del mondo (leggi della fisica/magia)
- Strutture di potere
- Sistemi economici/sociali

**⚡ EVENTS** - Cerca nei testi:
- Date specifiche o riferimenti temporali
- Battaglie, incoronazioni, catastrofi
- "X anni fa", "durante la guerra di", etc.
- Cambiamenti di status quo

**📚 STORIES/EPISODES** - Cerca nei testi:
- Capitoli, parti, libri
- Archi narrativi
- Sequenze temporali di eventi
- POV (Point of View) changes

### Fase 4: Estrazione Dettagliata 🔍

**Obiettivo**: Per ogni entità identificata, estrarre il massimo dei dettagli possibili.

#### Template di estrazione per CHARACTERS:
```bash
# Informazioni base:
- name: identificatore_unico_snake_case
- display_name: "Nome Completo Come Scritto"
- status: Active|Inactive|Deceased|Archived

# Metadata da cercare nei testi:
- age: numero o range (es. "giovane", "anziano")  
- profession: lavoro/ruolo sociale
- personality: tratti caratteriali
- appearance: descrizione fisica
- skills: abilità speciali
- weapon: armi favorite
- titles: titoli nobiliari/sociali

# Relazioni da identificare:
- location: dove vive/è nato (*resident, *born_in)
- faction: a quale gruppo appartiene (*member, *leader, *enemy)
- race: che specie è (*is_of_race)
- character: relazioni con altri personaggi (*family, *friend, *enemy)
```

#### Template di estrazione per LOCATIONS:
```bash
# Informazioni base:
- name: identificatore_snake_case
- display_name: "Nome Geografico Completo"
- status: Active|Inactive|Destroyed|Hidden|Unknown

# Metadata da cercare:
- type: city|castle|forest|mountain|etc
- region: area geografica più ampia
- climate: caldo|freddo|temperato|etc
- population: size o numero abitanti
- government: tipo di governo locale
- economy: base economica principale

# Relazioni da identificare:
- location: luoghi parent/child (*part_of, *contains)
- faction: chi controlla (*controls, *seat_of)
- character: chi ci vive (*rules, *born_here)
```

#### Sistema di Interazione con l'Umano:
```
🤖 AI: "Ho trovato 15 personaggi nei tuoi file. Per 'Aragorn' ho estratto:
      - age: ~30 anni (dal testo 'giovane ranger')  
      - profession: ranger, re
      - location: nato a Gondor, vive nelle terre selvagge
      
      ❓ Domanda: Come vuoi gestire il suo status di re? 'Active' o creare un evento 'incoronazione'?"

👤 UMANO: "Crea l'evento incoronazione e metti Aragorn come Active con profession=king"

🤖 AI: "✅ Perfetto! Procedo con la prossima entità..."
```

### Fase 5: Generazione Comandi CLI 📜

**Obiettivo**: Tradurre i dati estratti in comandi Multiverse CLI seguendo il CHEATSHEET.md.

#### Struttura del file import.sh:
```bash
#!/bin/bash
# Auto-generato dall'AI - Importazione metadati mondo: NOME_MONDO
# Generato il: $(date)
# Fonte: File .md nella root del progetto

echo "🌍 Importazione mondo: NOME_MONDO"
echo "📊 Entità da importare: X characters, Y locations, Z factions..."

# ======================
# FASE 1: ENTITÀ BASE  
# ======================

echo "👥 Creando personaggi..."
# SEMPRE usa --set per TUTTI i campi (no --display-name!)
multiverse character create aragorn \
  --set display_name="Aragorn II Elessar" \
  --set status=Active \
  --set age=30 \
  --set profession=king \
  --set titles="King of Gondor"

multiverse character create gandalf \
  --set display_name="Gandalf the Grey" \
  --set status=Active \
  --set race=maiar \
  --set profession=wizard

echo "🏰 Creando luoghi..."
multiverse location create minas_tirith \
  --set display_name="Minas Tirith" \
  --set status=Active \
  --set type=city \
  --set region=gondor

echo "⚔️ Creando fazioni..."  
multiverse faction create gondor \
  --set display_name="Kingdom of Gondor" \
  --set status=Active \
  --set type=kingdom \
  --set government=monarchy

# ======================
# FASE 2: RELAZIONI
# ======================

echo "🔗 Stabilendo relazioni..."
# Sintassi: --set entity_type=target*relationship_type
multiverse character update aragorn \
  --set location=minas_tirith*rules \
  --set faction=gondor*leader \
  --set race=human*is_of_race

multiverse location update minas_tirith \
  --set faction=gondor*capital_of

# ======================
# FASE 3: EVENTI STORICI  
# ======================

echo "⚡ Creando timeline..."
multiverse event create war_of_ring \
  --set display_name="War of the Ring" \
  --set date_text="Third Age 3019" \
  --set sort_key=3019 \
  --set status=Completed

multiverse event update war_of_ring \
  --set character=aragorn*hero \
  --set location=minas_tirith*final_battle

# ======================
# FASE 4: STORIE/EPISODI
# ======================

echo "📚 Creando struttura narrativa..."
multiverse story create lotr_main \
  --set display_name="The Lord of the Rings" \
  --set story_type="Epic Fantasy" \
  --set status=Published

multiverse episode create lotr_main 1 \
  --set title="The Fellowship of the Ring" \
  --set status=Published

multiverse episode update lotr_main:1 \
  --set character=aragorn*protagonist

echo "✅ Importazione completata!"
echo "📊 Verifica risultati con: multiverse query 'SELECT COUNT(*) FROM characters'"
```

#### Comandi di verifica automatica:
```bash
# Aggiungi sempre alla fine del file import.sh
echo "🔍 Verifiche automatiche..."

# Conteggi entità
multiverse query "SELECT 'characters' as type, COUNT(*) as count FROM characters
UNION SELECT 'locations', COUNT(*) FROM locations  
UNION SELECT 'factions', COUNT(*) FROM factions
UNION SELECT 'events', COUNT(*) FROM events"

# Controlli di consistenza (vedi AI_COLAB per query avanzate)
multiverse query "SELECT c.display_name as orphaned_character 
FROM characters c LEFT JOIN character_faction_relations r ON c.id = r.from_id 
WHERE r.from_id IS NULL"
```

### Fase 6: Supervisione e Personalizzazione 🎛️

**Obiettivo**: Dare controllo completo all'umano per personalizzare l'estrazione.

#### Opzioni di personalizzazione:

**🎯 Filtraggio per tipo di entità:**
```bash
# L'umano può dire: "Estrai solo personaggi e luoghi"
🤖 AI: "Genero import_characters_locations.sh con solo queste entità"

# Oppure: "Prima fai solo i personaggi principali"
🤖 AI: "Genero import_main_characters.sh con i primi 5 personaggi"
```

**📂 Divisione in più file:**
```bash
# L'umano può richiedere divisione logica:
- import_characters.sh (solo personaggi)
- import_world.sh (locations + factions + systems)  
- import_timeline.sh (events)
- import_stories.sh (stories + episodes)
```

**🔧 Controllo granulare:**
```bash
# L'AI deve sempre chiedere:
🤖 AI: "Ho identificato 47 personaggi. Vuoi che:
      1. Li importi tutti in un unico file?
      2. Li divida per importanza (main/secondary/minor)?  
      3. Li raggruppi per fazione?
      4. Ti mostri la lista per selezione manuale?"

👤 UMANO: "Opzione 2, e genera prima solo i main characters"
```

**⚙️ Personalizzazione comandi:**
```bash
# L'AI deve adattare la complessità:
- Modalità SEMPLICE: Solo campi base (name, display_name, status)
- Modalità COMPLETA: Tutti i metadata possibili  
- Modalità CUSTOM: L'umano specifica quali campi includere
```

## 🚨 Regole Importanti

### ✅ DA FARE:
1. **Sempre usare `--set` per TUTTI i campi** (no --display-name flag!)
2. **Sintassi relazioni corretta**: `entity=target*relationship`
3. **Status values corretti** per ogni tipo di entità
4. **Chiedere conferma** prima di generare file grandi
5. **Verificare sintassi comandi** contro CHEATSHEET.md
6. **Aggiungere controlli di verifica** al termine

### ❌ NON FARE:
1. **Mai sovrascrivere** dati esistenti senza chiedere
2. **Non inventare informazioni** non presenti nei .md
3. **Non usare flag inesistenti** (es. --display-name)
4. **Non generare relazioni** senza evidenza testuale
5. **Non procedere** senza supervisione umana

### 🔍 Controlli di Qualità:
```bash
# L'AI deve sempre includere questi controlli finali:
echo "🔍 Controlli di qualità..."

# Verifica sintassi comandi
bash -n import.sh

# Test connessione database  
multiverse info

# Verifica importazione
multiverse query "SELECT COUNT(*) as total_entities FROM (
  SELECT 'char' as type FROM characters 
  UNION ALL SELECT 'loc' FROM locations
  UNION ALL SELECT 'fact' FROM factions
  UNION ALL SELECT 'race' FROM races  
  UNION ALL SELECT 'sys' FROM systems
  UNION ALL SELECT 'evt' FROM events
  UNION ALL SELECT 'story' FROM stories
  UNION ALL SELECT 'ep' FROM episodes
)"
```

## 🎯 Risultato Finale

Il processo deve produrre:
1. **.multiverse/config.toml** aggiornato con metadata del mondo
2. **import.sh** (o file multipli) con tutti i comandi CLI
3. **report_extraction.md** con summary dell'estrazione
4. **verification_queries.sh** per controlli di qualità

L'AI deve sempre mantenere il controllo umano al centro del processo, chiedendo conferme e offrendo opzioni di personalizzazione ad ogni fase.
"##;

pub const INTEGRATION_GUIDE: &str = r##""##;

pub const AI_COLAB: &str = r##"# Guida AI Collaboration - Multiverse CLI per Produzione Creativa

## 🎯 Obiettivo
Guidare l'AI nell'uso della Multiverse CLI durante la produzione di storie e worldbuilding, e nell'esecuzione di controlli avanzati tramite query SQL.

## 📝 Workflow Creativo con CLI

### Fase 1: Inizializzazione Mondo
```bash
# Crea nuovo mondo/progetto
multiverse world init my_fantasy_world
cd my_fantasy_world

# Configura git (opzionale)  
multiverse world config git.remote "https://github.com/user/my_fantasy_world.git"
multiverse world push  # primo push

# Verifica struttura creata
multiverse world info
```

### Fase 2: Sviluppo Worldbuilding Iterativo

#### Creazione Entità Base
```bash
# SEMPRE crea entità base PRIMA di scrivere storie
multiverse character create john_snow --display-name "John Snow" --status Active
multiverse character create daenerys --display-name "Daenerys Targaryen" --status Active

multiverse location create winterfell --display-name "Winterfell" --status Active  
multiverse location create kings_landing --display-name "King's Landing" --status Active

multiverse faction create stark_house --display-name "House Stark" --status Active
multiverse faction create targaryen_house --display-name "House Targaryen" --status Disbanded

multiverse race create human --display-name "Human" --status Active
multiverse race create dragon --display-name "Dragon" --status Legendary
```

#### Aggiunta Metadata durante Sviluppo
```bash
# Aggiungi dettagli progressivamente mentre sviluppi
multiverse character update john_snow --set profession=lord_commander --set house=stark --set age=25

multiverse location update winterfell --set type=castle --set region=north --set climate=cold

multiverse faction update stark_house --set motto="Winter is Coming" --set seat=winterfell --set allegiance=north
```

#### Creazione Relazioni
```bash
# Collega entità man mano che sviluppi relazioni
multiverse character update john_snow --set location=winterfell*resident
multiverse character update john_snow --set faction=stark_house*bastard_son  
multiverse character update john_snow --set race=human*pure

multiverse location update winterfell --set faction=stark_house*ancestral_seat
```

### Fase 3: Sviluppo Narrativo

#### Struttura Storie ed Episodi
```bash
# Crea storia principale
multiverse story create got_main --display-name "Game of Thrones" --type "Epic Fantasy" --status InProgress

# Crea episodi/capitoli progressivamente  
multiverse episode create got_main 1 --title "The Beginning" --status Draft
multiverse episode create got_main 2 --title "The Road South" --status Draft

# Aggiungi metadata narrativi
multiverse story update got_main --set era=medieval_fantasy --set world=westeros --set themes="power,betrayal,honor"
multiverse episode update got_main:1 --set location=winterfell --set characters="john_snow,daenerys" --set themes=introduction
```

#### Eventi Storici e Timeline
```bash
# Crea eventi chiave per consistency
multiverse event create roberts_rebellion --display-name "Robert's Rebellion" --date "15 years ago" --status Completed
multiverse event create red_wedding --display-name "The Red Wedding" --date "Year 3" --status Pending

# Collega eventi a personaggi/luoghi
multiverse event update roberts_rebellion --set participants="ned_stark,robert_baratheon" --set outcome=success
multiverse event update red_wedding --set location=the_twins --set victims=stark_house
```

### Fase 4: Controlli di Consistenza

#### Query di Verifica Base
```bash
# Verifica completezza worldbuilding
multiverse query "SELECT COUNT(*) as total_characters FROM characters"
multiverse query "SELECT status, COUNT(*) FROM characters GROUP BY status"

# Controlla relazioni mancanti
multiverse query "
SELECT c.display_name as character 
FROM characters c 
LEFT JOIN character_location_relations clr ON c.id = clr.from_id 
WHERE clr.from_id IS NULL"

# Verifica consistenza fazioni
multiverse query "
SELECT c.display_name as character, f.display_name as faction, r.relationship_type
FROM characters c
JOIN character_faction_relations r ON c.id = r.from_id  
JOIN factions f ON f.id = r.to_id
WHERE f.status = 'Disbanded' AND r.relationship_type = 'member'"
```

## 🔍 Sistema Query Avanzato per Controlli

### Schema Completo Database

#### Tabelle Entità
```sql
-- CHARACTERS: Personaggi
CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- john_snow
    display_name TEXT NOT NULL,          -- John Snow  
    metadata TEXT DEFAULT '{}',          -- {"profession":"lord_commander","house":"stark"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active'         -- Active|Inactive|Deceased|Archived
);

-- LOCATIONS: Luoghi
CREATE TABLE locations (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- winterfell
    display_name TEXT NOT NULL,          -- Winterfell
    metadata TEXT DEFAULT '{}',          -- {"type":"castle","region":"north"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active'         -- Active|Inactive|Destroyed|Hidden|Unknown
);

-- SYSTEMS: Sistemi (stellari/magici/politici)
CREATE TABLE systems (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- westeros_political
    display_name TEXT NOT NULL,          -- Westeros Political System
    system_type TEXT NOT NULL,           -- political|magical|stellar
    metadata TEXT DEFAULT '{}',          -- {"government":"feudalism","magic_level":"low"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active'         -- Active|Inactive|Deprecated|Archived
);

-- FACTIONS: Casate/Organizzazioni
CREATE TABLE factions (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- stark_house
    display_name TEXT NOT NULL,          -- House Stark
    metadata TEXT DEFAULT '{}',          -- {"motto":"Winter is Coming","seat":"winterfell"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active'         -- Active|Inactive|Disbanded|Allied|Hostile
);

-- RACES: Specie/Razze  
CREATE TABLE races (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- human
    display_name TEXT NOT NULL,          -- Human
    metadata TEXT DEFAULT '{}',          -- {"lifespan":80,"magic_affinity":"low"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active'         -- Active|Inactive|Extinct|Legendary|Mythical
);

-- EVENTS: Eventi Storici
CREATE TABLE events (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- roberts_rebellion
    display_name TEXT NOT NULL,          -- Robert's Rebellion
    date_text TEXT DEFAULT '',           -- "15 years ago"
    sort_key INTEGER DEFAULT 0,          -- Ordinamento cronologico
    metadata TEXT DEFAULT '{}',          -- {"outcome":"success","duration":"1_year"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active'         -- Active|Inactive|Completed|Cancelled|Pending
);

-- STORIES: Archi Narrativi
CREATE TABLE stories (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- got_main
    display_name TEXT NOT NULL,          -- Game of Thrones
    story_type TEXT DEFAULT 'Fantasy',   -- Epic Fantasy|Urban Fantasy|SciFi|etc
    word_count INTEGER DEFAULT 0,
    metadata TEXT DEFAULT '{}',          -- {"era":"medieval","themes":"power,betrayal"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Draft'          -- Draft|InProgress|Review|Published|Archived
);

-- EPISODES: Capitoli/Episodi
CREATE TABLE episodes (
    id INTEGER PRIMARY KEY,
    story TEXT NOT NULL,                 -- FK: got_main
    number INTEGER NOT NULL,             -- 1, 2, 3...
    title TEXT DEFAULT '',               -- "The Beginning"
    word_count INTEGER DEFAULT 0,
    metadata TEXT DEFAULT '{}',          -- {"location":"winterfell","pov":"john_snow"}
    created_at TEXT NOT NULL,
    status TEXT DEFAULT 'Draft',         -- Draft|InProgress|Review|Published
    UNIQUE(story, number)
);
```

#### Tabelle Relazioni
```sql
-- Pattern: {from_entity}_{to_entity}_relations
CREATE TABLE character_location_relations (
    from_id TEXT NOT NULL,              -- characters.id
    to_id TEXT NOT NULL,                -- locations.id
    relationship_type TEXT,             -- resident|born_in|visited|rules
    metadata TEXT DEFAULT '{}'          -- {"duration":"5_years","status":"current"}
);

CREATE TABLE character_faction_relations (
    from_id TEXT NOT NULL,              -- characters.id  
    to_id TEXT NOT NULL,                -- factions.id
    relationship_type TEXT,             -- member|leader|ally|enemy|bastard_son
    metadata TEXT DEFAULT '{}'          -- {"rank":"lord_commander","sworn":"yes"}
);

CREATE TABLE location_faction_relations (
    from_id TEXT NOT NULL,              -- locations.id
    to_id TEXT NOT NULL,                -- factions.id  
    relationship_type TEXT,             -- ancestral_seat|stronghold|conquered|lost
    metadata TEXT DEFAULT '{}'          -- {"control_years":"300","fortification":"high"}
);

CREATE TABLE event_character_relations (
    from_id TEXT NOT NULL,              -- events.id
    to_id TEXT NOT NULL,                -- characters.id
    relationship_type TEXT,             -- participant|leader|victim|hero|villain
    metadata TEXT DEFAULT '{}'          -- {"role":"commander","survival":"yes"}
);

-- ... e tutte le altre combinazioni secondo necessità
```

### Query Avanzate per Controlli Narrativi

#### Controlli di Consistenza Worldbuilding
```bash
# 1. Personaggi senza casa/fazione
multiverse query "
SELECT c.display_name as orphaned_character
FROM characters c  
LEFT JOIN character_faction_relations cfr ON c.id = cfr.from_id
WHERE cfr.from_id IS NULL AND c.status = 'Active'"

# 2. Luoghi senza controllo politico  
multiverse query "
SELECT l.display_name as uncontrolled_location
FROM locations l
LEFT JOIN location_faction_relations lfr ON l.id = lfr.from_id  
WHERE lfr.from_id IS NULL AND l.status = 'Active'"

# 3. Eventi senza partecipanti
multiverse query "
SELECT e.display_name as empty_event
FROM events e
LEFT JOIN event_character_relations ecr ON e.id = ecr.from_id
WHERE ecr.from_id IS NULL"

# 4. Fazioni senza membri  
multiverse query "
SELECT f.display_name as memberless_faction, f.status
FROM factions f
LEFT JOIN character_faction_relations cfr ON f.id = cfr.to_id
WHERE cfr.to_id IS NULL AND f.status IN ('Active', 'Allied')"
```

#### Analisi Narrative Complex
```bash
# 5. Mappe relazioni per personaggio
multiverse query "
SELECT 
    c.display_name as character,
    'lives_in' as relation_type,
    l.display_name as target
FROM characters c
JOIN character_location_relations clr ON c.id = clr.from_id
JOIN locations l ON l.id = clr.to_id
WHERE c.name = 'john_snow'
UNION ALL  
SELECT 
    c.display_name,
    'member_of',
    f.display_name
FROM characters c
JOIN character_faction_relations cfr ON c.id = cfr.from_id  
JOIN factions f ON f.id = cfr.to_id
WHERE c.name = 'john_snow'"

# 6. Timeline eventi per location
multiverse query "
SELECT 
    e.display_name as event,
    e.date_text,
    e.sort_key,
    l.display_name as location,
    elr.relationship_type
FROM events e
JOIN event_location_relations elr ON e.id = elr.from_id
JOIN locations l ON l.id = elr.to_id  
WHERE l.name = 'winterfell'
ORDER BY e.sort_key"

# 7. Conflict matrix tra fazioni
multiverse query "
SELECT 
    f1.display_name as faction1,
    f2.display_name as faction2,
    'conflict' as relationship
FROM factions f1, factions f2
WHERE f1.status = 'Hostile' AND f2.status = 'Active' 
AND f1.id != f2.id"
```

#### Controlli Editoriali  
```bash
# 8. Episodi senza personaggi assegnati
multiverse query "
SELECT 
    s.display_name as story,
    e.number,
    e.title,
    'missing_characters' as issue
FROM stories s
JOIN episodes e ON s.name = e.story
LEFT JOIN character_episode_relations cer ON e.id = cer.to_id
WHERE cer.to_id IS NULL"

# 9. Word count progression
multiverse query "
SELECT 
    e.story,
    e.number,
    e.title,  
    e.word_count,
    SUM(e.word_count) OVER (PARTITION BY e.story ORDER BY e.number) as cumulative_words
FROM episodes e
ORDER BY e.story, e.number"

# 10. Status inconsistencies  
multiverse query "
SELECT 
    'character' as entity_type,
    c.display_name as name,
    c.status as current_status,
    'appears_in_future_events' as issue
FROM characters c
JOIN event_character_relations ecr ON c.id = ecr.to_id
JOIN events e ON e.id = ecr.from_id  
WHERE c.status = 'Deceased' AND e.status = 'Pending'"
```

## 🎯 Best Practices per Produzione

### Durante Worldbuilding:
1. **Crea entità base PRIMA** di sviluppare storie  
2. **Usa metadata incrementalmente** - aggiungi dettagli mentre sviluppi
3. **Verifica relazioni regolarmente** con query di consistency
4. **Mantieni timeline coerente** usando eventi e sort_key

### Durante Scrittura:
1. **Collega episodi a entità** per tracking automatico
2. **Usa query per plot holes** - controlla inconsistenze prima di scrivere
3. **Monitor word count progression** per pacing
4. **Track character arcs** attraverso relazioni events-characters

### Per Controlli Qualità:
1. **Query di completezza** - verifica coverage worldbuilding  
2. **Consistency checks** - controlla status e timeline
3. **Narrative analysis** - mappe relazioni e conflict detection
4. **Editorial review** - episodi completi, word count, character presence

L'AI può suggerire query SQL specifiche basate sui bisogni narrativi e aiutare a interpretare i risultati per migliorare consistency e depth del worldbuilding.
"##;
