
pub const EXTRACTION_GUIDE: &str = r##"
# LLM Extraction Guide - Multiverse CLI Onboarding

## Situazione
Questo progetto contiene un mondo narrativo esistente che deve essere processato per l'uso con Multiverse CLI. Il sistema di gestione è già stato inizializzato, ma i contenuti esistenti devono essere analizzati e importati nel database.

## Il tuo compito
Analizza tutti i file di contenuto narrativo in questo progetto e genera i file SQL necessari per popolare il database della CLI.

## Processo step-by-step

### 1. ANALISI CONFIGURAZIONE MONDO
- Esamina i file esistenti per comprendere l'identità del mondo narrativo
- Se necessario, suggerisci aggiornamenti al file `.multiverse/config.toml`
- Determina l'estetica generale e il formato di numerazione preferito

**Domande da fare all'utente:**
- "Qual è il nome principale di questo mondo narrativo?"
- "Quale estetica rappresenta meglio questo mondo? (fantasy, moderna, storica, cyberpunk, etc.)"
- "Preferisci numerazione 001, 1, o formato romano I?"

### 2. SCANSIONE E CLASSIFICAZIONE STORIE
Cerca directory e file che rappresentano serie narrative distinte.

**Per ogni serie trovata, determina:**
- **Nome serie**: Nome identificativo (es. "fenrik_mealor", "lettere_lyra")
- **Titolo**: Titolo leggibile per la storia
- **Narratore**: Chi scrive/racconta la storia (da inserire nei metadati)
- **Tipo**: "diary" (con firma F.M.) o "extra" (senza firma F.M.)
- **Descrizione**: Breve descrizione della serie

**Domande da fare all'utente per ogni serie:**
- "Qual è il titolo della serie '{}'?"
- "Chi è il narratore principale di questa serie?"
- "Come descriveresti brevemente questa serie?"
- "Qual è lo stato attuale? (Active, Paused, Completed, Archived)"

### 3. ANALISI EPISODI
Per ogni file di episodio (.md) trovato nelle serie:

**Estrai automaticamente:**
- **Numero episodio**: Dal nome file (001.md → 1)
- **Contenuto**: Leggi il file per analisi

**Determina dal contenuto:**
- **Titolo**: Cerca patterns come "# Titolo", prime righe significative, o riferimenti nel testo
- **Personaggi menzionati**: Nomi propri che ricorrono
- **Luoghi visitati**: Riferimenti geografici
- **Eventi temporali**: Date, età, riferimenti temporali

**Domande da fare all'utente per ogni episodio:**
- "Il titolo che ho estratto '{}' è corretto per l'episodio {}?"
- "Qual è lo stato attuale di questo episodio? (Draft, InProgress, Review, Published)"
- "Hai una stima del numero di parole per questo episodio?"

### 4. IDENTIFICAZIONE PERSONAGGI
Per ogni personaggio unico identificato negli episodi:

**Raccogli informazioni dal testo:**
- **Nome completo**
- **Episodi in cui appare**
- **Azioni/comportamenti descritti**
- **Relazioni con altri personaggi**

**Domande da fare all'utente per ogni personaggio:**
- "Che razza/specie è '{}'? (umano, elfo, etc.)"
- "Che età ha approssimativamente?"
- "Ha abilità magiche? Se sì, quali?"
- "Ha limitazioni specifiche? (es. non può lanciare incantesimi)"
- "Qual è la sua professione o ruolo?"
- "Note aggiuntive sul personaggio?"

### 5. IDENTIFICAZIONE LUOGHI
Per ogni luogo unico identificato negli episodi:

**Raccogli informazioni dal testo:**
- **Nome del luogo**
- **Contesto geografico**
- **Episodi in cui viene visitato**
- **Caratteristiche descritte**

**Domande da fare all'utente per ogni luogo:**
- "Che tipo di luogo è '{}'? (città, edificio_pubblico, regno, etc.)"
- "Fa parte di una località più grande? Quale?"
- "Che caratteristiche speciali ha questo luogo?"
- "Qual è lo stato attuale? (attiva, distrutta, abbandonata)"
- "Note aggiuntive sul luogo?"

## Generazione File SQL

Dopo aver raccolto tutte le informazioni, crea i seguenti file nella directory `sql/`:

### sql/01_stories.sql
```sql
-- Serie narrative identificate
INSERT INTO stories (name, title, story_type, metadata, description, created_at, status) VALUES
('serie_nome', 'Titolo Storia', 'diary', '{"narrator":"Nome Narratore"}', 'Descrizione serie', datetime('now'), 'Active');
-- ... altre serie
```

### sql/02_episodes.sql  
```sql
-- Episodi per ogni serie
INSERT INTO episodes (story_name, episode_number, title, status, word_count, created_at, updated_at) VALUES
('serie_nome', 1, 'Titolo Episodio', 'Published', 1500, datetime('now'), datetime('now'));
-- ... altri episodi
```

### sql/03_characters.sql
```sql
-- Personaggi identificati
INSERT INTO characters (name, race, has_magic_abilities, abilities, limitations, age, origin, profession, notes) VALUES
('Nome Personaggio', 'umano', 0, '[]', '["no_lancio_incantesimi"]', 45, 'Città Origine', 'Professione', 'Note aggiuntive');
-- ... altri personaggi
```

### sql/04_locations.sql
```sql
-- Luoghi identificati  
INSERT INTO locations (name, type, parent_location_id, characteristics, status, notes) VALUES
('Nome Luogo', 'città', NULL, '["caratteristica1", "caratteristica2"]', 'attiva', 'Note aggiuntive');
-- ... altri luoghi
```

### sql/05_relations.sql
```sql
-- Relazioni episodi-personaggi-luoghi
INSERT INTO episode_characters (episode_id, character_id) VALUES
((SELECT id FROM episodes WHERE story_name='serie' AND episode_number=1), 
 (SELECT id FROM characters WHERE name='Nome Personaggio'));

INSERT INTO episode_locations (episode_id, location_id) VALUES  
((SELECT id FROM episodes WHERE story_name='serie' AND episode_number=1),
 (SELECT id FROM locations WHERE name='Nome Luogo'));
-- ... altre relazioni
```

## Note importanti

### Formato dati JSON
- **metadata**: Oggetto JSON `{"narrator":"Nome Narratore"}`
- **abilities**: Array JSON di stringhe `["abilità1", "abilità2"]`
- **limitations**: Array JSON di stringhe `["limitazione1", "limitazione2"]` 
- **characteristics**: Array JSON di stringhe `["caratteristica1", "caratteristica2"]`

### Convenzioni nomi
- **Serie**: snake_case (es. "fenrik_mealor", "lettere_lyra")
- **Personaggi**: Nome Proprio normale (es. "Fenrik Mealor")
- **Luoghi**: Nome Proprio normale (es. "Biblioteca di Cogland")

### Gestione ID
- Usa subquery per i foreign key (come negli esempi relations.sql)
- Non assumere ID specifici, sempre basarsi sui nomi

## Workflow finale

1. Crea tutti i file SQL nella directory `sql/`
2. Verifica che non ci siano errori di sintassi
3. Informa l'utente che può importare con: `multiverse world import --all`

## Cosa fare se non riesci a determinare qualcosa

Se durante l'analisi non riesci a determinare un'informazione:
1. **Per configurazione mondo**: Chiedi esplicitamente all'utente
2. **Per metadati episodi**: Usa valori ragionevoli di default e chiedi conferma
3. **Per personaggi/luoghi**: Non inventare, chiedi sempre conferma all'utente

Ricorda: è meglio chiedere troppo che assumere dati sbagliati!
"##;
