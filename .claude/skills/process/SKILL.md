---
name: process
description: Process videos from AI queue - extract claims and add to knowledge base
allowed-tools: Bash(./target/release/engine *), Bash(./target/debug/engine *), Bash(cargo run -- *), Read, Grep
---

# AI Claim Extraction Processor

You are processing videos from the Historical Synthesis Engine's AI queue. Your job is to extract atomic claims from video transcripts and add them to the knowledge base.

## Current Queue Status
!`./target/debug/engine queue 2>/dev/null || cargo run -- queue 2>/dev/null`

## Processing Workflow

For each pending video in the queue, follow these steps:

### 1. Mark as In-Progress
```bash
./target/debug/engine queue-start <video-id>
```

### 2. Export and Read the Transcript
```bash
./target/debug/engine export-transcript <video-id>
```

Read through the transcript carefully, identifying:
- Factual statements
- Causal claims (X causes Y)
- Theoretical claims
- Interpretive claims
- Key quotes with timestamps

### 3. Extract Atomic Claims

For each significant claim in the transcript, generate and run a command:

```bash
./target/debug/engine add-claim <video-id> "claim text" \
  --quote "source quote from transcript" \
  --category <factual|causal|cyclical|memetic|geopolitical> \
  --confidence <high|medium|low> \
  --at <timestamp_in_seconds>
```

**Guidelines for claim extraction:**
- Each claim should be atomic - one idea per claim
- Include the exact quote from the transcript that supports the claim
- Use the timestamp from the transcript (convert MM:SS to seconds)
- Aim for 10-30 claims per video depending on content density
- Categorize appropriately:
  - `factual`: General historical facts
  - `causal`: X causes Y relationships
  - `cyclical`: Recurring historical patterns (cliodynamics)
  - `memetic`: Idea/ideology transmission
  - `geopolitical`: Core/periphery dynamics

### 4. Link Related Claims

After extracting claims, link related ones:

```bash
./target/debug/engine link <claim-id> <claim-id> --as <relationship>
```

Relationship types:
- `supports` - Second claim supports the first
- `contradicts` - Claims are in opposition
- `elaborates` - Second claim elaborates on the first
- `causes` - First claim causes the second
- `caused_by` - First claim is caused by the second
- `related` - General relationship

**Aim for 2+ links per claim** (Zettelkasten principle).

### 5. Add Analytical Frameworks (if applicable)

For cyclical patterns (Turchin's cliodynamics):
```bash
./target/debug/engine cyclical <video-id> -t <type> -e "Entity" "Description"
```
Types: elite_overproduction, fiscal_strain, social_unrest, population_pressure, asabiyyah, center_periphery

For causal chains:
```bash
./target/debug/engine causal <cause-claim-id> <effect-claim-id> -l <positive|negative|linear> -s <strong|moderate|weak>
```

For idea transmission:
```bash
./target/debug/engine transmission "idea" -f "source" -t "target" -y <horizontal|vertical|oblique> -v <video-id>
```

### 6. Mark as Completed

Count the claims you extracted and mark the video complete:

```bash
./target/debug/engine queue-complete <video-id> --claims <count>
```

### 7. Check for More Videos

After completing one video, check the queue for more:
```bash
./target/debug/engine queue
```

## Error Handling

If you encounter an error processing a video:
```bash
./target/debug/engine queue-fail <video-id> --reason "description of error"
```

## Example Session

```bash
# Check queue
./target/debug/engine queue

# Start processing
./target/debug/engine queue-start Jjqf9T59uY0

# Read transcript
./target/debug/engine export-transcript Jjqf9T59uY0

# Extract claims (example)
./target/debug/engine add-claim Jjqf9T59uY0 "Agriculture preceded permanent settlement" \
  --quote "farming actually happened first, and then later people started living in houses" \
  --category factual \
  --confidence high \
  --at 300

# Link claims
./target/debug/engine link 1 2 --as supports

# Complete
./target/debug/engine queue-complete Jjqf9T59uY0 --claims 15
```

## Important Notes

- Process one video at a time for reliability
- Always verify claims against the transcript text
- Be conservative with confidence levels
- Focus on substantive claims, not minor details
- If a video has no meaningful claims to extract, still mark it complete with --claims 0
