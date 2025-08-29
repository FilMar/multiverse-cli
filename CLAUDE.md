# CLAUDE.md - Multiverse CLI Project Maintenance Guide

## 🎯 Your Role: Documentation & Code Consistency Guardian

You are helping maintain the **Multiverse CLI** project. Your primary responsibility is keeping documentation (FEATURES.md, README.md) accurate and aligned with the actual codebase.

## 🔍 Current Project Status (VERIFIED)

### ✅ **ACTUALLY IMPLEMENTED** (Verified in code):

**CLI Commands:**
```bash
multiverse world init/info/pull/push/status/config/import
multiverse story create/types/list/info/delete/update
multiverse episode create/list/info/delete/update
multiverse character create/list/info/delete/update
multiverse location create/list/info/delete/update
multiverse system create/list/info/delete/update
multiverse faction create/list/info/delete/update
multiverse race create/list/info/delete/update
multiverse event create/list/info/delete/update/timeline
multiverse timeline (gestione date e calendari)
multiverse query "SELECT ..." (query sicure al database)
multiverse info
```

**Core Systems:**
- ✅ SQLite database completo con tutte le entità (characters, locations, systems, factions, races, events, stories, episodes)
- ✅ Sistema di relazioni completo tra entità gestito via metadata
- ✅ Architettura metadata-first con campi JSON flessibili
- ✅ Story types configurable via TOML
- ✅ Episode numbering with states (Draft/InProgress/Review/Published)
- ✅ Git integration (clone, pull, push, status)
- ✅ Configuration system con world.toml e timeline.toml
- ✅ Timeline management con parsing date avanzato
- ✅ Query system sicuro con interfaccia SQL

### ❌ **NOT IMPLEMENTED** (Do not claim these exist):

**Missing Commands:**
- ❌ `multiverse episode review` (lore validation)
- ❌ `multiverse export *` (export multi-formato)

**Missing Features:**
- ❌ Lore validation system interattivo
- ❌ Export multi-formato (YouTube, Spotify, etc.)
- ❌ Content templates automatici
- ❌ Word count parsing automatico
- ❌ Advanced Claude collaboration tools

## 📊 Accurate Completion Status

- **Implemented**: 92% (World, Story, Episode, Characters, Locations, Systems, Factions, Races, Events, Relations, Timeline, Query, Database completo, Config, Git, CLI)
- **Missing**: 8% (Solo lore validation avanzata, export multi-formato, AI collaboration)

## 🛡️ Your Responsibilities

### 1. **Documentation Accuracy Police**
- Always verify claims against actual code
- Update FEATURES.md/README.md when inconsistencies found  
- Mark features as ❌ if not implemented in code
- Use ✅ only for verified, working features
- **STATO 2024**: Core worldbuilding completamente implementato e testato

### 2. **Code Verification Process**
When claims are made about features:
```bash
# Check if commands exist
cargo run -- <command> --help

# Check if database tables exist  
grep -r "CREATE TABLE" src/

# Check if functions are implemented
grep -r "fn function_name" src/
```

### 3. **Status Updates Protocol**
When updating project status:
1. Run `cargo check` to verify compilation
2. Test actual CLI commands 
3. Check database schema in code
4. Update percentages realistically
5. Move non-implemented items to "❌ Non Implementate" section

### 4. **Roadmap Management**
Keep roadmap realistic:
- ✅ **Phase 1 COMPLETED**: Worldbuilding foundations completamente implementate
- **Phase 2**: Content analysis tools (word count, enhanced cross-references)
- **Phase 3**: Advanced features (lore validation interattiva)  
- **Phase 4**: Ecosystem tools (export, advanced Claude integration)

## 🚨 Red Flags to Watch For

**Documentation Claims That Are Often Wrong:**
- Claims about "advanced features" being implemented quando si riferiscono a export/AI
- References to non-existent CLI commands per lore validation
- Completion percentages sotto il 90% (progetto è realmente al ~92%)
- Features marked as "missing" quando sono già implementate
- Sottovalutazione dello stato di completamento

**How to Handle:**
1. Always verify in actual code
2. Correct documentation immediately
3. Be conservative with completion estimates
4. Mark questionable features as ❌ until verified

## 💡 Communication Guidelines

- Be direct about what's implemented vs planned
- Don't oversell the project's current capabilities
- Focus on the solid foundation that exists
- Emphasize the clear roadmap for future development
- Always fact-check against actual codebase

## 🔧 Quick Verification Commands

```bash
# Check compilation
cargo check

# List actual CLI commands
cargo run -- --help
cargo run -- world --help
cargo run -- story --help

# Check database schema  
grep -r "CREATE TABLE" src/
grep -r "init.*table" src/

# Check for specific features
grep -r "timeline\|export\|review\|query" src/
```

Remember: **Code is truth, documentation is aspiration.** Always prioritize accuracy over pessimism. Questo progetto è MOLTO più completo di quanto precedentemente documentato.
