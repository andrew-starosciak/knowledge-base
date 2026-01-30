# Historical Synthesis Engine

A CLI tool for building a knowledge base from YouTube video transcripts, focused on historical research and pattern analysis.

## Installation

### Docker (Recommended)

```bash
# Build and start the web server
docker compose up -d --build

# Run CLI commands
docker compose run --rm engine list
docker compose run --rm engine fetch "https://youtube.com/watch?v=..."

# View logs
docker compose logs -f

# Stop
docker compose down
```

### Local Development

```bash
cargo build --release
```

Requires `yt-dlp` for transcript fetching:
```bash
# macOS
brew install yt-dlp

# Linux
pip install yt-dlp
```

## Daily Workflow

### 1. Ingest Content

```bash
# Fetch a video transcript (auto-added to AI processing queue)
engine fetch "https://youtube.com/watch?v=..."

# Or fetch without queueing
engine fetch "https://youtube.com/watch?v=..." --no-queue

# Auto-tag based on title/description
engine auto-tag <video-id>
```

### 2. Extract Claims (Manual or AI-Assisted)

**Option A: AI-Assisted (Recommended)**

Use the `/process` Claude Code skill to automatically extract claims:
```bash
# In Claude Code, run:
/process
```

This will process queued videos, extracting claims, applying frameworks, and organizing into MOCs.

**Option B: Manual Extraction**

Watch the video and extract atomic factual statements:

```bash
# Add a claim with source quote
engine add-claim <video-id> "Elite overproduction precedes instability" \
  --quote "too many elites competing for positions" \
  --category causal \
  --at 342.5

# Connect related claims (aim for 2+ links per claim)
engine link 1 2 --as causes
engine link 3 1 --as supports
```

### 3. Search & Explore

```bash
# Full-text search
engine search "bronze age collapse"

# Filter by metadata
engine search "trade" --era "Bronze Age" --region "Mesopotamia"

# Browse by category
engine browse --era "Classical Antiquity"
```

### 4. Organize with MOCs

When a topic accumulates enough claims, create a Map of Content:

```bash
engine moc-create "Bronze Age Trade Networks"
engine moc-add 1 <claim-id>
engine moc-add 1 <claim-id>
engine moc 1  # View the MOC
```

### 5. Track Research Questions

```bash
# Create a question
engine ask "What conditions precede imperial collapse?"

# Link evidence as you find it
engine evidence 1 --claim 5 --relevance "Supports elite competition theory"

# Mark as answered when satisfied
engine answer-question 1 --status answered
```

### 6. Review & Maintain

```bash
# See what needs attention
engine review

# Check orphan claims (need more connections)
engine review --orphans

# Revisit stale claims (not accessed in 30+ days)
engine review --stale
```

## Analytical Frameworks

Track patterns using established historical theories:

```bash
# Cliodynamics (Turchin)
engine cyclical <video-id> -t elite_overproduction -e "Roman Empire" "Description..."

# Causal chains
engine causal <cause-claim> <effect-claim> --loop-type positive --strength strong

# World-systems (Wallerstein)
engine position "Rome" --era "Classical Antiquity" --position core
engine flow <from-id> <to-id> "grain" --era "Classical Antiquity"

# Idea transmission (Boyd/Richerson)
engine transmission "Greek philosophy" --from "Athens" --to "Rome" --type horizontal
```

## Quick Reference

```bash
engine list                    # List all videos
engine show <id>               # Show video details
engine claims <video-id>       # List claims for a video
engine claim <id>              # Show claim with links
engine stats                   # Database statistics
engine framework-stats         # Analytical framework stats
engine synthesis-stats         # MOCs, questions, patterns

# AI Processing Queue
engine queue                   # Show pending videos
engine queue --all             # Show all queue items
engine queue-add <id>          # Add video to queue
engine export-transcript <id>  # Export transcript for AI
```

## Data Location

- Database: `./data/knowledge.db` (SQLite)
- All data is local and portable
- Web UI available at `http://localhost:3000` when running the server
