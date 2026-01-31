---
name: process
description: Process videos from AI queue - extract claims, sources, scholars, visuals, terms, evidence, quotes, apply frameworks, organize with MOCs
allowed-tools: Bash(./target/release/engine *), Bash(./target/debug/engine *), Bash(cargo run -- *), Read, Grep
---

# AI Claim Extraction & Synthesis Processor

You are processing videos from the Historical Synthesis Engine's AI queue. Your job is to COMPREHENSIVELY extract knowledge including:
1. Tag videos with era and geographic region
2. Extract atomic claims from transcripts (aim for 50-100 per hour of content)
3. Extract sources (books, papers, documentaries mentioned)
4. Extract scholars (thinkers, researchers referenced)
5. Extract visuals (images, diagrams, artifacts described)
6. Extract terms (definitions, concepts explained)
7. Extract evidence (archaeological, genetic, textual evidence cited)
8. Extract quotes (notable statements)
9. Apply analytical frameworks
10. Extract locations for map visualization
11. Organize claims into Maps of Content
12. Generate research questions

**IMPORTANT: Extract EVERYTHING. A 1-hour lecture should yield 50+ claims, 5+ sources, 10+ scholars, 10+ visuals, 10+ terms, 10+ pieces of evidence, and 5+ quotes. When in doubt, extract it.**

## Current Queue Status
!`./target/debug/engine -d data/knowledge.db queue 2>/dev/null || cargo run -- -d data/knowledge.db queue 2>/dev/null`

## Processing Workflow

For each pending video, follow ALL these steps:

---

### Step 1: Mark as In-Progress
```bash
./target/debug/engine -d data/knowledge.db queue-start <video-id>
```

---

### Step 2: Export and Read the Transcript
```bash
./target/debug/engine -d data/knowledge.db export-transcript <video-id>
```

Read carefully, identifying ALL of the following:

**Claims & Arguments:**
- Factual statements with timestamps
- Causal claims (X causes Y)
- Cyclical patterns (recurring historical dynamics)
- Ideas being transmitted between cultures
- Geopolitical dynamics (core/periphery)

**Sources & Scholars:**
- Books, papers, documentaries mentioned
- Scholars, thinkers, researchers referenced (with their contributions)

**Visual Content:**
- Images, paintings, diagrams being discussed
- Artifacts, skeletons, archaeological remains shown
- Maps, charts, or symbols displayed

**Definitions & Evidence:**
- Key terms and concepts being defined
- Archaeological evidence cited
- Genetic/DNA evidence mentioned
- Textual/historical evidence referenced
- Scientific studies or findings

**Context:**
- Historical era and geographic region covered
- Specific locations mentioned (cities, sites, regions)
- Notable quotes or statements

---

### Step 3: Tag Era and Region

Based on the transcript content, tag the video with its historical era and geographic region:

```bash
./target/debug/engine -d data/knowledge.db tag <video-id> --era "<era>" --region "<region>"
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
./target/debug/engine -d data/knowledge.db regions
```

---

### Step 4: Extract Atomic Claims

For each significant claim:

```bash
./target/debug/engine -d data/knowledge.db add-claim <video-id> "claim text" \
  --quote "exact source quote" \
  --category <category> \
  --confidence <high|medium|low> \
  --at <timestamp_seconds>
```

**Categories:**
- `factual` - General facts (historical or otherwise)
- `causal` - X causes Y relationships
- `cyclical` - Recurring patterns
- `memetic` - Idea/ideology transmission
- `geopolitical` - Core/periphery dynamics
- `phenomenological` - First-person experiential claims (consciousness, spirituality)
- `metaphysical` - Claims about nature of reality

**Guidelines:**
- **Aim for 50-100 claims per hour of video content**
- Each claim = one atomic idea
- Always include source quote
- Convert MM:SS to seconds for --at
- Extract micro-claims, not just major theses
- If someone makes 3 related points, that's 3 claims

---

### Step 4b: Extract Sources

For every book, paper, documentary, or article mentioned:

```bash
./target/debug/engine -d data/knowledge.db add-source "Title" \
  --author "Author Name" \
  -t <book|paper|documentary|article|lecture> \
  -y <year>

# Then cite it in the video:
./target/debug/engine -d data/knowledge.db cite-source <video-id> <source-id> --at <timestamp> --context "How it was referenced"
```

Check existing sources first:
```bash
./target/debug/engine -d data/knowledge.db sources
```

---

### Step 4c: Extract Scholars

For every scholar, thinker, or researcher mentioned:

```bash
./target/debug/engine -d data/knowledge.db add-scholar "Name" \
  --field "Philosophy" \
  --era "19th century" \
  --contribution "Brief summary of their contribution"

# Then cite them in the video:
./target/debug/engine -d data/knowledge.db cite-scholar <video-id> <scholar-id> --at <timestamp> --context "What was said about them"
```

Check existing scholars first:
```bash
./target/debug/engine -d data/knowledge.db scholars
```

---

### Step 4d: Extract Visuals

For every image, diagram, artifact, or visual described:

```bash
./target/debug/engine -d data/knowledge.db add-visual <video-id> "Description of visual" \
  --at <timestamp> \
  -t <painting|map|diagram|artifact|chart|photo|skeleton|symbol|architecture|inscription> \
  --significance "Why this visual matters" \
  --location "Place name" \
  --era "Prehistoric"
```

---

### Step 4e: Extract Terms & Definitions

For every term or concept that is defined or explained:

```bash
./target/debug/engine -d data/knowledge.db define "Term" "Definition text" \
  --domain <philosophy|archaeology|religion|sociology|anthropology|history> \
  --video <video-id> \
  --at <timestamp> \
  --scholar "Scholar who coined it"
```

Check existing terms:
```bash
./target/debug/engine -d data/knowledge.db terms
```

---

### Step 4f: Extract Evidence

For every piece of evidence cited (archaeological finds, DNA studies, etc.):

```bash
./target/debug/engine -d data/knowledge.db add-evidence <video-id> "Description of evidence" \
  -t <archaeological|genetic|textual|anthropological|linguistic|artistic|scientific|historical> \
  --at <timestamp> \
  --location "Where found" \
  --era "Time period"
```

---

### Step 4g: Extract Quotes

For notable or memorable statements:

```bash
./target/debug/engine -d data/knowledge.db add-quote <video-id> "The quote text" \
  --speaker "Who said it" \
  --at <timestamp> \
  --context "Why this quote matters"
```

---

### Step 5: Link Related Claims

Connect claims to build the knowledge graph:

```bash
./target/debug/engine -d data/knowledge.db link <claim-id> <claim-id> --as <relationship>
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

**Note:** Not all frameworks apply to all content. Skip frameworks that don't fit:
- **Historical content** → Use all applicable frameworks below
- **Philosophy/Spirituality** → Primarily use Causal Chains and Idea Transmission; skip Cliodynamics/World-Systems unless discussing historical religious movements
- **Science/Technical** → Primarily use Causal Chains

#### 6a. Cliodynamics (Turchin) - Cyclical Patterns
*Use for: Historical content about civilizations, empires, social movements*
*Skip for: Abstract philosophy, consciousness studies, technical content*

When you identify recurring historical patterns:

```bash
./target/debug/engine -d data/knowledge.db cyclical <video-id> -t <type> -e "Entity" "Description"
```

Types:
- `elite_overproduction` - Too many elites competing
- `fiscal_strain` - State financial stress
- `social_unrest` - Instability indicators
- `population_pressure` - Demographic dynamics
- `asabiyyah` - Social cohesion (Ibn Khaldun)
- `center_periphery` - Core vs edge dynamics

#### 6b. Causal Chains
*Use for: All content types - universal framework*

When claims have cause-effect relationships:

```bash
./target/debug/engine -d data/knowledge.db causal <cause-claim-id> <effect-claim-id> \
  -l <positive|negative|linear> \
  -s <strong|moderate|weak|speculative>
```

#### 6c. Idea Transmission (Boyd/Richerson)
*Use for: Historical AND philosophical content - tracks how ideas spread*

When ideas spread between cultures or traditions:

```bash
./target/debug/engine -d data/knowledge.db transmission "idea name" \
  -f "source culture" \
  -t "target culture" \
  -y <horizontal|vertical|oblique> \
  -v <video-id>
```

#### 6d. World-Systems (Wallerstein)
*Use for: Historical content about economics, trade, imperialism*
*Skip for: Philosophy, consciousness, spirituality*

When geopolitical dynamics are discussed:

```bash
# Define entity positions
./target/debug/engine -d data/knowledge.db position "Rome" --era "Classical Antiquity" -p core
./target/debug/engine -d data/knowledge.db position "Gaul" --era "Classical Antiquity" -p periphery

# Track surplus flows
./target/debug/engine -d data/knowledge.db flow <from-entity-id> <to-entity-id> "grain" --era "Classical Antiquity"
```

#### 6e. Braudel's Timescales
*Use for: Historical content; can apply to philosophy (perennial vs contemporary ideas)*

Classify claims by temporal scope:

```bash
./target/debug/engine -d data/knowledge.db timescale <claim-id> -s <event|conjuncture|longue_duree>
```

- `event` - Short-term events
- `conjuncture` - Medium cycles (decades)
- `longue_duree` - Long-term structural patterns

---

### Step 7: Extract Locations for Map

Identify geographic locations mentioned in the transcript and add them for map visualization:

```bash
./target/debug/engine -d data/knowledge.db locate <video-id> \
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
./target/debug/engine -d data/knowledge.db locations
```

---

### Step 8: Organize with Maps of Content (MOCs)

If the video covers a coherent topic with 5+ related claims, create or add to a MOC:

```bash
# Create new MOC
./target/debug/engine -d data/knowledge.db moc-create "Topic Name" --description "What this MOC covers"

# Add claims to MOC
./target/debug/engine -d data/knowledge.db moc-add <moc-id> <claim-id>
```

Check existing MOCs first:
```bash
./target/debug/engine -d data/knowledge.db mocs
```

---

### Step 9: Generate Research Questions

If the video raises interesting questions for further investigation:

```bash
./target/debug/engine -d data/knowledge.db ask "What conditions precede X?" --notes "Raised by video content"
```

Link evidence to existing questions:
```bash
./target/debug/engine -d data/knowledge.db evidence <question-id> --claim <claim-id> --relevance "How it relates"
```

Check existing questions:
```bash
./target/debug/engine -d data/knowledge.db questions
```

---

### Step 10: Detect Patterns

If you notice patterns across this video and others:

```bash
./target/debug/engine -d data/knowledge.db pattern -t <type> "Description" --claims "1,2,3" --videos "id1,id2"
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
./target/debug/engine -d data/knowledge.db queue-complete <video-id> --claims <count>
```

---

### Step 12: Check for More

```bash
./target/debug/engine -d data/knowledge.db queue
```

---

## Error Handling

If processing fails:
```bash
./target/debug/engine -d data/knowledge.db queue-fail <video-id> --reason "description"
```

---

## Example Full Session

```bash
# Start
./target/debug/engine -d data/knowledge.db queue-start x1E5rRmCiT4
./target/debug/engine -d data/knowledge.db export-transcript x1E5rRmCiT4

# Tag era and region
./target/debug/engine -d data/knowledge.db tag x1E5rRmCiT4 --era "Prehistoric" --region "Europe"

# === EXTRACT SOURCES ===
./target/debug/engine -d data/knowledge.db add-source "The Dawn of Everything" --author "David Graeber, David Wengrow" -t book -y 2021
./target/debug/engine -d data/knowledge.db cite-source x1E5rRmCiT4 1 --at 1820 --context "Referenced for dwarf burial evidence"

# === EXTRACT SCHOLARS ===
./target/debug/engine -d data/knowledge.db add-scholar "Immanuel Kant" --field "Philosophy" --era "18th century" --contribution "Critique of Pure Reason - reality as mental construction"
./target/debug/engine -d data/knowledge.db cite-scholar x1E5rRmCiT4 1 --at 2246 --context "Discussed reality as constructed by the mind"

./target/debug/engine -d data/knowledge.db add-scholar "Emile Durkheim" --field "Sociology" --era "19th-20th century" --contribution "Founder of sociology - religion as collective consciousness"
./target/debug/engine -d data/knowledge.db cite-scholar x1E5rRmCiT4 2 --at 2570 --context "Quoted on religion creating society"

./target/debug/engine -d data/knowledge.db add-scholar "Genevieve von Petzinger" --field "Anthropology" --contribution "Research on recurring symbols in Ice Age cave paintings"
./target/debug/engine -d data/knowledge.db cite-scholar x1E5rRmCiT4 3 --at 1928 --context "Documented 32 recurring symbols in cave paintings worldwide"

# === EXTRACT VISUALS ===
./target/debug/engine -d data/knowledge.db add-visual x1E5rRmCiT4 "Cave painting of lions at Chauvet Cave" --at 306 -t painting --significance "Shows artistic sophistication 30,000 years ago"
./target/debug/engine -d data/knowledge.db add-visual x1E5rRmCiT4 "Cave painting of horses at Lascaux" --at 352 -t painting --significance "Picasso said 'we learned nothing in 10,000 years'"
./target/debug/engine -d data/knowledge.db add-visual x1E5rRmCiT4 "Bird-like shaman figures in cave painting" --at 1476 -t painting --significance "Evidence of shamans dressing as animals"
./target/debug/engine -d data/knowledge.db add-visual x1E5rRmCiT4 "Skeleton of dwarf at Romito cave" --at 1562 -t skeleton --significance "Evidence of care for disabled in prehistoric societies"
./target/debug/engine -d data/knowledge.db add-visual x1E5rRmCiT4 "Chart of 32 recurring cave painting symbols" --at 1928 -t chart --significance "Evidence of proto-written language"

# === EXTRACT TERMS ===
./target/debug/engine -d data/knowledge.db define "Animism" "Belief that all living things have souls and are interconnected - probably the first religion" --domain religion --video x1E5rRmCiT4 --at 1259
./target/debug/engine -d data/knowledge.db define "Collective consciousness" "Shared beliefs and ideas that allow society to function - religion creates this" --domain sociology --video x1E5rRmCiT4 --at 2790 --scholar "Emile Durkheim"
./target/debug/engine -d data/knowledge.db define "Shamanism" "Spiritual practice of communicating with the spirit world, often through altered states" --domain religion --video x1E5rRmCiT4 --at 1476

# === EXTRACT EVIDENCE ===
./target/debug/engine -d data/knowledge.db add-evidence x1E5rRmCiT4 "Cave paintings found in areas with best acoustics - suggests ritual with music" -t archaeological --at 457 --era "Prehistoric"
./target/debug/engine -d data/knowledge.db add-evidence x1E5rRmCiT4 "Musical instruments (flutes) found alongside cave paintings" -t archaeological --at 492 --era "Prehistoric"
./target/debug/engine -d data/knowledge.db add-evidence x1E5rRmCiT4 "DNA analysis shows dwarf received same food quality as rest of community" -t genetic --at 1639 --era "Prehistoric"
./target/debug/engine -d data/knowledge.db add-evidence x1E5rRmCiT4 "High frequency of disabled individuals in elaborate Ice Age burials" -t anthropological --at 1878 --era "Prehistoric"

# === EXTRACT QUOTES ===
./target/debug/engine -d data/knowledge.db add-quote x1E5rRmCiT4 "We learned nothing in 10,000 years" --speaker "Pablo Picasso" --at 368 --context "On viewing Lascaux cave paintings - their artistic quality equals modern art"
./target/debug/engine -d data/knowledge.db add-quote x1E5rRmCiT4 "Religion is a system of ideas by which men imagine the society of which they are members" --speaker "Emile Durkheim" --at 2590 --context "Defining religion as collective consciousness"

# === EXTRACT CLAIMS (many more in practice) ===
./target/debug/engine -d data/knowledge.db add-claim x1E5rRmCiT4 "Religion is what makes humans fundamentally human" \
  --quote "in fact religion is what makes us fundamentally human" --category factual --confidence high --at 63

./target/debug/engine -d data/knowledge.db add-claim x1E5rRmCiT4 "Cave paintings are expressions of religious beliefs, not merely art" \
  --quote "these paintings are not about art it's really about religion" --category factual --confidence high --at 528

./target/debug/engine -d data/knowledge.db add-claim x1E5rRmCiT4 "Caves symbolized wombs - portals between physical and spirit world" \
  --quote "the cave is a portal into another world" --category factual --confidence medium --at 941

# ... (50+ more claims)

# Link claims
./target/debug/engine -d data/knowledge.db link 1 2 --as supports
./target/debug/engine -d data/knowledge.db link 2 3 --as elaborates

# Apply frameworks
./target/debug/engine -d data/knowledge.db transmission "Animism" -f "Africa (origins)" -t "Global (all continents)" -y horizontal -v x1E5rRmCiT4
./target/debug/engine -d data/knowledge.db timescale 1 -s longue_duree

# Add locations for map
./target/debug/engine -d data/knowledge.db locate x1E5rRmCiT4 --place "Chauvet Cave" --lat 44.388 --lon 4.415 --era "Prehistoric" --note "Cave paintings from ~30,000 years ago"
./target/debug/engine -d data/knowledge.db locate x1E5rRmCiT4 --place "Lascaux Cave" --lat 45.054 --lon 1.169 --era "Prehistoric" --note "Cave paintings from ~20,000 years ago"

# Create/update MOC
./target/debug/engine -d data/knowledge.db moc-create "Origins of Religion" --description "Theories on emergence of religious belief in prehistoric humans"
./target/debug/engine -d data/knowledge.db moc-add 1 1
./target/debug/engine -d data/knowledge.db moc-add 1 2
./target/debug/engine -d data/knowledge.db moc-add 1 3

# Research questions
./target/debug/engine -d data/knowledge.db ask "What caused the transition from animism to monotheism?"
./target/debug/engine -d data/knowledge.db evidence 1 --claim 1 --relevance "Animism was the original religion"

# Complete (note: 50+ claims for 1 hour video)
./target/debug/engine -d data/knowledge.db queue-complete x1E5rRmCiT4 --claims 55
```

---

## Quality Checklist

Before marking complete, verify:

**Minimum extraction targets (per hour of content):**
- [ ] 50+ claims extracted
- [ ] 3+ sources (books/papers) added
- [ ] 5+ scholars added
- [ ] 5+ visuals described
- [ ] 5+ terms defined
- [ ] 5+ pieces of evidence recorded
- [ ] 3+ quotes captured

**Organization:**
- [ ] Video tagged with era and region
- [ ] Claims linked (2+ links each)
- [ ] Analytical frameworks applied where relevant
- [ ] Key locations added for map visualization
- [ ] Claims added to appropriate MOC
- [ ] Research questions generated if applicable
- [ ] Patterns noted if cross-video connections exist

**If a video seems to have less content, ask yourself: "Did the lecturer really only mention 1 book? Only reference 2 scholars? Only show 3 images?" Usually the answer is no - dig deeper.**
