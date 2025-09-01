# Multiverse CLI - Complete Cheatsheet

## 🌌 Quick Start

```bash
# Initialize new project
multiverse world init my_world
cd my_world

# Show project info
multiverse info
```

## 📋 Universal Command Pattern

All entity commands follow this pattern:
```bash
multiverse <entity> <action> [name] [options]
```

**Entity Types:** `character`, `location`, `faction`, `race`, `system`, `event`, `story`, `episode`

## 🔧 Universal --set System

All fields (including display_name) use `--set`:

```bash
# Create with --set for ALL fields (no --display-name flag exists!)
multiverse character create luke --set display_name="Luke Skywalker" --set status=Active

# Multiple metadata fields
multiverse character create luke \
  --set display_name="Luke Skywalker" \
  --set status=Active \
  --set age=19 \
  --set profession=jedi \
  --set homeworld=tatooine

# Relations with relationship type
multiverse character create luke \
  --set display_name="Luke Skywalker" \
  --set location=tatooine*born_on \
  --set faction=rebel_alliance*member \
  --set race=human*is_of_race
```

### Relation Syntax
```bash
--set entity_type=target_name*relationship_type
--set location=tatooine*born_on           # Luke was born on Tatooine  
--set faction=rebels*member               # Luke is member of Rebels
--set faction=rebels*leader,empire*enemy  # Multiple relations (comma-separated)
```

## 🏛️ World Management

```bash
multiverse world init <name>              # Create new world
multiverse world info                     # Show world details
multiverse world config <key> <value>    # Set config (git.remote, etc.)
multiverse world status                   # Git status
multiverse world pull                     # Git pull 
multiverse world push                     # Git push
multiverse world import <file.sql>        # Import SQL data
```

## 👤 Characters

```bash
# Create character
multiverse character create john_snow \
  --set display_name="John Snow" \
  --set status=Active \
  --set age=25 \
  --set profession=lord_commander \
  --set house=stark

# Add relations  
multiverse character update john_snow \
  --set location=winterfell*resident \
  --set faction=nights_watch*member \
  --set race=human*is_of_race

# List and info
multiverse character list
multiverse character info john_snow
multiverse character delete john_snow --force
```

**Character Status Values:** `Active`, `Inactive`, `Deceased`, `Archived`

## 🏰 Locations

```bash
# Create location
multiverse location create winterfell \
  --set display_name="Winterfell" \
  --set type=castle \
  --set region=north \
  --set climate=cold

# Nested locations (parent/child)
multiverse location update winterfell \
  --set location=north*capital_of

# List and info
multiverse location list
multiverse location info winterfell
```

**Location Status Values:** `Active`, `Inactive`, `Destroyed`, `Hidden`, `Unknown`

## ⚔️ Factions

```bash
# Create faction
multiverse faction create stark_house \
  --set display_name="House Stark" \
  --set type=noble_house \
  --set motto="Winter is Coming"

# Add members (reverse relation)
multiverse faction update stark_house \
  --set character=ned_stark*head,john_snow*bastard

multiverse faction list
multiverse faction info stark_house
```

**Faction Status Values:** `Active`, `Inactive`, `Disbanded`, `Allied`, `Hostile`

## 🧬 Races

```bash
# Create race
multiverse race create human \
  --set display_name="Human" \
  --set lifespan=80 \
  --set homeworld=unknown

# System relations
multiverse race update human \
  --set system=solar_system*native_to

multiverse race list
multiverse race info human
```

**Race Status Values:** `Active`, `Inactive`, `Extinct`, `Legendary`, `Mythical`

## 🌟 Systems

```bash
# Create system
multiverse system create solar_system \
  --set display_name="Solar System" \
  --set system_type=stellar \
  --set star_count=1

# Reverse relations
multiverse system update solar_system \
  --set location=earth*contains \
  --set race=human*home_to

multiverse system list
multiverse system info solar_system
```

**System Types:** `stellar`, `political`, `magical`, `technological`
**System Status:** `Active`, `Inactive`, `Deprecated`, `Archived`

## ⚡ Events

```bash
# Create event
multiverse event create battle_of_hastings \
  --set display_name="Battle of Hastings" \
  --set date_text="1066 AD" \
  --set sort_key=1066

# Add participants and locations
multiverse event update battle_of_hastings \
  --set character=william*leader,harold*victim \
  --set location=hastings*battlefield \
  --set faction=normans*victor

multiverse event list
multiverse event timeline                  # Chronological view
multiverse event info battle_of_hastings
```

**Event Status Values:** `Active`, `Inactive`, `Completed`, `Cancelled`, `Pending`

## 📚 Stories & Episodes

```bash
# Create story
multiverse story create got_main \
  --set display_name="Game of Thrones" \
  --set story_type="Epic Fantasy" \
  --set themes="power,betrayal"

# Create episodes (belong to story automatically)
multiverse episode create got_main 1 \
  --set title="The Beginning" \
  --set word_count=5000

# Add characters to episode
multiverse episode update got_main:1 \
  --set character=john_snow*protagonist,ned_stark*mentor

multiverse story list
multiverse story info got_main
multiverse episode list got_main
multiverse episode info got_main:1
```

**Story Status:** `Draft`, `InProgress`, `Review`, `Published`, `Archived`
**Episode Status:** `Draft`, `InProgress`, `Review`, `Published`

## 📅 Timeline Management

```bash
multiverse timeline init                   # Setup timeline config
multiverse timeline config                # Show current config
multiverse timeline year add 2024         # Add custom year
multiverse timeline month add "Wintermoon" # Add custom month
multiverse timeline day add "Moonday"     # Add custom day
```

## 🔍 Database Queries

```bash
# Safe SELECT queries only
multiverse query "SELECT * FROM characters LIMIT 10"

# Count entities
multiverse query "SELECT COUNT(*) FROM characters"

# Complex joins for relations
multiverse query "
SELECT c.display_name, l.display_name, r.relationship_type 
FROM characters c
JOIN character_location_relations r ON c.id = r.from_id
JOIN locations l ON l.id = r.to_id
"

# Find orphaned entities
multiverse query "
SELECT c.display_name 
FROM characters c 
LEFT JOIN character_faction_relations r ON c.id = r.from_id 
WHERE r.from_id IS NULL
"
```

## 🎯 Complete Entity Creation Example

```bash
#!/bin/bash
# Complete world setup script

# Characters with full metadata and relations
multiverse character create luke_skywalker \
  --set display_name="Luke Skywalker" \
  --set status=Active \
  --set age=19 \
  --set profession=jedi \
  --set weapon=lightsaber \
  --set location=tatooine*born_on \
  --set faction=rebel_alliance*member \
  --set race=human*is_of_race

# Locations with nested relations
multiverse location create tatooine \
  --set display_name="Tatooine" \
  --set type=planet \
  --set climate=desert \
  --set system=tatoo_system*planet_in_system \
  --set character=luke_skywalker*birthplace_of

# Systems
multiverse system create tatoo_system \
  --set display_name="Tatoo System" \
  --set system_type=stellar \
  --set star_count=2 \
  --set location=tatooine*contains

# Events with participants
multiverse event create battle_of_yavin \
  --set display_name="Battle of Yavin" \
  --set date_text="0 ABY" \
  --set sort_key=0 \
  --set character=luke_skywalker*hero \
  --set location=death_star*target \
  --set faction=rebel_alliance*victor
```

## 🚨 Important Notes

1. **No --display-name flag** - use `--set display_name="Name"`
2. **Relations use * syntax** - `target*relationship_type`
3. **Comma-separated multiple relations** - `entity1*role1,entity2*role2`
4. **Forward and reverse relations** - work bidirectionally
5. **Query safety** - Only SELECT statements allowed
6. **Status values** - Each entity type has specific allowed status values
7. **Episode naming** - Episodes use `story:number` format (e.g., `got_main:1`)

## 🔗 Common Relation Types

- **Characters:** `member`, `leader`, `enemy`, `ally`, `born_on`, `resident`, `is_of_race`
- **Locations:** `capital_of`, `part_of`, `contains`, `planet_in_system`, `stronghold`  
- **Events:** `participant`, `leader`, `victim`, `hero`, `villain`, `target`, `victor`
- **Factions:** `head`, `member`, `ally`, `enemy`, `controls`, `founded_by`
- **Episodes:** `protagonist`, `antagonist`, `mentor`, `appears_in`