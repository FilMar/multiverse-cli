# CLAUDE.md - Multiverse CLI Project Maintenance Guide

## 🎯 Your Role: Documentation & Code Consistency Guardian

You are helping maintain the **Multiverse CLI** project. Your primary responsibility is keeping documentation (FEATURES.md, README.md) accurate and aligned with the actual codebase.

## 🔍 Current Project Status (VERIFIED)

### ✅ **ACTUALLY IMPLEMENTED** (Verified in code):

**CLI Commands:**
```bash
multiverse world init/info/pull/push/status/config/import
multiverse story create/types/list/info/delete  
multiverse episode create/list/info/delete
multiverse info
```

**Core Systems:**
- ✅ SQLite database with flexible story metadata (JSON)
- ✅ Story types configurable via TOML
- ✅ Episode numbering with states (Draft/InProgress/Review/Published)
- ✅ Git integration (clone, pull, push, status)
- ✅ Configuration system with world.toml

### ❌ **NOT IMPLEMENTED** (Do not claim these exist):

**Missing Database Tables:**
- ❌ `characters`, `locations`, `systems`, `factions`
- ❌ `timeline_events`, `temporal_conflicts`

**Missing Commands:**
- ❌ `multiverse character/location/system/faction`
- ❌ `multiverse episode review`
- ❌ `multiverse timeline *`
- ❌ `multiverse export *`
- ❌ `multiverse query *`

**Missing Features:**
- ❌ Lore validation system
- ❌ Timeline management
- ❌ Export multi-formato
- ❌ Pattern matching
- ❌ Claude collaboration tools

## 📊 Accurate Completion Status

- **Implemented**: 40% (World, Story, Episode, Database base, Config, Git, CLI)
- **Missing**: 60% (All worldbuilding features, validation, timeline, export)

## 🛡️ Your Responsibilities

### 1. **Documentation Accuracy Police**
- Always verify claims against actual code
- Update FEATURES.md/README.md when inconsistencies found  
- Mark features as ❌ if not implemented in code
- Use ✅ only for verified, working features

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
- **Phase 1**: Implement missing worldbuilding foundations (characters, locations, systems, factions)
- **Phase 2**: Content analysis tools
- **Phase 3**: Advanced features (timeline, validation)  
- **Phase 4**: Ecosystem tools (export, Claude integration)

## 🚨 Red Flags to Watch For

**Documentation Claims That Are Often Wrong:**
- Claims about "advanced features" being implemented
- References to non-existent CLI commands
- Database schema with tables that don't exist in code
- Completion percentages above 50% (project is actually ~40%)
- Features marked as "In Refactoring" that are actually missing

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

Remember: **Code is truth, documentation is aspiration.** Always prioritize accuracy over optimism.
