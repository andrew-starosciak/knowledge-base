use anyhow::Result;
use clap::{Parser, Subcommand};
use engine::{Database, Fetcher, SourceType, VisualType, EvidenceType};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "engine")]
#[command(about = "YouTube transcript knowledge base", long_about = None)]
struct Cli {
    /// Database file path
    #[arg(short, long, default_value = "knowledge.db")]
    database: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch a YouTube video transcript and store it
    Fetch {
        /// YouTube URL or video ID
        url: String,
        /// Don't add to AI processing queue
        #[arg(long)]
        no_queue: bool,
    },
    /// List all stored videos
    List,
    /// Show a video and its transcript
    Show {
        /// Video ID
        id: String,
        /// Show full transcript
        #[arg(short, long)]
        full: bool,
    },
    /// Search transcripts (basic full-text search)
    Search {
        /// Search query
        query: String,
        /// Filter by era
        #[arg(short, long)]
        era: Option<String>,
        /// Filter by region
        #[arg(short, long)]
        region: Option<String>,
        /// Filter by topic
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// Tag a video with era and/or region
    Tag {
        /// Video ID
        id: String,
        /// Era (e.g., "Bronze Age", "Classical Antiquity")
        #[arg(short, long)]
        era: Option<String>,
        /// Region/civilization (e.g., "Mesopotamia", "Egypt")
        #[arg(short, long)]
        region: Option<String>,
    },
    /// List all eras
    Eras,
    /// List all regions
    Regions,
    /// Add a new region
    AddRegion {
        /// Region name
        name: String,
        /// Parent region (optional)
        #[arg(short, long)]
        parent: Option<String>,
    },
    /// Browse videos by era and/or region
    Browse {
        /// Filter by era
        #[arg(short, long)]
        era: Option<String>,
        /// Filter by region
        #[arg(short, long)]
        region: Option<String>,
    },
    /// Add a topic to a video
    Topic {
        /// Video ID
        id: String,
        /// Topic name to add
        #[arg(short, long)]
        add: Option<String>,
    },
    /// List all topics
    Topics,
    /// Browse videos by topic
    ByTopic {
        /// Topic name
        name: String,
    },
    /// Add a video to a collection
    Collect {
        /// Video ID
        id: String,
        /// Collection name
        #[arg(short, long)]
        into: String,
    },
    /// List all collections or show videos in a collection
    Collections {
        /// Collection name (optional, shows videos in collection)
        name: Option<String>,
    },
    /// Create a new collection
    NewCollection {
        /// Collection name
        name: String,
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Add a note to a video
    Note {
        /// Video ID
        id: String,
        /// Note text
        text: String,
        /// Timestamp in seconds (optional)
        #[arg(short, long)]
        at: Option<f64>,
    },
    /// Show notes for a video
    Notes {
        /// Video ID
        id: String,
    },
    /// Add a location to a video for map visualization
    Locate {
        /// Video ID
        id: String,
        /// Place name (e.g., "Athens", "Babylon")
        #[arg(short, long)]
        place: String,
        /// Latitude
        #[arg(long)]
        lat: f64,
        /// Longitude
        #[arg(long)]
        lon: f64,
        /// Era for this location reference
        #[arg(short, long)]
        era: Option<String>,
        /// Topic for this location reference
        #[arg(short, long)]
        topic: Option<String>,
        /// Timestamp in video (seconds)
        #[arg(long)]
        at: Option<f64>,
        /// Note about this location
        #[arg(short, long)]
        note: Option<String>,
    },
    /// List all locations
    Locations,
    /// Start web server for map visualization
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Suggest tags for a video based on title/description
    SuggestTags {
        /// Video ID
        id: String,
    },
    /// Auto-tag a video based on title/description
    AutoTag {
        /// Video ID (or "all" to tag all videos)
        id: String,
    },
    /// Rebuild the search index
    RebuildIndex,

    // Phase 5: Research Tools

    /// Save a search for later use
    SaveSearch {
        /// Name for the saved search
        name: String,
        /// Search query (optional)
        #[arg(short, long)]
        query: Option<String>,
        /// Era filter (optional)
        #[arg(short, long)]
        era: Option<String>,
        /// Region filter (optional)
        #[arg(short, long)]
        region: Option<String>,
        /// Topic filter (optional)
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// List all saved searches
    Searches,
    /// Run a saved search
    RunSearch {
        /// Name of the saved search
        name: String,
    },
    /// Delete a saved search
    DeleteSearch {
        /// Name of the saved search
        name: String,
    },
    /// Export a collection as markdown
    Export {
        /// Collection name
        collection: String,
        /// Output file (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Export map data as GeoJSON
    ExportMap {
        /// Filter by era
        #[arg(short, long)]
        era: Option<String>,
        /// Filter by topic
        #[arg(short, long)]
        topic: Option<String>,
        /// Output file (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Generate summary reports
    Report {
        /// Report type: era, region, or topic
        #[arg(short, long, default_value = "era")]
        by: String,
    },
    /// Show database statistics
    Stats,

    // Phase 6: Claim Extraction & Atomic Notes

    /// Add a claim extracted from a video
    AddClaim {
        /// Video ID
        video_id: String,
        /// The claim text (atomic factual statement)
        text: String,
        /// Source quote from transcript
        #[arg(short, long)]
        quote: String,
        /// Category: cyclical, causal, memetic, geopolitical, factual
        #[arg(short, long, default_value = "factual")]
        category: String,
        /// Confidence: high, medium, low
        #[arg(long, default_value = "medium")]
        confidence: String,
        /// Timestamp in video (seconds)
        #[arg(short, long)]
        at: Option<f64>,
    },
    /// List claims for a video
    Claims {
        /// Video ID
        video_id: String,
    },
    /// List all claims or filter by category
    AllClaims {
        /// Filter by category: cyclical, causal, memetic, geopolitical, factual
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Show a claim with its links
    Claim {
        /// Claim ID
        id: i64,
    },
    /// Link two claims together
    Link {
        /// Source claim ID
        source: i64,
        /// Target claim ID
        target: i64,
        /// Link type: supports, contradicts, elaborates, caused_by, causes, related
        #[arg(short, long, default_value = "related")]
        r#as: String,
    },
    /// Remove a link between claims
    Unlink {
        /// Source claim ID
        source: i64,
        /// Target claim ID
        target: i64,
    },
    /// Show claims that need more connections (< 2 links)
    Unlinked,
    /// Delete a claim
    DeleteClaim {
        /// Claim ID
        id: i64,
    },
    /// Generate chunks from a video transcript
    Chunk {
        /// Video ID (or "all" for all videos)
        id: String,
        /// Target tokens per chunk (default: 2000)
        #[arg(short, long, default_value = "2000")]
        tokens: i32,
        /// Overlap percentage (default: 15)
        #[arg(short, long, default_value = "15")]
        overlap: i32,
    },
    /// Show chunks for a video
    Chunks {
        /// Video ID
        video_id: String,
    },
    /// Save a progressive summarization layer
    Summarize {
        /// Video ID
        video_id: String,
        /// Layer number (2-4)
        #[arg(short, long)]
        layer: u8,
        /// Summary content (reads from stdin if not provided)
        #[arg(short, long)]
        content: Option<String>,
    },
    /// Show summary layers for a video
    Layers {
        /// Video ID
        video_id: String,
    },
    /// Show claim extraction statistics
    ClaimStats,

    // Phase 7: Semantic Search & Embeddings

    /// Store an embedding for a video, chunk, or claim
    Embed {
        /// Source type: video, chunk, claim
        #[arg(short, long)]
        source: String,
        /// Source ID (video_id, video_id:chunk_index, or claim_id)
        id: String,
        /// Embedding vector as JSON array (e.g., "[0.1, 0.2, ...]")
        #[arg(short, long)]
        vector: String,
        /// Model name (default: "default")
        #[arg(short, long, default_value = "default")]
        model: String,
    },
    /// Import embeddings from a JSON file
    ImportEmbeddings {
        /// Path to JSON file with embeddings
        file: String,
        /// Model name (default: "default")
        #[arg(short, long, default_value = "default")]
        model: String,
    },
    /// Export items that need embeddings (for external processing)
    ExportForEmbedding {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Source type to export: video, chunk, claim, all
        #[arg(short, long, default_value = "all")]
        source: String,
    },
    /// Semantic search using a query embedding
    Semantic {
        /// Query embedding as JSON array
        #[arg(short, long)]
        vector: String,
        /// Filter by source type: video, chunk, claim
        #[arg(short, long)]
        source: Option<String>,
        /// Number of results (default: 10)
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Hybrid search combining keyword and semantic
    Hybrid {
        /// Text query for keyword search
        query: String,
        /// Query embedding as JSON array (optional)
        #[arg(short, long)]
        vector: Option<String>,
        /// Keyword weight (default: 0.5)
        #[arg(long, default_value = "0.5")]
        kw_weight: f32,
        /// Semantic weight (default: 0.5)
        #[arg(long, default_value = "0.5")]
        sem_weight: f32,
        /// Number of results (default: 10)
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Find similar items to a given embedding source
    Similar {
        /// Source type: video, chunk, claim
        #[arg(short, long)]
        source: String,
        /// Source ID
        id: String,
        /// Number of results (default: 10)
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Show embedding statistics
    EmbedStats,

    // Phase 8: Analytical Frameworks

    /// Add a cyclical pattern indicator (Cliodynamics)
    Cyclical {
        /// Video ID
        video_id: String,
        /// Indicator type: elite_overproduction, fiscal_strain, social_unrest, population_pressure, asabiyyah, center_periphery
        #[arg(short, long)]
        r#type: String,
        /// Entity (civilization/state) being described
        #[arg(short, long)]
        entity: String,
        /// Description of the indicator
        description: String,
        /// Optional claim ID to link
        #[arg(short, long)]
        claim: Option<i64>,
        /// Era name (optional)
        #[arg(long)]
        era: Option<String>,
        /// Timestamp in video
        #[arg(long)]
        at: Option<f64>,
    },
    /// List cyclical indicators
    ListCyclical {
        /// Filter by type
        #[arg(short, long)]
        r#type: Option<String>,
        /// Filter by entity
        #[arg(short, long)]
        entity: Option<String>,
    },
    /// Delete a cyclical indicator
    DeleteCyclical {
        /// Indicator ID
        id: i64,
    },
    /// Track a causal relationship between claims
    Causal {
        /// Cause claim ID
        cause: i64,
        /// Effect claim ID
        effect: i64,
        /// Loop type: positive, negative, linear
        #[arg(short, long, default_value = "linear")]
        loop_type: String,
        /// Relation strength: strong, moderate, weak, speculative
        #[arg(short, long, default_value = "moderate")]
        strength: String,
        /// Notes about the relationship
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List causal relations
    ListCausal {
        /// Filter by loop type
        #[arg(short, long)]
        loop_type: Option<String>,
        /// Show relations for a specific claim
        #[arg(short, long)]
        claim: Option<i64>,
    },
    /// Delete a causal relation
    DeleteCausal {
        /// Relation ID
        id: i64,
    },
    /// Track idea transmission (memetic)
    Transmission {
        /// The idea being transmitted
        idea: String,
        /// Source entity (culture/institution)
        #[arg(short, long)]
        from: String,
        /// Target entity (culture/institution)
        #[arg(short, long)]
        to: String,
        /// Transmission type: horizontal, vertical, oblique
        #[arg(short = 'y', long, default_value = "horizontal")]
        r#type: String,
        /// Video ID
        #[arg(short, long)]
        video: String,
        /// Era name
        #[arg(short, long)]
        era: Option<String>,
        /// Region name
        #[arg(short, long)]
        region: Option<String>,
        /// Claim ID
        #[arg(short, long)]
        claim: Option<i64>,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List idea transmissions
    ListTransmissions {
        /// Filter by idea (partial match)
        #[arg(short, long)]
        idea: Option<String>,
        /// Filter by type
        #[arg(short, long)]
        r#type: Option<String>,
    },
    /// Delete an idea transmission
    DeleteTransmission {
        /// Transmission ID
        id: i64,
    },
    /// Define a geopolitical entity in a world-system
    Position {
        /// Entity name (civilization/state)
        name: String,
        /// Era name
        #[arg(short, long)]
        era: String,
        /// System position: core, semi_periphery, periphery
        #[arg(short, long)]
        position: String,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List geopolitical entities
    ListPositions {
        /// Filter by era
        #[arg(short, long)]
        era: Option<String>,
        /// Filter by position
        #[arg(short, long)]
        position: Option<String>,
    },
    /// Update a geopolitical entity's position
    UpdatePosition {
        /// Entity ID
        id: i64,
        /// New position: core, semi_periphery, periphery
        #[arg(short, long)]
        position: String,
    },
    /// Track surplus flow between entities
    Flow {
        /// Source entity ID
        from: i64,
        /// Target entity ID
        to: i64,
        /// Commodity being transferred
        commodity: String,
        /// Era name
        #[arg(short, long)]
        era: String,
        /// Video ID
        #[arg(short, long)]
        video: Option<String>,
        /// Claim ID
        #[arg(short, long)]
        claim: Option<i64>,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List surplus flows
    ListFlows {
        /// Filter by era
        #[arg(short, long)]
        era: Option<String>,
        /// Show flows for a specific entity ID
        #[arg(long)]
        entity: Option<i64>,
    },
    /// Delete a surplus flow
    DeleteFlow {
        /// Flow ID
        id: i64,
    },
    /// Classify a claim by Braudel's timescale
    Timescale {
        /// Claim ID
        claim_id: i64,
        /// Timescale: event, conjuncture, longue_duree
        #[arg(short, long)]
        scale: String,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List temporal observations by timescale
    ListTimescales {
        /// Filter by timescale
        #[arg(short, long)]
        scale: Option<String>,
    },
    /// Show analytical framework statistics
    FrameworkStats,

    // Phase 9: Synthesis & Pattern Detection

    /// Create a Map of Content
    MocCreate {
        /// MOC title
        title: String,
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List all Maps of Content
    Mocs,
    /// Show a Map of Content with its claims
    Moc {
        /// MOC ID or title
        id: String,
    },
    /// Add a claim to a MOC
    MocAdd {
        /// MOC ID
        moc: i64,
        /// Claim ID
        claim: i64,
        /// Sort order (optional)
        #[arg(short, long, default_value = "0")]
        order: i32,
    },
    /// Remove a claim from a MOC
    MocRemove {
        /// MOC ID
        moc: i64,
        /// Claim ID
        claim: i64,
    },
    /// Delete a Map of Content
    DeleteMoc {
        /// MOC ID
        id: i64,
    },
    /// Create a research question
    Ask {
        /// The research question
        question: String,
        /// Parent question ID (for sub-questions)
        #[arg(short, long)]
        parent: Option<i64>,
        /// Notes
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List research questions
    Questions {
        /// Filter by status: active, answered, refined, parked
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Show a research question with evidence
    Question {
        /// Question ID
        id: i64,
    },
    /// Add evidence to a research question
    Evidence {
        /// Question ID
        question: i64,
        /// Claim ID (optional)
        #[arg(short, long)]
        claim: Option<i64>,
        /// Video ID (optional)
        #[arg(short, long)]
        video: Option<String>,
        /// How this evidence relates to the question
        #[arg(short, long)]
        relevance: Option<String>,
    },
    /// Update question status
    AnswerQuestion {
        /// Question ID
        id: i64,
        /// New status: active, answered, refined, parked
        #[arg(short, long)]
        status: String,
    },
    /// Delete a research question
    DeleteQuestion {
        /// Question ID
        id: i64,
    },
    /// Record a detected pattern
    Pattern {
        /// Pattern type: recurring_theme, contradiction, consensus, evolution, parallel
        #[arg(short, long)]
        r#type: String,
        /// Description of the pattern
        description: String,
        /// Video IDs involved (comma-separated)
        #[arg(short, long)]
        videos: Option<String>,
        /// Claim IDs involved (comma-separated)
        #[arg(short, long)]
        claims: Option<String>,
        /// Confidence (0.0-1.0)
        #[arg(long, default_value = "0.7")]
        confidence: f32,
    },
    /// List detected patterns
    Patterns {
        /// Filter by type
        #[arg(short, long)]
        r#type: Option<String>,
    },
    /// Delete a detected pattern
    DeletePattern {
        /// Pattern ID
        id: i64,
    },
    /// Show review queue (items needing attention)
    Review {
        /// Show only stale claims (not accessed in 30+ days)
        #[arg(long)]
        stale: bool,
        /// Show only orphan claims (< 2 links)
        #[arg(long)]
        orphans: bool,
        /// Number of random suggestions
        #[arg(short, long, default_value = "5")]
        random: usize,
    },
    /// Show synthesis statistics
    SynthesisStats,

    // Phase 10: AI Processing Queue

    /// Show AI processing queue
    Queue {
        /// Show all items including completed
        #[arg(short, long)]
        all: bool,
    },
    /// Add a video to the processing queue
    QueueAdd {
        /// Video ID
        video_id: String,
        /// Priority (higher = process first)
        #[arg(short, long, default_value = "0")]
        priority: i32,
    },
    /// Skip a video (won't be processed)
    QueueSkip {
        /// Video ID
        video_id: String,
    },
    /// Reset a failed/skipped video to pending
    QueueReset {
        /// Video ID
        video_id: String,
    },
    /// Mark a video as in-progress
    QueueStart {
        /// Video ID
        video_id: String,
    },
    /// Mark a video as completed
    QueueComplete {
        /// Video ID
        video_id: String,
        /// Number of claims extracted
        #[arg(short, long)]
        claims: i32,
    },
    /// Mark a video as failed
    QueueFail {
        /// Video ID
        video_id: String,
        /// Error message
        #[arg(short, long)]
        reason: String,
    },
    /// Clear completed or failed items from queue
    QueueClear {
        /// Clear completed items
        #[arg(long)]
        completed: bool,
        /// Clear failed items
        #[arg(long)]
        failed: bool,
    },
    /// Export transcript as plain text for AI processing
    ExportTranscript {
        /// Video ID
        video_id: String,
    },
    /// Export pending video IDs from queue
    ExportQueue,

    // Phase 12: Expanded Knowledge Entities

    /// Add a source (book, paper, documentary)
    #[command(name = "add-source")]
    AddSource {
        /// Title of the source
        title: String,
        /// Author(s)
        #[arg(long)]
        author: Option<String>,
        /// Type: book, paper, documentary, article, lecture
        #[arg(short = 't', long, default_value = "book")]
        source_type: String,
        /// Publication year
        #[arg(short, long)]
        year: Option<i32>,
        /// URL if available
        #[arg(long)]
        url: Option<String>,
        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },
    /// List all sources
    Sources,
    /// Cite a source in a video
    #[command(name = "cite-source")]
    CiteSource {
        /// Video ID
        video_id: String,
        /// Source ID
        source_id: i64,
        /// Timestamp when mentioned
        #[arg(long)]
        at: Option<f64>,
        /// Context of the citation
        #[arg(long)]
        context: Option<String>,
    },

    /// Add a scholar/thinker
    #[command(name = "add-scholar")]
    AddScholar {
        /// Name of the scholar
        name: String,
        /// Field of study
        #[arg(long)]
        field: Option<String>,
        /// Era they lived/worked
        #[arg(long)]
        era: Option<String>,
        /// Brief summary of their contribution
        #[arg(long)]
        contribution: Option<String>,
    },
    /// List all scholars
    Scholars,
    /// Cite a scholar in a video
    #[command(name = "cite-scholar")]
    CiteScholar {
        /// Video ID
        video_id: String,
        /// Scholar ID
        scholar_id: i64,
        /// Timestamp when mentioned
        #[arg(long)]
        at: Option<f64>,
        /// Context of the mention
        #[arg(long)]
        context: Option<String>,
    },

    /// Add a visual (image, diagram, artifact shown in video)
    #[command(name = "add-visual")]
    AddVisual {
        /// Video ID
        video_id: String,
        /// Description of the visual
        description: String,
        /// Timestamp when shown
        #[arg(long)]
        at: f64,
        /// Type: painting, map, diagram, artifact, chart, photo, skeleton, symbol
        #[arg(short = 't', long, default_value = "photo")]
        visual_type: String,
        /// Why this visual is significant
        #[arg(long)]
        significance: Option<String>,
        /// Location name (will be looked up)
        #[arg(long)]
        location: Option<String>,
        /// Era name
        #[arg(long)]
        era: Option<String>,
    },
    /// List visuals for a video
    Visuals {
        /// Video ID
        video_id: String,
    },

    /// Define a term/concept
    Define {
        /// The term to define
        term: String,
        /// Definition text
        definition: String,
        /// Domain: philosophy, archaeology, religion, sociology, etc.
        #[arg(long)]
        domain: Option<String>,
        /// Video where first defined
        #[arg(long)]
        video: Option<String>,
        /// Timestamp when defined
        #[arg(long)]
        at: Option<f64>,
        /// Scholar who coined it (name, will be looked up)
        #[arg(long)]
        scholar: Option<String>,
    },
    /// List all terms
    Terms,

    /// Add evidence cited in a video
    #[command(name = "add-evidence")]
    AddEvidence {
        /// Video ID
        video_id: String,
        /// Description of the evidence
        description: String,
        /// Type: archaeological, genetic, textual, anthropological, linguistic, artistic, scientific
        #[arg(short = 't', long, default_value = "archaeological")]
        evidence_type: String,
        /// Timestamp when discussed
        #[arg(long)]
        at: Option<f64>,
        /// Location name (will be looked up)
        #[arg(long)]
        location: Option<String>,
        /// Era name
        #[arg(long)]
        era: Option<String>,
    },
    /// List evidence for a video
    #[command(name = "video-evidence")]
    VideoEvidence {
        /// Video ID
        video_id: String,
    },

    /// Add a notable quote
    #[command(name = "add-quote")]
    AddQuote {
        /// Video ID
        video_id: String,
        /// The quote text
        text: String,
        /// Who said it
        #[arg(long)]
        speaker: Option<String>,
        /// Timestamp
        #[arg(long)]
        at: Option<f64>,
        /// Context/significance
        #[arg(long)]
        context: Option<String>,
    },
    /// List quotes from a video
    Quotes {
        /// Video ID
        video_id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let db = Database::open(&cli.database)?;

    match cli.command {
        Commands::Fetch { url, no_queue } => cmd_fetch(&db, &url, no_queue),
        Commands::List => cmd_list(&db),
        Commands::Show { id, full } => cmd_show(&db, &id, full),
        Commands::Search { query, era, region, topic } => {
            cmd_search(&db, &query, era.as_deref(), region.as_deref(), topic.as_deref())
        }
        Commands::Tag { id, era, region } => cmd_tag(&db, &id, era.as_deref(), region.as_deref()),
        Commands::Eras => cmd_eras(&db),
        Commands::Regions => cmd_regions(&db),
        Commands::AddRegion { name, parent } => cmd_add_region(&db, &name, parent.as_deref()),
        Commands::Browse { era, region } => cmd_browse(&db, era.as_deref(), region.as_deref()),
        Commands::Topic { id, add } => cmd_topic(&db, &id, add.as_deref()),
        Commands::Topics => cmd_topics(&db),
        Commands::ByTopic { name } => cmd_by_topic(&db, &name),
        Commands::Collect { id, into } => cmd_collect(&db, &id, &into),
        Commands::Collections { name } => cmd_collections(&db, name.as_deref()),
        Commands::NewCollection { name, description } => cmd_new_collection(&db, &name, description.as_deref()),
        Commands::Note { id, text, at } => cmd_note(&db, &id, &text, at),
        Commands::Notes { id } => cmd_notes(&db, &id),
        Commands::Locate { id, place, lat, lon, era, topic, at, note } => {
            cmd_locate(&db, &id, &place, lat, lon, era.as_deref(), topic.as_deref(), at, note.as_deref())
        }
        Commands::Locations => cmd_locations(&db),
        Commands::Serve { port } => cmd_serve(cli.database, port),
        Commands::SuggestTags { id } => cmd_suggest_tags(&db, &id),
        Commands::AutoTag { id } => cmd_auto_tag(&db, &id),
        Commands::RebuildIndex => cmd_rebuild_index(&db),
        // Phase 5 commands
        Commands::SaveSearch { name, query, era, region, topic } => {
            cmd_save_search(&db, &name, query.as_deref(), era.as_deref(), region.as_deref(), topic.as_deref())
        }
        Commands::Searches => cmd_list_searches(&db),
        Commands::RunSearch { name } => cmd_run_search(&db, &name),
        Commands::DeleteSearch { name } => cmd_delete_search(&db, &name),
        Commands::Export { collection, output } => cmd_export(&db, &collection, output.as_deref()),
        Commands::ExportMap { era, topic, output } => {
            cmd_export_map(&db, era.as_deref(), topic.as_deref(), output.as_deref())
        }
        Commands::Report { by } => cmd_report(&db, &by),
        Commands::Stats => cmd_stats(&db),
        // Phase 6 commands
        Commands::AddClaim { video_id, text, quote, category, confidence, at } => {
            cmd_add_claim(&db, &video_id, &text, &quote, &category, &confidence, at)
        }
        Commands::Claims { video_id } => cmd_claims(&db, &video_id),
        Commands::AllClaims { category } => cmd_all_claims(&db, category.as_deref()),
        Commands::Claim { id } => cmd_claim(&db, id),
        Commands::Link { source, target, r#as } => cmd_link(&db, source, target, &r#as),
        Commands::Unlink { source, target } => cmd_unlink(&db, source, target),
        Commands::Unlinked => cmd_unlinked(&db),
        Commands::DeleteClaim { id } => cmd_delete_claim(&db, id),
        Commands::Chunk { id, tokens, overlap } => cmd_chunk(&db, &id, tokens, overlap),
        Commands::Chunks { video_id } => cmd_chunks(&db, &video_id),
        Commands::Summarize { video_id, layer, content } => {
            cmd_summarize(&db, &video_id, layer, content.as_deref())
        }
        Commands::Layers { video_id } => cmd_layers(&db, &video_id),
        Commands::ClaimStats => cmd_claim_stats(&db),
        // Phase 7 commands
        Commands::Embed { source, id, vector, model } => {
            cmd_embed(&db, &source, &id, &vector, &model)
        }
        Commands::ImportEmbeddings { file, model } => cmd_import_embeddings(&db, &file, &model),
        Commands::ExportForEmbedding { output, source } => {
            cmd_export_for_embedding(&db, output.as_deref(), &source)
        }
        Commands::Semantic { vector, source, limit } => {
            cmd_semantic(&db, &vector, source.as_deref(), limit)
        }
        Commands::Hybrid { query, vector, kw_weight, sem_weight, limit } => {
            cmd_hybrid(&db, &query, vector.as_deref(), kw_weight, sem_weight, limit)
        }
        Commands::Similar { source, id, limit } => cmd_similar(&db, &source, &id, limit),
        Commands::EmbedStats => cmd_embed_stats(&db),
        // Phase 8 commands
        Commands::Cyclical { video_id, r#type, entity, description, claim, era, at } => {
            cmd_cyclical(&db, &video_id, &r#type, &entity, &description, claim, era.as_deref(), at)
        }
        Commands::ListCyclical { r#type, entity } => {
            cmd_list_cyclical(&db, r#type.as_deref(), entity.as_deref())
        }
        Commands::DeleteCyclical { id } => cmd_delete_cyclical(&db, id),
        Commands::Causal { cause, effect, loop_type, strength, notes } => {
            cmd_causal(&db, cause, effect, &loop_type, &strength, notes.as_deref())
        }
        Commands::ListCausal { loop_type, claim } => {
            cmd_list_causal(&db, loop_type.as_deref(), claim)
        }
        Commands::DeleteCausal { id } => cmd_delete_causal(&db, id),
        Commands::Transmission { idea, from, to, r#type, video, era, region, claim, notes } => {
            cmd_transmission(&db, &idea, &from, &to, &r#type, &video, era.as_deref(), region.as_deref(), claim, notes.as_deref())
        }
        Commands::ListTransmissions { idea, r#type } => {
            cmd_list_transmissions(&db, idea.as_deref(), r#type.as_deref())
        }
        Commands::DeleteTransmission { id } => cmd_delete_transmission(&db, id),
        Commands::Position { name, era, position, notes } => {
            cmd_position(&db, &name, &era, &position, notes.as_deref())
        }
        Commands::ListPositions { era, position } => {
            cmd_list_positions(&db, era.as_deref(), position.as_deref())
        }
        Commands::UpdatePosition { id, position } => cmd_update_position(&db, id, &position),
        Commands::Flow { from, to, commodity, era, video, claim, notes } => {
            cmd_flow(&db, from, to, &commodity, &era, video.as_deref(), claim, notes.as_deref())
        }
        Commands::ListFlows { era, entity } => cmd_list_flows(&db, era.as_deref(), entity),
        Commands::DeleteFlow { id } => cmd_delete_flow(&db, id),
        Commands::Timescale { claim_id, scale, notes } => {
            cmd_timescale(&db, claim_id, &scale, notes.as_deref())
        }
        Commands::ListTimescales { scale } => cmd_list_timescales(&db, scale.as_deref()),
        Commands::FrameworkStats => cmd_framework_stats(&db),
        // Phase 9 commands
        Commands::MocCreate { title, description } => cmd_moc_create(&db, &title, description.as_deref()),
        Commands::Mocs => cmd_list_mocs(&db),
        Commands::Moc { id } => cmd_show_moc(&db, &id),
        Commands::MocAdd { moc, claim, order } => cmd_moc_add(&db, moc, claim, order),
        Commands::MocRemove { moc, claim } => cmd_moc_remove(&db, moc, claim),
        Commands::DeleteMoc { id } => cmd_delete_moc(&db, id),
        Commands::Ask { question, parent, notes } => {
            cmd_ask(&db, &question, parent, notes.as_deref())
        }
        Commands::Questions { status } => cmd_list_questions(&db, status.as_deref()),
        Commands::Question { id } => cmd_show_question(&db, id),
        Commands::Evidence { question, claim, video, relevance } => {
            cmd_add_evidence(&db, question, claim, video.as_deref(), relevance.as_deref())
        }
        Commands::AnswerQuestion { id, status } => cmd_answer_question(&db, id, &status),
        Commands::DeleteQuestion { id } => cmd_delete_question(&db, id),
        Commands::Pattern { r#type, description, videos, claims, confidence } => {
            cmd_add_pattern(&db, &r#type, &description, videos.as_deref(), claims.as_deref(), confidence)
        }
        Commands::Patterns { r#type } => cmd_list_patterns(&db, r#type.as_deref()),
        Commands::DeletePattern { id } => cmd_delete_pattern(&db, id),
        Commands::Review { stale, orphans, random } => cmd_review(&db, stale, orphans, random),
        Commands::SynthesisStats => cmd_synthesis_stats(&db),

        // Phase 10: AI Processing Queue
        Commands::Queue { all } => cmd_queue(&db, all),
        Commands::QueueAdd { video_id, priority } => cmd_queue_add(&db, &video_id, priority),
        Commands::QueueSkip { video_id } => cmd_queue_skip(&db, &video_id),
        Commands::QueueReset { video_id } => cmd_queue_reset(&db, &video_id),
        Commands::QueueStart { video_id } => cmd_queue_start(&db, &video_id),
        Commands::QueueComplete { video_id, claims } => cmd_queue_complete(&db, &video_id, claims),
        Commands::QueueFail { video_id, reason } => cmd_queue_fail(&db, &video_id, &reason),
        Commands::QueueClear { completed, failed } => cmd_queue_clear(&db, completed, failed),
        Commands::ExportTranscript { video_id } => cmd_export_transcript(&db, &video_id),
        Commands::ExportQueue => cmd_export_queue(&db),

        // Phase 12: Expanded Knowledge Entities
        Commands::AddSource { title, author, source_type, year, url, notes } =>
            cmd_add_source(&db, &title, author.as_deref(), &source_type, year, url.as_deref(), notes.as_deref()),
        Commands::Sources => cmd_list_sources(&db),
        Commands::CiteSource { video_id, source_id, at, context } =>
            cmd_cite_source(&db, &video_id, source_id, at, context.as_deref()),
        Commands::AddScholar { name, field, era, contribution } =>
            cmd_add_scholar(&db, &name, field.as_deref(), era.as_deref(), contribution.as_deref()),
        Commands::Scholars => cmd_list_scholars(&db),
        Commands::CiteScholar { video_id, scholar_id, at, context } =>
            cmd_cite_scholar(&db, &video_id, scholar_id, at, context.as_deref()),
        Commands::AddVisual { video_id, description, at, visual_type, significance, location, era } =>
            cmd_add_visual(&db, &video_id, &description, at, &visual_type, significance.as_deref(), location.as_deref(), era.as_deref()),
        Commands::Visuals { video_id } => cmd_list_visuals(&db, &video_id),
        Commands::Define { term, definition, domain, video, at, scholar } =>
            cmd_define_term(&db, &term, &definition, domain.as_deref(), video.as_deref(), at, scholar.as_deref()),
        Commands::Terms => cmd_list_terms(&db),
        Commands::AddEvidence { video_id, description, evidence_type, at, location, era } =>
            cmd_add_cited_evidence(&db, &video_id, &description, &evidence_type, at, location.as_deref(), era.as_deref()),
        Commands::VideoEvidence { video_id } => cmd_list_cited_evidence(&db, &video_id),
        Commands::AddQuote { video_id, text, speaker, at, context } =>
            cmd_add_quote(&db, &video_id, &text, speaker.as_deref(), at, context.as_deref()),
        Commands::Quotes { video_id } => cmd_list_quotes(&db, &video_id),
    }
}

fn cmd_fetch(db: &Database, url: &str, no_queue: bool) -> Result<()> {
    println!("Fetching: {}", url);

    let fetcher = Fetcher::new();
    let (video, transcript) = fetcher.fetch(url)?;

    println!("Title: {}", video.title);
    if let Some(ref channel) = video.channel {
        println!("Channel: {}", channel);
    }

    db.insert_video(&video)?;

    if let Some(ref t) = transcript {
        db.insert_transcript(t)?;
        println!("Transcript: {} segments, {} chars", t.segments.len(), t.full_text.len());

        // Add to AI processing queue unless --no-queue is set
        if !no_queue {
            db.add_to_queue(&video.id, 0)?;
            println!("Added to AI processing queue");
        }
    } else {
        println!("Transcript: not available");
    }

    println!("Saved: {}", video.id);
    Ok(())
}

fn cmd_list(db: &Database) -> Result<()> {
    let videos = db.list_videos()?;

    if videos.is_empty() {
        println!("No videos stored yet.");
        return Ok(());
    }

    println!("{:<12} {:<50} {}", "ID", "TITLE", "CHANNEL");
    println!("{}", "-".repeat(80));

    for video in videos {
        let title = if video.title.len() > 48 {
            format!("{}...", &video.title[..45])
        } else {
            video.title.clone()
        };
        let channel = video.channel.unwrap_or_default();
        println!("{:<12} {:<50} {}", video.id, title, channel);
    }

    Ok(())
}

fn cmd_show(db: &Database, id: &str, full: bool) -> Result<()> {
    let video = db.get_video(id)?;

    match video {
        Some(v) => {
            println!("Title: {}", v.title);
            println!("ID: {}", v.id);
            println!("URL: {}", v.url);
            if let Some(ref channel) = v.channel {
                println!("Channel: {}", channel);
            }
            if let Some(date) = v.upload_date {
                println!("Upload Date: {}", date);
            }

            // Show eras, regions, topics, collections
            let eras = db.get_video_eras(id)?;
            let regions = db.get_video_regions(id)?;
            let topics = db.get_video_topics(id)?;
            let collections = db.get_video_collections(id)?;

            if !eras.is_empty() {
                let era_names: Vec<_> = eras.iter().map(|e| e.name.as_str()).collect();
                println!("Eras: {}", era_names.join(", "));
            }
            if !regions.is_empty() {
                let region_names: Vec<_> = regions.iter().map(|r| r.name.as_str()).collect();
                println!("Regions: {}", region_names.join(", "));
            }
            if !topics.is_empty() {
                let topic_names: Vec<_> = topics.iter().map(|t| t.name.as_str()).collect();
                println!("Topics: {}", topic_names.join(", "));
            }
            if !collections.is_empty() {
                let coll_names: Vec<_> = collections.iter().map(|c| c.name.as_str()).collect();
                println!("Collections: {}", coll_names.join(", "));
            }

            if let Some(ref desc) = v.description {
                let desc_preview = if desc.len() > 200 && !full {
                    format!("{}...", &desc[..200])
                } else {
                    desc.clone()
                };
                println!("\nDescription:\n{}", desc_preview);
            }

            if let Some(transcript) = db.get_transcript(id)? {
                println!("\n--- Transcript ({} segments) ---\n", transcript.segments.len());
                if full {
                    for seg in &transcript.segments {
                        let mins = (seg.start_time / 60.0) as u32;
                        let secs = (seg.start_time % 60.0) as u32;
                        println!("[{:02}:{:02}] {}", mins, secs, seg.text);
                    }
                } else {
                    let preview = if transcript.full_text.len() > 500 {
                        format!("{}...", &transcript.full_text[..500])
                    } else {
                        transcript.full_text.clone()
                    };
                    println!("{}", preview);
                    println!("\n(Use --full to see complete transcript)");
                }
            } else {
                println!("\nNo transcript available.");
            }
        }
        None => {
            println!("Video not found: {}", id);
        }
    }

    Ok(())
}

fn cmd_search(
    db: &Database,
    query: &str,
    era: Option<&str>,
    region: Option<&str>,
    topic: Option<&str>,
) -> Result<()> {
    // Use advanced search if any filters are provided
    let has_filters = era.is_some() || region.is_some() || topic.is_some();

    if has_filters {
        let results = db.advanced_search(Some(query), era, region, topic)?;

        if results.is_empty() {
            let mut filter_desc = vec![format!("query '{}'", query)];
            if let Some(e) = era { filter_desc.push(format!("era '{}'", e)); }
            if let Some(r) = region { filter_desc.push(format!("region '{}'", r)); }
            if let Some(t) = topic { filter_desc.push(format!("topic '{}'", t)); }
            println!("No results found for: {}", filter_desc.join(", "));
            return Ok(());
        }

        let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();
        println!("Found {} matches across {} videos\n", total_matches, results.len());

        for result in results {
            println!("--- {} ---", result.video.id);
            println!("Title: {}", result.video.title);
            if let Some(ref channel) = result.video.channel {
                println!("Channel: {}", channel);
            }
            println!("URL: {}", result.video.url);

            if !result.eras.is_empty() {
                println!("Eras: {}", result.eras.join(", "));
            }
            if !result.regions.is_empty() {
                println!("Regions: {}", result.regions.join(", "));
            }
            if !result.topics.is_empty() {
                println!("Topics: {}", result.topics.join(", "));
            }
            println!();

            for m in &result.matches {
                let mins = (m.start_time / 60.0) as u32;
                let secs = (m.start_time % 60.0) as u32;
                let url_with_time = format!("{}&t={}s", result.video.url, m.start_time as u32);
                println!("  [{:02}:{:02}] {}", mins, secs, m.text);
                println!("          {}", url_with_time);
                println!();
            }
        }
    } else {
        // Use basic search for simple queries
        let results = db.search_with_timestamps(query)?;

        if results.is_empty() {
            println!("No results found for: {}", query);
            return Ok(());
        }

        let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();
        println!("Found {} matches across {} videos for: {}\n", total_matches, results.len(), query);

        for result in results {
            println!("--- {} ---", result.video.id);
            println!("Title: {}", result.video.title);
            if let Some(ref channel) = result.video.channel {
                println!("Channel: {}", channel);
            }
            println!("URL: {}", result.video.url);
            println!();

            for m in &result.matches {
                let mins = (m.start_time / 60.0) as u32;
                let secs = (m.start_time % 60.0) as u32;
                let url_with_time = format!("{}&t={}s", result.video.url, m.start_time as u32);
                println!("  [{:02}:{:02}] {}", mins, secs, m.text);
                println!("          {}", url_with_time);
                println!();
            }
        }
    }

    Ok(())
}

fn cmd_tag(db: &Database, video_id: &str, era: Option<&str>, region: Option<&str>) -> Result<()> {
    // Verify video exists
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    if era.is_none() && region.is_none() {
        println!("Please specify --era and/or --region");
        return Ok(());
    }

    if let Some(era_name) = era {
        if let Some(era_obj) = db.get_era_by_name(era_name)? {
            db.tag_video_era(video_id, era_obj.id)?;
            println!("Tagged with era: {}", era_obj.name);
        } else {
            println!("Era not found: {}. Use 'eras' command to see available eras.", era_name);
        }
    }

    if let Some(region_name) = region {
        let region_obj = match db.get_region_by_name(region_name)? {
            Some(r) => r,
            None => {
                // Auto-create region if it doesn't exist
                println!("Creating new region: {}", region_name);
                db.create_region(region_name, None)?
            }
        };
        db.tag_video_region(video_id, region_obj.id)?;
        println!("Tagged with region: {}", region_obj.name);
    }

    Ok(())
}

fn cmd_eras(db: &Database) -> Result<()> {
    let eras = db.list_eras()?;

    if eras.is_empty() {
        println!("No eras defined.");
        return Ok(());
    }

    println!("Available eras:\n");
    for era in eras {
        println!("  {}", era.name);
    }

    Ok(())
}

fn cmd_regions(db: &Database) -> Result<()> {
    let regions = db.list_regions()?;

    if regions.is_empty() {
        println!("No regions defined yet. Use 'add-region' or tag a video with --region to create one.");
        return Ok(());
    }

    println!("Available regions:\n");
    for region in regions {
        if let Some(_parent_id) = region.parent_id {
            println!("  - {}", region.name);
        } else {
            println!("  {}", region.name);
        }
    }

    Ok(())
}

fn cmd_add_region(db: &Database, name: &str, parent: Option<&str>) -> Result<()> {
    let parent_id = if let Some(parent_name) = parent {
        match db.get_region_by_name(parent_name)? {
            Some(p) => Some(p.id),
            None => {
                println!("Parent region not found: {}", parent_name);
                return Ok(());
            }
        }
    } else {
        None
    };

    let region = db.create_region(name, parent_id)?;
    println!("Created region: {}", region.name);

    Ok(())
}

fn cmd_browse(db: &Database, era: Option<&str>, region: Option<&str>) -> Result<()> {
    let videos = db.browse_videos(era, region)?;

    if videos.is_empty() {
        let filter = match (era, region) {
            (Some(e), Some(r)) => format!("era '{}' and region '{}'", e, r),
            (Some(e), None) => format!("era '{}'", e),
            (None, Some(r)) => format!("region '{}'", r),
            (None, None) => "no filters".to_string(),
        };
        println!("No videos found for {}.", filter);
        return Ok(());
    }

    let header = match (era, region) {
        (Some(e), Some(r)) => format!("Videos tagged {} + {}:", e, r),
        (Some(e), None) => format!("Videos tagged {}:", e),
        (None, Some(r)) => format!("Videos tagged {}:", r),
        (None, None) => "All videos:".to_string(),
    };

    println!("{}\n", header);
    println!("{:<12} {:<50} {}", "ID", "TITLE", "CHANNEL");
    println!("{}", "-".repeat(80));

    for video in videos {
        let title = if video.title.len() > 48 {
            format!("{}...", &video.title[..45])
        } else {
            video.title.clone()
        };
        let channel = video.channel.unwrap_or_default();
        println!("{:<12} {:<50} {}", video.id, title, channel);
    }

    Ok(())
}

fn cmd_topic(db: &Database, video_id: &str, add: Option<&str>) -> Result<()> {
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    match add {
        Some(topic_name) => {
            let topic = db.get_or_create_topic(topic_name)?;
            db.tag_video_topic(video_id, topic.id)?;
            println!("Tagged with topic: {}", topic.name);
        }
        None => {
            let topics = db.get_video_topics(video_id)?;
            if topics.is_empty() {
                println!("No topics for this video. Use --add to add one.");
            } else {
                println!("Topics:");
                for topic in topics {
                    println!("  {}", topic.name);
                }
            }
        }
    }

    Ok(())
}

fn cmd_topics(db: &Database) -> Result<()> {
    let topics = db.list_topics()?;

    if topics.is_empty() {
        println!("No topics defined yet. Use 'topic VIDEO_ID --add NAME' to create one.");
        return Ok(());
    }

    println!("Available topics:\n");
    for topic in topics {
        println!("  {}", topic.name);
    }

    Ok(())
}

fn cmd_by_topic(db: &Database, topic_name: &str) -> Result<()> {
    let videos = db.browse_by_topic(topic_name)?;

    if videos.is_empty() {
        println!("No videos found for topic: {}", topic_name);
        return Ok(());
    }

    println!("Videos tagged '{}':\n", topic_name);
    println!("{:<12} {:<50} {}", "ID", "TITLE", "CHANNEL");
    println!("{}", "-".repeat(80));

    for video in videos {
        let title = if video.title.len() > 48 {
            format!("{}...", &video.title[..45])
        } else {
            video.title.clone()
        };
        let channel = video.channel.unwrap_or_default();
        println!("{:<12} {:<50} {}", video.id, title, channel);
    }

    Ok(())
}

fn cmd_collect(db: &Database, video_id: &str, collection_name: &str) -> Result<()> {
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    let collection = match db.get_collection_by_name(collection_name)? {
        Some(c) => c,
        None => {
            println!("Collection '{}' not found. Create it first with 'new-collection'.", collection_name);
            return Ok(());
        }
    };

    db.add_video_to_collection(video_id, collection.id)?;
    println!("Added to collection: {}", collection.name);

    Ok(())
}

fn cmd_collections(db: &Database, name: Option<&str>) -> Result<()> {
    match name {
        Some(collection_name) => {
            let videos = db.get_collection_videos(collection_name)?;

            if videos.is_empty() {
                println!("No videos in collection: {}", collection_name);
                return Ok(());
            }

            println!("Collection '{}':\n", collection_name);
            println!("{:<12} {:<50} {}", "ID", "TITLE", "CHANNEL");
            println!("{}", "-".repeat(80));

            for video in videos {
                let title = if video.title.len() > 48 {
                    format!("{}...", &video.title[..45])
                } else {
                    video.title.clone()
                };
                let channel = video.channel.unwrap_or_default();
                println!("{:<12} {:<50} {}", video.id, title, channel);
            }
        }
        None => {
            let collections = db.list_collections()?;

            if collections.is_empty() {
                println!("No collections yet. Use 'new-collection' to create one.");
                return Ok(());
            }

            println!("Collections:\n");
            for c in collections {
                if let Some(desc) = c.description {
                    println!("  {} - {}", c.name, desc);
                } else {
                    println!("  {}", c.name);
                }
            }
        }
    }

    Ok(())
}

fn cmd_new_collection(db: &Database, name: &str, description: Option<&str>) -> Result<()> {
    let collection = db.create_collection(name, description)?;
    println!("Created collection: {}", collection.name);
    Ok(())
}

fn cmd_note(db: &Database, video_id: &str, text: &str, timestamp: Option<f64>) -> Result<()> {
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    let note = db.add_note(video_id, timestamp, text)?;

    if let Some(ts) = note.timestamp {
        let mins = (ts / 60.0) as u32;
        let secs = (ts % 60.0) as u32;
        println!("Note added at [{:02}:{:02}]", mins, secs);
    } else {
        println!("Note added");
    }

    Ok(())
}

fn cmd_notes(db: &Database, video_id: &str) -> Result<()> {
    let video = match db.get_video(video_id)? {
        Some(v) => v,
        None => {
            println!("Video not found: {}", video_id);
            return Ok(());
        }
    };

    let notes = db.get_video_notes(video_id)?;

    if notes.is_empty() {
        println!("No notes for: {}", video.title);
        return Ok(());
    }

    println!("Notes for: {}\n", video.title);

    for note in notes {
        if let Some(ts) = note.timestamp {
            let mins = (ts / 60.0) as u32;
            let secs = (ts % 60.0) as u32;
            let url = format!("{}&t={}s", video.url, ts as u32);
            println!("[{:02}:{:02}] {}", mins, secs, note.text);
            println!("        {}", url);
        } else {
            println!("[general] {}", note.text);
        }
        println!();
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_locate(
    db: &Database,
    video_id: &str,
    place: &str,
    lat: f64,
    lon: f64,
    era: Option<&str>,
    topic: Option<&str>,
    timestamp: Option<f64>,
    note: Option<&str>,
) -> Result<()> {
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    let location = db.get_or_create_location(place, lat, lon)?;

    let era_id = if let Some(era_name) = era {
        match db.get_era_by_name(era_name)? {
            Some(e) => Some(e.id),
            None => {
                println!("Era not found: {}", era_name);
                return Ok(());
            }
        }
    } else {
        None
    };

    let topic_id = if let Some(topic_name) = topic {
        Some(db.get_or_create_topic(topic_name)?.id)
    } else {
        None
    };

    db.add_video_location(video_id, location.id, era_id, topic_id, timestamp, note)?;

    println!("Added location: {} ({}, {})", location.name, location.lat, location.lon);
    if let Some(e) = era {
        println!("  Era: {}", e);
    }
    if let Some(t) = topic {
        println!("  Topic: {}", t);
    }

    Ok(())
}

fn cmd_locations(db: &Database) -> Result<()> {
    let locations = db.list_locations()?;

    if locations.is_empty() {
        println!("No locations defined yet. Use 'locate' to add one.");
        return Ok(());
    }

    println!("Locations:\n");
    println!("{:<20} {:>10} {:>10}", "NAME", "LAT", "LON");
    println!("{}", "-".repeat(42));

    for loc in locations {
        println!("{:<20} {:>10.4} {:>10.4}", loc.name, loc.lat, loc.lon);
    }

    Ok(())
}

fn cmd_serve(db_path: PathBuf, port: u16) -> Result<()> {
    use axum::{
        extract::{Path, Query, State},
        http::StatusCode,
        response::Json,
        routing::get,
        Router,
    };
    use std::sync::Arc;
    use tower_http::cors::CorsLayer;

    #[derive(Clone)]
    struct AppState {
        db_path: PathBuf,
    }

    fn open_db(state: &AppState) -> Result<Database, StatusCode> {
        Database::open(&state.db_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[derive(serde::Deserialize)]
    struct MapQuery {
        era: Option<String>,  // Comma-separated eras
        topic: Option<String>,
    }

    fn parse_eras(era: &Option<String>) -> Vec<String> {
        era.as_ref()
            .map(|s| s.split(',').map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect())
            .unwrap_or_default()
    }

    #[derive(serde::Deserialize)]
    struct ClaimsQuery {
        video_id: Option<String>,
        category: Option<String>,
        limit: Option<usize>,
    }

    #[derive(serde::Deserialize)]
    struct GraphQuery {
        video_id: Option<String>,
        moc_id: Option<i64>,
        era: Option<String>,  // Comma-separated eras
        topic: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct SearchQuery {
        q: String,                      // Search query
        types: Option<String>,          // Comma-separated: "claim,video,scholar"
        video_id: Option<String>,       // Filter to specific video
        limit: Option<usize>,           // Max results (default 50)
        fuzzy_threshold: Option<f64>,   // 0.0-1.0, default 0.6
    }

    // Graph node/edge structures for vis.js
    #[derive(serde::Serialize)]
    struct GraphNode {
        id: i64,
        label: String,
        title: String,      // Hover text
        group: String,      // Category for coloring
        value: usize,       // Node size (connection count)
        video_id: String,
        timestamp: Option<f64>,
    }

    #[derive(serde::Serialize)]
    struct GraphEdge {
        from: i64,
        to: i64,
        label: String,
        arrows: String,
        dashes: bool,       // Dashed for contradicts
        color: EdgeColor,
    }

    #[derive(serde::Serialize)]
    struct EdgeColor {
        color: String,
    }

    #[derive(serde::Serialize)]
    struct GraphData {
        nodes: Vec<GraphNode>,
        edges: Vec<GraphEdge>,
    }

    #[derive(serde::Serialize)]
    struct MocSummary {
        id: i64,
        title: String,
        description: Option<String>,
        claim_count: usize,
    }

    #[derive(serde::Serialize)]
    struct QuestionSummary {
        id: i64,
        question: String,
        status: String,
        evidence_count: usize,
    }

    #[derive(serde::Serialize)]
    struct QueueSummary {
        pending: usize,
        in_progress: usize,
        completed: usize,
        failed: usize,
        current: Option<String>,
    }

    #[derive(serde::Serialize)]
    struct FullStats {
        videos: i64,
        claims: i64,
        links: i64,
        mocs: i64,
        questions: i64,
        active_questions: i64,
        patterns: i64,
        orphan_claims: usize,
        stale_claims: usize,
        framework: engine::FrameworkStats,
        claims_by_category: Vec<CategoryCount>,
        // Phase 12: Expanded knowledge entities
        sources: i64,
        scholars: i64,
        terms: i64,
        visuals: i64,
        evidence: i64,
        quotes: i64,
    }

    #[derive(serde::Serialize)]
    struct CategoryCount {
        category: String,
        count: i64,
    }

    #[derive(serde::Serialize)]
    struct VideoSummary {
        id: String,
        title: String,
    }

    async fn get_pins(
        State(state): State<Arc<AppState>>,
        Query(q): Query<MapQuery>,
    ) -> Result<Json<Vec<engine::MapPin>>, StatusCode> {
        let db = open_db(&state)?;
        let eras = parse_eras(&q.era);
        let pins = if eras.is_empty() {
            // No era filter - show all pins
            db.get_map_pins(None, q.topic.as_deref())
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else {
            // Multiple eras - union pins from each era
            let mut all_pins = Vec::new();
            let mut seen_ids = std::collections::HashSet::new();
            for era in &eras {
                let era_pins = db.get_map_pins(Some(era), q.topic.as_deref())
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                for pin in era_pins {
                    let key = (pin.location.id, pin.video_id.clone());
                    if seen_ids.insert(key) {
                        all_pins.push(pin);
                    }
                }
            }
            all_pins
        };
        Ok(Json(pins))
    }

    async fn get_eras(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Era>>, StatusCode> {
        let db = open_db(&state)?;
        let eras = db.list_eras().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(eras))
    }

    async fn get_topics(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Topic>>, StatusCode> {
        let db = open_db(&state)?;
        let topics = db.list_topics().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(topics))
    }

    async fn get_videos(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<VideoSummary>>, StatusCode> {
        let db = open_db(&state)?;
        let videos = db.list_videos().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(videos.into_iter().map(|v| VideoSummary {
            id: v.id,
            title: v.title,
        }).collect()))
    }

    async fn get_claims(
        State(state): State<Arc<AppState>>,
        Query(q): Query<ClaimsQuery>,
    ) -> Result<Json<Vec<engine::Claim>>, StatusCode> {
        let db = open_db(&state)?;
        let claims = if let Some(video_id) = q.video_id {
            db.list_claims_for_video(&video_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else {
            // Get all claims (limited)
            db.get_random_claims(q.limit.unwrap_or(100)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        };
        Ok(Json(claims))
    }

    async fn get_claim(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>,
    ) -> Result<Json<engine::ClaimWithLinks>, StatusCode> {
        let db = open_db(&state)?;
        let claim = db.get_claim_with_links(id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        Ok(Json(claim))
    }

    async fn get_graph(
        State(state): State<Arc<AppState>>,
        Query(q): Query<GraphQuery>,
    ) -> Result<Json<GraphData>, StatusCode> {
        let db = open_db(&state)?;

        // Get claims based on filter
        let claims: Vec<engine::Claim> = if let Some(video_id) = q.video_id {
            db.list_claims_for_video(&video_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else if let Some(moc_id) = q.moc_id {
            let moc = db.get_moc_with_claims(moc_id)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?;
            moc.claims
        } else if q.era.is_some() {
            // Filter by era(s): get videos with these eras, then get their claims
            let eras = parse_eras(&q.era);
            let mut era_claims = Vec::new();
            let mut seen_videos = std::collections::HashSet::new();
            for era in &eras {
                let videos = db.browse_videos(Some(era), None).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                for video in videos {
                    if seen_videos.insert(video.id.clone()) {
                        era_claims.extend(db.list_claims_for_video(&video.id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?);
                    }
                }
            }
            era_claims
        } else if let Some(ref topic) = q.topic {
            // Filter by topic: get videos with this topic, then get their claims
            let videos = db.browse_by_topic(topic).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut topic_claims = Vec::new();
            for video in videos {
                topic_claims.extend(db.list_claims_for_video(&video.id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?);
            }
            topic_claims
        } else {
            // Default: get all claims (limited to 500 for performance)
            db.get_all_claims_limited(500).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let claim_ids: std::collections::HashSet<i64> = claims.iter().map(|c| c.id).collect();

        // Build nodes
        let mut nodes = Vec::new();
        for claim in &claims {
            let link_count = db.get_claim_link_count(claim.id).unwrap_or(0);
            let label = if claim.text.len() > 40 {
                format!("{}...", &claim.text[..37])
            } else {
                claim.text.clone()
            };
            nodes.push(GraphNode {
                id: claim.id,
                label,
                title: claim.text.clone(),
                group: claim.category.as_str().to_string(),
                value: (link_count + 1) as usize,
                video_id: claim.video_id.clone(),
                timestamp: claim.timestamp,
            });
        }

        // Build edges
        let mut edges = Vec::new();
        for claim in &claims {
            if let Ok(claim_with_links) = db.get_claim_with_links(claim.id) {
                if let Some(cwl) = claim_with_links {
                    for (link, _target) in &cwl.outgoing_links {
                        // Only include edges where both nodes are in our set
                        if claim_ids.contains(&link.target_claim_id) {
                            let (color, dashes) = match link.link_type {
                                engine::LinkType::Supports => ("#4CAF50", false),
                                engine::LinkType::Contradicts => ("#f44336", true),
                                engine::LinkType::Elaborates => ("#2196F3", false),
                                engine::LinkType::Causes => ("#FF9800", false),
                                engine::LinkType::CausedBy => ("#FF9800", false),
                                engine::LinkType::Related => ("#9E9E9E", true),
                            };
                            edges.push(GraphEdge {
                                from: link.source_claim_id,
                                to: link.target_claim_id,
                                label: link.link_type.as_str().to_string(),
                                arrows: "to".to_string(),
                                dashes,
                                color: EdgeColor { color: color.to_string() },
                            });
                        }
                    }
                }
            }
        }

        Ok(Json(GraphData { nodes, edges }))
    }

    async fn get_mocs(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<MocSummary>>, StatusCode> {
        let db = open_db(&state)?;
        let mocs = db.list_mocs().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut summaries = Vec::new();
        for moc in mocs {
            let claim_count = db.get_moc_with_claims(moc.id)
                .map(|m| m.map(|x| x.claims.len()).unwrap_or(0))
                .unwrap_or(0);
            summaries.push(MocSummary {
                id: moc.id,
                title: moc.title,
                description: moc.description,
                claim_count,
            });
        }
        Ok(Json(summaries))
    }

    async fn get_moc(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>,
    ) -> Result<Json<engine::MocWithClaims>, StatusCode> {
        let db = open_db(&state)?;
        let moc = db.get_moc_with_claims(id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        Ok(Json(moc))
    }

    async fn get_questions(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<QuestionSummary>>, StatusCode> {
        let db = open_db(&state)?;
        let questions = db.list_research_questions(None)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut summaries = Vec::new();
        for q in questions {
            let evidence_count = db.get_question_with_evidence(q.id)
                .map(|qe| qe.map(|x| x.claims.len() + x.videos.len()).unwrap_or(0))
                .unwrap_or(0);
            summaries.push(QuestionSummary {
                id: q.id,
                question: q.question,
                status: q.status.as_str().to_string(),
                evidence_count,
            });
        }
        Ok(Json(summaries))
    }

    async fn get_question(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>,
    ) -> Result<Json<engine::QuestionWithEvidence>, StatusCode> {
        let db = open_db(&state)?;
        let question = db.get_question_with_evidence(id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        Ok(Json(question))
    }

    async fn get_stats(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<FullStats>, StatusCode> {
        let db = open_db(&state)?;
        let synthesis = db.get_synthesis_stats().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let framework = db.get_framework_stats().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Get counts
        let videos = db.list_videos().map(|v| v.len() as i64).unwrap_or(0);
        let claims = db.get_random_claims(10000).map(|c| c.len() as i64).unwrap_or(0);
        let orphans = db.get_orphan_claims().map(|o| o.len()).unwrap_or(0);
        let stale = db.get_stale_claims(30).map(|s| s.len()).unwrap_or(0);

        // Count claims by category (simplified)
        let claims_by_category = vec![
            CategoryCount { category: "factual".to_string(), count: 0 },
            CategoryCount { category: "causal".to_string(), count: 0 },
            CategoryCount { category: "cyclical".to_string(), count: 0 },
            CategoryCount { category: "memetic".to_string(), count: 0 },
            CategoryCount { category: "geopolitical".to_string(), count: 0 },
        ];

        // Phase 12: Get expanded entity counts
        let sources = db.get_sources().map(|s| s.len() as i64).unwrap_or(0);
        let scholars = db.get_scholars().map(|s| s.len() as i64).unwrap_or(0);
        let terms = db.get_terms().map(|t| t.len() as i64).unwrap_or(0);
        let visuals = db.get_all_visuals().map(|v| v.len() as i64).unwrap_or(0);
        let evidence = db.get_all_evidence().map(|e| e.len() as i64).unwrap_or(0);
        let quotes = db.get_all_quotes().map(|q| q.len() as i64).unwrap_or(0);

        Ok(Json(FullStats {
            videos,
            claims,
            links: 0, // Would need a query
            mocs: synthesis.mocs,
            questions: synthesis.research_questions,
            active_questions: synthesis.active_questions,
            patterns: synthesis.detected_patterns,
            orphan_claims: orphans,
            stale_claims: stale,
            framework,
            claims_by_category,
            sources,
            scholars,
            terms,
            visuals,
            evidence,
            quotes,
        }))
    }

    async fn get_review_orphans(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Claim>>, StatusCode> {
        let db = open_db(&state)?;
        let orphans = db.get_orphan_claims().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(orphans))
    }

    async fn get_review_stale(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Claim>>, StatusCode> {
        let db = open_db(&state)?;
        let stale = db.get_stale_claims(30).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(stale))
    }

    async fn get_queue(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<QueueSummary>, StatusCode> {
        let db = open_db(&state)?;
        let items = db.get_queue(true).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut pending = 0;
        let mut in_progress = 0;
        let mut completed = 0;
        let mut failed = 0;
        let mut current = None;

        for item in items {
            match item.status {
                engine::ProcessingStatus::Pending => pending += 1,
                engine::ProcessingStatus::InProgress => {
                    in_progress += 1;
                    current = Some(item.video_id.clone());
                }
                engine::ProcessingStatus::Completed => completed += 1,
                engine::ProcessingStatus::Failed => failed += 1,
                engine::ProcessingStatus::Skipped => {}
            }
        }

        Ok(Json(QueueSummary { pending, in_progress, completed, failed, current }))
    }

    // Phase 12: API endpoints for expanded knowledge entities

    async fn get_sources(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Source>>, StatusCode> {
        let db = open_db(&state)?;
        let sources = db.get_sources().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(sources))
    }

    async fn get_scholars(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Scholar>>, StatusCode> {
        let db = open_db(&state)?;
        let scholars = db.get_scholars().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(scholars))
    }

    async fn get_terms(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Term>>, StatusCode> {
        let db = open_db(&state)?;
        let terms = db.get_terms().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(terms))
    }

    async fn get_visuals(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Visual>>, StatusCode> {
        let db = open_db(&state)?;
        let visuals = db.get_all_visuals().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(visuals))
    }

    async fn get_evidence(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Evidence>>, StatusCode> {
        let db = open_db(&state)?;
        let evidence = db.get_all_evidence().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(evidence))
    }

    async fn get_quotes(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<engine::Quote>>, StatusCode> {
        let db = open_db(&state)?;
        let quotes = db.get_all_quotes().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(quotes))
    }

    async fn search(
        State(state): State<Arc<AppState>>,
        Query(q): Query<SearchQuery>,
    ) -> Result<Json<engine::SearchResponse>, StatusCode> {
        let db = open_db(&state)?;

        // Parse comma-separated types
        let types: Option<Vec<&str>> = q.types.as_ref()
            .map(|t| t.split(',').map(|s| s.trim()).collect());

        let results = db.unified_search(
            &q.q,
            types.as_deref(),
            q.video_id.as_deref(),
            q.limit.unwrap_or(50),
            q.fuzzy_threshold.unwrap_or(0.6),
        ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(results))
    }

    async fn get_index() -> axum::response::Html<&'static str> {
        axum::response::Html(include_str!("../static/index.html"))
    }

    let state = Arc::new(AppState { db_path });

    let app = Router::new()
        .route("/", get(get_index))
        .route("/api/pins", get(get_pins))
        .route("/api/eras", get(get_eras))
        .route("/api/topics", get(get_topics))
        .route("/api/videos", get(get_videos))
        .route("/api/claims", get(get_claims))
        .route("/api/claims/:id", get(get_claim))
        .route("/api/graph", get(get_graph))
        .route("/api/mocs", get(get_mocs))
        .route("/api/mocs/:id", get(get_moc))
        .route("/api/questions", get(get_questions))
        .route("/api/questions/:id", get(get_question))
        .route("/api/stats", get(get_stats))
        .route("/api/review/orphans", get(get_review_orphans))
        .route("/api/review/stale", get(get_review_stale))
        .route("/api/queue", get(get_queue))
        // Phase 12: Expanded knowledge entity endpoints
        .route("/api/sources", get(get_sources))
        .route("/api/scholars", get(get_scholars))
        .route("/api/terms", get(get_terms))
        .route("/api/visuals", get(get_visuals))
        .route("/api/evidence", get(get_evidence))
        .route("/api/quotes", get(get_quotes))
        // Unified search endpoint
        .route("/api/search", get(search))
        .layer(CorsLayer::permissive())
        .with_state(state);

    println!("Starting server at http://localhost:{}", port);
    println!("Open in your browser to view the knowledge base.");

    tokio::runtime::Runtime::new()?
        .block_on(async {
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
            axum::serve(listener, app).await
        })
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))
}

fn cmd_suggest_tags(db: &Database, video_id: &str) -> Result<()> {
    let video = match db.get_video(video_id)? {
        Some(v) => v,
        None => {
            println!("Video not found: {}", video_id);
            return Ok(());
        }
    };

    println!("Analyzing: {}\n", video.title);

    let tags = db.suggest_tags(video_id)?;

    if tags.eras.is_empty() && tags.regions.is_empty() && tags.topics.is_empty() {
        println!("No tags detected from title/description.");
        return Ok(());
    }

    if !tags.eras.is_empty() {
        println!("Suggested eras: {}", tags.eras.join(", "));
    }
    if !tags.regions.is_empty() {
        println!("Suggested regions: {}", tags.regions.join(", "));
    }
    if !tags.topics.is_empty() {
        println!("Suggested topics: {}", tags.topics.join(", "));
    }

    println!("\nUse 'auto-tag {}' to apply these tags.", video_id);

    Ok(())
}

fn cmd_auto_tag(db: &Database, id: &str) -> Result<()> {
    if id == "all" {
        let videos = db.list_videos()?;
        let mut total_tags = 0;

        for video in &videos {
            let tags = db.apply_auto_tags(&video.id)?;
            let count = tags.eras.len() + tags.regions.len() + tags.topics.len();
            if count > 0 {
                println!("{}: {} tags applied", video.id, count);
                total_tags += count;
            }
        }

        println!("\nAuto-tagged {} videos, {} total tags applied.", videos.len(), total_tags);
    } else {
        let video = match db.get_video(id)? {
            Some(v) => v,
            None => {
                println!("Video not found: {}", id);
                return Ok(());
            }
        };

        println!("Auto-tagging: {}\n", video.title);

        let tags = db.apply_auto_tags(id)?;

        if tags.eras.is_empty() && tags.regions.is_empty() && tags.topics.is_empty() {
            println!("No tags detected from title/description.");
            return Ok(());
        }

        if !tags.eras.is_empty() {
            println!("Applied eras: {}", tags.eras.join(", "));
        }
        if !tags.regions.is_empty() {
            println!("Applied regions: {}", tags.regions.join(", "));
        }
        if !tags.topics.is_empty() {
            println!("Applied topics: {}", tags.topics.join(", "));
        }
    }

    Ok(())
}

fn cmd_rebuild_index(db: &Database) -> Result<()> {
    println!("Rebuilding search index...");
    let count = db.rebuild_search_index()?;
    println!("Indexed {} videos.", count);
    Ok(())
}

// Phase 5: Research Tools

fn cmd_save_search(
    db: &Database,
    name: &str,
    query: Option<&str>,
    era: Option<&str>,
    region: Option<&str>,
    topic: Option<&str>,
) -> Result<()> {
    if query.is_none() && era.is_none() && region.is_none() && topic.is_none() {
        println!("Please specify at least one of: --query, --era, --region, --topic");
        return Ok(());
    }

    let saved = db.save_search(name, query, era, region, topic)?;
    println!("Saved search '{}'", saved.name);

    let mut filters = Vec::new();
    if let Some(q) = &saved.query {
        filters.push(format!("query: \"{}\"", q));
    }
    if let Some(e) = &saved.era {
        filters.push(format!("era: {}", e));
    }
    if let Some(r) = &saved.region {
        filters.push(format!("region: {}", r));
    }
    if let Some(t) = &saved.topic {
        filters.push(format!("topic: {}", t));
    }
    println!("  {}", filters.join(", "));

    Ok(())
}

fn cmd_list_searches(db: &Database) -> Result<()> {
    let searches = db.list_saved_searches()?;

    if searches.is_empty() {
        println!("No saved searches.");
        return Ok(());
    }

    println!("Saved Searches:\n");
    println!("{:<20} {}", "NAME", "FILTERS");
    println!("{}", "-".repeat(60));

    for search in searches {
        let mut filters = Vec::new();
        if let Some(q) = &search.query {
            filters.push(format!("\"{}\"", q));
        }
        if let Some(e) = &search.era {
            filters.push(format!("era:{}", e));
        }
        if let Some(r) = &search.region {
            filters.push(format!("region:{}", r));
        }
        if let Some(t) = &search.topic {
            filters.push(format!("topic:{}", t));
        }
        println!("{:<20} {}", search.name, filters.join(" + "));
    }

    Ok(())
}

fn cmd_run_search(db: &Database, name: &str) -> Result<()> {
    let search = match db.get_saved_search(name)? {
        Some(s) => s,
        None => {
            println!("Saved search not found: {}", name);
            return Ok(());
        }
    };

    println!("Running saved search: {}\n", search.name);

    let results = db.advanced_search(
        search.query.as_deref(),
        search.era.as_deref(),
        search.region.as_deref(),
        search.topic.as_deref(),
    )?;

    if results.is_empty() {
        println!("No results found.");
        return Ok(());
    }

    let total_matches: usize = results.iter().map(|r| r.matches.len()).sum();
    println!("Found {} videos ({} transcript matches)\n", results.len(), total_matches);

    for result in results {
        println!("--- {} ---", result.video.id);
        println!("Title: {}", result.video.title);
        if let Some(ref channel) = result.video.channel {
            println!("Channel: {}", channel);
        }
        println!("URL: {}", result.video.url);

        if !result.eras.is_empty() {
            println!("Eras: {}", result.eras.join(", "));
        }
        if !result.regions.is_empty() {
            println!("Regions: {}", result.regions.join(", "));
        }
        if !result.topics.is_empty() {
            println!("Topics: {}", result.topics.join(", "));
        }
        println!();

        for m in &result.matches {
            let mins = (m.start_time / 60.0) as u32;
            let secs = (m.start_time % 60.0) as u32;
            let url_with_time = format!("{}&t={}s", result.video.url, m.start_time as u32);
            println!("  [{:02}:{:02}] {}", mins, secs, m.text);
            println!("          {}", url_with_time);
            println!();
        }
    }

    Ok(())
}

fn cmd_delete_search(db: &Database, name: &str) -> Result<()> {
    if db.delete_saved_search(name)? {
        println!("Deleted saved search: {}", name);
    } else {
        println!("Saved search not found: {}", name);
    }
    Ok(())
}

fn cmd_export(db: &Database, collection: &str, output: Option<&str>) -> Result<()> {
    match db.export_collection_markdown(collection)? {
        Some(markdown) => {
            if let Some(path) = output {
                std::fs::write(path, &markdown)?;
                println!("Exported collection '{}' to {}", collection, path);
            } else {
                println!("{}", markdown);
            }
        }
        None => {
            println!("Collection not found: {}", collection);
        }
    }
    Ok(())
}

fn cmd_export_map(
    db: &Database,
    era: Option<&str>,
    topic: Option<&str>,
    output: Option<&str>,
) -> Result<()> {
    let geojson = db.export_map_geojson(era, topic)?;

    if geojson.features.is_empty() {
        let mut filter_desc = Vec::new();
        if let Some(e) = era { filter_desc.push(format!("era '{}'", e)); }
        if let Some(t) = topic { filter_desc.push(format!("topic '{}'", t)); }
        if filter_desc.is_empty() {
            println!("No locations in database.");
        } else {
            println!("No locations found for: {}", filter_desc.join(", "));
        }
        return Ok(());
    }

    let json = serde_json::to_string_pretty(&geojson)?;

    if let Some(path) = output {
        std::fs::write(path, &json)?;
        println!("Exported {} locations to {}", geojson.features.len(), path);
    } else {
        println!("{}", json);
    }

    Ok(())
}

fn cmd_report(db: &Database, by: &str) -> Result<()> {
    match by.to_lowercase().as_str() {
        "era" | "eras" => {
            let entries = db.report_by_era()?;
            println!("Videos by Era:\n");
            println!("{:<25} {:>10}", "ERA", "VIDEOS");
            println!("{}", "-".repeat(37));
            let mut total = 0;
            for entry in entries {
                println!("{:<25} {:>10}", entry.name, entry.video_count);
                total += entry.video_count;
            }
            println!("{}", "-".repeat(37));
            println!("{:<25} {:>10}", "TOTAL (with era tag)", total);
        }
        "region" | "regions" => {
            let entries = db.report_by_region()?;
            println!("Videos by Region:\n");
            println!("{:<25} {:>10}", "REGION", "VIDEOS");
            println!("{}", "-".repeat(37));
            let mut total = 0;
            for entry in entries {
                if entry.video_count > 0 {
                    println!("{:<25} {:>10}", entry.name, entry.video_count);
                    total += entry.video_count;
                }
            }
            println!("{}", "-".repeat(37));
            println!("{:<25} {:>10}", "TOTAL (with region tag)", total);
        }
        "topic" | "topics" => {
            let entries = db.report_by_topic()?;
            println!("Videos by Topic:\n");
            println!("{:<25} {:>10}", "TOPIC", "VIDEOS");
            println!("{}", "-".repeat(37));
            let mut total = 0;
            for entry in entries {
                if entry.video_count > 0 {
                    println!("{:<25} {:>10}", entry.name, entry.video_count);
                    total += entry.video_count;
                }
            }
            println!("{}", "-".repeat(37));
            println!("{:<25} {:>10}", "TOTAL (with topic tag)", total);
        }
        _ => {
            println!("Unknown report type: {}", by);
            println!("Valid options: era, region, topic");
        }
    }
    Ok(())
}

fn cmd_stats(db: &Database) -> Result<()> {
    let (videos, transcripts, locations, notes, collections, searches, claims, chunks, embeddings) = db.get_summary_stats()?;

    println!("Database Statistics:\n");
    println!("{:<20} {:>10}", "Videos", videos);
    println!("{:<20} {:>10}", "Transcripts", transcripts);
    println!("{:<20} {:>10}", "Chunks", chunks);
    println!("{:<20} {:>10}", "Claims", claims);
    println!("{:<20} {:>10}", "Embeddings", embeddings);
    println!("{:<20} {:>10}", "Locations", locations);
    println!("{:<20} {:>10}", "Notes", notes);
    println!("{:<20} {:>10}", "Collections", collections);
    println!("{:<20} {:>10}", "Saved Searches", searches);

    Ok(())
}

// Phase 6: Claim Extraction & Atomic Notes

fn cmd_add_claim(
    db: &Database,
    video_id: &str,
    text: &str,
    quote: &str,
    category: &str,
    confidence: &str,
    timestamp: Option<f64>,
) -> Result<()> {
    use engine::{ClaimCategory, Confidence};

    // Verify video exists
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    let cat = match ClaimCategory::from_str(category) {
        Some(c) => c,
        None => {
            println!("Invalid category: {}", category);
            println!("Valid options: cyclical, causal, memetic, geopolitical, factual, phenomenological, metaphysical");
            return Ok(());
        }
    };

    let conf = match Confidence::from_str(confidence) {
        Some(c) => c,
        None => {
            println!("Invalid confidence: {}", confidence);
            println!("Valid options: high, medium, low");
            return Ok(());
        }
    };

    let claim = db.create_claim(text, video_id, timestamp, quote, cat, conf)?;
    println!("Created claim #{}", claim.id);
    println!("  Text: {}", claim.text);
    println!("  Category: {}", claim.category.as_str());
    println!("  Confidence: {}", claim.confidence.as_str());

    let link_count = db.get_claim_link_count(claim.id)?;
    if link_count < 2 {
        println!("\nNote: This claim needs {} more connection(s) to meet the minimum of 2.", 2 - link_count);
        println!("Use 'link {} <other-claim-id> --as <type>' to connect claims.", claim.id);
    }

    Ok(())
}

fn cmd_claims(db: &Database, video_id: &str) -> Result<()> {
    let video = match db.get_video(video_id)? {
        Some(v) => v,
        None => {
            println!("Video not found: {}", video_id);
            return Ok(());
        }
    };

    let claims = db.list_claims_for_video(video_id)?;

    if claims.is_empty() {
        println!("No claims extracted from: {}", video.title);
        return Ok(());
    }

    println!("Claims from: {}\n", video.title);
    println!("{:<6} {:<12} {:<10} {}", "ID", "CATEGORY", "CONF", "TEXT");
    println!("{}", "-".repeat(80));

    for claim in claims {
        let text_preview = if claim.text.len() > 45 {
            format!("{}...", &claim.text[..42])
        } else {
            claim.text.clone()
        };
        let link_count = db.get_claim_link_count(claim.id)?;
        let link_indicator = if link_count < 2 { " !" } else { "" };
        println!(
            "{:<6} {:<12} {:<10} {}{}",
            claim.id,
            claim.category.as_str(),
            claim.confidence.as_str(),
            text_preview,
            link_indicator
        );
    }

    println!("\n! = needs more connections (< 2 links)");

    Ok(())
}

fn cmd_all_claims(db: &Database, category: Option<&str>) -> Result<()> {
    use engine::ClaimCategory;

    let claims = if let Some(cat_str) = category {
        match ClaimCategory::from_str(cat_str) {
            Some(cat) => db.list_claims_by_category(cat)?,
            None => {
                println!("Invalid category: {}", cat_str);
                println!("Valid options: cyclical, causal, memetic, geopolitical, factual, phenomenological, metaphysical");
                return Ok(());
            }
        }
    } else {
        db.list_all_claims()?
    };

    if claims.is_empty() {
        if category.is_some() {
            println!("No claims found for category: {}", category.unwrap());
        } else {
            println!("No claims in database.");
        }
        return Ok(());
    }

    let header = if let Some(cat) = category {
        format!("Claims (category: {})", cat)
    } else {
        "All Claims".to_string()
    };

    println!("{}\n", header);
    println!("{:<6} {:<12} {:<12} {}", "ID", "VIDEO", "CATEGORY", "TEXT");
    println!("{}", "-".repeat(80));

    for claim in claims {
        let text_preview = if claim.text.len() > 40 {
            format!("{}...", &claim.text[..37])
        } else {
            claim.text.clone()
        };
        let video_id_short = if claim.video_id.len() > 10 {
            format!("{}...", &claim.video_id[..7])
        } else {
            claim.video_id.clone()
        };
        println!(
            "{:<6} {:<12} {:<12} {}",
            claim.id,
            video_id_short,
            claim.category.as_str(),
            text_preview
        );
    }

    Ok(())
}

fn cmd_claim(db: &Database, id: i64) -> Result<()> {
    let claim_with_links = match db.get_claim_with_links(id)? {
        Some(c) => c,
        None => {
            println!("Claim not found: {}", id);
            return Ok(());
        }
    };

    let claim = &claim_with_links.claim;

    println!("Claim #{}\n", claim.id);
    println!("Text: {}", claim.text);
    println!("Source Quote: \"{}\"", claim.source_quote);
    println!("Video: {}", claim.video_id);
    if let Some(ts) = claim.timestamp {
        let mins = (ts / 60.0) as u32;
        let secs = (ts % 60.0) as u32;
        println!("Timestamp: {:02}:{:02}", mins, secs);
    }
    println!("Category: {}", claim.category.as_str());
    println!("Confidence: {}", claim.confidence.as_str());
    println!("Created: {}", claim.created_at.format("%Y-%m-%d %H:%M"));

    let total_links = claim_with_links.outgoing_links.len() + claim_with_links.incoming_links.len();
    println!("\nConnections: {} total", total_links);

    if !claim_with_links.outgoing_links.is_empty() {
        println!("\nOutgoing links:");
        for (link, target) in &claim_with_links.outgoing_links {
            let text_preview = if target.text.len() > 50 {
                format!("{}...", &target.text[..47])
            } else {
                target.text.clone()
            };
            println!("  -> [{}] #{}: {}", link.link_type.as_str(), target.id, text_preview);
        }
    }

    if !claim_with_links.incoming_links.is_empty() {
        println!("\nIncoming links:");
        for (link, source) in &claim_with_links.incoming_links {
            let text_preview = if source.text.len() > 50 {
                format!("{}...", &source.text[..47])
            } else {
                source.text.clone()
            };
            println!("  <- [{}] #{}: {}", link.link_type.as_str(), source.id, text_preview);
        }
    }

    if total_links < 2 {
        println!("\nWarning: This claim needs {} more connection(s).", 2 - total_links);
    }

    Ok(())
}

fn cmd_link(db: &Database, source: i64, target: i64, link_type: &str) -> Result<()> {
    use engine::LinkType;

    // Verify both claims exist
    if db.get_claim(source)?.is_none() {
        println!("Source claim not found: {}", source);
        return Ok(());
    }
    if db.get_claim(target)?.is_none() {
        println!("Target claim not found: {}", target);
        return Ok(());
    }

    if source == target {
        println!("Cannot link a claim to itself.");
        return Ok(());
    }

    let lt = match LinkType::from_str(link_type) {
        Some(t) => t,
        None => {
            println!("Invalid link type: {}", link_type);
            println!("Valid options: supports, contradicts, elaborates, caused_by, causes, related");
            return Ok(());
        }
    };

    db.create_claim_link(source, target, lt)?;
    println!("Linked claim #{} -> #{} ({})", source, target, lt.as_str());

    Ok(())
}

fn cmd_unlink(db: &Database, source: i64, target: i64) -> Result<()> {
    if db.delete_claim_link(source, target)? {
        println!("Removed link: #{} -> #{}", source, target);
    } else {
        println!("Link not found: #{} -> #{}", source, target);
    }
    Ok(())
}

fn cmd_unlinked(db: &Database) -> Result<()> {
    let claims = db.get_unlinked_claims()?;

    if claims.is_empty() {
        println!("All claims have at least 2 connections.");
        return Ok(());
    }

    println!("Claims needing more connections (< 2 links):\n");
    println!("{:<6} {:<12} {:<8} {}", "ID", "VIDEO", "LINKS", "TEXT");
    println!("{}", "-".repeat(70));

    for claim in claims {
        let link_count = db.get_claim_link_count(claim.id)?;
        let text_preview = if claim.text.len() > 35 {
            format!("{}...", &claim.text[..32])
        } else {
            claim.text.clone()
        };
        let video_short = if claim.video_id.len() > 10 {
            format!("{}...", &claim.video_id[..7])
        } else {
            claim.video_id.clone()
        };
        println!("{:<6} {:<12} {:<8} {}", claim.id, video_short, link_count, text_preview);
    }

    println!("\nUse 'link <source> <target> --as <type>' to connect claims.");

    Ok(())
}

fn cmd_delete_claim(db: &Database, id: i64) -> Result<()> {
    if db.delete_claim(id)? {
        println!("Deleted claim #{}", id);
    } else {
        println!("Claim not found: {}", id);
    }
    Ok(())
}

fn cmd_chunk(db: &Database, id: &str, target_tokens: i32, overlap_percent: i32) -> Result<()> {
    use engine::TranscriptChunk;

    let process_video = |video_id: &str| -> Result<usize> {
        let transcript = match db.get_transcript(video_id)? {
            Some(t) => t,
            None => {
                println!("  No transcript for: {}", video_id);
                return Ok(0);
            }
        };

        // Simple token estimation: ~4 chars per token (rough approximation)
        let chars_per_token: usize = 4;
        let target_chars = (target_tokens as usize) * chars_per_token;
        let overlap_chars = (target_chars * overlap_percent as usize) / 100;

        let mut chunks = Vec::new();
        let mut current_chunk_text = String::new();
        let mut current_chunk_start = 0.0_f64;
        let mut current_chunk_end = 0.0_f64;
        let mut chunk_index = 0;
        let mut overlap_text = String::new();

        for segment in &transcript.segments {
            // Add overlap from previous chunk if starting new chunk
            if current_chunk_text.is_empty() && !overlap_text.is_empty() {
                current_chunk_text = overlap_text.clone();
                current_chunk_start = segment.start_time;
            }

            if current_chunk_text.is_empty() {
                current_chunk_start = segment.start_time;
            }

            current_chunk_text.push_str(&segment.text);
            current_chunk_text.push(' ');
            current_chunk_end = segment.start_time + segment.duration;

            // Check if we've reached target size
            if current_chunk_text.len() >= target_chars {
                let token_count = (current_chunk_text.len() / chars_per_token) as i32;

                chunks.push(TranscriptChunk {
                    id: 0, // Will be set by database
                    video_id: video_id.to_string(),
                    chunk_index,
                    start_time: current_chunk_start,
                    end_time: current_chunk_end,
                    text: current_chunk_text.trim().to_string(),
                    token_count,
                    overlap_with_previous: chunk_index > 0,
                });

                // Save overlap for next chunk
                if current_chunk_text.len() > overlap_chars {
                    overlap_text = current_chunk_text[current_chunk_text.len() - overlap_chars..].to_string();
                } else {
                    overlap_text = current_chunk_text.clone();
                }

                current_chunk_text = String::new();
                chunk_index += 1;
            }
        }

        // Don't forget the last chunk
        if !current_chunk_text.is_empty() {
            let token_count = (current_chunk_text.len() / chars_per_token) as i32;
            chunks.push(TranscriptChunk {
                id: 0,
                video_id: video_id.to_string(),
                chunk_index,
                start_time: current_chunk_start,
                end_time: current_chunk_end,
                text: current_chunk_text.trim().to_string(),
                token_count,
                overlap_with_previous: chunk_index > 0,
            });
        }

        let chunk_count = chunks.len();
        db.save_transcript_chunks(video_id, &chunks)?;

        Ok(chunk_count)
    };

    if id == "all" {
        let videos = db.list_videos()?;
        let mut total_chunks = 0;

        println!("Chunking all videos (target: {} tokens, {}% overlap)...\n", target_tokens, overlap_percent);

        for video in &videos {
            match process_video(&video.id) {
                Ok(count) if count > 0 => {
                    println!("  {}: {} chunks", video.id, count);
                    total_chunks += count;
                }
                Ok(_) => {}
                Err(e) => println!("  {}: error - {}", video.id, e),
            }
        }

        println!("\nCreated {} chunks from {} videos.", total_chunks, videos.len());
    } else {
        let video = match db.get_video(id)? {
            Some(v) => v,
            None => {
                println!("Video not found: {}", id);
                return Ok(());
            }
        };

        println!("Chunking: {} (target: {} tokens, {}% overlap)", video.title, target_tokens, overlap_percent);

        let count = process_video(id)?;
        println!("Created {} chunks.", count);
    }

    Ok(())
}

fn cmd_chunks(db: &Database, video_id: &str) -> Result<()> {
    let video = match db.get_video(video_id)? {
        Some(v) => v,
        None => {
            println!("Video not found: {}", video_id);
            return Ok(());
        }
    };

    let chunks = db.get_transcript_chunks(video_id)?;

    if chunks.is_empty() {
        println!("No chunks for: {}", video.title);
        println!("Use 'chunk {}' to generate chunks.", video_id);
        return Ok(());
    }

    println!("Chunks for: {}\n", video.title);
    println!("{:<6} {:>8} {:>8} {:>8} {}", "INDEX", "START", "END", "TOKENS", "PREVIEW");
    println!("{}", "-".repeat(75));

    for chunk in chunks {
        let start_mins = (chunk.start_time / 60.0) as u32;
        let start_secs = (chunk.start_time % 60.0) as u32;
        let end_mins = (chunk.end_time / 60.0) as u32;
        let end_secs = (chunk.end_time % 60.0) as u32;

        let preview = if chunk.text.len() > 30 {
            format!("{}...", &chunk.text[..27])
        } else {
            chunk.text.clone()
        };

        println!(
            "{:<6} {:02}:{:02} {:02}:{:02} {:>8} {}",
            chunk.chunk_index,
            start_mins, start_secs,
            end_mins, end_secs,
            chunk.token_count,
            preview
        );
    }

    Ok(())
}

fn cmd_summarize(db: &Database, video_id: &str, layer: u8, content: Option<&str>) -> Result<()> {
    if layer < 2 || layer > 4 {
        println!("Layer must be 2, 3, or 4.");
        println!("  Layer 2: Key passages (bolded)");
        println!("  Layer 3: Best of best (highlighted)");
        println!("  Layer 4: Executive summary");
        return Ok(());
    }

    let video = match db.get_video(video_id)? {
        Some(v) => v,
        None => {
            println!("Video not found: {}", video_id);
            return Ok(());
        }
    };

    let content_str = match content {
        Some(c) => c.to_string(),
        None => {
            // Read from stdin
            println!("Enter layer {} content for '{}' (Ctrl+D to finish):", layer, video.title);
            let mut buffer = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut buffer)?;
            buffer
        }
    };

    if content_str.trim().is_empty() {
        println!("No content provided.");
        return Ok(());
    }

    db.save_transcript_layer(video_id, layer, content_str.trim())?;
    println!("Saved layer {} for: {}", layer, video.title);

    Ok(())
}

fn cmd_layers(db: &Database, video_id: &str) -> Result<()> {
    let video = match db.get_video(video_id)? {
        Some(v) => v,
        None => {
            println!("Video not found: {}", video_id);
            return Ok(());
        }
    };

    let layers = db.list_transcript_layers(video_id)?;

    println!("Summary Layers for: {}\n", video.title);

    // Layer 1 is always the raw transcript
    let has_transcript = db.get_transcript(video_id)?.is_some();
    println!("Layer 1 (Raw Transcript): {}", if has_transcript { "Yes" } else { "No" });

    for layer_num in 2..=4 {
        let layer_name = match layer_num {
            2 => "Key Passages",
            3 => "Best of Best",
            4 => "Executive Summary",
            _ => "Unknown",
        };

        if let Some(layer) = layers.iter().find(|l| l.layer == layer_num) {
            let preview = if layer.content.len() > 60 {
                format!("{}...", &layer.content[..57])
            } else {
                layer.content.clone()
            };
            println!("Layer {} ({}): {} chars", layer_num, layer_name, layer.content.len());
            println!("  Preview: {}", preview.replace('\n', " "));
        } else {
            println!("Layer {} ({}): Not created", layer_num, layer_name);
        }
    }

    Ok(())
}

fn cmd_claim_stats(db: &Database) -> Result<()> {
    let (total, linked, links) = db.get_claim_stats()?;
    let unlinked = total - linked;

    println!("Claim Statistics:\n");
    println!("{:<25} {:>10}", "Total Claims", total);
    println!("{:<25} {:>10}", "Well-Connected (≥2 links)", linked);
    println!("{:<25} {:>10}", "Need Connections (<2)", unlinked);
    println!("{:<25} {:>10}", "Total Links", links);

    if total > 0 {
        let connection_rate = (linked as f64 / total as f64) * 100.0;
        println!("\nConnection Rate: {:.1}%", connection_rate);
    }

    // Category breakdown
    println!("\nBy Category:");
    use engine::ClaimCategory;
    for cat in &[
        ClaimCategory::Factual,
        ClaimCategory::CyclicalPattern,
        ClaimCategory::CausalClaim,
        ClaimCategory::MemeticTransmission,
        ClaimCategory::GeopoliticalDynamic,
    ] {
        let count = db.list_claims_by_category(*cat)?.len();
        if count > 0 {
            println!("  {:<20} {:>6}", cat.as_str(), count);
        }
    }

    Ok(())
}

// Phase 7: Semantic Search & Embeddings

fn cmd_embed(db: &Database, source: &str, id: &str, vector: &str, model: &str) -> Result<()> {
    use engine::EmbeddingSource;

    let source_type = match EmbeddingSource::from_str(source) {
        Some(s) => s,
        None => {
            println!("Invalid source type: {}", source);
            println!("Valid options: video, chunk, claim");
            return Ok(());
        }
    };

    // Parse vector from JSON
    let vec: Vec<f32> = match serde_json::from_str(vector) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid vector JSON: {}", e);
            println!("Expected format: [0.1, 0.2, 0.3, ...]");
            return Ok(());
        }
    };

    if vec.is_empty() {
        println!("Vector cannot be empty.");
        return Ok(());
    }

    db.save_embedding(source_type, id, model, &vec)?;
    println!("Saved embedding for {} '{}' ({} dimensions, model: {})", source, id, vec.len(), model);

    Ok(())
}

fn cmd_import_embeddings(db: &Database, file: &str, model: &str) -> Result<()> {
    use engine::EmbeddingSource;

    let content = std::fs::read_to_string(file)?;

    // Expected format: array of objects with source_type, source_id, vector
    #[derive(serde::Deserialize)]
    struct EmbeddingInput {
        source_type: String,
        source_id: String,
        vector: Vec<f32>,
    }

    let inputs: Vec<EmbeddingInput> = serde_json::from_str(&content)?;

    let mut count = 0;
    for input in inputs {
        if let Some(source_type) = EmbeddingSource::from_str(&input.source_type) {
            db.save_embedding(source_type, &input.source_id, model, &input.vector)?;
            count += 1;
        } else {
            println!("Skipping invalid source_type: {}", input.source_type);
        }
    }

    println!("Imported {} embeddings from {} (model: {})", count, file, model);

    Ok(())
}

fn cmd_export_for_embedding(db: &Database, output: Option<&str>, source: &str) -> Result<()> {
    #[derive(serde::Serialize)]
    struct ExportItem {
        source_type: String,
        source_id: String,
        text: String,
    }

    let mut items = Vec::new();

    let export_videos = source == "all" || source == "video";
    let export_chunks = source == "all" || source == "chunk";
    let export_claims = source == "all" || source == "claim";

    // Export videos
    if export_videos {
        let videos = db.list_videos()?;
        for video in videos {
            if !db.has_embedding(engine::EmbeddingSource::Video, &video.id)? {
                let text = format!(
                    "{}\n{}",
                    video.title,
                    video.description.unwrap_or_default()
                );
                items.push(ExportItem {
                    source_type: "video".to_string(),
                    source_id: video.id,
                    text,
                });
            }
        }
    }

    // Export chunks
    if export_chunks {
        let videos = db.list_videos()?;
        for video in videos {
            let chunks = db.get_transcript_chunks(&video.id)?;
            for chunk in chunks {
                let source_id = format!("{}:{}", video.id, chunk.chunk_index);
                if !db.has_embedding(engine::EmbeddingSource::Chunk, &source_id)? {
                    items.push(ExportItem {
                        source_type: "chunk".to_string(),
                        source_id,
                        text: chunk.text,
                    });
                }
            }
        }
    }

    // Export claims
    if export_claims {
        let claims = db.list_all_claims()?;
        for claim in claims {
            let source_id = claim.id.to_string();
            if !db.has_embedding(engine::EmbeddingSource::Claim, &source_id)? {
                items.push(ExportItem {
                    source_type: "claim".to_string(),
                    source_id,
                    text: claim.text,
                });
            }
        }
    }

    let json = serde_json::to_string_pretty(&items)?;

    if let Some(path) = output {
        std::fs::write(path, &json)?;
        println!("Exported {} items to {} for embedding", items.len(), path);
    } else {
        println!("{}", json);
    }

    Ok(())
}

fn cmd_semantic(db: &Database, vector: &str, source: Option<&str>, limit: usize) -> Result<()> {
    use engine::EmbeddingSource;

    let query_vec: Vec<f32> = match serde_json::from_str(vector) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid vector JSON: {}", e);
            return Ok(());
        }
    };

    let source_type = source.and_then(EmbeddingSource::from_str);

    let similar = db.find_similar(&query_vec, source_type, limit)?;

    if similar.is_empty() {
        println!("No results found. Make sure embeddings exist in the database.");
        return Ok(());
    }

    let results = db.build_similarity_results(similar)?;

    println!("Semantic Search Results:\n");
    println!("{:<8} {:<10} {:<15} {}", "SCORE", "TYPE", "ID", "TEXT");
    println!("{}", "-".repeat(80));

    for result in results {
        let text_preview = if result.text.len() > 40 {
            format!("{}...", &result.text[..37].replace('\n', " "))
        } else {
            result.text.replace('\n', " ")
        };
        let id_short = if result.source_id.len() > 13 {
            format!("{}...", &result.source_id[..10])
        } else {
            result.source_id.clone()
        };
        println!(
            "{:<8.4} {:<10} {:<15} {}",
            result.score,
            result.source_type.as_str(),
            id_short,
            text_preview
        );
    }

    Ok(())
}

fn cmd_hybrid(
    db: &Database,
    query: &str,
    vector: Option<&str>,
    kw_weight: f32,
    sem_weight: f32,
    limit: usize,
) -> Result<()> {
    let query_vec: Option<Vec<f32>> = if let Some(v) = vector {
        match serde_json::from_str(v) {
            Ok(vec) => Some(vec),
            Err(e) => {
                println!("Invalid vector JSON: {}", e);
                return Ok(());
            }
        }
    } else {
        None
    };

    let results = db.hybrid_search(
        query,
        query_vec.as_deref(),
        kw_weight,
        sem_weight,
        limit,
    )?;

    if results.is_empty() {
        println!("No results found for: {}", query);
        return Ok(());
    }

    println!("Hybrid Search Results (kw:{:.1}, sem:{:.1}):\n", kw_weight, sem_weight);
    println!("{:<8} {:<6} {:<6} {:<12} {}", "SCORE", "KW", "SEM", "ID", "TITLE");
    println!("{}", "-".repeat(80));

    for result in results {
        let title_preview = if result.video.title.len() > 35 {
            format!("{}...", &result.video.title[..32])
        } else {
            result.video.title.clone()
        };
        println!(
            "{:<8.3} {:<6.3} {:<6.3} {:<12} {}",
            result.combined_score,
            result.keyword_score,
            result.semantic_score,
            result.video.id,
            title_preview
        );

        if !result.matching_chunks.is_empty() {
            println!("  Matching chunks: {}", result.matching_chunks.len());
        }
        if !result.matching_claims.is_empty() {
            println!("  Matching claims: {}", result.matching_claims.len());
        }
    }

    Ok(())
}

fn cmd_similar(db: &Database, source: &str, id: &str, limit: usize) -> Result<()> {
    use engine::EmbeddingSource;

    let source_type = match EmbeddingSource::from_str(source) {
        Some(s) => s,
        None => {
            println!("Invalid source type: {}", source);
            println!("Valid options: video, chunk, claim");
            return Ok(());
        }
    };

    // Get the embedding for the source item
    let embedding = match db.get_embedding(source_type, id, "default")? {
        Some(e) => e,
        None => {
            println!("No embedding found for {} '{}'", source, id);
            println!("Use 'embed' or 'import-embeddings' to add embeddings first.");
            return Ok(());
        }
    };

    // Find similar items
    let similar = db.find_similar(&embedding.vector, None, limit + 1)?;

    // Filter out the source item itself
    let similar: Vec<_> = similar
        .into_iter()
        .filter(|(e, _)| !(e.source_type == source_type && e.source_id == id))
        .take(limit)
        .collect();

    if similar.is_empty() {
        println!("No similar items found.");
        return Ok(());
    }

    let results = db.build_similarity_results(similar)?;

    // Get the source text for context
    let source_text = db.get_text_for_embedding(&embedding)?.unwrap_or_default();
    let source_preview = if source_text.len() > 60 {
        format!("{}...", &source_text[..57].replace('\n', " "))
    } else {
        source_text.replace('\n', " ")
    };

    println!("Similar to {} '{}': {}\n", source, id, source_preview);
    println!("{:<8} {:<10} {:<15} {}", "SCORE", "TYPE", "ID", "TEXT");
    println!("{}", "-".repeat(80));

    for result in results {
        let text_preview = if result.text.len() > 40 {
            format!("{}...", &result.text[..37].replace('\n', " "))
        } else {
            result.text.replace('\n', " ")
        };
        let id_short = if result.source_id.len() > 13 {
            format!("{}...", &result.source_id[..10])
        } else {
            result.source_id.clone()
        };
        println!(
            "{:<8.4} {:<10} {:<15} {}",
            result.score,
            result.source_type.as_str(),
            id_short,
            text_preview
        );
    }

    Ok(())
}

fn cmd_embed_stats(db: &Database) -> Result<()> {
    let stats = db.get_embedding_stats()?;

    println!("Embedding Statistics:\n");
    println!("{:<25} {:>10}", "Total Embeddings", stats.total_embeddings);
    println!("{:<25} {:>10}", "Video Embeddings", stats.video_embeddings);
    println!("{:<25} {:>10}", "Chunk Embeddings", stats.chunk_embeddings);
    println!("{:<25} {:>10}", "Claim Embeddings", stats.claim_embeddings);
    println!("{:<25} {:>10}", "Summary Embeddings", stats.summary_embeddings);

    if let Some(model) = stats.model {
        println!("\nModel: {}", model);
    }
    if let Some(dims) = stats.dimensions {
        println!("Dimensions: {}", dims);
    }

    // Show what needs embeddings
    let (videos, chunks, claims) = db.get_items_needing_embeddings()?;
    let needs_count = videos.len() + chunks.len() + claims.len();

    if needs_count > 0 {
        println!("\nItems needing embeddings:");
        if !videos.is_empty() {
            println!("  Videos: {}", videos.len());
        }
        if !chunks.is_empty() {
            println!("  Chunks: {}", chunks.len());
        }
        if !claims.is_empty() {
            println!("  Claims: {}", claims.len());
        }
        println!("\nUse 'export-for-embedding' to export text for external embedding.");
    }

    Ok(())
}

// Phase 8: Analytical Frameworks

fn cmd_cyclical(
    db: &Database,
    video_id: &str,
    type_str: &str,
    entity: &str,
    description: &str,
    claim_id: Option<i64>,
    era_name: Option<&str>,
    timestamp: Option<f64>,
) -> Result<()> {
    use engine::CyclicalType;

    let indicator_type = match CyclicalType::from_str(type_str) {
        Some(t) => t,
        None => {
            println!("Invalid indicator type: {}", type_str);
            println!("Valid types: elite_overproduction, fiscal_strain, social_unrest, population_pressure, asabiyyah, center_periphery");
            return Ok(());
        }
    };

    // Verify video exists
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    // Get era ID if name provided
    let era_id = if let Some(name) = era_name {
        match db.get_era_by_name(name)? {
            Some(e) => Some(e.id),
            None => {
                println!("Era not found: {}", name);
                return Ok(());
            }
        }
    } else {
        None
    };

    let indicator = db.create_cyclical_indicator(
        video_id,
        claim_id,
        indicator_type,
        entity,
        era_id,
        description,
        timestamp,
    )?;

    println!("Created cyclical indicator #{}", indicator.id);
    println!("  Type: {}", indicator.indicator_type.as_str());
    println!("  Entity: {}", indicator.entity);
    println!("  Description: {}", indicator.description);

    Ok(())
}

fn cmd_list_cyclical(db: &Database, type_filter: Option<&str>, entity_filter: Option<&str>) -> Result<()> {
    use engine::CyclicalType;

    let indicators = if let Some(type_str) = type_filter {
        match CyclicalType::from_str(type_str) {
            Some(t) => db.list_cyclical_indicators_by_type(t)?,
            None => {
                println!("Invalid type: {}", type_str);
                return Ok(());
            }
        }
    } else if let Some(entity) = entity_filter {
        db.list_cyclical_indicators_by_entity(entity)?
    } else {
        db.list_all_cyclical_indicators()?
    };

    if indicators.is_empty() {
        println!("No cyclical indicators found.");
        return Ok(());
    }

    println!("{:<5} {:<20} {:<20} {}", "ID", "TYPE", "ENTITY", "DESCRIPTION");
    println!("{}", "-".repeat(80));

    for ind in indicators {
        let desc = if ind.description.len() > 30 {
            format!("{}...", &ind.description[..27])
        } else {
            ind.description.clone()
        };
        println!(
            "{:<5} {:<20} {:<20} {}",
            ind.id,
            ind.indicator_type.as_str(),
            ind.entity,
            desc
        );
    }

    Ok(())
}

fn cmd_delete_cyclical(db: &Database, id: i64) -> Result<()> {
    if db.delete_cyclical_indicator(id)? {
        println!("Deleted cyclical indicator #{}", id);
    } else {
        println!("Cyclical indicator #{} not found", id);
    }
    Ok(())
}

fn cmd_causal(
    db: &Database,
    cause_id: i64,
    effect_id: i64,
    loop_type_str: &str,
    strength_str: &str,
    notes: Option<&str>,
) -> Result<()> {
    use engine::{LoopType, RelationStrength};

    let loop_type = match LoopType::from_str(loop_type_str) {
        Some(l) => l,
        None => {
            println!("Invalid loop type: {}", loop_type_str);
            println!("Valid types: positive, negative, linear");
            return Ok(());
        }
    };

    let strength = match RelationStrength::from_str(strength_str) {
        Some(s) => s,
        None => {
            println!("Invalid strength: {}", strength_str);
            println!("Valid strengths: strong, moderate, weak, speculative");
            return Ok(());
        }
    };

    // Verify claims exist
    let cause_claim = match db.get_claim(cause_id)? {
        Some(c) => c,
        None => {
            println!("Cause claim #{} not found", cause_id);
            return Ok(());
        }
    };

    if db.get_claim(effect_id)?.is_none() {
        println!("Effect claim #{} not found", effect_id);
        return Ok(());
    }

    let relation = db.create_causal_relation(
        cause_id,
        effect_id,
        loop_type,
        strength,
        &cause_claim.video_id,
        notes,
    )?;

    println!("Created causal relation #{}", relation.id);
    println!("  {} -> {} ({})", cause_id, effect_id, relation.loop_type.as_str());
    println!("  Strength: {}", relation.strength.as_str());

    Ok(())
}

fn cmd_list_causal(db: &Database, loop_type_filter: Option<&str>, claim_filter: Option<i64>) -> Result<()> {
    use engine::LoopType;

    let relations = if let Some(loop_str) = loop_type_filter {
        match LoopType::from_str(loop_str) {
            Some(l) => db.list_causal_relations_by_type(l)?,
            None => {
                println!("Invalid loop type: {}", loop_str);
                return Ok(());
            }
        }
    } else if let Some(claim_id) = claim_filter {
        db.get_causal_relations_for_claim(claim_id)?
    } else {
        db.list_all_causal_relations()?
    };

    if relations.is_empty() {
        println!("No causal relations found.");
        return Ok(());
    }

    println!("{:<5} {:<8} {:<8} {:<10} {:<12} {}", "ID", "CAUSE", "EFFECT", "LOOP", "STRENGTH", "NOTES");
    println!("{}", "-".repeat(80));

    for rel in relations {
        let notes_preview = rel.notes.as_ref().map(|n| {
            if n.len() > 20 { format!("{}...", &n[..17]) } else { n.clone() }
        }).unwrap_or_default();
        println!(
            "{:<5} {:<8} {:<8} {:<10} {:<12} {}",
            rel.id,
            rel.cause_claim_id,
            rel.effect_claim_id,
            rel.loop_type.as_str(),
            rel.strength.as_str(),
            notes_preview
        );
    }

    Ok(())
}

fn cmd_delete_causal(db: &Database, id: i64) -> Result<()> {
    if db.delete_causal_relation(id)? {
        println!("Deleted causal relation #{}", id);
    } else {
        println!("Causal relation #{} not found", id);
    }
    Ok(())
}

fn cmd_transmission(
    db: &Database,
    idea: &str,
    source_entity: &str,
    target_entity: &str,
    type_str: &str,
    video_id: &str,
    era_name: Option<&str>,
    region_name: Option<&str>,
    claim_id: Option<i64>,
    notes: Option<&str>,
) -> Result<()> {
    use engine::TransmissionType;

    let transmission_type = match TransmissionType::from_str(type_str) {
        Some(t) => t,
        None => {
            println!("Invalid transmission type: {}", type_str);
            println!("Valid types: horizontal, vertical, oblique");
            return Ok(());
        }
    };

    // Verify video exists
    if db.get_video(video_id)?.is_none() {
        println!("Video not found: {}", video_id);
        return Ok(());
    }

    // Get era ID if provided
    let era_id = if let Some(name) = era_name {
        match db.get_era_by_name(name)? {
            Some(e) => Some(e.id),
            None => {
                println!("Era not found: {}", name);
                return Ok(());
            }
        }
    } else {
        None
    };

    // Get region ID if provided
    let region_id = if let Some(name) = region_name {
        match db.get_region_by_name(name)? {
            Some(r) => Some(r.id),
            None => {
                println!("Region not found: {}", name);
                return Ok(());
            }
        }
    } else {
        None
    };

    let transmission = db.create_idea_transmission(
        idea,
        source_entity,
        target_entity,
        transmission_type,
        era_id,
        region_id,
        video_id,
        claim_id,
        notes,
    )?;

    println!("Created idea transmission #{}", transmission.id);
    println!("  Idea: {}", transmission.idea);
    println!("  {} -> {} ({})", source_entity, target_entity, transmission_type.as_str());

    Ok(())
}

fn cmd_list_transmissions(db: &Database, idea_filter: Option<&str>, type_filter: Option<&str>) -> Result<()> {
    use engine::TransmissionType;

    let transmissions = if let Some(idea) = idea_filter {
        db.list_idea_transmissions_by_idea(idea)?
    } else if let Some(type_str) = type_filter {
        match TransmissionType::from_str(type_str) {
            Some(t) => db.list_idea_transmissions_by_type(t)?,
            None => {
                println!("Invalid type: {}", type_str);
                return Ok(());
            }
        }
    } else {
        db.list_all_idea_transmissions()?
    };

    if transmissions.is_empty() {
        println!("No idea transmissions found.");
        return Ok(());
    }

    println!("{:<5} {:<25} {:<15} {:<15} {}", "ID", "IDEA", "FROM", "TO", "TYPE");
    println!("{}", "-".repeat(80));

    for trans in transmissions {
        let idea_preview = if trans.idea.len() > 23 {
            format!("{}...", &trans.idea[..20])
        } else {
            trans.idea.clone()
        };
        let source_preview = if trans.source_entity.len() > 13 {
            format!("{}...", &trans.source_entity[..10])
        } else {
            trans.source_entity.clone()
        };
        let target_preview = if trans.target_entity.len() > 13 {
            format!("{}...", &trans.target_entity[..10])
        } else {
            trans.target_entity.clone()
        };
        println!(
            "{:<5} {:<25} {:<15} {:<15} {}",
            trans.id,
            idea_preview,
            source_preview,
            target_preview,
            trans.transmission_type.as_str()
        );
    }

    Ok(())
}

fn cmd_delete_transmission(db: &Database, id: i64) -> Result<()> {
    if db.delete_idea_transmission(id)? {
        println!("Deleted idea transmission #{}", id);
    } else {
        println!("Idea transmission #{} not found", id);
    }
    Ok(())
}

fn cmd_position(
    db: &Database,
    name: &str,
    era_name: &str,
    position_str: &str,
    notes: Option<&str>,
) -> Result<()> {
    use engine::SystemPosition;

    let position = match SystemPosition::from_str(position_str) {
        Some(p) => p,
        None => {
            println!("Invalid position: {}", position_str);
            println!("Valid positions: core, semi_periphery, periphery");
            return Ok(());
        }
    };

    // Get era
    let era = match db.get_era_by_name(era_name)? {
        Some(e) => e,
        None => {
            println!("Era not found: {}", era_name);
            return Ok(());
        }
    };

    // Check if entity already exists for this era
    if let Some(existing) = db.get_geopolitical_entity_by_name(name, era.id)? {
        println!("Entity '{}' already exists for era '{}' (ID: {})", name, era_name, existing.id);
        println!("Use 'update-position' to change its position.");
        return Ok(());
    }

    let entity = db.create_geopolitical_entity(name, era.id, position, notes)?;

    println!("Created geopolitical entity #{}", entity.id);
    println!("  Name: {}", entity.name);
    println!("  Era: {}", era_name);
    println!("  Position: {}", entity.position.as_str());

    Ok(())
}

fn cmd_list_positions(db: &Database, era_filter: Option<&str>, position_filter: Option<&str>) -> Result<()> {
    use engine::SystemPosition;

    let entities = if let Some(era_name) = era_filter {
        let era = match db.get_era_by_name(era_name)? {
            Some(e) => e,
            None => {
                println!("Era not found: {}", era_name);
                return Ok(());
            }
        };
        db.list_geopolitical_entities_by_era(era.id)?
    } else if let Some(pos_str) = position_filter {
        match SystemPosition::from_str(pos_str) {
            Some(p) => db.list_geopolitical_entities_by_position(p)?,
            None => {
                println!("Invalid position: {}", pos_str);
                return Ok(());
            }
        }
    } else {
        db.list_all_geopolitical_entities()?
    };

    if entities.is_empty() {
        println!("No geopolitical entities found.");
        return Ok(());
    }

    // Get era names for display
    println!("{:<5} {:<25} {:<20} {}", "ID", "NAME", "POSITION", "ERA");
    println!("{}", "-".repeat(70));

    for entity in entities {
        let era_name = db.get_era(entity.era_id)?
            .map(|e| e.name)
            .unwrap_or_else(|| "Unknown".to_string());
        println!(
            "{:<5} {:<25} {:<20} {}",
            entity.id,
            entity.name,
            entity.position.as_str(),
            era_name
        );
    }

    Ok(())
}

fn cmd_update_position(db: &Database, id: i64, position_str: &str) -> Result<()> {
    use engine::SystemPosition;

    let position = match SystemPosition::from_str(position_str) {
        Some(p) => p,
        None => {
            println!("Invalid position: {}", position_str);
            println!("Valid positions: core, semi_periphery, periphery");
            return Ok(());
        }
    };

    if db.update_geopolitical_entity_position(id, position)? {
        println!("Updated entity #{} to position: {}", id, position.as_str());
    } else {
        println!("Entity #{} not found", id);
    }

    Ok(())
}

fn cmd_flow(
    db: &Database,
    from_entity_id: i64,
    to_entity_id: i64,
    commodity: &str,
    era_name: &str,
    video_id: Option<&str>,
    claim_id: Option<i64>,
    notes: Option<&str>,
) -> Result<()> {
    // Get era
    let era = match db.get_era_by_name(era_name)? {
        Some(e) => e,
        None => {
            println!("Era not found: {}", era_name);
            return Ok(());
        }
    };

    // Verify entities exist
    let from_entity = match db.get_geopolitical_entity(from_entity_id)? {
        Some(e) => e,
        None => {
            println!("Source entity #{} not found", from_entity_id);
            return Ok(());
        }
    };

    let to_entity = match db.get_geopolitical_entity(to_entity_id)? {
        Some(e) => e,
        None => {
            println!("Target entity #{} not found", to_entity_id);
            return Ok(());
        }
    };

    let flow = db.create_surplus_flow(
        from_entity_id,
        to_entity_id,
        commodity,
        era.id,
        video_id,
        claim_id,
        notes,
    )?;

    println!("Created surplus flow #{}", flow.id);
    println!("  {} -> {}", from_entity.name, to_entity.name);
    println!("  Commodity: {}", commodity);

    Ok(())
}

fn cmd_list_flows(db: &Database, era_filter: Option<&str>, entity_filter: Option<i64>) -> Result<()> {
    let flows = if let Some(era_name) = era_filter {
        let era = match db.get_era_by_name(era_name)? {
            Some(e) => e,
            None => {
                println!("Era not found: {}", era_name);
                return Ok(());
            }
        };
        db.list_surplus_flows_by_era(era.id)?
    } else if let Some(entity_id) = entity_filter {
        db.list_surplus_flows_for_entity(entity_id)?
    } else {
        db.list_all_surplus_flows()?
    };

    if flows.is_empty() {
        println!("No surplus flows found.");
        return Ok(());
    }

    println!("{:<5} {:<20} {:<20} {}", "ID", "FROM", "TO", "COMMODITY");
    println!("{}", "-".repeat(70));

    for flow in flows {
        let from_name = db.get_geopolitical_entity(flow.from_entity_id)?
            .map(|e| e.name)
            .unwrap_or_else(|| format!("#{}", flow.from_entity_id));
        let to_name = db.get_geopolitical_entity(flow.to_entity_id)?
            .map(|e| e.name)
            .unwrap_or_else(|| format!("#{}", flow.to_entity_id));

        let from_preview = if from_name.len() > 18 {
            format!("{}...", &from_name[..15])
        } else {
            from_name
        };
        let to_preview = if to_name.len() > 18 {
            format!("{}...", &to_name[..15])
        } else {
            to_name
        };

        println!(
            "{:<5} {:<20} {:<20} {}",
            flow.id,
            from_preview,
            to_preview,
            flow.commodity
        );
    }

    Ok(())
}

fn cmd_delete_flow(db: &Database, id: i64) -> Result<()> {
    if db.delete_surplus_flow(id)? {
        println!("Deleted surplus flow #{}", id);
    } else {
        println!("Surplus flow #{} not found", id);
    }
    Ok(())
}

fn cmd_timescale(db: &Database, claim_id: i64, scale_str: &str, notes: Option<&str>) -> Result<()> {
    use engine::BraudelTimescale;

    let timescale = match BraudelTimescale::from_str(scale_str) {
        Some(t) => t,
        None => {
            println!("Invalid timescale: {}", scale_str);
            println!("Valid timescales: event, conjuncture, longue_duree");
            return Ok(());
        }
    };

    // Verify claim exists
    let claim = match db.get_claim(claim_id)? {
        Some(c) => c,
        None => {
            println!("Claim #{} not found", claim_id);
            return Ok(());
        }
    };

    let observation = db.create_temporal_observation(claim_id, timescale, notes)?;

    println!("Created temporal observation #{}", observation.id);
    println!("  Claim: {}", if claim.text.len() > 50 { format!("{}...", &claim.text[..47]) } else { claim.text });
    println!("  Timescale: {}", observation.timescale.as_str());

    Ok(())
}

fn cmd_list_timescales(db: &Database, scale_filter: Option<&str>) -> Result<()> {
    use engine::BraudelTimescale;

    let observations = if let Some(scale_str) = scale_filter {
        match BraudelTimescale::from_str(scale_str) {
            Some(t) => db.list_temporal_observations_by_timescale(t)?,
            None => {
                println!("Invalid timescale: {}", scale_str);
                return Ok(());
            }
        }
    } else {
        db.list_all_temporal_observations()?
    };

    if observations.is_empty() {
        println!("No temporal observations found.");
        return Ok(());
    }

    println!("{:<5} {:<8} {:<15} {}", "ID", "CLAIM", "TIMESCALE", "NOTES");
    println!("{}", "-".repeat(60));

    for obs in observations {
        let notes_preview = obs.notes.as_ref().map(|n| {
            if n.len() > 20 { format!("{}...", &n[..17]) } else { n.clone() }
        }).unwrap_or_default();
        println!(
            "{:<5} {:<8} {:<15} {}",
            obs.id,
            obs.claim_id,
            obs.timescale.as_str(),
            notes_preview
        );
    }

    Ok(())
}

fn cmd_framework_stats(db: &Database) -> Result<()> {
    let stats = db.get_framework_stats()?;

    println!("Analytical Framework Statistics:\n");
    println!("{:<30} {:>10}", "Cyclical Indicators", stats.cyclical_indicators);
    println!("{:<30} {:>10}", "Causal Relations", stats.causal_relations);
    println!("{:<30} {:>10}", "Idea Transmissions", stats.idea_transmissions);
    println!("{:<30} {:>10}", "Geopolitical Entities", stats.geopolitical_entities);
    println!("{:<30} {:>10}", "Surplus Flows", stats.surplus_flows);
    println!("{:<30} {:>10}", "Temporal Observations", stats.temporal_observations);

    let total = stats.cyclical_indicators + stats.causal_relations + stats.idea_transmissions
        + stats.geopolitical_entities + stats.surplus_flows + stats.temporal_observations;

    println!("{}", "-".repeat(42));
    println!("{:<30} {:>10}", "Total Framework Items", total);

    Ok(())
}

// Phase 9: Synthesis & Pattern Detection

fn cmd_moc_create(db: &Database, title: &str, description: Option<&str>) -> Result<()> {
    // Check if already exists
    if db.get_moc_by_title(title)?.is_some() {
        println!("MOC '{}' already exists.", title);
        return Ok(());
    }

    let moc = db.create_moc(title, description)?;
    println!("Created MOC #{}: {}", moc.id, moc.title);
    if let Some(desc) = description {
        println!("  Description: {}", desc);
    }
    println!("\nUse 'moc-add {} <claim-id>' to add claims.", moc.id);

    Ok(())
}

fn cmd_list_mocs(db: &Database) -> Result<()> {
    let mocs = db.list_mocs()?;

    if mocs.is_empty() {
        println!("No Maps of Content yet.");
        println!("Use 'moc-create <title>' to create one.");
        return Ok(());
    }

    println!("{:<5} {:<30} {:<8} {}", "ID", "TITLE", "CLAIMS", "UPDATED");
    println!("{}", "-".repeat(70));

    for moc in mocs {
        let claim_count = db.get_moc_claim_count(moc.id)?;
        let updated = moc.updated_at.format("%Y-%m-%d").to_string();
        let title_preview = if moc.title.len() > 28 {
            format!("{}...", &moc.title[..25])
        } else {
            moc.title.clone()
        };
        println!("{:<5} {:<30} {:<8} {}", moc.id, title_preview, claim_count, updated);
    }

    Ok(())
}

fn cmd_show_moc(db: &Database, id_or_title: &str) -> Result<()> {
    // Try as ID first, then as title
    let moc_with_claims = if let Ok(id) = id_or_title.parse::<i64>() {
        db.get_moc_with_claims(id)?
    } else {
        match db.get_moc_by_title(id_or_title)? {
            Some(moc) => db.get_moc_with_claims(moc.id)?,
            None => None,
        }
    };

    let mwc = match moc_with_claims {
        Some(m) => m,
        None => {
            println!("MOC not found: {}", id_or_title);
            return Ok(());
        }
    };

    println!("Map of Content: {} (ID: {})", mwc.moc.title, mwc.moc.id);
    if let Some(desc) = &mwc.moc.description {
        println!("Description: {}", desc);
    }
    println!("Created: {}", mwc.moc.created_at.format("%Y-%m-%d %H:%M"));
    println!("Updated: {}", mwc.moc.updated_at.format("%Y-%m-%d %H:%M"));

    if !mwc.claims.is_empty() {
        println!("\nClaims ({}):", mwc.claims.len());
        println!("{}", "-".repeat(60));
        for claim in &mwc.claims {
            let text_preview = if claim.text.len() > 55 {
                format!("{}...", &claim.text[..52])
            } else {
                claim.text.clone()
            };
            println!("  [{}] {}", claim.id, text_preview);
        }
    } else {
        println!("\nNo claims yet. Use 'moc-add {} <claim-id>' to add claims.", mwc.moc.id);
    }

    if !mwc.sub_mocs.is_empty() {
        println!("\nSub-MOCs:");
        for sub in &mwc.sub_mocs {
            println!("  - {} (ID: {})", sub.title, sub.id);
        }
    }

    Ok(())
}

fn cmd_moc_add(db: &Database, moc_id: i64, claim_id: i64, order: i32) -> Result<()> {
    // Verify MOC exists
    if db.get_moc(moc_id)?.is_none() {
        println!("MOC #{} not found", moc_id);
        return Ok(());
    }

    // Verify claim exists
    let claim = match db.get_claim(claim_id)? {
        Some(c) => c,
        None => {
            println!("Claim #{} not found", claim_id);
            return Ok(());
        }
    };

    db.add_claim_to_moc(moc_id, claim_id, order)?;
    println!("Added claim #{} to MOC #{}", claim_id, moc_id);
    println!("  Claim: {}", if claim.text.len() > 50 { format!("{}...", &claim.text[..47]) } else { claim.text });

    Ok(())
}

fn cmd_moc_remove(db: &Database, moc_id: i64, claim_id: i64) -> Result<()> {
    if db.remove_claim_from_moc(moc_id, claim_id)? {
        println!("Removed claim #{} from MOC #{}", claim_id, moc_id);
    } else {
        println!("Claim #{} not found in MOC #{}", claim_id, moc_id);
    }
    Ok(())
}

fn cmd_delete_moc(db: &Database, id: i64) -> Result<()> {
    if db.delete_moc(id)? {
        println!("Deleted MOC #{}", id);
    } else {
        println!("MOC #{} not found", id);
    }
    Ok(())
}

fn cmd_ask(db: &Database, question: &str, parent_id: Option<i64>, notes: Option<&str>) -> Result<()> {
    // Verify parent exists if specified
    if let Some(pid) = parent_id {
        if db.get_research_question(pid)?.is_none() {
            println!("Parent question #{} not found", pid);
            return Ok(());
        }
    }

    let q = db.create_research_question(question, parent_id, notes)?;
    println!("Created research question #{}", q.id);
    println!("  {}", q.question);
    if parent_id.is_some() {
        println!("  (Sub-question of #{})", parent_id.unwrap());
    }
    println!("\nUse 'evidence {} --claim <id>' to add supporting evidence.", q.id);

    Ok(())
}

fn cmd_list_questions(db: &Database, status_filter: Option<&str>) -> Result<()> {
    use engine::QuestionStatus;

    let status = status_filter.and_then(QuestionStatus::from_str);
    let questions = db.list_research_questions(status)?;

    if questions.is_empty() {
        println!("No research questions yet.");
        println!("Use 'ask \"Your question?\"' to create one.");
        return Ok(());
    }

    println!("{:<5} {:<10} {:<50} {}", "ID", "STATUS", "QUESTION", "EVIDENCE");
    println!("{}", "-".repeat(80));

    for q in questions {
        let claims = db.get_question_evidence_claims(q.id)?;
        let videos = db.get_question_evidence_videos(q.id)?;
        let evidence_count = claims.len() + videos.len();

        let question_preview = if q.question.len() > 48 {
            format!("{}...", &q.question[..45])
        } else {
            q.question.clone()
        };

        println!(
            "{:<5} {:<10} {:<50} {}",
            q.id,
            q.status.as_str(),
            question_preview,
            evidence_count
        );
    }

    Ok(())
}

fn cmd_show_question(db: &Database, id: i64) -> Result<()> {
    let qwe = match db.get_question_with_evidence(id)? {
        Some(q) => q,
        None => {
            println!("Question #{} not found", id);
            return Ok(());
        }
    };

    println!("Research Question #{}", qwe.question.id);
    println!("  {}", qwe.question.question);
    println!("\nStatus: {}", qwe.question.status.as_str());

    if let Some(notes) = &qwe.question.notes {
        println!("Notes: {}", notes);
    }

    if !qwe.claims.is_empty() {
        println!("\nSupporting Claims ({}):", qwe.claims.len());
        for claim in &qwe.claims {
            let text_preview = if claim.text.len() > 55 {
                format!("{}...", &claim.text[..52])
            } else {
                claim.text.clone()
            };
            println!("  [{}] {}", claim.id, text_preview);
        }
    }

    if !qwe.videos.is_empty() {
        println!("\nRelated Videos ({}):", qwe.videos.len());
        for video in &qwe.videos {
            println!("  [{}] {}", video.id, video.title);
        }
    }

    if !qwe.sub_questions.is_empty() {
        println!("\nSub-questions:");
        for sub in &qwe.sub_questions {
            println!("  [{}] {} ({})", sub.id, sub.question, sub.status.as_str());
        }
    }

    // Record access for the claims shown
    for claim in &qwe.claims {
        db.record_claim_access(claim.id)?;
    }

    Ok(())
}

fn cmd_add_evidence(
    db: &Database,
    question_id: i64,
    claim_id: Option<i64>,
    video_id: Option<&str>,
    relevance: Option<&str>,
) -> Result<()> {
    if claim_id.is_none() && video_id.is_none() {
        println!("Must specify either --claim or --video");
        return Ok(());
    }

    // Verify question exists
    if db.get_research_question(question_id)?.is_none() {
        println!("Question #{} not found", question_id);
        return Ok(());
    }

    // Verify claim exists if specified
    if let Some(cid) = claim_id {
        if db.get_claim(cid)?.is_none() {
            println!("Claim #{} not found", cid);
            return Ok(());
        }
    }

    // Verify video exists if specified
    if let Some(vid) = video_id {
        if db.get_video(vid)?.is_none() {
            println!("Video '{}' not found", vid);
            return Ok(());
        }
    }

    db.add_evidence_to_question(question_id, claim_id, video_id, relevance)?;
    println!("Added evidence to question #{}", question_id);
    if let Some(cid) = claim_id {
        println!("  Claim: #{}", cid);
    }
    if let Some(vid) = video_id {
        println!("  Video: {}", vid);
    }

    Ok(())
}

fn cmd_answer_question(db: &Database, id: i64, status_str: &str) -> Result<()> {
    use engine::QuestionStatus;

    let status = match QuestionStatus::from_str(status_str) {
        Some(s) => s,
        None => {
            println!("Invalid status: {}", status_str);
            println!("Valid statuses: active, answered, refined, parked");
            return Ok(());
        }
    };

    if db.update_question_status(id, status)? {
        println!("Updated question #{} to status: {}", id, status.as_str());
    } else {
        println!("Question #{} not found", id);
    }

    Ok(())
}

fn cmd_delete_question(db: &Database, id: i64) -> Result<()> {
    if db.delete_research_question(id)? {
        println!("Deleted question #{}", id);
    } else {
        println!("Question #{} not found", id);
    }
    Ok(())
}

fn cmd_add_pattern(
    db: &Database,
    type_str: &str,
    description: &str,
    videos_str: Option<&str>,
    claims_str: Option<&str>,
    confidence: f32,
) -> Result<()> {
    use engine::PatternType;

    let pattern_type = match PatternType::from_str(type_str) {
        Some(t) => t,
        None => {
            println!("Invalid pattern type: {}", type_str);
            println!("Valid types: recurring_theme, contradiction, consensus, evolution, parallel");
            return Ok(());
        }
    };

    let video_ids: Vec<String> = videos_str
        .map(|s| s.split(',').map(|v| v.trim().to_string()).collect())
        .unwrap_or_default();

    let claim_ids: Vec<i64> = claims_str
        .map(|s| {
            s.split(',')
                .filter_map(|c| c.trim().parse().ok())
                .collect()
        })
        .unwrap_or_default();

    if video_ids.is_empty() && claim_ids.is_empty() {
        println!("Must specify at least one video or claim.");
        return Ok(());
    }

    let pattern = db.save_detected_pattern(pattern_type, description, &video_ids, &claim_ids, confidence)?;
    println!("Created pattern #{}", pattern.id);
    println!("  Type: {}", pattern.pattern_type.as_str());
    println!("  Description: {}", description);
    println!("  Confidence: {:.0}%", confidence * 100.0);

    Ok(())
}

fn cmd_list_patterns(db: &Database, type_filter: Option<&str>) -> Result<()> {
    use engine::PatternType;

    let pattern_type = type_filter.and_then(PatternType::from_str);
    let patterns = db.list_detected_patterns(pattern_type)?;

    if patterns.is_empty() {
        println!("No patterns detected yet.");
        return Ok(());
    }

    println!("{:<5} {:<20} {:<45} {}", "ID", "TYPE", "DESCRIPTION", "CONF");
    println!("{}", "-".repeat(80));

    for p in patterns {
        let desc_preview = if p.description.len() > 43 {
            format!("{}...", &p.description[..40])
        } else {
            p.description.clone()
        };
        println!(
            "{:<5} {:<20} {:<45} {:.0}%",
            p.id,
            p.pattern_type.as_str(),
            desc_preview,
            p.confidence * 100.0
        );
    }

    Ok(())
}

fn cmd_delete_pattern(db: &Database, id: i64) -> Result<()> {
    if db.delete_detected_pattern(id)? {
        println!("Deleted pattern #{}", id);
    } else {
        println!("Pattern #{} not found", id);
    }
    Ok(())
}

fn cmd_review(db: &Database, stale_only: bool, orphans_only: bool, random_count: usize) -> Result<()> {
    if stale_only {
        let stale = db.get_stale_claims(30)?;
        if stale.is_empty() {
            println!("No stale claims (all accessed within 30 days).");
        } else {
            println!("Stale Claims (not accessed in 30+ days): {}\n", stale.len());
            for claim in stale.iter().take(20) {
                let text_preview = if claim.text.len() > 55 {
                    format!("{}...", &claim.text[..52])
                } else {
                    claim.text.clone()
                };
                println!("  [{}] {}", claim.id, text_preview);
            }
            if stale.len() > 20 {
                println!("  ... and {} more", stale.len() - 20);
            }
        }
        return Ok(());
    }

    if orphans_only {
        let orphans = db.get_orphan_claims()?;
        if orphans.is_empty() {
            println!("No orphan claims (all have 2+ connections).");
        } else {
            println!("Orphan Claims (fewer than 2 connections): {}\n", orphans.len());
            for claim in orphans.iter().take(20) {
                let link_count = db.get_claim_link_count(claim.id)?;
                let text_preview = if claim.text.len() > 50 {
                    format!("{}...", &claim.text[..47])
                } else {
                    claim.text.clone()
                };
                println!("  [{}] ({} links) {}", claim.id, link_count, text_preview);
            }
            if orphans.len() > 20 {
                println!("  ... and {} more", orphans.len() - 20);
            }
        }
        return Ok(());
    }

    // Full review queue
    let queue = db.get_review_queue(30, random_count)?;

    println!("Review Queue:\n");

    println!("Stale Claims (30+ days): {}", queue.stale_claims.len());
    if !queue.stale_claims.is_empty() {
        for claim in queue.stale_claims.iter().take(5) {
            let text_preview = if claim.text.len() > 50 {
                format!("{}...", &claim.text[..47])
            } else {
                claim.text.clone()
            };
            println!("  [{}] {}", claim.id, text_preview);
        }
        if queue.stale_claims.len() > 5 {
            println!("  ... use 'review --stale' to see all");
        }
    }

    println!("\nOrphan Claims (<2 links): {}", queue.orphan_claims.len());
    if !queue.orphan_claims.is_empty() {
        for claim in queue.orphan_claims.iter().take(5) {
            let text_preview = if claim.text.len() > 50 {
                format!("{}...", &claim.text[..47])
            } else {
                claim.text.clone()
            };
            println!("  [{}] {}", claim.id, text_preview);
        }
        if queue.orphan_claims.len() > 5 {
            println!("  ... use 'review --orphans' to see all");
        }
    }

    if !queue.random_suggestions.is_empty() {
        println!("\nRandom Suggestions (for serendipitous review):");
        for claim in &queue.random_suggestions {
            let text_preview = if claim.text.len() > 50 {
                format!("{}...", &claim.text[..47])
            } else {
                claim.text.clone()
            };
            println!("  [{}] {}", claim.id, text_preview);
            // Record access
            db.record_claim_access(claim.id)?;
        }
    }

    Ok(())
}

fn cmd_synthesis_stats(db: &Database) -> Result<()> {
    let stats = db.get_synthesis_stats()?;

    println!("Synthesis Statistics:\n");
    println!("{:<30} {:>10}", "Maps of Content", stats.mocs);
    println!("{:<30} {:>10}", "Research Questions", stats.research_questions);
    println!("{:<30} {:>10}", "  Active Questions", stats.active_questions);
    println!("{:<30} {:>10}", "Detected Patterns", stats.detected_patterns);
    println!("\nReview Queue:");
    println!("{:<30} {:>10}", "  Stale Claims (30+ days)", stats.stale_claims);
    println!("{:<30} {:>10}", "  Orphan Claims (<2 links)", stats.orphan_claims);

    Ok(())
}

// Phase 10: AI Processing Queue Commands

fn cmd_queue(db: &Database, show_all: bool) -> Result<()> {
    let items = db.get_queue(show_all)?;

    if items.is_empty() {
        println!("AI processing queue is empty.");
        return Ok(());
    }

    println!("AI Processing Queue:\n");
    println!("{:<15} {:<12} {:<8} {:<20} {:<6}", "VIDEO_ID", "STATUS", "PRIORITY", "CREATED", "CLAIMS");
    println!("{}", "-".repeat(65));

    for item in items {
        let created = item.created_at.format("%Y-%m-%d %H:%M").to_string();
        println!(
            "{:<15} {:<12} {:<8} {:<20} {:<6}",
            &item.video_id[..item.video_id.len().min(14)],
            item.status.as_str(),
            item.priority,
            created,
            item.claims_extracted
        );
        if let Some(ref err) = item.error_message {
            println!("  Error: {}", err);
        }
    }

    Ok(())
}

fn cmd_queue_add(db: &Database, video_id: &str, priority: i32) -> Result<()> {
    // Check if video exists
    if db.get_video(video_id)?.is_none() {
        anyhow::bail!("Video '{}' not found", video_id);
    }

    db.add_to_queue(video_id, priority)?;
    println!("Added '{}' to processing queue with priority {}", video_id, priority);
    Ok(())
}

fn cmd_queue_skip(db: &Database, video_id: &str) -> Result<()> {
    if db.queue_skip(video_id)? {
        println!("Skipped '{}'", video_id);
    } else {
        println!("Video '{}' not found in queue", video_id);
    }
    Ok(())
}

fn cmd_queue_reset(db: &Database, video_id: &str) -> Result<()> {
    if db.queue_reset(video_id)? {
        println!("Reset '{}' to pending", video_id);
    } else {
        println!("Video '{}' not found in queue", video_id);
    }
    Ok(())
}

fn cmd_queue_start(db: &Database, video_id: &str) -> Result<()> {
    if db.queue_start(video_id)? {
        println!("Started processing '{}'", video_id);
    } else {
        println!("Video '{}' not found in queue or not pending", video_id);
    }
    Ok(())
}

fn cmd_queue_complete(db: &Database, video_id: &str, claims: i32) -> Result<()> {
    if db.queue_complete(video_id, claims)? {
        println!("Completed '{}' with {} claims extracted", video_id, claims);
    } else {
        println!("Video '{}' not found in queue", video_id);
    }
    Ok(())
}

fn cmd_queue_fail(db: &Database, video_id: &str, reason: &str) -> Result<()> {
    if db.queue_fail(video_id, reason)? {
        println!("Marked '{}' as failed: {}", video_id, reason);
    } else {
        println!("Video '{}' not found in queue", video_id);
    }
    Ok(())
}

fn cmd_queue_clear(db: &Database, completed: bool, failed: bool) -> Result<()> {
    use engine::ProcessingStatus;

    if !completed && !failed {
        println!("Specify --completed or --failed to clear");
        return Ok(());
    }

    let mut cleared = 0;
    if completed {
        cleared += db.queue_clear(ProcessingStatus::Completed)?;
    }
    if failed {
        cleared += db.queue_clear(ProcessingStatus::Failed)?;
    }

    println!("Cleared {} items from queue", cleared);
    Ok(())
}

fn cmd_export_transcript(db: &Database, video_id: &str) -> Result<()> {
    let video = db.get_video(video_id)?
        .ok_or_else(|| anyhow::anyhow!("Video '{}' not found", video_id))?;

    let transcript = db.get_transcript(video_id)?
        .ok_or_else(|| anyhow::anyhow!("No transcript for video '{}'", video_id))?;

    // Print header comment with video info
    println!("# Video: {}", video.title);
    println!("# ID: {}", video.id);
    if let Some(ref channel) = video.channel {
        println!("# Channel: {}", channel);
    }
    println!("# Segments: {}", transcript.segments.len());
    println!("#");
    println!("# Transcript:");
    println!();

    // Print transcript with timestamps
    for segment in &transcript.segments {
        let minutes = (segment.start_time / 60.0) as u32;
        let seconds = (segment.start_time % 60.0) as u32;
        println!("[{:02}:{:02}] {}", minutes, seconds, segment.text);
    }

    Ok(())
}

fn cmd_export_queue(db: &Database) -> Result<()> {
    let ids = db.get_pending_video_ids()?;

    if ids.is_empty() {
        println!("# No pending videos in queue");
        return Ok(());
    }

    println!("# Pending videos in AI processing queue:");
    for id in ids {
        println!("{}", id);
    }

    Ok(())
}

// ============================================
// Phase 12: Expanded Knowledge Entity Commands
// ============================================

fn cmd_add_source(
    db: &Database,
    title: &str,
    author: Option<&str>,
    source_type: &str,
    year: Option<i32>,
    url: Option<&str>,
    notes: Option<&str>,
) -> Result<()> {
    let st = SourceType::from_str(source_type)
        .ok_or_else(|| anyhow::anyhow!("Invalid source type: {}. Valid options: book, paper, documentary, article, lecture, website", source_type))?;

    let id = db.add_source(title, author, st, year, url, notes)?;
    println!("Added source #{}: {}", id, title);
    if let Some(a) = author {
        println!("  Author: {}", a);
    }
    println!("  Type: {}", source_type);
    if let Some(y) = year {
        println!("  Year: {}", y);
    }
    Ok(())
}

fn cmd_list_sources(db: &Database) -> Result<()> {
    let sources = db.get_sources()?;
    if sources.is_empty() {
        println!("No sources in knowledge base.");
        return Ok(());
    }

    println!("{:<5} {:<40} {:<25} {:<12} {:<6}", "ID", "TITLE", "AUTHOR", "TYPE", "YEAR");
    println!("{}", "-".repeat(90));
    for s in sources {
        println!("{:<5} {:<40} {:<25} {:<12} {:<6}",
            s.id,
            truncate(&s.title, 38),
            s.author.as_deref().map(|a| truncate(a, 23)).unwrap_or("-".to_string()),
            s.source_type.as_str(),
            s.year.map(|y| y.to_string()).unwrap_or("-".to_string()),
        );
    }
    Ok(())
}

fn cmd_cite_source(db: &Database, video_id: &str, source_id: i64, timestamp: Option<f64>, context: Option<&str>) -> Result<()> {
    db.cite_source(video_id, source_id, timestamp, context)?;
    println!("Cited source #{} in video {}", source_id, video_id);
    Ok(())
}

fn cmd_add_scholar(
    db: &Database,
    name: &str,
    field: Option<&str>,
    era: Option<&str>,
    contribution: Option<&str>,
) -> Result<()> {
    // Check if scholar already exists
    if let Some(existing) = db.find_scholar_by_name(name)? {
        println!("Scholar already exists: #{} {}", existing.id, existing.name);
        return Ok(());
    }

    let id = db.add_scholar(name, field, era, contribution)?;
    println!("Added scholar #{}: {}", id, name);
    if let Some(f) = field {
        println!("  Field: {}", f);
    }
    if let Some(e) = era {
        println!("  Era: {}", e);
    }
    Ok(())
}

fn cmd_list_scholars(db: &Database) -> Result<()> {
    let scholars = db.get_scholars()?;
    if scholars.is_empty() {
        println!("No scholars in knowledge base.");
        return Ok(());
    }

    println!("{:<5} {:<30} {:<20} {:<15}", "ID", "NAME", "FIELD", "ERA");
    println!("{}", "-".repeat(72));
    for s in scholars {
        println!("{:<5} {:<30} {:<20} {:<15}",
            s.id,
            truncate(&s.name, 28),
            s.field.as_deref().map(|f| truncate(f, 18)).unwrap_or("-".to_string()),
            s.era.as_deref().map(|e| truncate(e, 13)).unwrap_or("-".to_string()),
        );
    }
    Ok(())
}

fn cmd_cite_scholar(db: &Database, video_id: &str, scholar_id: i64, timestamp: Option<f64>, context: Option<&str>) -> Result<()> {
    db.cite_scholar(video_id, scholar_id, timestamp, context)?;
    println!("Cited scholar #{} in video {}", scholar_id, video_id);
    Ok(())
}

fn cmd_add_visual(
    db: &Database,
    video_id: &str,
    description: &str,
    timestamp: f64,
    visual_type: &str,
    significance: Option<&str>,
    location_name: Option<&str>,
    era_name: Option<&str>,
) -> Result<()> {
    let vt = VisualType::from_str(visual_type)
        .ok_or_else(|| anyhow::anyhow!("Invalid visual type: {}. Valid options: painting, map, diagram, artifact, chart, photo, skeleton, symbol, architecture, inscription", visual_type))?;

    // Look up location ID if provided
    let location_id = if let Some(loc) = location_name {
        db.get_location_by_name(loc)?.map(|l| l.id)
    } else {
        None
    };

    // Look up era ID if provided
    let era_id = if let Some(era) = era_name {
        db.get_era_by_name(era)?.map(|e| e.id)
    } else {
        None
    };

    let id = db.add_visual(video_id, timestamp, vt, description, significance, location_id, era_id)?;
    println!("Added visual #{}: {}", id, truncate(description, 50));
    println!("  Type: {}", visual_type);
    println!("  Timestamp: {}s", timestamp);
    if let Some(loc) = location_name {
        println!("  Location: {}", loc);
    }
    Ok(())
}

fn cmd_list_visuals(db: &Database, video_id: &str) -> Result<()> {
    let visuals = db.get_visuals_for_video(video_id)?;
    if visuals.is_empty() {
        println!("No visuals for video {}.", video_id);
        return Ok(());
    }

    println!("Visuals for video {}:\n", video_id);
    for v in visuals {
        println!("[{:>6.0}s] {} - {}", v.timestamp, v.visual_type.as_str(), v.description);
        if let Some(sig) = &v.significance {
            println!("         Significance: {}", sig);
        }
    }
    Ok(())
}

fn cmd_define_term(
    db: &Database,
    term: &str,
    definition: &str,
    domain: Option<&str>,
    video_id: Option<&str>,
    timestamp: Option<f64>,
    scholar_name: Option<&str>,
) -> Result<()> {
    // Look up scholar ID if provided
    let scholar_id = if let Some(name) = scholar_name {
        db.find_scholar_by_name(name)?.map(|s| s.id)
    } else {
        None
    };

    let id = db.add_term(term, definition, domain, video_id, timestamp, scholar_id)?;
    println!("Defined term #{}: {}", id, term);
    println!("  Definition: {}", truncate(definition, 60));
    if let Some(d) = domain {
        println!("  Domain: {}", d);
    }
    Ok(())
}

fn cmd_list_terms(db: &Database) -> Result<()> {
    let terms = db.get_terms()?;
    if terms.is_empty() {
        println!("No terms in knowledge base.");
        return Ok(());
    }

    println!("{:<5} {:<25} {:<50} {:<15}", "ID", "TERM", "DEFINITION", "DOMAIN");
    println!("{}", "-".repeat(97));
    for t in terms {
        println!("{:<5} {:<25} {:<50} {:<15}",
            t.id,
            truncate(&t.term, 23),
            truncate(&t.definition, 48),
            t.domain.as_deref().map(|d| truncate(d, 13)).unwrap_or("-".to_string()),
        );
    }
    Ok(())
}

fn cmd_add_cited_evidence(
    db: &Database,
    video_id: &str,
    description: &str,
    evidence_type: &str,
    timestamp: Option<f64>,
    location_name: Option<&str>,
    era_name: Option<&str>,
) -> Result<()> {
    let et = EvidenceType::from_str(evidence_type)
        .ok_or_else(|| anyhow::anyhow!("Invalid evidence type: {}. Valid options: archaeological, genetic, textual, anthropological, linguistic, artistic, scientific, historical", evidence_type))?;

    // Look up location ID if provided
    let location_id = if let Some(loc) = location_name {
        db.get_location_by_name(loc)?.map(|l| l.id)
    } else {
        None
    };

    // Look up era ID if provided
    let era_id = if let Some(era) = era_name {
        db.get_era_by_name(era)?.map(|e| e.id)
    } else {
        None
    };

    let id = db.add_evidence(video_id, et, description, location_id, era_id, timestamp, None)?;
    println!("Added evidence #{}: {}", id, truncate(description, 50));
    println!("  Type: {}", evidence_type);
    if let Some(t) = timestamp {
        println!("  Timestamp: {}s", t);
    }
    Ok(())
}

fn cmd_list_cited_evidence(db: &Database, video_id: &str) -> Result<()> {
    let evidence = db.get_evidence_for_video(video_id)?;
    if evidence.is_empty() {
        println!("No evidence for video {}.", video_id);
        return Ok(());
    }

    println!("Evidence cited in video {}:\n", video_id);
    for e in evidence {
        let ts = e.timestamp.map(|t| format!("[{:>6.0}s]", t)).unwrap_or("        ".to_string());
        println!("{} {} - {}", ts, e.evidence_type.as_str(), e.description);
    }
    Ok(())
}

fn cmd_add_quote(
    db: &Database,
    video_id: &str,
    text: &str,
    speaker: Option<&str>,
    timestamp: Option<f64>,
    context: Option<&str>,
) -> Result<()> {
    // Try to find scholar ID if speaker matches a scholar name
    let scholar_id = if let Some(name) = speaker {
        db.find_scholar_by_name(name)?.map(|s| s.id)
    } else {
        None
    };

    let id = db.add_quote(video_id, text, speaker, scholar_id, timestamp, context)?;
    println!("Added quote #{}: \"{}\"", id, truncate(text, 50));
    if let Some(s) = speaker {
        println!("  Speaker: {}", s);
    }
    if let Some(t) = timestamp {
        println!("  Timestamp: {}s", t);
    }
    Ok(())
}

fn cmd_list_quotes(db: &Database, video_id: &str) -> Result<()> {
    let quotes = db.get_quotes_for_video(video_id)?;
    if quotes.is_empty() {
        println!("No quotes for video {}.", video_id);
        return Ok(());
    }

    println!("Quotes from video {}:\n", video_id);
    for q in quotes {
        let ts = q.timestamp.map(|t| format!("[{:>6.0}s]", t)).unwrap_or("        ".to_string());
        let speaker = q.speaker.as_deref().unwrap_or("(unknown)");
        println!("{} {} said:", ts, speaker);
        println!("         \"{}\"", q.text);
        if let Some(ctx) = &q.context {
            println!("         Context: {}", ctx);
        }
        println!();
    }
    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
