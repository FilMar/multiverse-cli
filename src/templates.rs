
pub const EXTRACTION_GUIDE: &str = r##"# Guida all'Estrazione e Preparazione del Mondo per Multiverse CLI

**Obiettivo**: Configurare un nuovo progetto Multiverse e importare una base di conoscenza testuale (file Markdown) estraendo entità di worldbuilding (personaggi, luoghi, etc.), convertendole in file `.sql` pronti per l'importazione.

## Workflow Generale

1.  **Configura il Mondo**: Imposta i dettagli base del mondo nel file `.multiverse/config.toml`.
2.  **Configura la Timeline**: Se il mondo ha un calendario personalizzato, crea e configura il file `.multiverse/timeline.toml`.
3.  **Estrai Entità in SQL**: Analizza i file `.md`, estrai le entità e convertile in istruzioni `INSERT` SQL, salvandole in file numerati nella cartella `sql/`.

---

## Passo 1: Configurazione del Mondo (`config.toml`)

Prima di tutto, devi configurare le informazioni di base del mondo.

**Azione**: Chiedi all'utente il nome del mondo e una sua breve descrizione.

**Esempio di interazione**:
> Utente: "Iniziamo a importare il mio mondo, Aethel."
> Tu: "Perfetto. Potresti darmi una breve descrizione di Aethel?"
> Utente: "È un mondo fantasy dark, post-apocalittico, dove la magia sta morendo."

Una volta ottenute le informazioni, usa i seguenti comandi per aggiornare la configurazione:

```bash
multiverse world config --set world.name --value "Aethel"
multiverse world config --set world.description --value "Un mondo fantasy dark, post-apocalittico, dove la magia sta morendo."
```

---

## Passo 2: Configurazione della Timeline (`timeline.toml`)

Ora, determina se il mondo usa un calendario standard o uno personalizzato.

**Azione**: Chiedi all'utente che tipo di calendario usa il mondo.

**Esempio di interazione**:
> Tu: "Il tuo mondo, Aethel, usa un calendario standard (Gregoriano, come il nostro) o ne ha uno personalizzato (con mesi, ere e nomi dei giorni specifici)?"

-   **Caso A: Calendario Standard**: Se l'utente risponde "standard", "reale", o simile, **non fare nulla**. Salta questo passo e procedi al Passo 3. Il sistema userà le date standard.

-   **Caso B: Calendario Personalizzato**: Se l'utente conferma di avere un calendario personalizzato, devi creare il file `.multiverse/timeline.toml`.

    **Azione**: Chiedi all'utente i dettagli del suo calendario (o chiedigli di fornirti un file con le regole). Raccogli informazioni su:
    - Nomi dei mesi e loro abbreviazioni.
    - Numero di mesi all'anno e di giorni al mese/settimana.
    - Nomi delle ere e l'anno in cui iniziano.
    - Nomi dei "blocchi" del giorno (es. Mattina, Pomeriggio, Sera).

    Usa queste informazioni per creare un file `.multiverse/timeline.toml` come segue.

    **Esempio di `timeline.toml`**:
    ```toml
    # Anno di riferimento per l'inizio del conteggio assoluto
    creation_year = 0

    # Configurazione base del calendario
    [calendar]
    name = "Calendario Aetheleano"
    year_name = "Ciclo"
    year_days = 360
    months_per_year = 12
    days_per_month = 30
    weeks_per_month = 3
    week_name = "Decade"
    days_per_week = 10
    day_name = "Alba"

    # Suddivisione della giornata
    [day_structure]
    blocks_per_day = 5
    candles_per_block = 4

    # Formato di visualizzazione delle date
    [date_formats]
    full = "{day_name} {penta}/{month} {year} {era}"
    abbreviated = "{penta}A/{month_abbrev} {year} {era_abbrev}"

    # Nomi e abbreviazioni dei blocchi del giorno
    [day_blocks]
    names = ["Crepuscolo", "Aurora", "Meriggio", "Vespro", "Notte"]
    abbrevs = ["Cr", "Au", "Me", "Ve", "No"]
    meanings = ["Prime ore del mattino", "Mattina", "Mezzogiorno", "Pomeriggio", "Notte"]

    # Nomi e abbreviazioni dei mesi
    [months]
    names = ["Primus", "Secundus", "Tertius", "Quartus", "Quintus", "Sextus", "Septimus", "Octavus", "Nonus", "Decimus", "Undecimus", "Duodecimus"]
    abbrevs = ["Pri", "Sec", "Ter", "Qua", "Qui", "Sex", "Sep", "Oct", "Non", "Dec", "Und", "Duo"]
    meanings = ["Mese della semina", "Mese della crescita", ...]

    # Definizione delle Ere
    [era_events]
    [era_events.diaspora]
    name = "Diaspora"
    abbrev = "DF" # Dopo la Frammentazione
    year = 1200 # Anno assoluto in cui è iniziata la Diaspora

    [era_events.foundation]
    name = "Fondazione"
    abbrev = "AF" # Anni della Fondazione
    year = 0
    ```

---

## Passo 3: Estrazione delle Entità in SQL

Questo è il passo finale. Analizza i file di testo e converti le entità in istruzioni SQL.

**Azione**: Leggi i file `.md`, identifica le entità, chiedi chiarimenti se necessario, e genera i file `.sql` nella cartella `sql/`.

(Il resto della guida da qui in poi rimane come quella che hai generato in precedenza, con le istruzioni per `characters`, `locations`, `systems`, `factions`, `events`, la gestione delle informazioni mancanti e la struttura dei file SQL.)
"##;

pub const AI_COLAB: &str = r##"# Guida alla Collaborazione AI con Multiverse CLI

**Obiettivo**: Questa guida ti spiega come usare i comandi di `multiverse-cli` per costruire e gestire un mondo narrativo in modo programmatico. Il tuo ruolo è tradurre le richieste dell'utente in comandi CLI per popolare il database del mondo.

## Concetto Fondamentale: La CLI è la Fonte della Verità

Tutta la conoscenza strutturata del mondo (chi sono i personaggi, dove si trovano i luoghi, etc.) deve essere gestita tramite comandi CLI. Non modificare mai direttamente il database. Usa i comandi per creare, aggiornare e interrogare il mondo.

---

## Gestione di Storie ed Episodi

Le narrazioni sono organizzate in Storie, che contengono Episodi.

1.  **Crea una Storia**: Una storia è un contenitore per una serie di eventi o narrazioni. Può avere metadati propri.
    ```bash
    multiverse story create le-cronache-di-aethel --type book --set genre=Fantasy --set author="Fenrik Mealor"
    ```

2.  **Crea un Episodio**: Un episodio è un singolo file `.md` dove scriverai il contenuto narrativo. La CLI gestisce la numerazione per te.
    ```bash
    multiverse episode create --story le-cronache-di-aethel --title "L'Alba della Rovina"
    ```
    Questo comando creerà un file come `stories/le-cronache-di-aethel/001.md`.

---

## Sincronizzazione: La Regola più Importante

Dopo che hai scritto o modificato il contenuto di un file di episodio (`.md`), il tuo lavoro non è finito. Devi aggiornare il database con le nuove informazioni che hai introdotto.

**Workflow Chiave**: Scrivi la storia -> Estrai le entità -> Aggiorna il database con la CLI.

**Esempio**:

1.  **Richiesta Utente**: "Nel nuovo episodio, Kaelen forgia una spada magica chiamata 'Luce Morente'."
2.  **Azione 1: Scrivi nel File**: Apri il file `.md` dell'episodio e scrivi la scena.
3.  **Azione 2: Estrai e Aggiorna**:
    -   **Pensa**: "L'utente ha introdotto un nuovo 'sistema' (un oggetto magico). Devo aggiungerlo al database."
    -   **Esegui il Comando**:
        ```bash
        multiverse system create luce_morente --display-name "Luce Morente" --type "Magic Item" --set creator=kaelen --set "first_appearance=le-cronache-di-aethel/001"
        ```

### Gestire le Relazioni Personaggio-Episodio (Workaround Attuale)

**Attenzione**: Al momento, non esiste un comando diretto per collegare un personaggio a un episodio (es. `multiverse character episode add ...`). Questa funzionalità è prevista per il futuro.

**Soluzione Temporanea**: Usa i metadati per registrare la partecipazione di un personaggio a un episodio. Aggiungi un campo `episodes` o `appearances` al personaggio.

**Esempio**:
> Se Kaelen appare nell'episodio 1 e 2, puoi aggiornare il suo profilo così:

```bash
# (Questo comando non esiste ancora, è un esempio di come potrebbe funzionare in futuro)
# multiverse character update kaelen --set "episodes=['001', '002']"

# Per ora, puoi solo aggiungere metadati alla creazione.
# Quindi, quando crei un personaggio, puoi annotare la sua prima apparizione.
multiverse character create kaelen --display-name "Kaelen" --set "first_appearance=le-cronache-di-aethel/001"
```

---

## Comandi Essenziali per la Creazione

(Il resto della guida rimane invariato: Il Potere dei Metadati, Riferimenti ai Comandi, Esempio di Workflow, Leggere i Dati, Regole d'Oro)

### Il Potere dei Metadati con `--set`

Il flag `--set` è il tuo strumento più potente. Ti permette di aggiungere qualsiasi dato strutturato a un'entità. La sintassi è `key=value`. Se il valore contiene spazi, usa le virgolette: `key="some value"`.

-   **Personaggio**: `multiverse character create <id> --display-name <nome> --set <metadati>`
-   **Luogo**: `multiverse location create <id> --display-name <nome> --type <tipo> --set <metadati>`
-   **Sistema**: `multiverse system create <id> --display-name <nome> --type <tipo> --set <metadati>`
-   **Fazione**: `multiverse faction create <id> --display-name <nome> --type <tipo> --set <metadati>`
-   **Evento**: `multiverse event create <id> --display-name <nome> --type <tipo> --date <data> --set <metadati>`

## Regole d'Oro

1.  **Verifica Prima di Creare**: Usa sempre `info` o `list` per evitare di creare duplicati.
2.  **Chiedi per Chiarire**: Se una richiesta è ambigua, fai domande all'utente.
3.  **Usa `--set` Generosamente**: Più metadati aggiungi, più ricco diventerà il mondo.
4.  **Sincronizza Sempre**: Dopo aver scritto la lore, aggiorna i metadati con la CLI.
"##;
