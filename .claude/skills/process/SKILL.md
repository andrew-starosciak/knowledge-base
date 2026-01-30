---
name: process
description: Process videos from AI queue - extract claims, apply frameworks, organize with MOCs
allowed-tools: Bash(./target/release/engine *), Bash(./target/debug/engine *), Bash(cargo run -- *), Read, Grep
---

# AI Claim Extraction & Synthesis Processor

You are processing videos from the Historical Synthesis Engine's AI queue. Your job is to:
1. Extract atomic claims from transcripts
2. Apply analytical frameworks
3. Organize claims into Maps of Content
4. Generate research questions

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
- Factual statements with timestamps
- Causal claims (X causes Y)
- Cyclical patterns (recurring historical dynamics)
- Ideas being transmitted between cultures
- Geopolitical dynamics (core/periphery)

---

### Step 3: Extract Atomic Claims

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

### Step 4: Link Related Claims

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

### Step 5: Apply Analytical Frameworks

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

### Step 6: Organize with Maps of Content (MOCs)

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

### Step 7: Generate Research Questions

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

### Step 8: Detect Patterns

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

### Step 9: Mark as Completed

Count claims extracted and complete:

```bash
./target/debug/engine queue-complete <video-id> --claims <count>
```

---

### Step 10: Check for More

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

# Extract claims
./target/debug/engine add-claim Jjqf9T59uY0 "Agriculture preceded permanent settlement" \
  --quote "farming actually happened first" --category factual --confidence high --at 300

./target/debug/engine add-claim Jjqf9T59uY0 "Religion drove agricultural transition" \
  --quote "people built temples first, then started farming" --category causal --confidence high --at 840

# Link claims
./target/debug/engine link 1 2 --as supports

# Apply frameworks
./target/debug/engine transmission "Mother goddess cult" -f "Levant" -t "Anatolia" -y expansion -v Jjqf9T59uY0
./target/debug/engine timescale 2 -s longue_duree

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
- [ ] All major claims extracted with quotes
- [ ] Claims linked (2+ links each)
- [ ] Analytical frameworks applied where relevant
- [ ] Claims added to appropriate MOC
- [ ] Research questions generated if applicable
- [ ] Patterns noted if cross-video connections exist
