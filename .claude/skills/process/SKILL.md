---
name: process
description: Process videos from AI queue - extract claims, tag eras/regions, map locations, apply frameworks, organize with MOCs
allowed-tools: Bash(./target/release/engine *), Bash(./target/debug/engine *), Bash(cargo run -- *), Read, Grep
---

# AI Claim Extraction & Synthesis Processor

You are processing videos from the Historical Synthesis Engine's AI queue. Your job is to:
1. Tag videos with era and geographic region
2. Extract atomic claims from transcripts
3. Apply analytical frameworks
4. Extract locations for map visualization
5. Organize claims into Maps of Content
6. Generate research questions

## Current Queue Status
!`./target/debug/engine queue 2>/dev/null || cargo run -- queue 2>/dev/null`

## Processing Workflow

For each pending video, follow ALL these steps:

---

### Step 1: Mark as In-Progress
```bash
./target/debug/engine queue-start <video-id>
```

---

### Step 2: Export and Read the Transcript
```bash
./target/debug/engine export-transcript <video-id>
```

Read carefully, identifying:
- Historical era and geographic region covered
- Factual statements with timestamps
- Causal claims (X causes Y)
- Cyclical patterns (recurring historical dynamics)
- Ideas being transmitted between cultures
- Geopolitical dynamics (core/periphery)
- Specific locations mentioned (cities, sites, regions)

---

### Step 3: Tag Era and Region

Based on the transcript content, tag the video with its historical era and geographic region:

```bash
./target/debug/engine tag <video-id> --era "<era>" --region "<region>"
```

**Available Eras:**
- `Prehistoric` - Before written records (~10,000 BCE and earlier)
- `Bronze Age` - ~3300-1200 BCE
- `Iron Age` - ~1200-500 BCE
- `Classical Antiquity` - ~500 BCE-500 CE
- `Late Antiquity` - ~300-700 CE
- `Medieval` - ~500-1500 CE
- `Early Modern` - ~1500-1800 CE
- `Modern` - ~1800 CE-present

**Regions** (create as needed):
- Geographic: "Mesopotamia", "Egypt", "Levant", "Anatolia", "Mediterranean", "China", "India"
- Cultural: "Near East", "Central Asia", "Mesoamerica"

Check existing regions:
```bash
./target/debug/engine regions
```

---

### Step 4: Extract Atomic Claims

For each significant claim:

```bash
./target/debug/engine add-claim <video-id> "claim text" \
  --quote "exact source quote" \
  --category <category> \
  --confidence <high|medium|low> \
  --at <timestamp_seconds>
```

**Categories:**
- `factual` - General historical facts
- `causal` - X causes Y relationships
- `cyclical` - Recurring historical patterns
- `memetic` - Idea/ideology transmission
- `geopolitical` - Core/periphery dynamics

**Guidelines:**
- Aim for 10-30 claims per video
- Each claim = one atomic idea
- Always include source quote
- Convert MM:SS to seconds for --at

---

### Step 5: Link Related Claims

Connect claims to build the knowledge graph:

```bash
./target/debug/engine link <claim-id> <claim-id> --as <relationship>
```

**Relationships:**
- `supports` - Evidence for
- `contradicts` - In opposition
- `elaborates` - Expands on
- `causes` - Leads to
- `caused_by` - Results from
- `related` - General connection

**Aim for 2+ links per claim** (Zettelkasten principle).

---

### Step 6: Apply Analytical Frameworks

#### 5a. Cliodynamics (Turchin) - Cyclical Patterns
When you identify recurring historical patterns:

```bash
./target/debug/engine cyclical <video-id> -t <type> -e "Entity" "Description"
```

Types:
- `elite_overproduction` - Too many elites competing
- `fiscal_strain` - State financial stress
- `social_unrest` - Instability indicators
- `population_pressure` - Demographic dynamics
- `asabiyyah` - Social cohesion (Ibn Khaldun)
- `center_periphery` - Core vs edge dynamics

#### 5b. Causal Chains
When claims have cause-effect relationships:

```bash
./target/debug/engine causal <cause-claim-id> <effect-claim-id> \
  -l <positive|negative|linear> \
  -s <strong|moderate|weak|speculative>
```

#### 5c. Idea Transmission (Boyd/Richerson)
When ideas spread between cultures:

```bash
./target/debug/engine transmission "idea name" \
  -f "source culture" \
  -t "target culture" \
  -y <horizontal|vertical|oblique> \
  -v <video-id>
```

#### 5d. World-Systems (Wallerstein)
When geopolitical dynamics are discussed:

```bash
# Define entity positions
./target/debug/engine position "Rome" --era "Classical Antiquity" -p core
./target/debug/engine position "Gaul" --era "Classical Antiquity" -p periphery

# Track surplus flows
./target/debug/engine flow <from-entity-id> <to-entity-id> "grain" --era "Classical Antiquity"
```

#### 5e. Braudel's Timescales
Classify claims by temporal scope:

```bash
./target/debug/engine timescale <claim-id> -s <event|conjuncture|longue_duree>
```

- `event` - Short-term events
- `conjuncture` - Medium cycles (decades)
- `longue_duree` - Long-term structural patterns

---

### Step 7: Extract Locations for Map

Identify geographic locations mentioned in the transcript and add them for map visualization:

```bash
./target/debug/engine locate <video-id> \
  --place "Place Name" \
  --lat <latitude> \
  --lon <longitude> \
  --era "<era>" \
  --at <timestamp_seconds> \
  --note "Why this location is significant"
```

**Common Historical Locations:**
| Place | Lat | Lon |
|-------|-----|-----|
| Göbekli Tepe | 37.223 | 38.922 |
| Çatalhöyük | 37.666 | 32.828 |
| Jericho | 31.871 | 35.444 |
| Uruk | 31.322 | 45.636 |
| Babylon | 32.536 | 44.421 |
| Memphis (Egypt) | 29.846 | 31.254 |
| Athens | 37.976 | 23.735 |
| Rome | 41.902 | 12.496 |
| Constantinople | 41.008 | 28.978 |
| Jerusalem | 31.778 | 35.235 |

For unlisted locations, use approximate coordinates. Check existing:
```bash
./target/debug/engine locations
```

---

### Step 8: Organize with Maps of Content (MOCs)

If the video covers a coherent topic with 5+ related claims, create or add to a MOC:

```bash
# Create new MOC
./target/debug/engine moc-create "Topic Name" --description "What this MOC covers"

# Add claims to MOC
./target/debug/engine moc-add <moc-id> <claim-id>
```

Check existing MOCs first:
```bash
./target/debug/engine mocs
```

---

### Step 9: Generate Research Questions

If the video raises interesting questions for further investigation:

```bash
./target/debug/engine ask "What conditions precede X?" --notes "Raised by video content"
```

Link evidence to existing questions:
```bash
./target/debug/engine evidence <question-id> --claim <claim-id> --relevance "How it relates"
```

Check existing questions:
```bash
./target/debug/engine questions
```

---

### Step 10: Detect Patterns

If you notice patterns across this video and others:

```bash
./target/debug/engine pattern -t <type> "Description" --claims "1,2,3" --videos "id1,id2"
```

Types:
- `recurring_theme` - Same idea appears multiple times
- `contradiction` - Conflicting claims
- `consensus` - Agreement across sources
- `evolution` - Idea changing over time
- `parallel` - Similar events in different contexts

---

### Step 11: Mark as Completed

Count claims extracted and complete:

```bash
./target/debug/engine queue-complete <video-id> --claims <count>
```

---

### Step 12: Check for More

```bash
./target/debug/engine queue
```

---

## Error Handling

If processing fails:
```bash
./target/debug/engine queue-fail <video-id> --reason "description"
```

---

## Example Full Session

```bash
# Start
./target/debug/engine queue-start Jjqf9T59uY0
./target/debug/engine export-transcript Jjqf9T59uY0

# Tag era and region
./target/debug/engine tag Jjqf9T59uY0 --era "Prehistoric" --region "Levant"

# Extract claims
./target/debug/engine add-claim Jjqf9T59uY0 "Agriculture preceded permanent settlement" \
  --quote "farming actually happened first" --category factual --confidence high --at 300

./target/debug/engine add-claim Jjqf9T59uY0 "Religion drove agricultural transition" \
  --quote "people built temples first, then started farming" --category causal --confidence high --at 840

# Link claims
./target/debug/engine link 1 2 --as supports

# Apply frameworks
./target/debug/engine transmission "Mother goddess cult" -f "Levant" -t "Anatolia" -y horizontal -v Jjqf9T59uY0
./target/debug/engine timescale 2 -s longue_duree

# Add locations for map
./target/debug/engine locate Jjqf9T59uY0 --place "Göbekli Tepe" --lat 37.223 --lon 38.922 --era "Prehistoric" --note "Oldest known temple complex"
./target/debug/engine locate Jjqf9T59uY0 --place "Çatalhöyük" --lat 37.666 --lon 32.828 --era "Prehistoric" --note "Early Neolithic settlement, 8000 people"
./target/debug/engine locate Jjqf9T59uY0 --place "Jericho" --lat 31.871 --lon 35.444 --era "Prehistoric" --note "Tower of Jericho, Natufian site"

# Create MOC
./target/debug/engine moc-create "Neolithic Revolution" --description "Theories on agricultural transition"
./target/debug/engine moc-add 1 1
./target/debug/engine moc-add 1 2

# Research question
./target/debug/engine ask "What role did religion play in early state formation?"
./target/debug/engine evidence 1 --claim 2 --relevance "Religion preceded agriculture"

# Complete
./target/debug/engine queue-complete Jjqf9T59uY0 --claims 15
```

---

## Quality Checklist

Before marking complete, verify:
- [ ] Video tagged with era and region
- [ ] All major claims extracted with quotes
- [ ] Claims linked (2+ links each)
- [ ] Analytical frameworks applied where relevant
- [ ] Key locations added for map visualization
- [ ] Claims added to appropriate MOC
- [ ] Research questions generated if applicable
- [ ] Patterns noted if cross-video connections exist
