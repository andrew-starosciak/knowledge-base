# Historical Synthesis Engine - Research Process Overview

This document explains the research methodology built into the engine, with analogies to make each concept intuitive.

---

## The Big Picture

Think of this system as building a **detective's evidence board** for history. You're collecting clues (claims), connecting them with string (links), grouping related evidence (MOCs), and looking for patterns that explain what happened and why.

---

## 1. Ingestion: Collecting Raw Material

**What it does:** Fetches YouTube video transcripts and stores them.

**Analogy: The Librarian**
Like a librarian acquiring new books for a collection. The video goes on the shelf with metadata (title, channel, date) so you can find it later. The transcript is the "text" you'll mine for insights.

```bash
engine fetch "https://youtube.com/watch?v=..."
```

---

## 2. Claim Extraction: Mining for Gold

**What it does:** Extracts atomic factual statements from transcripts.

**Analogy: The Gold Miner**
A transcript is like a riverbed full of silt. Most of it is filler, but scattered throughout are nuggets of gold—specific, verifiable claims. Your job is to sift through and collect each nugget individually, noting exactly where you found it (timestamp + quote).

**Guidelines:**
- One claim = one idea (atomic)
- Always record the source quote
- Categorize: factual, causal, cyclical, memetic, geopolitical

```bash
engine add-claim <video-id> "Elite overproduction precedes instability" \
  --quote "too many elites competing for limited positions" \
  --category causal
```

---

## 3. Claim Linking: Building the Web

**What it does:** Connects related claims to form a knowledge graph.

**Analogy: The Spider's Web**
Each claim is a node, but isolated nodes are useless. Like a spider building a web, you connect claims with threads (relationships). The more connections, the stronger the web. A claim with no connections is an orphan—floating in space, hard to find, easy to forget.

**Relationship types:**
- `supports` / `contradicts` - Evidence relationships
- `causes` / `caused_by` - Causal chains
- `elaborates` - Adds detail
- `related` - General connection

**Rule of thumb:** Aim for 2+ links per claim (Zettelkasten principle).

```bash
engine link 1 2 --as supports
```

---

## 4. Era & Region Tagging: The Filing Cabinet

**What it does:** Categorizes videos by time period and geography.

**Analogy: The Filing Cabinet**
Imagine a filing cabinet with drawers labeled by era (Prehistoric, Bronze Age, Iron Age...) and folders within for regions (Mesopotamia, Egypt, Levant...). Tagging puts each video in the right drawer and folder so you can browse by period or place.

```bash
engine tag <video-id> --era "Bronze Age" --region "Mesopotamia"
```

---

## 5. Location Mapping: Pins on the Wall Map

**What it does:** Places geographic markers for locations mentioned in videos.

**Analogy: The War Room Map**
Like a general's war room with a large map and pins marking key locations. Each pin represents a place discussed in your sources, with notes about why it matters. Click a pin to see what happened there and jump to the relevant video.

```bash
engine locate <video-id> --place "Göbekli Tepe" --lat 37.223 --lon 38.922 \
  --note "Oldest known temple complex"
```

---

## 6. Analytical Frameworks: The Detective's Theories

These are structured lenses for analyzing historical patterns. Each framework is like a different detective theory about how history works.

### 6a. Cliodynamics (Peter Turchin): The Pendulum

**What it does:** Tracks cyclical patterns in civilizations.

**Analogy: The Pendulum**
History swings like a pendulum. Societies build up (integration), then break down (disintegration), then rebuild. Turchin identified recurring indicators: elite overproduction, fiscal strain, social unrest. When you spot these, you're seeing the pendulum in motion.

```bash
engine cyclical <video-id> -t elite_overproduction -e "Roman Empire" "Description"
```

### 6b. Causal Chains: The Domino Effect

**What it does:** Maps cause-and-effect relationships between claims.

**Analogy: Dominoes**
One event triggers another, which triggers another. Line up your claims like dominoes and trace how pushing the first one affects the rest. Some chains are linear (A→B→C), others form feedback loops (A→B→A).

```bash
engine causal <cause-claim-id> <effect-claim-id> -l positive -s strong
```

### 6c. Idea Transmission (Boyd/Richerson): The Viral Meme

**What it does:** Tracks how ideas spread between cultures.

**Analogy: Going Viral**
Ideas spread like viruses. Some pass vertically (parent to child), others horizontally (peer to peer), others obliquely (teacher to student from different generation). Tracking transmission shows how an idea in Athens ends up in Rome ends up in Baghdad.

```bash
engine transmission "Greek philosophy" -f "Athens" -t "Rome" -y horizontal
```

### 6d. World-Systems (Wallerstein): The Economic Food Chain

**What it does:** Maps core-periphery relationships and resource flows.

**Analogy: The Food Chain**
The world economy is like an ecosystem. Some regions are apex predators (core)—they extract wealth from others. Some are prey (periphery)—their resources flow outward. Semi-periphery are the middle tier. Tracking this reveals exploitation patterns across civilizations.

```bash
engine position "Rome" --era "Classical Antiquity" -p core
engine flow <from-id> <to-id> "grain" --era "Classical Antiquity"
```

### 6e. Braudel's Timescales: The Zoom Lens

**What it does:** Classifies claims by temporal scope.

**Analogy: Google Maps Zoom**
History operates at different zoom levels:
- **Event** (street view): Short-term happenings—battles, treaties, deaths
- **Conjuncture** (city view): Medium cycles over decades—economic booms, cultural movements
- **Longue durée** (satellite view): Deep structural patterns over centuries—geography, climate, mentalities

```bash
engine timescale <claim-id> -s longue_duree
```

---

## 7. Maps of Content (MOCs): The Table of Contents

**What it does:** Groups related claims into navigable collections.

**Analogy: A Book's Table of Contents**
As claims accumulate, you need organization. A MOC is like a chapter heading that groups related claims. "Bronze Age Collapse" might collect 20 claims about that event. MOCs are your entry points for exploring a topic—the table of contents for your knowledge.

```bash
engine moc-create "Bronze Age Collapse" --description "Evidence for the 1200 BCE collapse"
engine moc-add <moc-id> <claim-id>
```

---

## 8. Research Questions: The Open Cases

**What it does:** Tracks questions you're investigating.

**Analogy: The Detective's Open Cases**
Every detective has open cases—questions that nag at them. "What caused the Bronze Age collapse?" is an open case. As you find evidence, you link it to the case. Some cases get solved (answered), others stay open. This keeps your research goal-directed rather than aimless.

```bash
engine ask "What conditions precede imperial collapse?"
engine evidence <question-id> --claim <claim-id> --relevance "Supports elite theory"
```

---

## 9. Pattern Detection: The Conspiracy Board

**What it does:** Identifies patterns across multiple videos and claims.

**Analogy: The Conspiracy Board**
You've seen it in movies—the detective's wall covered with photos, newspaper clippings, and red string connecting everything. Pattern detection is when you step back and notice: "Wait, this same dynamic appears in Rome AND Han China AND the Mayans." That's a pattern worth naming.

**Pattern types:**
- `recurring_theme` - Same idea appears multiple times
- `contradiction` - Conflicting claims across sources
- `consensus` - Agreement across sources
- `parallel` - Similar events in different contexts
- `evolution` - Idea changing over time

```bash
engine pattern -t parallel "Elite overproduction precedes collapse in multiple empires" \
  --claims "1,5,12" --videos "id1,id2"
```

---

## The Complete Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. INGEST          Fetch video → Store transcript              │
├─────────────────────────────────────────────────────────────────┤
│  2. TAG             Assign era + region                         │
├─────────────────────────────────────────────────────────────────┤
│  3. EXTRACT         Pull atomic claims with quotes              │
├─────────────────────────────────────────────────────────────────┤
│  4. LINK            Connect claims (2+ links each)              │
├─────────────────────────────────────────────────────────────────┤
│  5. ANALYZE         Apply frameworks (cycles, causes, flows)    │
├─────────────────────────────────────────────────────────────────┤
│  6. LOCATE          Pin locations on map                        │
├─────────────────────────────────────────────────────────────────┤
│  7. ORGANIZE        Group into MOCs                             │
├─────────────────────────────────────────────────────────────────┤
│  8. QUESTION        Track research questions + evidence         │
├─────────────────────────────────────────────────────────────────┤
│  9. SYNTHESIZE      Detect cross-source patterns                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Why This Matters

Traditional note-taking is linear and siloed. This system is:

- **Atomic** - Break everything into smallest units
- **Connected** - Link everything to everything relevant
- **Multi-lens** - Apply different analytical frameworks
- **Queryable** - Find patterns across hundreds of sources
- **Geographic** - See history spatially, not just temporally
- **Goal-directed** - Research questions keep you focused

The result: Instead of a pile of notes, you build a living knowledge graph that surfaces insights you'd never find by reading linearly.
