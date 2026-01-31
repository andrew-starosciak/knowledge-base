use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub url: String,
    pub title: String,
    pub channel: Option<String>,
    pub upload_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    pub video_id: String,
    pub language: String,
    pub segments: Vec<TranscriptSegment>,
    pub full_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub start_time: f64,
    pub duration: f64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub video: Video,
    pub matches: Vec<SegmentMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentMatch {
    pub start_time: f64,
    pub duration: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Era {
    pub id: i64,
    pub name: String,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub video_id: String,
    pub timestamp: Option<f64>,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoLocation {
    pub id: i64,
    pub video_id: String,
    pub location_id: i64,
    pub era_id: Option<i64>,
    pub topic_id: Option<i64>,
    pub timestamp: Option<f64>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPin {
    pub location: Location,
    pub video_id: String,
    pub video_title: String,
    pub era: Option<String>,
    pub topic: Option<String>,
    pub timestamp: Option<f64>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AutoTags {
    pub eras: Vec<String>,
    pub regions: Vec<String>,
    pub topics: Vec<String>,
}

// Phase 5: Research Tools

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearch {
    pub id: i64,
    pub name: String,
    pub query: Option<String>,
    pub era: Option<String>,
    pub region: Option<String>,
    pub topic: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSearchResult {
    pub video: Video,
    pub matches: Vec<SegmentMatch>,
    pub eras: Vec<String>,
    pub regions: Vec<String>,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportEntry {
    pub name: String,
    pub video_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonFeature {
    pub r#type: String,
    pub geometry: GeoJsonGeometry,
    pub properties: GeoJsonProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonGeometry {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonProperties {
    pub name: String,
    pub video_id: String,
    pub video_title: String,
    pub era: Option<String>,
    pub topic: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonCollection {
    pub r#type: String,
    pub features: Vec<GeoJsonFeature>,
}

// Phase 6: Claim Extraction & Atomic Notes

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimCategory {
    CyclicalPattern,      // Recurring historical patterns
    CausalClaim,          // X causes Y relationships
    MemeticTransmission,  // Idea/ideology spread
    GeopoliticalDynamic,  // Core/periphery, power relations
    Factual,              // General historical facts
    Phenomenological,     // First-person experiential claims (consciousness, spirituality)
    Metaphysical,         // Claims about nature of reality
}

impl ClaimCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClaimCategory::CyclicalPattern => "cyclical",
            ClaimCategory::CausalClaim => "causal",
            ClaimCategory::MemeticTransmission => "memetic",
            ClaimCategory::GeopoliticalDynamic => "geopolitical",
            ClaimCategory::Factual => "factual",
            ClaimCategory::Phenomenological => "phenomenological",
            ClaimCategory::Metaphysical => "metaphysical",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cyclical" | "cyclical_pattern" => Some(ClaimCategory::CyclicalPattern),
            "causal" | "causal_claim" => Some(ClaimCategory::CausalClaim),
            "memetic" | "memetic_transmission" => Some(ClaimCategory::MemeticTransmission),
            "geopolitical" | "geopolitical_dynamic" => Some(ClaimCategory::GeopoliticalDynamic),
            "factual" => Some(ClaimCategory::Factual),
            "phenomenological" => Some(ClaimCategory::Phenomenological),
            "metaphysical" => Some(ClaimCategory::Metaphysical),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

impl Confidence {
    pub fn as_str(&self) -> &'static str {
        match self {
            Confidence::High => "high",
            Confidence::Medium => "medium",
            Confidence::Low => "low",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "high" => Some(Confidence::High),
            "medium" | "med" => Some(Confidence::Medium),
            "low" => Some(Confidence::Low),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: i64,
    pub text: String,
    pub video_id: String,
    pub timestamp: Option<f64>,
    pub source_quote: String,
    pub category: ClaimCategory,
    pub confidence: Confidence,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    Supports,
    Contradicts,
    Elaborates,
    CausedBy,
    Causes,
    Related,
}

impl LinkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkType::Supports => "supports",
            LinkType::Contradicts => "contradicts",
            LinkType::Elaborates => "elaborates",
            LinkType::CausedBy => "caused_by",
            LinkType::Causes => "causes",
            LinkType::Related => "related",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "supports" => Some(LinkType::Supports),
            "contradicts" => Some(LinkType::Contradicts),
            "elaborates" => Some(LinkType::Elaborates),
            "caused_by" | "causedby" => Some(LinkType::CausedBy),
            "causes" => Some(LinkType::Causes),
            "related" => Some(LinkType::Related),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimLink {
    pub id: i64,
    pub source_claim_id: i64,
    pub target_claim_id: i64,
    pub link_type: LinkType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimWithLinks {
    pub claim: Claim,
    pub outgoing_links: Vec<(ClaimLink, Claim)>,
    pub incoming_links: Vec<(ClaimLink, Claim)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptLayer {
    pub id: i64,
    pub video_id: String,
    pub layer: u8,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptChunk {
    pub id: i64,
    pub video_id: String,
    pub chunk_index: i32,
    pub start_time: f64,
    pub end_time: f64,
    pub text: String,
    pub token_count: i32,
    pub overlap_with_previous: bool,
}

// Phase 7: Semantic Search & Embeddings

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingSource {
    Chunk,
    Claim,
    Summary,
    Video,
}

impl EmbeddingSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmbeddingSource::Chunk => "chunk",
            EmbeddingSource::Claim => "claim",
            EmbeddingSource::Summary => "summary",
            EmbeddingSource::Video => "video",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "chunk" => Some(EmbeddingSource::Chunk),
            "claim" => Some(EmbeddingSource::Claim),
            "summary" => Some(EmbeddingSource::Summary),
            "video" => Some(EmbeddingSource::Video),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub id: i64,
    pub source_type: EmbeddingSource,
    pub source_id: String,  // video_id for Video/Summary, chunk_id or claim_id as string
    pub model: String,
    pub vector: Vec<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub source_type: EmbeddingSource,
    pub source_id: String,
    pub score: f32,
    pub text: String,
    pub video_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchResult {
    pub video: Video,
    pub keyword_score: f32,
    pub semantic_score: f32,
    pub combined_score: f32,
    pub matching_chunks: Vec<ChunkMatch>,
    pub matching_claims: Vec<Claim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMatch {
    pub chunk: TranscriptChunk,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingStats {
    pub total_embeddings: i64,
    pub video_embeddings: i64,
    pub chunk_embeddings: i64,
    pub claim_embeddings: i64,
    pub summary_embeddings: i64,
    pub model: Option<String>,
    pub dimensions: Option<i32>,
}

// Phase 8: Analytical Frameworks

// 8.1 Cyclical Pattern Tracking (Cliodynamics)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CyclicalType {
    EliteOverproduction,  // Surplus educated competing for positions
    FiscalStrain,         // State financial health
    SocialUnrest,         // Instability indicators
    PopulationPressure,   // Demographic dynamics
    Asabiyyah,            // Social cohesion (Ibn Khaldun)
    CenterPeriphery,      // Core vs. edge dynamics
}

impl CyclicalType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CyclicalType::EliteOverproduction => "elite_overproduction",
            CyclicalType::FiscalStrain => "fiscal_strain",
            CyclicalType::SocialUnrest => "social_unrest",
            CyclicalType::PopulationPressure => "population_pressure",
            CyclicalType::Asabiyyah => "asabiyyah",
            CyclicalType::CenterPeriphery => "center_periphery",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "elite_overproduction" | "elite" => Some(CyclicalType::EliteOverproduction),
            "fiscal_strain" | "fiscal" => Some(CyclicalType::FiscalStrain),
            "social_unrest" | "unrest" => Some(CyclicalType::SocialUnrest),
            "population_pressure" | "population" => Some(CyclicalType::PopulationPressure),
            "asabiyyah" | "cohesion" => Some(CyclicalType::Asabiyyah),
            "center_periphery" | "center" => Some(CyclicalType::CenterPeriphery),
            _ => None,
        }
    }

    pub fn all() -> &'static [CyclicalType] {
        &[
            CyclicalType::EliteOverproduction,
            CyclicalType::FiscalStrain,
            CyclicalType::SocialUnrest,
            CyclicalType::PopulationPressure,
            CyclicalType::Asabiyyah,
            CyclicalType::CenterPeriphery,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclicalIndicator {
    pub id: i64,
    pub video_id: String,
    pub claim_id: Option<i64>,
    pub indicator_type: CyclicalType,
    pub entity: String,              // Civilization/state being described
    pub era_id: Option<i64>,
    pub description: String,
    pub timestamp: Option<f64>,
    pub created_at: DateTime<Utc>,
}

// 8.2 Causal Chain Tracking

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopType {
    Positive,   // Amplifying feedback
    Negative,   // Dampening feedback
    Linear,     // Simple cause-effect
}

impl LoopType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoopType::Positive => "positive",
            LoopType::Negative => "negative",
            LoopType::Linear => "linear",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "positive" | "amplifying" | "+" => Some(LoopType::Positive),
            "negative" | "dampening" | "-" => Some(LoopType::Negative),
            "linear" | "simple" => Some(LoopType::Linear),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationStrength {
    Strong,
    Moderate,
    Weak,
    Speculative,
}

impl RelationStrength {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationStrength::Strong => "strong",
            RelationStrength::Moderate => "moderate",
            RelationStrength::Weak => "weak",
            RelationStrength::Speculative => "speculative",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "strong" => Some(RelationStrength::Strong),
            "moderate" | "medium" => Some(RelationStrength::Moderate),
            "weak" => Some(RelationStrength::Weak),
            "speculative" | "uncertain" => Some(RelationStrength::Speculative),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalRelation {
    pub id: i64,
    pub cause_claim_id: i64,
    pub effect_claim_id: i64,
    pub loop_type: LoopType,
    pub strength: RelationStrength,
    pub video_id: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// 8.3 Memetic Transmission Tracking (Boyd/Richerson dual inheritance)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransmissionType {
    Horizontal,  // Peer-to-peer, same generation
    Vertical,    // Parent-to-child
    Oblique,     // Institutions to individuals
}

impl TransmissionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransmissionType::Horizontal => "horizontal",
            TransmissionType::Vertical => "vertical",
            TransmissionType::Oblique => "oblique",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "horizontal" | "peer" => Some(TransmissionType::Horizontal),
            "vertical" | "parent" => Some(TransmissionType::Vertical),
            "oblique" | "institutional" => Some(TransmissionType::Oblique),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaTransmission {
    pub id: i64,
    pub idea: String,
    pub source_entity: String,
    pub target_entity: String,
    pub transmission_type: TransmissionType,
    pub era_id: Option<i64>,
    pub region_id: Option<i64>,
    pub video_id: String,
    pub claim_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// 8.4 Geopolitical Dynamics (World-Systems)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemPosition {
    Core,          // Dominant, extracts surplus
    SemiPeriphery, // Intermediate position
    Periphery,     // Subordinate, surplus extracted
}

impl SystemPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemPosition::Core => "core",
            SystemPosition::SemiPeriphery => "semi_periphery",
            SystemPosition::Periphery => "periphery",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "core" => Some(SystemPosition::Core),
            "semi_periphery" | "semiperiphery" | "semi" => Some(SystemPosition::SemiPeriphery),
            "periphery" => Some(SystemPosition::Periphery),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeopoliticalEntity {
    pub id: i64,
    pub name: String,
    pub era_id: i64,
    pub position: SystemPosition,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurplusFlow {
    pub id: i64,
    pub from_entity_id: i64,
    pub to_entity_id: i64,
    pub commodity: String,
    pub era_id: i64,
    pub video_id: Option<String>,
    pub claim_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Braudel's timescales for temporal observation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BraudelTimescale {
    Event,       // Short-term events
    Conjuncture, // Medium cycles (decades)
    LongueDuree, // Long-term structural patterns
}

impl BraudelTimescale {
    pub fn as_str(&self) -> &'static str {
        match self {
            BraudelTimescale::Event => "event",
            BraudelTimescale::Conjuncture => "conjuncture",
            BraudelTimescale::LongueDuree => "longue_duree",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "event" | "short" => Some(BraudelTimescale::Event),
            "conjuncture" | "medium" | "cycle" => Some(BraudelTimescale::Conjuncture),
            "longue_duree" | "longueduree" | "long" | "structural" => Some(BraudelTimescale::LongueDuree),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalObservation {
    pub id: i64,
    pub claim_id: i64,
    pub timescale: BraudelTimescale,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Framework statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkStats {
    pub cyclical_indicators: i64,
    pub causal_relations: i64,
    pub idea_transmissions: i64,
    pub geopolitical_entities: i64,
    pub surplus_flows: i64,
    pub temporal_observations: i64,
}

// Phase 9: Synthesis & Pattern Detection

// 9.1 Maps of Content

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapOfContent {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MocClaim {
    pub moc_id: i64,
    pub claim_id: i64,
    pub sort_order: i32,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MocWithClaims {
    pub moc: MapOfContent,
    pub claims: Vec<Claim>,
    pub sub_mocs: Vec<MapOfContent>,
}

// 9.2 Research Questions

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionStatus {
    Active,
    Answered,
    Refined,   // Superseded by better question
    Parked,
}

impl QuestionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuestionStatus::Active => "active",
            QuestionStatus::Answered => "answered",
            QuestionStatus::Refined => "refined",
            QuestionStatus::Parked => "parked",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(QuestionStatus::Active),
            "answered" => Some(QuestionStatus::Answered),
            "refined" => Some(QuestionStatus::Refined),
            "parked" => Some(QuestionStatus::Parked),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchQuestion {
    pub id: i64,
    pub question: String,
    pub status: QuestionStatus,
    pub parent_question_id: Option<i64>,  // For sub-questions
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionEvidence {
    pub question_id: i64,
    pub claim_id: Option<i64>,
    pub video_id: Option<String>,
    pub relevance: Option<String>,  // How it relates to the question
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionWithEvidence {
    pub question: ResearchQuestion,
    pub claims: Vec<Claim>,
    pub videos: Vec<Video>,
    pub sub_questions: Vec<ResearchQuestion>,
}

// 9.3 Pattern Detection Results

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    pub id: i64,
    pub pattern_type: PatternType,
    pub description: String,
    pub video_ids: Vec<String>,
    pub claim_ids: Vec<i64>,
    pub confidence: f32,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    RecurringTheme,
    Contradiction,
    Consensus,
    Evolution,      // Idea evolving over time
    Parallel,       // Similar events in different contexts
}

impl PatternType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PatternType::RecurringTheme => "recurring_theme",
            PatternType::Contradiction => "contradiction",
            PatternType::Consensus => "consensus",
            PatternType::Evolution => "evolution",
            PatternType::Parallel => "parallel",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "recurring_theme" | "theme" | "recurring" => Some(PatternType::RecurringTheme),
            "contradiction" => Some(PatternType::Contradiction),
            "consensus" => Some(PatternType::Consensus),
            "evolution" => Some(PatternType::Evolution),
            "parallel" => Some(PatternType::Parallel),
            _ => None,
        }
    }
}

// 9.4 Review Queue

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewQueue {
    pub stale_claims: Vec<Claim>,        // Not accessed in 30+ days
    pub orphan_claims: Vec<Claim>,       // <2 connections
    pub random_suggestions: Vec<Claim>,  // For serendipitous review
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimAccess {
    pub claim_id: i64,
    pub last_accessed: DateTime<Utc>,
}

// 9.5 LLM Configuration (for future use)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProvider {
    Anthropic,
    OpenAI,
    Local,
}

impl LLMProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            LLMProvider::Anthropic => "anthropic",
            LLMProvider::OpenAI => "openai",
            LLMProvider::Local => "local",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "anthropic" | "claude" => Some(LLMProvider::Anthropic),
            "openai" | "gpt" => Some(LLMProvider::OpenAI),
            "local" | "ollama" => Some(LLMProvider::Local),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub model: String,
    pub temperature: f32,
}

// Synthesis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisStats {
    pub mocs: i64,
    pub research_questions: i64,
    pub active_questions: i64,
    pub detected_patterns: i64,
    pub stale_claims: i64,
    pub orphan_claims: i64,
}

// Phase 10: AI Processing Queue

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingStatus {
    Pending,     // Awaiting processing
    InProgress,  // Currently being processed
    Completed,   // Successfully processed
    Failed,      // Processing failed
    Skipped,     // Manually skipped
}

impl ProcessingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProcessingStatus::Pending => "pending",
            ProcessingStatus::InProgress => "in_progress",
            ProcessingStatus::Completed => "completed",
            ProcessingStatus::Failed => "failed",
            ProcessingStatus::Skipped => "skipped",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(ProcessingStatus::Pending),
            "in_progress" | "inprogress" | "processing" => Some(ProcessingStatus::InProgress),
            "completed" | "done" => Some(ProcessingStatus::Completed),
            "failed" | "error" => Some(ProcessingStatus::Failed),
            "skipped" | "skip" => Some(ProcessingStatus::Skipped),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProcessingQueue {
    pub id: i64,
    pub video_id: String,
    pub status: ProcessingStatus,
    pub priority: i32,                        // Higher = process first
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,        // If processing failed
    pub claims_extracted: i32,                // Count of claims added
}
