use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;
use super::models::{Video, Transcript, TranscriptSegment, SearchResult, SegmentMatch, Era, Region, Topic, Collection, Note, Location, MapPin, AutoTags, SavedSearch, AdvancedSearchResult, ReportEntry, GeoJsonFeature, GeoJsonGeometry, GeoJsonProperties, GeoJsonCollection, Claim, ClaimCategory, Confidence, ClaimLink, LinkType, ClaimWithLinks, TranscriptLayer, TranscriptChunk, Embedding, EmbeddingSource, SimilarityResult, HybridSearchResult, ChunkMatch, EmbeddingStats, CyclicalType, CyclicalIndicator, LoopType, RelationStrength, CausalRelation, TransmissionType, IdeaTransmission, SystemPosition, GeopoliticalEntity, SurplusFlow, BraudelTimescale, TemporalObservation, FrameworkStats, MapOfContent, MocWithClaims, QuestionStatus, ResearchQuestion, QuestionWithEvidence, DetectedPattern, PatternType, ReviewQueue, SynthesisStats};
use chrono::{DateTime, NaiveDate, Utc};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS videos (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                channel TEXT,
                upload_date TEXT,
                description TEXT,
                added_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS transcripts (
                id INTEGER PRIMARY KEY,
                video_id TEXT NOT NULL REFERENCES videos(id),
                language TEXT NOT NULL,
                full_text TEXT NOT NULL,
                segments_json TEXT NOT NULL,
                UNIQUE(video_id, language)
            );

            CREATE INDEX IF NOT EXISTS idx_transcripts_video ON transcripts(video_id);

            CREATE TABLE IF NOT EXISTS eras (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                sort_order INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS regions (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                parent_id INTEGER REFERENCES regions(id)
            );

            CREATE TABLE IF NOT EXISTS video_eras (
                video_id TEXT NOT NULL REFERENCES videos(id),
                era_id INTEGER NOT NULL REFERENCES eras(id),
                PRIMARY KEY (video_id, era_id)
            );

            CREATE TABLE IF NOT EXISTS video_regions (
                video_id TEXT NOT NULL REFERENCES videos(id),
                region_id INTEGER NOT NULL REFERENCES regions(id),
                PRIMARY KEY (video_id, region_id)
            );

            CREATE INDEX IF NOT EXISTS idx_video_eras_era ON video_eras(era_id);
            CREATE INDEX IF NOT EXISTS idx_video_regions_region ON video_regions(region_id);

            CREATE TABLE IF NOT EXISTS topics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS video_topics (
                video_id TEXT NOT NULL REFERENCES videos(id),
                topic_id INTEGER NOT NULL REFERENCES topics(id),
                PRIMARY KEY (video_id, topic_id)
            );

            CREATE INDEX IF NOT EXISTS idx_video_topics_topic ON video_topics(topic_id);

            CREATE TABLE IF NOT EXISTS collections (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT
            );

            CREATE TABLE IF NOT EXISTS video_collections (
                video_id TEXT NOT NULL REFERENCES videos(id),
                collection_id INTEGER NOT NULL REFERENCES collections(id),
                PRIMARY KEY (video_id, collection_id)
            );

            CREATE INDEX IF NOT EXISTS idx_video_collections_collection ON video_collections(collection_id);

            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY,
                video_id TEXT NOT NULL REFERENCES videos(id),
                timestamp REAL,
                text TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_notes_video ON notes(video_id);

            CREATE TABLE IF NOT EXISTS locations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                lat REAL NOT NULL,
                lon REAL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS video_locations (
                id INTEGER PRIMARY KEY,
                video_id TEXT NOT NULL REFERENCES videos(id),
                location_id INTEGER NOT NULL REFERENCES locations(id),
                era_id INTEGER REFERENCES eras(id),
                topic_id INTEGER REFERENCES topics(id),
                timestamp REAL,
                note TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_video_locations_video ON video_locations(video_id);
            CREATE INDEX IF NOT EXISTS idx_video_locations_location ON video_locations(location_id);
            CREATE INDEX IF NOT EXISTS idx_video_locations_era ON video_locations(era_id);
            CREATE INDEX IF NOT EXISTS idx_video_locations_topic ON video_locations(topic_id);

            CREATE TABLE IF NOT EXISTS saved_searches (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                query TEXT,
                era TEXT,
                region TEXT,
                topic TEXT,
                created_at TEXT NOT NULL
            );

            -- Phase 6: Claim Extraction & Atomic Notes

            CREATE TABLE IF NOT EXISTS claims (
                id INTEGER PRIMARY KEY,
                text TEXT NOT NULL,
                video_id TEXT NOT NULL REFERENCES videos(id),
                timestamp REAL,
                source_quote TEXT NOT NULL,
                category TEXT NOT NULL,
                confidence TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_claims_video ON claims(video_id);
            CREATE INDEX IF NOT EXISTS idx_claims_category ON claims(category);

            CREATE TABLE IF NOT EXISTS claim_links (
                id INTEGER PRIMARY KEY,
                source_claim_id INTEGER NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
                target_claim_id INTEGER NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
                link_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(source_claim_id, target_claim_id, link_type)
            );

            CREATE INDEX IF NOT EXISTS idx_claim_links_source ON claim_links(source_claim_id);
            CREATE INDEX IF NOT EXISTS idx_claim_links_target ON claim_links(target_claim_id);

            CREATE TABLE IF NOT EXISTS transcript_layers (
                id INTEGER PRIMARY KEY,
                video_id TEXT NOT NULL REFERENCES videos(id),
                layer INTEGER NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(video_id, layer)
            );

            CREATE INDEX IF NOT EXISTS idx_transcript_layers_video ON transcript_layers(video_id);

            CREATE TABLE IF NOT EXISTS transcript_chunks (
                id INTEGER PRIMARY KEY,
                video_id TEXT NOT NULL REFERENCES videos(id),
                chunk_index INTEGER NOT NULL,
                start_time REAL NOT NULL,
                end_time REAL NOT NULL,
                text TEXT NOT NULL,
                token_count INTEGER NOT NULL,
                overlap_with_previous INTEGER NOT NULL DEFAULT 0,
                UNIQUE(video_id, chunk_index)
            );

            CREATE INDEX IF NOT EXISTS idx_transcript_chunks_video ON transcript_chunks(video_id);

            -- Phase 7: Semantic Search & Embeddings

            CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY,
                source_type TEXT NOT NULL,
                source_id TEXT NOT NULL,
                model TEXT NOT NULL,
                vector_json TEXT NOT NULL,
                dimensions INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(source_type, source_id, model)
            );

            CREATE INDEX IF NOT EXISTS idx_embeddings_source ON embeddings(source_type, source_id);
            CREATE INDEX IF NOT EXISTS idx_embeddings_model ON embeddings(model);

            -- Phase 8: Analytical Frameworks

            -- 8.1 Cyclical Pattern Tracking (Cliodynamics)
            CREATE TABLE IF NOT EXISTS cyclical_indicators (
                id INTEGER PRIMARY KEY,
                video_id TEXT NOT NULL REFERENCES videos(id),
                claim_id INTEGER REFERENCES claims(id),
                indicator_type TEXT NOT NULL,
                entity TEXT NOT NULL,
                era_id INTEGER REFERENCES eras(id),
                description TEXT NOT NULL,
                timestamp REAL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_cyclical_indicators_video ON cyclical_indicators(video_id);
            CREATE INDEX IF NOT EXISTS idx_cyclical_indicators_type ON cyclical_indicators(indicator_type);
            CREATE INDEX IF NOT EXISTS idx_cyclical_indicators_entity ON cyclical_indicators(entity);

            -- 8.2 Causal Chain Tracking
            CREATE TABLE IF NOT EXISTS causal_relations (
                id INTEGER PRIMARY KEY,
                cause_claim_id INTEGER NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
                effect_claim_id INTEGER NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
                loop_type TEXT NOT NULL,
                strength TEXT NOT NULL,
                video_id TEXT NOT NULL REFERENCES videos(id),
                notes TEXT,
                created_at TEXT NOT NULL,
                UNIQUE(cause_claim_id, effect_claim_id)
            );

            CREATE INDEX IF NOT EXISTS idx_causal_relations_cause ON causal_relations(cause_claim_id);
            CREATE INDEX IF NOT EXISTS idx_causal_relations_effect ON causal_relations(effect_claim_id);
            CREATE INDEX IF NOT EXISTS idx_causal_relations_loop ON causal_relations(loop_type);

            -- 8.3 Memetic Transmission Tracking
            CREATE TABLE IF NOT EXISTS idea_transmissions (
                id INTEGER PRIMARY KEY,
                idea TEXT NOT NULL,
                source_entity TEXT NOT NULL,
                target_entity TEXT NOT NULL,
                transmission_type TEXT NOT NULL,
                era_id INTEGER REFERENCES eras(id),
                region_id INTEGER REFERENCES regions(id),
                video_id TEXT NOT NULL REFERENCES videos(id),
                claim_id INTEGER REFERENCES claims(id),
                notes TEXT,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_idea_transmissions_video ON idea_transmissions(video_id);
            CREATE INDEX IF NOT EXISTS idx_idea_transmissions_type ON idea_transmissions(transmission_type);
            CREATE INDEX IF NOT EXISTS idx_idea_transmissions_idea ON idea_transmissions(idea);

            -- 8.4 Geopolitical Dynamics (World-Systems)
            CREATE TABLE IF NOT EXISTS geopolitical_entities (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                era_id INTEGER NOT NULL REFERENCES eras(id),
                position TEXT NOT NULL,
                notes TEXT,
                created_at TEXT NOT NULL,
                UNIQUE(name, era_id)
            );

            CREATE INDEX IF NOT EXISTS idx_geopolitical_entities_era ON geopolitical_entities(era_id);
            CREATE INDEX IF NOT EXISTS idx_geopolitical_entities_position ON geopolitical_entities(position);

            CREATE TABLE IF NOT EXISTS surplus_flows (
                id INTEGER PRIMARY KEY,
                from_entity_id INTEGER NOT NULL REFERENCES geopolitical_entities(id) ON DELETE CASCADE,
                to_entity_id INTEGER NOT NULL REFERENCES geopolitical_entities(id) ON DELETE CASCADE,
                commodity TEXT NOT NULL,
                era_id INTEGER NOT NULL REFERENCES eras(id),
                video_id TEXT REFERENCES videos(id),
                claim_id INTEGER REFERENCES claims(id),
                notes TEXT,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_surplus_flows_from ON surplus_flows(from_entity_id);
            CREATE INDEX IF NOT EXISTS idx_surplus_flows_to ON surplus_flows(to_entity_id);
            CREATE INDEX IF NOT EXISTS idx_surplus_flows_era ON surplus_flows(era_id);

            -- Braudel's Timescales
            CREATE TABLE IF NOT EXISTS temporal_observations (
                id INTEGER PRIMARY KEY,
                claim_id INTEGER NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
                timescale TEXT NOT NULL,
                notes TEXT,
                created_at TEXT NOT NULL,
                UNIQUE(claim_id, timescale)
            );

            CREATE INDEX IF NOT EXISTS idx_temporal_observations_claim ON temporal_observations(claim_id);
            CREATE INDEX IF NOT EXISTS idx_temporal_observations_timescale ON temporal_observations(timescale);

            -- Phase 9: Synthesis & Pattern Detection

            -- 9.1 Maps of Content
            CREATE TABLE IF NOT EXISTS mocs (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS moc_claims (
                moc_id INTEGER NOT NULL REFERENCES mocs(id) ON DELETE CASCADE,
                claim_id INTEGER NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
                sort_order INTEGER NOT NULL DEFAULT 0,
                added_at TEXT NOT NULL,
                PRIMARY KEY (moc_id, claim_id)
            );

            CREATE TABLE IF NOT EXISTS moc_hierarchy (
                parent_moc_id INTEGER NOT NULL REFERENCES mocs(id) ON DELETE CASCADE,
                child_moc_id INTEGER NOT NULL REFERENCES mocs(id) ON DELETE CASCADE,
                PRIMARY KEY (parent_moc_id, child_moc_id)
            );

            CREATE INDEX IF NOT EXISTS idx_moc_claims_moc ON moc_claims(moc_id);
            CREATE INDEX IF NOT EXISTS idx_moc_claims_claim ON moc_claims(claim_id);

            -- 9.2 Research Questions
            CREATE TABLE IF NOT EXISTS research_questions (
                id INTEGER PRIMARY KEY,
                question TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active',
                parent_question_id INTEGER REFERENCES research_questions(id),
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS question_evidence (
                id INTEGER PRIMARY KEY,
                question_id INTEGER NOT NULL REFERENCES research_questions(id) ON DELETE CASCADE,
                claim_id INTEGER REFERENCES claims(id) ON DELETE CASCADE,
                video_id TEXT REFERENCES videos(id),
                relevance TEXT,
                added_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_research_questions_status ON research_questions(status);
            CREATE INDEX IF NOT EXISTS idx_research_questions_parent ON research_questions(parent_question_id);
            CREATE INDEX IF NOT EXISTS idx_question_evidence_question ON question_evidence(question_id);
            CREATE INDEX IF NOT EXISTS idx_question_evidence_claim ON question_evidence(claim_id);

            -- 9.3 Pattern Detection
            CREATE TABLE IF NOT EXISTS detected_patterns (
                id INTEGER PRIMARY KEY,
                pattern_type TEXT NOT NULL,
                description TEXT NOT NULL,
                video_ids_json TEXT NOT NULL,
                claim_ids_json TEXT NOT NULL,
                confidence REAL NOT NULL,
                detected_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_detected_patterns_type ON detected_patterns(pattern_type);

            -- 9.4 Claim Access Tracking (for review system)
            CREATE TABLE IF NOT EXISTS claim_access (
                claim_id INTEGER PRIMARY KEY REFERENCES claims(id) ON DELETE CASCADE,
                last_accessed TEXT NOT NULL
            );
            "#,
        )?;

        self.seed_default_eras()?;

        // Create unified search index FTS table
        let search_fts_exists: bool = self.conn.query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='search_index'",
            [],
            |row| row.get(0),
        )?;

        if !search_fts_exists {
            self.conn.execute_batch(
                r#"
                CREATE VIRTUAL TABLE search_index USING fts5(
                    video_id,
                    title,
                    description,
                    transcript,
                    tokenize='porter'
                );
                "#,
            )?;
        }

        // Legacy FTS table for backwards compatibility
        let fts_exists: bool = self.conn.query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='transcript_fts'",
            [],
            |row| row.get(0),
        )?;

        if !fts_exists {
            self.conn.execute_batch(
                r#"
                CREATE VIRTUAL TABLE transcript_fts USING fts5(
                    video_id,
                    full_text,
                    content='transcripts',
                    content_rowid='id'
                );

                CREATE TRIGGER transcripts_ai AFTER INSERT ON transcripts BEGIN
                    INSERT INTO transcript_fts(rowid, video_id, full_text)
                    VALUES (new.id, new.video_id, new.full_text);
                END;

                CREATE TRIGGER transcripts_ad AFTER DELETE ON transcripts BEGIN
                    INSERT INTO transcript_fts(transcript_fts, rowid, video_id, full_text)
                    VALUES('delete', old.id, old.video_id, old.full_text);
                END;

                CREATE TRIGGER transcripts_au AFTER UPDATE ON transcripts BEGIN
                    INSERT INTO transcript_fts(transcript_fts, rowid, video_id, full_text)
                    VALUES('delete', old.id, old.video_id, old.full_text);
                    INSERT INTO transcript_fts(rowid, video_id, full_text)
                    VALUES (new.id, new.video_id, new.full_text);
                END;
                "#,
            )?;
        }

        Ok(())
    }

    pub fn insert_video(&self, video: &Video) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO videos (id, url, title, channel, upload_date, description, added_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                video.id,
                video.url,
                video.title,
                video.channel,
                video.upload_date.map(|d| d.format("%Y-%m-%d").to_string()),
                video.description,
                video.added_at.to_rfc3339(),
            ],
        )?;
        self.update_search_index(&video.id)?;
        Ok(())
    }

    pub fn insert_transcript(&self, transcript: &Transcript) -> Result<()> {
        let segments_json = serde_json::to_string(&transcript.segments)?;
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO transcripts (video_id, language, full_text, segments_json)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![
                transcript.video_id,
                transcript.language,
                transcript.full_text,
                segments_json,
            ],
        )?;
        self.update_search_index(&transcript.video_id)?;
        Ok(())
    }

    fn update_search_index(&self, video_id: &str) -> Result<()> {
        // Get video info
        let video = match self.get_video(video_id)? {
            Some(v) => v,
            None => return Ok(()),
        };

        // Get transcript if exists
        let transcript_text = self.get_transcript(video_id)?
            .map(|t| t.full_text)
            .unwrap_or_default();

        // Delete existing entry
        self.conn.execute(
            "DELETE FROM search_index WHERE video_id = ?1",
            params![video_id],
        )?;

        // Insert updated entry
        self.conn.execute(
            "INSERT INTO search_index (video_id, title, description, transcript) VALUES (?1, ?2, ?3, ?4)",
            params![
                video_id,
                video.title,
                video.description.unwrap_or_default(),
                transcript_text,
            ],
        )?;

        Ok(())
    }

    pub fn rebuild_search_index(&self) -> Result<usize> {
        // Clear existing index
        self.conn.execute("DELETE FROM search_index", [])?;

        // Get all videos
        let videos = self.list_videos()?;
        let count = videos.len();

        for video in videos {
            let transcript_text = self.get_transcript(&video.id)?
                .map(|t| t.full_text)
                .unwrap_or_default();

            self.conn.execute(
                "INSERT INTO search_index (video_id, title, description, transcript) VALUES (?1, ?2, ?3, ?4)",
                params![
                    video.id,
                    video.title,
                    video.description.unwrap_or_default(),
                    transcript_text,
                ],
            )?;
        }

        Ok(count)
    }

    pub fn get_video(&self, id: &str) -> Result<Option<Video>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, channel, upload_date, description, added_at FROM videos WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_video(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_transcript(&self, video_id: &str) -> Result<Option<Transcript>> {
        let mut stmt = self.conn.prepare(
            "SELECT video_id, language, full_text, segments_json FROM transcripts WHERE video_id = ?1"
        )?;

        let mut rows = stmt.query(params![video_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_transcript(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_videos(&self) -> Result<Vec<Video>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, channel, upload_date, description, added_at FROM videos ORDER BY added_at DESC"
        )?;

        let mut videos = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            videos.push(self.row_to_video(row)?);
        }

        Ok(videos)
    }

    pub fn search(&self, query: &str) -> Result<Vec<(Video, String)>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT v.id, v.url, v.title, v.channel, v.upload_date, v.description, v.added_at,
                   snippet(transcript_fts, 1, '>>>', '<<<', '...', 32) as snippet
            FROM transcript_fts
            JOIN videos v ON v.id = transcript_fts.video_id
            WHERE transcript_fts MATCH ?1
            ORDER BY rank
            "#
        )?;

        let mut results = Vec::new();
        let mut rows = stmt.query(params![query])?;

        while let Some(row) = rows.next()? {
            let video = self.row_to_video(row)?;
            let snippet: String = row.get(7)?;
            results.push((video, snippet));
        }

        Ok(results)
    }

    pub fn search_with_timestamps(&self, query: &str) -> Result<Vec<SearchResult>> {
        // Use weighted search: title (10x), description (5x), transcript (1x)
        // bm25() returns negative scores, lower is better
        let mut stmt = self.conn.prepare(
            r#"
            SELECT v.id, v.url, v.title, v.channel, v.upload_date, v.description, v.added_at,
                   t.segments_json,
                   bm25(search_index, 0.0, 10.0, 5.0, 1.0) as rank
            FROM search_index
            JOIN videos v ON v.id = search_index.video_id
            LEFT JOIN transcripts t ON t.video_id = v.id
            WHERE search_index MATCH ?1
            ORDER BY rank
            "#
        )?;

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        let mut rows = stmt.query(params![query])?;

        while let Some(row) = rows.next()? {
            let video = self.row_to_video(row)?;
            let segments_json: Option<String> = row.get(7)?;

            // Find segments containing the query (if transcript exists)
            let mut matches = Vec::new();
            if let Some(json) = segments_json {
                let segments: Vec<TranscriptSegment> = serde_json::from_str(&json)?;
                for seg in &segments {
                    if seg.text.to_lowercase().contains(&query_lower) {
                        matches.push(SegmentMatch {
                            start_time: seg.start_time,
                            duration: seg.duration,
                            text: seg.text.clone(),
                        });
                    }
                }
            }

            // Include video even if no transcript matches (title/description matched)
            results.push(SearchResult { video, matches });
        }

        Ok(results)
    }

    fn row_to_video(&self, row: &rusqlite::Row) -> Result<Video> {
        let upload_date: Option<String> = row.get(4)?;
        let added_at: String = row.get(6)?;

        Ok(Video {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            channel: row.get(3)?,
            upload_date: upload_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            description: row.get(5)?,
            added_at: DateTime::parse_from_rfc3339(&added_at)?.with_timezone(&Utc),
        })
    }

    fn row_to_transcript(&self, row: &rusqlite::Row) -> Result<Transcript> {
        let segments_json: String = row.get(3)?;
        let segments: Vec<TranscriptSegment> = serde_json::from_str(&segments_json)?;

        Ok(Transcript {
            video_id: row.get(0)?,
            language: row.get(1)?,
            full_text: row.get(2)?,
            segments,
        })
    }

    // Era operations

    fn seed_default_eras(&self) -> Result<()> {
        let default_eras = [
            ("Prehistoric", 0),
            ("Bronze Age", 10),
            ("Iron Age", 20),
            ("Classical Antiquity", 30),
            ("Late Antiquity", 40),
            ("Medieval", 50),
            ("Early Modern", 60),
            ("Modern", 70),
        ];

        for (name, order) in default_eras {
            self.conn.execute(
                "INSERT OR IGNORE INTO eras (name, sort_order) VALUES (?1, ?2)",
                params![name, order],
            )?;
        }
        Ok(())
    }

    pub fn list_eras(&self) -> Result<Vec<Era>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, sort_order FROM eras ORDER BY sort_order"
        )?;

        let mut eras = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            eras.push(Era {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
            });
        }
        Ok(eras)
    }

    pub fn get_era_by_name(&self, name: &str) -> Result<Option<Era>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, sort_order FROM eras WHERE name = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Era {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_era(&self, id: i64) -> Result<Option<Era>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, sort_order FROM eras WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Era {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_era(&self, name: &str, sort_order: i32) -> Result<Era> {
        self.conn.execute(
            "INSERT INTO eras (name, sort_order) VALUES (?1, ?2)",
            params![name, sort_order],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Era { id, name: name.to_string(), sort_order })
    }

    // Region operations

    pub fn list_regions(&self) -> Result<Vec<Region>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, parent_id FROM regions ORDER BY name"
        )?;

        let mut regions = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            regions.push(Region {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
            });
        }
        Ok(regions)
    }

    pub fn get_region_by_name(&self, name: &str) -> Result<Option<Region>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, parent_id FROM regions WHERE name = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Region {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_region(&self, name: &str, parent_id: Option<i64>) -> Result<Region> {
        self.conn.execute(
            "INSERT INTO regions (name, parent_id) VALUES (?1, ?2)",
            params![name, parent_id],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Region { id, name: name.to_string(), parent_id })
    }

    // Video tagging

    pub fn tag_video_era(&self, video_id: &str, era_id: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO video_eras (video_id, era_id) VALUES (?1, ?2)",
            params![video_id, era_id],
        )?;
        Ok(())
    }

    pub fn tag_video_region(&self, video_id: &str, region_id: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO video_regions (video_id, region_id) VALUES (?1, ?2)",
            params![video_id, region_id],
        )?;
        Ok(())
    }

    pub fn get_video_eras(&self, video_id: &str) -> Result<Vec<Era>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT e.id, e.name, e.sort_order
            FROM eras e
            JOIN video_eras ve ON ve.era_id = e.id
            WHERE ve.video_id = ?1
            ORDER BY e.sort_order
            "#
        )?;

        let mut eras = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            eras.push(Era {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
            });
        }
        Ok(eras)
    }

    pub fn get_video_regions(&self, video_id: &str) -> Result<Vec<Region>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT r.id, r.name, r.parent_id
            FROM regions r
            JOIN video_regions vr ON vr.region_id = r.id
            WHERE vr.video_id = ?1
            ORDER BY r.name
            "#
        )?;

        let mut regions = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            regions.push(Region {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
            });
        }
        Ok(regions)
    }

    pub fn browse_videos(&self, era: Option<&str>, region: Option<&str>) -> Result<Vec<Video>> {
        let mut query = String::from(
            "SELECT DISTINCT v.id, v.url, v.title, v.channel, v.upload_date, v.description, v.added_at FROM videos v"
        );
        let mut conditions: Vec<&str> = Vec::new();
        let mut joins = Vec::new();

        if era.is_some() {
            joins.push("JOIN video_eras ve ON ve.video_id = v.id JOIN eras e ON e.id = ve.era_id");
            conditions.push("e.name = ?1 COLLATE NOCASE");
        }

        if region.is_some() {
            joins.push("JOIN video_regions vr ON vr.video_id = v.id JOIN regions r ON r.id = vr.region_id");
            if era.is_some() {
                conditions.push("r.name = ?2 COLLATE NOCASE");
            } else {
                conditions.push("r.name = ?1 COLLATE NOCASE");
            }
        }

        for join in &joins {
            query.push(' ');
            query.push_str(join);
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY v.added_at DESC");

        let mut stmt = self.conn.prepare(&query)?;
        let mut videos = Vec::new();

        let mut rows = match (era, region) {
            (Some(e), Some(r)) => stmt.query(params![e, r])?,
            (Some(e), None) => stmt.query(params![e])?,
            (None, Some(r)) => stmt.query(params![r])?,
            (None, None) => stmt.query([])?,
        };

        while let Some(row) = rows.next()? {
            videos.push(self.row_to_video(row)?);
        }

        Ok(videos)
    }

    // Topic operations

    pub fn list_topics(&self) -> Result<Vec<Topic>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM topics ORDER BY name")?;
        let mut topics = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            topics.push(Topic {
                id: row.get(0)?,
                name: row.get(1)?,
            });
        }
        Ok(topics)
    }

    pub fn get_topic_by_name(&self, name: &str) -> Result<Option<Topic>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name FROM topics WHERE name = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Topic {
                id: row.get(0)?,
                name: row.get(1)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_topic(&self, name: &str) -> Result<Topic> {
        self.conn.execute("INSERT INTO topics (name) VALUES (?1)", params![name])?;
        let id = self.conn.last_insert_rowid();
        Ok(Topic { id, name: name.to_string() })
    }

    pub fn get_or_create_topic(&self, name: &str) -> Result<Topic> {
        if let Some(topic) = self.get_topic_by_name(name)? {
            Ok(topic)
        } else {
            self.create_topic(name)
        }
    }

    pub fn tag_video_topic(&self, video_id: &str, topic_id: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO video_topics (video_id, topic_id) VALUES (?1, ?2)",
            params![video_id, topic_id],
        )?;
        Ok(())
    }

    pub fn get_video_topics(&self, video_id: &str) -> Result<Vec<Topic>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT t.id, t.name
            FROM topics t
            JOIN video_topics vt ON vt.topic_id = t.id
            WHERE vt.video_id = ?1
            ORDER BY t.name
            "#
        )?;

        let mut topics = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            topics.push(Topic {
                id: row.get(0)?,
                name: row.get(1)?,
            });
        }
        Ok(topics)
    }

    pub fn browse_by_topic(&self, topic_name: &str) -> Result<Vec<Video>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT v.id, v.url, v.title, v.channel, v.upload_date, v.description, v.added_at
            FROM videos v
            JOIN video_topics vt ON vt.video_id = v.id
            JOIN topics t ON t.id = vt.topic_id
            WHERE t.name = ?1 COLLATE NOCASE
            ORDER BY v.added_at DESC
            "#
        )?;

        let mut videos = Vec::new();
        let mut rows = stmt.query(params![topic_name])?;

        while let Some(row) = rows.next()? {
            videos.push(self.row_to_video(row)?);
        }
        Ok(videos)
    }

    // Collection operations

    pub fn list_collections(&self) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.prepare("SELECT id, name, description FROM collections ORDER BY name")?;
        let mut collections = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            collections.push(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            });
        }
        Ok(collections)
    }

    pub fn get_collection_by_name(&self, name: &str) -> Result<Option<Collection>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description FROM collections WHERE name = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_collection(&self, name: &str, description: Option<&str>) -> Result<Collection> {
        self.conn.execute(
            "INSERT INTO collections (name, description) VALUES (?1, ?2)",
            params![name, description],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Collection { id, name: name.to_string(), description: description.map(|s| s.to_string()) })
    }

    pub fn add_video_to_collection(&self, video_id: &str, collection_id: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO video_collections (video_id, collection_id) VALUES (?1, ?2)",
            params![video_id, collection_id],
        )?;
        Ok(())
    }

    pub fn get_collection_videos(&self, collection_name: &str) -> Result<Vec<Video>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT v.id, v.url, v.title, v.channel, v.upload_date, v.description, v.added_at
            FROM videos v
            JOIN video_collections vc ON vc.video_id = v.id
            JOIN collections c ON c.id = vc.collection_id
            WHERE c.name = ?1 COLLATE NOCASE
            ORDER BY v.added_at DESC
            "#
        )?;

        let mut videos = Vec::new();
        let mut rows = stmt.query(params![collection_name])?;

        while let Some(row) = rows.next()? {
            videos.push(self.row_to_video(row)?);
        }
        Ok(videos)
    }

    pub fn get_video_collections(&self, video_id: &str) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.name, c.description
            FROM collections c
            JOIN video_collections vc ON vc.collection_id = c.id
            WHERE vc.video_id = ?1
            ORDER BY c.name
            "#
        )?;

        let mut collections = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            collections.push(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            });
        }
        Ok(collections)
    }

    // Note operations

    pub fn add_note(&self, video_id: &str, timestamp: Option<f64>, text: &str) -> Result<Note> {
        let created_at = Utc::now();
        self.conn.execute(
            "INSERT INTO notes (video_id, timestamp, text, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![video_id, timestamp, text, created_at.to_rfc3339()],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Note {
            id,
            video_id: video_id.to_string(),
            timestamp,
            text: text.to_string(),
            created_at,
        })
    }

    pub fn get_video_notes(&self, video_id: &str) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, timestamp, text, created_at FROM notes WHERE video_id = ?1 ORDER BY timestamp NULLS FIRST, created_at"
        )?;

        let mut notes = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            let created_at: String = row.get(4)?;
            notes.push(Note {
                id: row.get(0)?,
                video_id: row.get(1)?,
                timestamp: row.get(2)?,
                text: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            });
        }
        Ok(notes)
    }

    pub fn delete_note(&self, note_id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM notes WHERE id = ?1", params![note_id])?;
        Ok(affected > 0)
    }

    // Location operations

    pub fn list_locations(&self) -> Result<Vec<Location>> {
        let mut stmt = self.conn.prepare("SELECT id, name, lat, lon FROM locations ORDER BY name")?;
        let mut locations = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            locations.push(Location {
                id: row.get(0)?,
                name: row.get(1)?,
                lat: row.get(2)?,
                lon: row.get(3)?,
            });
        }
        Ok(locations)
    }

    pub fn get_location_by_name(&self, name: &str) -> Result<Option<Location>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, lat, lon FROM locations WHERE name = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Location {
                id: row.get(0)?,
                name: row.get(1)?,
                lat: row.get(2)?,
                lon: row.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_location(&self, name: &str, lat: f64, lon: f64) -> Result<Location> {
        self.conn.execute(
            "INSERT INTO locations (name, lat, lon) VALUES (?1, ?2, ?3)",
            params![name, lat, lon],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Location { id, name: name.to_string(), lat, lon })
    }

    pub fn get_or_create_location(&self, name: &str, lat: f64, lon: f64) -> Result<Location> {
        if let Some(loc) = self.get_location_by_name(name)? {
            Ok(loc)
        } else {
            self.create_location(name, lat, lon)
        }
    }

    pub fn add_video_location(
        &self,
        video_id: &str,
        location_id: i64,
        era_id: Option<i64>,
        topic_id: Option<i64>,
        timestamp: Option<f64>,
        note: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO video_locations (video_id, location_id, era_id, topic_id, timestamp, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![video_id, location_id, era_id, topic_id, timestamp, note],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_map_pins(&self, era: Option<&str>, topic: Option<&str>) -> Result<Vec<MapPin>> {
        let mut query = String::from(
            r#"
            SELECT l.id, l.name, l.lat, l.lon,
                   v.id, v.title,
                   e.name, t.name,
                   vl.timestamp, vl.note
            FROM video_locations vl
            JOIN locations l ON l.id = vl.location_id
            JOIN videos v ON v.id = vl.video_id
            LEFT JOIN eras e ON e.id = vl.era_id
            LEFT JOIN topics t ON t.id = vl.topic_id
            WHERE 1=1
            "#
        );

        if era.is_some() {
            query.push_str(" AND e.name = ?1 COLLATE NOCASE");
        }
        if topic.is_some() {
            if era.is_some() {
                query.push_str(" AND t.name = ?2 COLLATE NOCASE");
            } else {
                query.push_str(" AND t.name = ?1 COLLATE NOCASE");
            }
        }

        query.push_str(" ORDER BY l.name, v.title");

        let mut stmt = self.conn.prepare(&query)?;
        let mut pins = Vec::new();

        let mut rows = match (era, topic) {
            (Some(e), Some(t)) => stmt.query(params![e, t])?,
            (Some(e), None) => stmt.query(params![e])?,
            (None, Some(t)) => stmt.query(params![t])?,
            (None, None) => stmt.query([])?,
        };

        while let Some(row) = rows.next()? {
            pins.push(MapPin {
                location: Location {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    lat: row.get(2)?,
                    lon: row.get(3)?,
                },
                video_id: row.get(4)?,
                video_title: row.get(5)?,
                era: row.get(6)?,
                topic: row.get(7)?,
                timestamp: row.get(8)?,
                note: row.get(9)?,
            });
        }

        Ok(pins)
    }

    // Auto-tagging from title and description

    pub fn suggest_tags(&self, video_id: &str) -> Result<AutoTags> {
        let video = match self.get_video(video_id)? {
            Some(v) => v,
            None => return Ok(AutoTags::default()),
        };

        let text = format!(
            "{} {}",
            video.title,
            video.description.unwrap_or_default()
        ).to_lowercase();

        let mut tags = AutoTags::default();

        // Era keywords
        let era_patterns = [
            ("prehistoric", "Prehistoric"),
            ("stone age", "Prehistoric"),
            ("bronze age", "Bronze Age"),
            ("iron age", "Iron Age"),
            ("classical", "Classical Antiquity"),
            ("ancient greece", "Classical Antiquity"),
            ("ancient rome", "Classical Antiquity"),
            ("roman empire", "Classical Antiquity"),
            ("greek", "Classical Antiquity"),
            ("late antiquity", "Late Antiquity"),
            ("late roman", "Late Antiquity"),
            ("medieval", "Medieval"),
            ("middle ages", "Medieval"),
            ("viking", "Medieval"),
            ("crusade", "Medieval"),
            ("early modern", "Early Modern"),
            ("renaissance", "Early Modern"),
            ("colonial", "Early Modern"),
            ("modern", "Modern"),
            ("20th century", "Modern"),
            ("world war", "Modern"),
        ];

        for (pattern, era) in era_patterns {
            if text.contains(pattern) && !tags.eras.contains(&era.to_string()) {
                tags.eras.push(era.to_string());
            }
        }

        // Region keywords
        let region_patterns = [
            ("mesopotamia", "Mesopotamia"),
            ("babylon", "Mesopotamia"),
            ("sumer", "Mesopotamia"),
            ("assyria", "Mesopotamia"),
            ("egypt", "Egypt"),
            ("pharaoh", "Egypt"),
            ("nile", "Egypt"),
            ("pyramid", "Egypt"),
            ("greece", "Greece"),
            ("athens", "Greece"),
            ("sparta", "Greece"),
            ("rome", "Rome"),
            ("roman", "Rome"),
            ("italy", "Rome"),
            ("persia", "Persia"),
            ("persian", "Persia"),
            ("iran", "Persia"),
            ("china", "China"),
            ("chinese", "China"),
            ("india", "India"),
            ("indian", "India"),
            ("levant", "Levant"),
            ("canaan", "Levant"),
            ("israel", "Levant"),
            ("judea", "Levant"),
            ("phoenicia", "Levant"),
            ("anatolia", "Anatolia"),
            ("turkey", "Anatolia"),
            ("hittite", "Anatolia"),
            ("britain", "Britain"),
            ("british", "Britain"),
            ("england", "Britain"),
            ("gaul", "Gaul"),
            ("france", "Gaul"),
            ("celtic", "Gaul"),
            ("scandinavia", "Scandinavia"),
            ("norse", "Scandinavia"),
            ("viking", "Scandinavia"),
            ("mesoamerica", "Mesoamerica"),
            ("maya", "Mesoamerica"),
            ("aztec", "Mesoamerica"),
            ("africa", "Africa"),
            ("african", "Africa"),
        ];

        for (pattern, region) in region_patterns {
            if text.contains(pattern) && !tags.regions.contains(&region.to_string()) {
                tags.regions.push(region.to_string());
            }
        }

        // Topic keywords
        let topic_patterns = [
            ("war", "warfare"),
            ("battle", "warfare"),
            ("military", "warfare"),
            ("army", "warfare"),
            ("conquest", "warfare"),
            ("trade", "trade"),
            ("commerce", "trade"),
            ("merchant", "trade"),
            ("economy", "trade"),
            ("religion", "religion"),
            ("temple", "religion"),
            ("god", "religion"),
            ("worship", "religion"),
            ("ritual", "religion"),
            ("philosophy", "philosophy"),
            ("philosopher", "philosophy"),
            ("thought", "philosophy"),
            ("agriculture", "agriculture"),
            ("farming", "agriculture"),
            ("crop", "agriculture"),
            ("mining", "mining"),
            ("metal", "mining"),
            ("bronze", "mining"),
            ("iron", "mining"),
            ("copper", "mining"),
            ("architecture", "architecture"),
            ("building", "architecture"),
            ("monument", "architecture"),
            ("art", "art"),
            ("sculpture", "art"),
            ("painting", "art"),
            ("politics", "politics"),
            ("government", "politics"),
            ("king", "politics"),
            ("emperor", "politics"),
            ("dynasty", "politics"),
            ("law", "law"),
            ("legal", "law"),
            ("code", "law"),
            ("science", "science"),
            ("astronomy", "science"),
            ("mathematics", "science"),
            ("medicine", "science"),
            ("technology", "technology"),
            ("invention", "technology"),
            ("tool", "technology"),
        ];

        for (pattern, topic) in topic_patterns {
            if text.contains(pattern) && !tags.topics.contains(&topic.to_string()) {
                tags.topics.push(topic.to_string());
            }
        }

        Ok(tags)
    }

    pub fn apply_auto_tags(&self, video_id: &str) -> Result<AutoTags> {
        let tags = self.suggest_tags(video_id)?;

        // Apply era tags
        for era_name in &tags.eras {
            if let Some(era) = self.get_era_by_name(era_name)? {
                self.tag_video_era(video_id, era.id)?;
            }
        }

        // Apply region tags (create if needed)
        for region_name in &tags.regions {
            let region = match self.get_region_by_name(region_name)? {
                Some(r) => r,
                None => self.create_region(region_name, None)?,
            };
            self.tag_video_region(video_id, region.id)?;
        }

        // Apply topic tags (create if needed)
        for topic_name in &tags.topics {
            let topic = self.get_or_create_topic(topic_name)?;
            self.tag_video_topic(video_id, topic.id)?;
        }

        Ok(tags)
    }

    pub fn get_video_locations(&self, video_id: &str) -> Result<Vec<MapPin>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT l.id, l.name, l.lat, l.lon,
                   v.id, v.title,
                   e.name, t.name,
                   vl.timestamp, vl.note
            FROM video_locations vl
            JOIN locations l ON l.id = vl.location_id
            JOIN videos v ON v.id = vl.video_id
            LEFT JOIN eras e ON e.id = vl.era_id
            LEFT JOIN topics t ON t.id = vl.topic_id
            WHERE vl.video_id = ?1
            ORDER BY l.name
            "#
        )?;

        let mut pins = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            pins.push(MapPin {
                location: Location {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    lat: row.get(2)?,
                    lon: row.get(3)?,
                },
                video_id: row.get(4)?,
                video_title: row.get(5)?,
                era: row.get(6)?,
                topic: row.get(7)?,
                timestamp: row.get(8)?,
                note: row.get(9)?,
            });
        }

        Ok(pins)
    }

    // Phase 5: Research Tools

    // Advanced search combining full-text with metadata filters
    pub fn advanced_search(
        &self,
        query: Option<&str>,
        era: Option<&str>,
        region: Option<&str>,
        topic: Option<&str>,
    ) -> Result<Vec<AdvancedSearchResult>> {
        // Build the query dynamically based on filters
        let has_text_query = query.is_some() && !query.unwrap().trim().is_empty();

        // If we have a text query, start with FTS results
        // Otherwise, start with all videos and filter by metadata
        let video_ids: Vec<String> = if has_text_query {
            let mut stmt = self.conn.prepare(
                r#"
                SELECT DISTINCT video_id
                FROM search_index
                WHERE search_index MATCH ?1
                "#
            )?;
            let mut ids = Vec::new();
            let mut rows = stmt.query(params![query.unwrap()])?;
            while let Some(row) = rows.next()? {
                ids.push(row.get(0)?);
            }
            ids
        } else {
            let mut stmt = self.conn.prepare("SELECT id FROM videos")?;
            let mut ids = Vec::new();
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                ids.push(row.get(0)?);
            }
            ids
        };

        // Now filter by metadata and build results
        let mut results = Vec::new();
        let query_lower = query.map(|q| q.to_lowercase());

        for video_id in video_ids {
            // Check era filter
            if let Some(era_filter) = era {
                let video_eras = self.get_video_eras(&video_id)?;
                if !video_eras.iter().any(|e| e.name.eq_ignore_ascii_case(era_filter)) {
                    continue;
                }
            }

            // Check region filter
            if let Some(region_filter) = region {
                let video_regions = self.get_video_regions(&video_id)?;
                if !video_regions.iter().any(|r| r.name.eq_ignore_ascii_case(region_filter)) {
                    continue;
                }
            }

            // Check topic filter
            if let Some(topic_filter) = topic {
                let video_topics = self.get_video_topics(&video_id)?;
                if !video_topics.iter().any(|t| t.name.eq_ignore_ascii_case(topic_filter)) {
                    continue;
                }
            }

            // Get video and transcript
            let video = match self.get_video(&video_id)? {
                Some(v) => v,
                None => continue,
            };

            // Find matching segments if we have a text query
            let mut matches = Vec::new();
            if let Some(ref q_lower) = query_lower {
                if let Some(transcript) = self.get_transcript(&video_id)? {
                    for seg in &transcript.segments {
                        if seg.text.to_lowercase().contains(q_lower) {
                            matches.push(SegmentMatch {
                                start_time: seg.start_time,
                                duration: seg.duration,
                                text: seg.text.clone(),
                            });
                        }
                    }
                }
            }

            // Get metadata
            let eras: Vec<String> = self.get_video_eras(&video_id)?
                .into_iter().map(|e| e.name).collect();
            let regions: Vec<String> = self.get_video_regions(&video_id)?
                .into_iter().map(|r| r.name).collect();
            let topics: Vec<String> = self.get_video_topics(&video_id)?
                .into_iter().map(|t| t.name).collect();

            results.push(AdvancedSearchResult {
                video,
                matches,
                eras,
                regions,
                topics,
            });
        }

        Ok(results)
    }

    // Saved search operations

    pub fn save_search(
        &self,
        name: &str,
        query: Option<&str>,
        era: Option<&str>,
        region: Option<&str>,
        topic: Option<&str>,
    ) -> Result<SavedSearch> {
        let created_at = Utc::now();
        self.conn.execute(
            "INSERT INTO saved_searches (name, query, era, region, topic, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, query, era, region, topic, created_at.to_rfc3339()],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(SavedSearch {
            id,
            name: name.to_string(),
            query: query.map(|s| s.to_string()),
            era: era.map(|s| s.to_string()),
            region: region.map(|s| s.to_string()),
            topic: topic.map(|s| s.to_string()),
            created_at,
        })
    }

    pub fn list_saved_searches(&self) -> Result<Vec<SavedSearch>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, query, era, region, topic, created_at FROM saved_searches ORDER BY name"
        )?;

        let mut searches = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let created_at: String = row.get(6)?;
            searches.push(SavedSearch {
                id: row.get(0)?,
                name: row.get(1)?,
                query: row.get(2)?,
                era: row.get(3)?,
                region: row.get(4)?,
                topic: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            });
        }
        Ok(searches)
    }

    pub fn get_saved_search(&self, name: &str) -> Result<Option<SavedSearch>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, query, era, region, topic, created_at FROM saved_searches WHERE name = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            let created_at: String = row.get(6)?;
            Ok(Some(SavedSearch {
                id: row.get(0)?,
                name: row.get(1)?,
                query: row.get(2)?,
                era: row.get(3)?,
                region: row.get(4)?,
                topic: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete_saved_search(&self, name: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM saved_searches WHERE name = ?1 COLLATE NOCASE",
            params![name],
        )?;
        Ok(affected > 0)
    }

    // Report generation

    pub fn report_by_era(&self) -> Result<Vec<ReportEntry>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT e.name, COUNT(DISTINCT ve.video_id) as count
            FROM eras e
            LEFT JOIN video_eras ve ON ve.era_id = e.id
            GROUP BY e.id, e.name
            ORDER BY e.sort_order
            "#
        )?;

        let mut entries = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            entries.push(ReportEntry {
                name: row.get(0)?,
                video_count: row.get(1)?,
            });
        }
        Ok(entries)
    }

    pub fn report_by_region(&self) -> Result<Vec<ReportEntry>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT r.name, COUNT(DISTINCT vr.video_id) as count
            FROM regions r
            LEFT JOIN video_regions vr ON vr.region_id = r.id
            GROUP BY r.id, r.name
            ORDER BY count DESC, r.name
            "#
        )?;

        let mut entries = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            entries.push(ReportEntry {
                name: row.get(0)?,
                video_count: row.get(1)?,
            });
        }
        Ok(entries)
    }

    pub fn report_by_topic(&self) -> Result<Vec<ReportEntry>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT t.name, COUNT(DISTINCT vt.video_id) as count
            FROM topics t
            LEFT JOIN video_topics vt ON vt.topic_id = t.id
            GROUP BY t.id, t.name
            ORDER BY count DESC, t.name
            "#
        )?;

        let mut entries = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            entries.push(ReportEntry {
                name: row.get(0)?,
                video_count: row.get(1)?,
            });
        }
        Ok(entries)
    }

    // Export functions

    pub fn export_collection_markdown(&self, collection_name: &str) -> Result<Option<String>> {
        let collection = match self.get_collection_by_name(collection_name)? {
            Some(c) => c,
            None => return Ok(None),
        };

        let videos = self.get_collection_videos(collection_name)?;

        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", collection.name));

        if let Some(desc) = &collection.description {
            md.push_str(&format!("{}\n\n", desc));
        }

        md.push_str(&format!("**{} videos**\n\n", videos.len()));
        md.push_str("---\n\n");

        for video in &videos {
            md.push_str(&format!("## {}\n\n", video.title));
            md.push_str(&format!("- **ID**: {}\n", video.id));
            md.push_str(&format!("- **URL**: {}\n", video.url));

            if let Some(channel) = &video.channel {
                md.push_str(&format!("- **Channel**: {}\n", channel));
            }

            if let Some(date) = &video.upload_date {
                md.push_str(&format!("- **Upload Date**: {}\n", date));
            }

            // Get tags
            let eras = self.get_video_eras(&video.id)?;
            let regions = self.get_video_regions(&video.id)?;
            let topics = self.get_video_topics(&video.id)?;

            if !eras.is_empty() {
                let era_names: Vec<&str> = eras.iter().map(|e| e.name.as_str()).collect();
                md.push_str(&format!("- **Eras**: {}\n", era_names.join(", ")));
            }

            if !regions.is_empty() {
                let region_names: Vec<&str> = regions.iter().map(|r| r.name.as_str()).collect();
                md.push_str(&format!("- **Regions**: {}\n", region_names.join(", ")));
            }

            if !topics.is_empty() {
                let topic_names: Vec<&str> = topics.iter().map(|t| t.name.as_str()).collect();
                md.push_str(&format!("- **Topics**: {}\n", topic_names.join(", ")));
            }

            // Get notes
            let notes = self.get_video_notes(&video.id)?;
            if !notes.is_empty() {
                md.push_str("\n### Notes\n\n");
                for note in &notes {
                    if let Some(ts) = note.timestamp {
                        let mins = (ts / 60.0) as u32;
                        let secs = (ts % 60.0) as u32;
                        md.push_str(&format!("- **[{:02}:{:02}]** {}\n", mins, secs, note.text));
                    } else {
                        md.push_str(&format!("- {}\n", note.text));
                    }
                }
            }

            if let Some(desc) = &video.description {
                if !desc.is_empty() {
                    md.push_str("\n### Description\n\n");
                    md.push_str(&format!("{}\n", desc));
                }
            }

            md.push_str("\n---\n\n");
        }

        Ok(Some(md))
    }

    pub fn export_map_geojson(&self, era: Option<&str>, topic: Option<&str>) -> Result<GeoJsonCollection> {
        let pins = self.get_map_pins(era, topic)?;

        let features: Vec<GeoJsonFeature> = pins.iter().map(|pin| {
            GeoJsonFeature {
                r#type: "Feature".to_string(),
                geometry: GeoJsonGeometry {
                    r#type: "Point".to_string(),
                    coordinates: [pin.location.lon, pin.location.lat],
                },
                properties: GeoJsonProperties {
                    name: pin.location.name.clone(),
                    video_id: pin.video_id.clone(),
                    video_title: pin.video_title.clone(),
                    era: pin.era.clone(),
                    topic: pin.topic.clone(),
                    note: pin.note.clone(),
                },
            }
        }).collect();

        Ok(GeoJsonCollection {
            r#type: "FeatureCollection".to_string(),
            features,
        })
    }

    pub fn get_summary_stats(&self) -> Result<(i64, i64, i64, i64, i64, i64, i64, i64, i64)> {
        let video_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM videos", [], |row| row.get(0)
        )?;
        let transcript_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM transcripts", [], |row| row.get(0)
        )?;
        let location_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM locations", [], |row| row.get(0)
        )?;
        let note_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes", [], |row| row.get(0)
        )?;
        let collection_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM collections", [], |row| row.get(0)
        )?;
        let saved_search_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM saved_searches", [], |row| row.get(0)
        )?;
        let claim_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM claims", [], |row| row.get(0)
        )?;
        let chunk_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM transcript_chunks", [], |row| row.get(0)
        )?;
        let embedding_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings", [], |row| row.get(0)
        )?;

        Ok((video_count, transcript_count, location_count, note_count, collection_count, saved_search_count, claim_count, chunk_count, embedding_count))
    }

    // Phase 6: Claim Extraction & Atomic Notes

    // Claim operations

    pub fn create_claim(
        &self,
        text: &str,
        video_id: &str,
        timestamp: Option<f64>,
        source_quote: &str,
        category: ClaimCategory,
        confidence: Confidence,
    ) -> Result<Claim> {
        let created_at = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO claims (text, video_id, timestamp, source_quote, category, confidence, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                text,
                video_id,
                timestamp,
                source_quote,
                category.as_str(),
                confidence.as_str(),
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Claim {
            id,
            text: text.to_string(),
            video_id: video_id.to_string(),
            timestamp,
            source_quote: source_quote.to_string(),
            category,
            confidence,
            created_at,
        })
    }

    pub fn get_claim(&self, id: i64) -> Result<Option<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, video_id, timestamp, source_quote, category, confidence, created_at FROM claims WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_claim(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_claims_for_video(&self, video_id: &str) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, video_id, timestamp, source_quote, category, confidence, created_at FROM claims WHERE video_id = ?1 ORDER BY timestamp NULLS LAST, created_at"
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn list_claims_by_category(&self, category: ClaimCategory) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, video_id, timestamp, source_quote, category, confidence, created_at FROM claims WHERE category = ?1 ORDER BY created_at DESC"
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query(params![category.as_str()])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn list_all_claims(&self) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, video_id, timestamp, source_quote, category, confidence, created_at FROM claims ORDER BY created_at DESC"
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn delete_claim(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM claims WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    pub fn update_claim(
        &self,
        id: i64,
        text: Option<&str>,
        category: Option<ClaimCategory>,
        confidence: Option<Confidence>,
    ) -> Result<bool> {
        let mut updates = Vec::new();
        let mut param_idx = 1;

        if text.is_some() {
            updates.push(format!("text = ?{}", param_idx));
            param_idx += 1;
        }
        if category.is_some() {
            updates.push(format!("category = ?{}", param_idx));
            param_idx += 1;
        }
        if confidence.is_some() {
            updates.push(format!("confidence = ?{}", param_idx));
            param_idx += 1;
        }

        if updates.is_empty() {
            return Ok(false);
        }

        let query = format!(
            "UPDATE claims SET {} WHERE id = ?{}",
            updates.join(", "),
            param_idx
        );

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        if let Some(t) = text {
            params_vec.push(Box::new(t.to_string()));
        }
        if let Some(c) = category {
            params_vec.push(Box::new(c.as_str().to_string()));
        }
        if let Some(c) = confidence {
            params_vec.push(Box::new(c.as_str().to_string()));
        }
        params_vec.push(Box::new(id));

        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let affected = self.conn.execute(&query, params_refs.as_slice())?;
        Ok(affected > 0)
    }

    fn row_to_claim(&self, row: &rusqlite::Row) -> Result<Claim> {
        let category_str: String = row.get(5)?;
        let confidence_str: String = row.get(6)?;
        let created_at: String = row.get(7)?;

        Ok(Claim {
            id: row.get(0)?,
            text: row.get(1)?,
            video_id: row.get(2)?,
            timestamp: row.get(3)?,
            source_quote: row.get(4)?,
            category: ClaimCategory::from_str(&category_str).unwrap_or(ClaimCategory::Factual),
            confidence: Confidence::from_str(&confidence_str).unwrap_or(Confidence::Medium),
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // Claim link operations

    pub fn create_claim_link(
        &self,
        source_claim_id: i64,
        target_claim_id: i64,
        link_type: LinkType,
    ) -> Result<ClaimLink> {
        let created_at = Utc::now();
        self.conn.execute(
            "INSERT OR IGNORE INTO claim_links (source_claim_id, target_claim_id, link_type, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![source_claim_id, target_claim_id, link_type.as_str(), created_at.to_rfc3339()],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(ClaimLink {
            id,
            source_claim_id,
            target_claim_id,
            link_type,
            created_at,
        })
    }

    pub fn delete_claim_link(&self, source_id: i64, target_id: i64) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM claim_links WHERE source_claim_id = ?1 AND target_claim_id = ?2",
            params![source_id, target_id],
        )?;
        Ok(affected > 0)
    }

    pub fn get_claim_with_links(&self, claim_id: i64) -> Result<Option<ClaimWithLinks>> {
        let claim = match self.get_claim(claim_id)? {
            Some(c) => c,
            None => return Ok(None),
        };

        // Get outgoing links
        let mut out_stmt = self.conn.prepare(
            r#"
            SELECT cl.id, cl.source_claim_id, cl.target_claim_id, cl.link_type, cl.created_at,
                   c.id, c.text, c.video_id, c.timestamp, c.source_quote, c.category, c.confidence, c.created_at
            FROM claim_links cl
            JOIN claims c ON c.id = cl.target_claim_id
            WHERE cl.source_claim_id = ?1
            "#
        )?;

        let mut outgoing_links = Vec::new();
        let mut out_rows = out_stmt.query(params![claim_id])?;
        while let Some(row) = out_rows.next()? {
            let link = self.row_to_claim_link(row)?;
            let target_claim = self.row_to_claim_from_offset(row, 5)?;
            outgoing_links.push((link, target_claim));
        }

        // Get incoming links
        let mut in_stmt = self.conn.prepare(
            r#"
            SELECT cl.id, cl.source_claim_id, cl.target_claim_id, cl.link_type, cl.created_at,
                   c.id, c.text, c.video_id, c.timestamp, c.source_quote, c.category, c.confidence, c.created_at
            FROM claim_links cl
            JOIN claims c ON c.id = cl.source_claim_id
            WHERE cl.target_claim_id = ?1
            "#
        )?;

        let mut incoming_links = Vec::new();
        let mut in_rows = in_stmt.query(params![claim_id])?;
        while let Some(row) = in_rows.next()? {
            let link = self.row_to_claim_link(row)?;
            let source_claim = self.row_to_claim_from_offset(row, 5)?;
            incoming_links.push((link, source_claim));
        }

        Ok(Some(ClaimWithLinks {
            claim,
            outgoing_links,
            incoming_links,
        }))
    }

    pub fn get_unlinked_claims(&self) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.text, c.video_id, c.timestamp, c.source_quote, c.category, c.confidence, c.created_at
            FROM claims c
            WHERE (
                SELECT COUNT(*) FROM claim_links cl
                WHERE cl.source_claim_id = c.id OR cl.target_claim_id = c.id
            ) < 2
            ORDER BY c.created_at DESC
            "#
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn get_claim_link_count(&self, claim_id: i64) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM claim_links WHERE source_claim_id = ?1 OR target_claim_id = ?1",
            params![claim_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    fn row_to_claim_link(&self, row: &rusqlite::Row) -> Result<ClaimLink> {
        let link_type_str: String = row.get(3)?;
        let created_at: String = row.get(4)?;

        Ok(ClaimLink {
            id: row.get(0)?,
            source_claim_id: row.get(1)?,
            target_claim_id: row.get(2)?,
            link_type: LinkType::from_str(&link_type_str).unwrap_or(LinkType::Related),
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    fn row_to_claim_from_offset(&self, row: &rusqlite::Row, offset: usize) -> Result<Claim> {
        let category_str: String = row.get(offset + 5)?;
        let confidence_str: String = row.get(offset + 6)?;
        let created_at: String = row.get(offset + 7)?;

        Ok(Claim {
            id: row.get(offset)?,
            text: row.get(offset + 1)?,
            video_id: row.get(offset + 2)?,
            timestamp: row.get(offset + 3)?,
            source_quote: row.get(offset + 4)?,
            category: ClaimCategory::from_str(&category_str).unwrap_or(ClaimCategory::Factual),
            confidence: Confidence::from_str(&confidence_str).unwrap_or(Confidence::Medium),
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // Transcript layer operations (progressive summarization)

    pub fn save_transcript_layer(&self, video_id: &str, layer: u8, content: &str) -> Result<TranscriptLayer> {
        let created_at = Utc::now();
        self.conn.execute(
            "INSERT OR REPLACE INTO transcript_layers (video_id, layer, content, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![video_id, layer, content, created_at.to_rfc3339()],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(TranscriptLayer {
            id,
            video_id: video_id.to_string(),
            layer,
            content: content.to_string(),
            created_at,
        })
    }

    pub fn get_transcript_layer(&self, video_id: &str, layer: u8) -> Result<Option<TranscriptLayer>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, layer, content, created_at FROM transcript_layers WHERE video_id = ?1 AND layer = ?2"
        )?;
        let mut rows = stmt.query(params![video_id, layer])?;

        if let Some(row) = rows.next()? {
            let created_at: String = row.get(4)?;
            Ok(Some(TranscriptLayer {
                id: row.get(0)?,
                video_id: row.get(1)?,
                layer: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_transcript_layers(&self, video_id: &str) -> Result<Vec<TranscriptLayer>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, layer, content, created_at FROM transcript_layers WHERE video_id = ?1 ORDER BY layer"
        )?;

        let mut layers = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            let created_at: String = row.get(4)?;
            layers.push(TranscriptLayer {
                id: row.get(0)?,
                video_id: row.get(1)?,
                layer: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            });
        }
        Ok(layers)
    }

    // Transcript chunk operations (intelligent chunking)

    pub fn save_transcript_chunks(&self, video_id: &str, chunks: &[TranscriptChunk]) -> Result<()> {
        // Clear existing chunks for this video
        self.conn.execute("DELETE FROM transcript_chunks WHERE video_id = ?1", params![video_id])?;

        for chunk in chunks {
            self.conn.execute(
                r#"
                INSERT INTO transcript_chunks (video_id, chunk_index, start_time, end_time, text, token_count, overlap_with_previous)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                "#,
                params![
                    video_id,
                    chunk.chunk_index,
                    chunk.start_time,
                    chunk.end_time,
                    chunk.text,
                    chunk.token_count,
                    chunk.overlap_with_previous as i32,
                ],
            )?;
        }
        Ok(())
    }

    pub fn get_transcript_chunks(&self, video_id: &str) -> Result<Vec<TranscriptChunk>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, chunk_index, start_time, end_time, text, token_count, overlap_with_previous FROM transcript_chunks WHERE video_id = ?1 ORDER BY chunk_index"
        )?;

        let mut chunks = Vec::new();
        let mut rows = stmt.query(params![video_id])?;

        while let Some(row) = rows.next()? {
            let overlap: i32 = row.get(7)?;
            chunks.push(TranscriptChunk {
                id: row.get(0)?,
                video_id: row.get(1)?,
                chunk_index: row.get(2)?,
                start_time: row.get(3)?,
                end_time: row.get(4)?,
                text: row.get(5)?,
                token_count: row.get(6)?,
                overlap_with_previous: overlap != 0,
            });
        }
        Ok(chunks)
    }

    pub fn has_chunks(&self, video_id: &str) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM transcript_chunks WHERE video_id = ?1",
            params![video_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // Claim statistics

    pub fn get_claim_stats(&self) -> Result<(i64, i64, i64)> {
        let total: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM claims", [], |row| row.get(0)
        )?;
        let linked: i64 = self.conn.query_row(
            r#"
            SELECT COUNT(DISTINCT c.id) FROM claims c
            WHERE (SELECT COUNT(*) FROM claim_links cl WHERE cl.source_claim_id = c.id OR cl.target_claim_id = c.id) >= 2
            "#,
            [],
            |row| row.get(0),
        )?;
        let links: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM claim_links", [], |row| row.get(0)
        )?;
        Ok((total, linked, links))
    }

    // Phase 7: Semantic Search & Embeddings

    pub fn save_embedding(
        &self,
        source_type: EmbeddingSource,
        source_id: &str,
        model: &str,
        vector: &[f32],
    ) -> Result<Embedding> {
        let created_at = Utc::now();
        let vector_json = serde_json::to_string(vector)?;
        let dimensions = vector.len() as i32;

        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO embeddings (source_type, source_id, model, vector_json, dimensions, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                source_type.as_str(),
                source_id,
                model,
                vector_json,
                dimensions,
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();

        Ok(Embedding {
            id,
            source_type,
            source_id: source_id.to_string(),
            model: model.to_string(),
            vector: vector.to_vec(),
            created_at,
        })
    }

    pub fn get_embedding(
        &self,
        source_type: EmbeddingSource,
        source_id: &str,
        model: &str,
    ) -> Result<Option<Embedding>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source_type, source_id, model, vector_json, created_at FROM embeddings WHERE source_type = ?1 AND source_id = ?2 AND model = ?3"
        )?;
        let mut rows = stmt.query(params![source_type.as_str(), source_id, model])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_embedding(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_embeddings_by_type(&self, source_type: EmbeddingSource) -> Result<Vec<Embedding>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source_type, source_id, model, vector_json, created_at FROM embeddings WHERE source_type = ?1 ORDER BY created_at DESC"
        )?;

        let mut embeddings = Vec::new();
        let mut rows = stmt.query(params![source_type.as_str()])?;

        while let Some(row) = rows.next()? {
            embeddings.push(self.row_to_embedding(row)?);
        }
        Ok(embeddings)
    }

    pub fn list_all_embeddings(&self) -> Result<Vec<Embedding>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source_type, source_id, model, vector_json, created_at FROM embeddings ORDER BY source_type, source_id"
        )?;

        let mut embeddings = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            embeddings.push(self.row_to_embedding(row)?);
        }
        Ok(embeddings)
    }

    pub fn delete_embedding(&self, source_type: EmbeddingSource, source_id: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM embeddings WHERE source_type = ?1 AND source_id = ?2",
            params![source_type.as_str(), source_id],
        )?;
        Ok(affected > 0)
    }

    pub fn has_embedding(&self, source_type: EmbeddingSource, source_id: &str) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings WHERE source_type = ?1 AND source_id = ?2",
            params![source_type.as_str(), source_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    fn row_to_embedding(&self, row: &rusqlite::Row) -> Result<Embedding> {
        let source_type_str: String = row.get(1)?;
        let vector_json: String = row.get(4)?;
        let created_at: String = row.get(5)?;

        Ok(Embedding {
            id: row.get(0)?,
            source_type: EmbeddingSource::from_str(&source_type_str).unwrap_or(EmbeddingSource::Chunk),
            source_id: row.get(2)?,
            model: row.get(3)?,
            vector: serde_json::from_str(&vector_json)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // Similarity search using cosine similarity
    pub fn find_similar(
        &self,
        query_vector: &[f32],
        source_type: Option<EmbeddingSource>,
        limit: usize,
    ) -> Result<Vec<(Embedding, f32)>> {
        let embeddings = if let Some(st) = source_type {
            self.list_embeddings_by_type(st)?
        } else {
            self.list_all_embeddings()?
        };

        let mut scored: Vec<(Embedding, f32)> = embeddings
            .into_iter()
            .map(|emb| {
                let score = cosine_similarity(query_vector, &emb.vector);
                (emb, score)
            })
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(scored.into_iter().take(limit).collect())
    }

    // Get text for a similarity result
    pub fn get_text_for_embedding(&self, embedding: &Embedding) -> Result<Option<String>> {
        match embedding.source_type {
            EmbeddingSource::Video => {
                if let Some(video) = self.get_video(&embedding.source_id)? {
                    let desc = video.description.unwrap_or_default();
                    Ok(Some(format!("{}\n{}", video.title, desc)))
                } else {
                    Ok(None)
                }
            }
            EmbeddingSource::Chunk => {
                // source_id is "video_id:chunk_index"
                let parts: Vec<&str> = embedding.source_id.split(':').collect();
                if parts.len() == 2 {
                    let video_id = parts[0];
                    let chunk_index: i32 = parts[1].parse().unwrap_or(0);
                    let chunks = self.get_transcript_chunks(video_id)?;
                    if let Some(chunk) = chunks.into_iter().find(|c| c.chunk_index == chunk_index) {
                        return Ok(Some(chunk.text));
                    }
                }
                Ok(None)
            }
            EmbeddingSource::Claim => {
                let claim_id: i64 = embedding.source_id.parse().unwrap_or(0);
                if let Some(claim) = self.get_claim(claim_id)? {
                    Ok(Some(claim.text))
                } else {
                    Ok(None)
                }
            }
            EmbeddingSource::Summary => {
                // source_id is "video_id:layer"
                let parts: Vec<&str> = embedding.source_id.split(':').collect();
                if parts.len() == 2 {
                    let video_id = parts[0];
                    let layer: u8 = parts[1].parse().unwrap_or(0);
                    if let Some(tl) = self.get_transcript_layer(video_id, layer)? {
                        return Ok(Some(tl.content));
                    }
                }
                Ok(None)
            }
        }
    }

    // Build similarity results with text
    pub fn build_similarity_results(
        &self,
        similar: Vec<(Embedding, f32)>,
    ) -> Result<Vec<SimilarityResult>> {
        let mut results = Vec::new();

        for (emb, score) in similar {
            let text = self.get_text_for_embedding(&emb)?.unwrap_or_default();
            let video_id = match emb.source_type {
                EmbeddingSource::Video => Some(emb.source_id.clone()),
                EmbeddingSource::Chunk | EmbeddingSource::Summary => {
                    emb.source_id.split(':').next().map(|s| s.to_string())
                }
                EmbeddingSource::Claim => {
                    let claim_id: i64 = emb.source_id.parse().unwrap_or(0);
                    self.get_claim(claim_id)?.map(|c| c.video_id)
                }
            };

            results.push(SimilarityResult {
                source_type: emb.source_type,
                source_id: emb.source_id,
                score,
                text,
                video_id,
            });
        }

        Ok(results)
    }

    // Hybrid search combining keyword and semantic
    pub fn hybrid_search(
        &self,
        query: &str,
        query_vector: Option<&[f32]>,
        keyword_weight: f32,
        semantic_weight: f32,
        limit: usize,
    ) -> Result<Vec<HybridSearchResult>> {
        // Get keyword results
        let keyword_results = self.search_with_timestamps(query)?;

        // Build a map of video_id -> keyword_score
        let mut keyword_scores: std::collections::HashMap<String, f32> = std::collections::HashMap::new();
        let max_keyword = keyword_results.len() as f32;
        for (i, result) in keyword_results.iter().enumerate() {
            // Score based on rank (higher rank = higher score)
            let score = if max_keyword > 0.0 {
                (max_keyword - i as f32) / max_keyword
            } else {
                0.0
            };
            keyword_scores.insert(result.video.id.clone(), score);
        }

        // Get semantic results if we have a query vector
        let mut semantic_scores: std::collections::HashMap<String, f32> = std::collections::HashMap::new();
        if let Some(qv) = query_vector {
            let similar = self.find_similar(qv, Some(EmbeddingSource::Video), limit * 2)?;
            for (emb, score) in similar {
                semantic_scores.insert(emb.source_id, score);
            }

            // Also check chunk embeddings
            let chunk_similar = self.find_similar(qv, Some(EmbeddingSource::Chunk), limit * 2)?;
            for (emb, score) in chunk_similar {
                if let Some(video_id) = emb.source_id.split(':').next() {
                    let entry = semantic_scores.entry(video_id.to_string()).or_insert(0.0);
                    *entry = entry.max(score);
                }
            }
        }

        // Combine scores
        let mut all_video_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        all_video_ids.extend(keyword_scores.keys().cloned());
        all_video_ids.extend(semantic_scores.keys().cloned());

        let mut combined: Vec<(String, f32, f32, f32)> = all_video_ids
            .into_iter()
            .map(|vid| {
                let kw = keyword_scores.get(&vid).copied().unwrap_or(0.0);
                let sem = semantic_scores.get(&vid).copied().unwrap_or(0.0);
                let combined = kw * keyword_weight + sem * semantic_weight;
                (vid, kw, sem, combined)
            })
            .collect();

        combined.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

        // Build results
        let mut results = Vec::new();
        for (video_id, kw_score, sem_score, comb_score) in combined.into_iter().take(limit) {
            if let Some(video) = self.get_video(&video_id)? {
                // Get matching chunks
                let chunks = self.get_transcript_chunks(&video_id)?;
                let matching_chunks: Vec<ChunkMatch> = if let Some(qv) = query_vector {
                    chunks
                        .into_iter()
                        .filter_map(|chunk| {
                            let chunk_id = format!("{}:{}", video_id, chunk.chunk_index);
                            if let Ok(Some(emb)) = self.get_embedding(EmbeddingSource::Chunk, &chunk_id, "default") {
                                let score = cosine_similarity(qv, &emb.vector);
                                Some(ChunkMatch { chunk, score })
                            } else {
                                None
                            }
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                // Get matching claims
                let claims = self.list_claims_for_video(&video_id)?;
                let matching_claims: Vec<Claim> = if let Some(qv) = query_vector {
                    claims
                        .into_iter()
                        .filter(|claim| {
                            if let Ok(Some(emb)) = self.get_embedding(EmbeddingSource::Claim, &claim.id.to_string(), "default") {
                                let score = cosine_similarity(qv, &emb.vector);
                                score > 0.5 // threshold
                            } else {
                                false
                            }
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                results.push(HybridSearchResult {
                    video,
                    keyword_score: kw_score,
                    semantic_score: sem_score,
                    combined_score: comb_score,
                    matching_chunks,
                    matching_claims,
                });
            }
        }

        Ok(results)
    }

    // Embedding statistics
    pub fn get_embedding_stats(&self) -> Result<EmbeddingStats> {
        let total: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings", [], |row| row.get(0)
        )?;

        let video_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings WHERE source_type = 'video'", [], |row| row.get(0)
        )?;

        let chunk_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings WHERE source_type = 'chunk'", [], |row| row.get(0)
        )?;

        let claim_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings WHERE source_type = 'claim'", [], |row| row.get(0)
        )?;

        let summary_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings WHERE source_type = 'summary'", [], |row| row.get(0)
        )?;

        // Get model and dimensions from first embedding
        let model_dims: Option<(String, i32)> = self.conn.query_row(
            "SELECT model, dimensions FROM embeddings LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).ok();

        Ok(EmbeddingStats {
            total_embeddings: total,
            video_embeddings: video_count,
            chunk_embeddings: chunk_count,
            claim_embeddings: claim_count,
            summary_embeddings: summary_count,
            model: model_dims.as_ref().map(|(m, _)| m.clone()),
            dimensions: model_dims.map(|(_, d)| d),
        })
    }

    // Get items that need embeddings
    pub fn get_items_needing_embeddings(&self) -> Result<(Vec<String>, Vec<String>, Vec<i64>)> {
        // Videos without embeddings
        let mut video_stmt = self.conn.prepare(
            r#"
            SELECT v.id FROM videos v
            WHERE NOT EXISTS (
                SELECT 1 FROM embeddings e WHERE e.source_type = 'video' AND e.source_id = v.id
            )
            "#
        )?;
        let mut videos = Vec::new();
        let mut rows = video_stmt.query([])?;
        while let Some(row) = rows.next()? {
            videos.push(row.get(0)?);
        }

        // Chunks without embeddings
        let mut chunk_stmt = self.conn.prepare(
            r#"
            SELECT tc.video_id || ':' || tc.chunk_index FROM transcript_chunks tc
            WHERE NOT EXISTS (
                SELECT 1 FROM embeddings e WHERE e.source_type = 'chunk' AND e.source_id = tc.video_id || ':' || tc.chunk_index
            )
            "#
        )?;
        let mut chunks = Vec::new();
        let mut rows = chunk_stmt.query([])?;
        while let Some(row) = rows.next()? {
            chunks.push(row.get(0)?);
        }

        // Claims without embeddings
        let mut claim_stmt = self.conn.prepare(
            r#"
            SELECT c.id FROM claims c
            WHERE NOT EXISTS (
                SELECT 1 FROM embeddings e WHERE e.source_type = 'claim' AND e.source_id = CAST(c.id AS TEXT)
            )
            "#
        )?;
        let mut claims = Vec::new();
        let mut rows = claim_stmt.query([])?;
        while let Some(row) = rows.next()? {
            claims.push(row.get(0)?);
        }

        Ok((videos, chunks, claims))
    }

    // Phase 8: Analytical Frameworks

    // 8.1 Cyclical Indicator Operations

    pub fn create_cyclical_indicator(
        &self,
        video_id: &str,
        claim_id: Option<i64>,
        indicator_type: CyclicalType,
        entity: &str,
        era_id: Option<i64>,
        description: &str,
        timestamp: Option<f64>,
    ) -> Result<CyclicalIndicator> {
        let created_at = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO cyclical_indicators (video_id, claim_id, indicator_type, entity, era_id, description, timestamp, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                video_id,
                claim_id,
                indicator_type.as_str(),
                entity,
                era_id,
                description,
                timestamp,
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(CyclicalIndicator {
            id,
            video_id: video_id.to_string(),
            claim_id,
            indicator_type,
            entity: entity.to_string(),
            era_id,
            description: description.to_string(),
            timestamp,
            created_at,
        })
    }

    pub fn get_cyclical_indicator(&self, id: i64) -> Result<Option<CyclicalIndicator>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, claim_id, indicator_type, entity, era_id, description, timestamp, created_at FROM cyclical_indicators WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_cyclical_indicator(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_cyclical_indicators_by_type(&self, indicator_type: CyclicalType) -> Result<Vec<CyclicalIndicator>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, claim_id, indicator_type, entity, era_id, description, timestamp, created_at FROM cyclical_indicators WHERE indicator_type = ?1 ORDER BY created_at DESC"
        )?;

        let mut indicators = Vec::new();
        let mut rows = stmt.query(params![indicator_type.as_str()])?;

        while let Some(row) = rows.next()? {
            indicators.push(self.row_to_cyclical_indicator(row)?);
        }
        Ok(indicators)
    }

    pub fn list_cyclical_indicators_by_entity(&self, entity: &str) -> Result<Vec<CyclicalIndicator>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, claim_id, indicator_type, entity, era_id, description, timestamp, created_at FROM cyclical_indicators WHERE entity = ?1 ORDER BY created_at DESC"
        )?;

        let mut indicators = Vec::new();
        let mut rows = stmt.query(params![entity])?;

        while let Some(row) = rows.next()? {
            indicators.push(self.row_to_cyclical_indicator(row)?);
        }
        Ok(indicators)
    }

    pub fn list_all_cyclical_indicators(&self) -> Result<Vec<CyclicalIndicator>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, video_id, claim_id, indicator_type, entity, era_id, description, timestamp, created_at FROM cyclical_indicators ORDER BY entity, indicator_type, created_at DESC"
        )?;

        let mut indicators = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            indicators.push(self.row_to_cyclical_indicator(row)?);
        }
        Ok(indicators)
    }

    pub fn delete_cyclical_indicator(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM cyclical_indicators WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_cyclical_indicator(&self, row: &rusqlite::Row) -> Result<CyclicalIndicator> {
        let indicator_type_str: String = row.get(3)?;
        let created_at: String = row.get(8)?;

        Ok(CyclicalIndicator {
            id: row.get(0)?,
            video_id: row.get(1)?,
            claim_id: row.get(2)?,
            indicator_type: CyclicalType::from_str(&indicator_type_str).unwrap_or(CyclicalType::SocialUnrest),
            entity: row.get(4)?,
            era_id: row.get(5)?,
            description: row.get(6)?,
            timestamp: row.get(7)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // 8.2 Causal Relation Operations

    pub fn create_causal_relation(
        &self,
        cause_claim_id: i64,
        effect_claim_id: i64,
        loop_type: LoopType,
        strength: RelationStrength,
        video_id: &str,
        notes: Option<&str>,
    ) -> Result<CausalRelation> {
        let created_at = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO causal_relations (cause_claim_id, effect_claim_id, loop_type, strength, video_id, notes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                cause_claim_id,
                effect_claim_id,
                loop_type.as_str(),
                strength.as_str(),
                video_id,
                notes,
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(CausalRelation {
            id,
            cause_claim_id,
            effect_claim_id,
            loop_type,
            strength,
            video_id: video_id.to_string(),
            notes: notes.map(|s| s.to_string()),
            created_at,
        })
    }

    pub fn get_causal_relation(&self, id: i64) -> Result<Option<CausalRelation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, cause_claim_id, effect_claim_id, loop_type, strength, video_id, notes, created_at FROM causal_relations WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_causal_relation(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_causal_relations_for_claim(&self, claim_id: i64) -> Result<Vec<CausalRelation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, cause_claim_id, effect_claim_id, loop_type, strength, video_id, notes, created_at FROM causal_relations WHERE cause_claim_id = ?1 OR effect_claim_id = ?1 ORDER BY created_at DESC"
        )?;

        let mut relations = Vec::new();
        let mut rows = stmt.query(params![claim_id])?;

        while let Some(row) = rows.next()? {
            relations.push(self.row_to_causal_relation(row)?);
        }
        Ok(relations)
    }

    pub fn list_causal_relations_by_type(&self, loop_type: LoopType) -> Result<Vec<CausalRelation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, cause_claim_id, effect_claim_id, loop_type, strength, video_id, notes, created_at FROM causal_relations WHERE loop_type = ?1 ORDER BY created_at DESC"
        )?;

        let mut relations = Vec::new();
        let mut rows = stmt.query(params![loop_type.as_str()])?;

        while let Some(row) = rows.next()? {
            relations.push(self.row_to_causal_relation(row)?);
        }
        Ok(relations)
    }

    pub fn list_all_causal_relations(&self) -> Result<Vec<CausalRelation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, cause_claim_id, effect_claim_id, loop_type, strength, video_id, notes, created_at FROM causal_relations ORDER BY loop_type, created_at DESC"
        )?;

        let mut relations = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            relations.push(self.row_to_causal_relation(row)?);
        }
        Ok(relations)
    }

    pub fn delete_causal_relation(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM causal_relations WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_causal_relation(&self, row: &rusqlite::Row) -> Result<CausalRelation> {
        let loop_type_str: String = row.get(3)?;
        let strength_str: String = row.get(4)?;
        let created_at: String = row.get(7)?;

        Ok(CausalRelation {
            id: row.get(0)?,
            cause_claim_id: row.get(1)?,
            effect_claim_id: row.get(2)?,
            loop_type: LoopType::from_str(&loop_type_str).unwrap_or(LoopType::Linear),
            strength: RelationStrength::from_str(&strength_str).unwrap_or(RelationStrength::Moderate),
            video_id: row.get(5)?,
            notes: row.get(6)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // 8.3 Idea Transmission Operations

    pub fn create_idea_transmission(
        &self,
        idea: &str,
        source_entity: &str,
        target_entity: &str,
        transmission_type: TransmissionType,
        era_id: Option<i64>,
        region_id: Option<i64>,
        video_id: &str,
        claim_id: Option<i64>,
        notes: Option<&str>,
    ) -> Result<IdeaTransmission> {
        let created_at = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO idea_transmissions (idea, source_entity, target_entity, transmission_type, era_id, region_id, video_id, claim_id, notes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                idea,
                source_entity,
                target_entity,
                transmission_type.as_str(),
                era_id,
                region_id,
                video_id,
                claim_id,
                notes,
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(IdeaTransmission {
            id,
            idea: idea.to_string(),
            source_entity: source_entity.to_string(),
            target_entity: target_entity.to_string(),
            transmission_type,
            era_id,
            region_id,
            video_id: video_id.to_string(),
            claim_id,
            notes: notes.map(|s| s.to_string()),
            created_at,
        })
    }

    pub fn get_idea_transmission(&self, id: i64) -> Result<Option<IdeaTransmission>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, idea, source_entity, target_entity, transmission_type, era_id, region_id, video_id, claim_id, notes, created_at FROM idea_transmissions WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_idea_transmission(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_idea_transmissions_by_idea(&self, idea: &str) -> Result<Vec<IdeaTransmission>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, idea, source_entity, target_entity, transmission_type, era_id, region_id, video_id, claim_id, notes, created_at FROM idea_transmissions WHERE idea LIKE ?1 ORDER BY created_at DESC"
        )?;

        let mut transmissions = Vec::new();
        let pattern = format!("%{}%", idea);
        let mut rows = stmt.query(params![pattern])?;

        while let Some(row) = rows.next()? {
            transmissions.push(self.row_to_idea_transmission(row)?);
        }
        Ok(transmissions)
    }

    pub fn list_idea_transmissions_by_type(&self, transmission_type: TransmissionType) -> Result<Vec<IdeaTransmission>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, idea, source_entity, target_entity, transmission_type, era_id, region_id, video_id, claim_id, notes, created_at FROM idea_transmissions WHERE transmission_type = ?1 ORDER BY created_at DESC"
        )?;

        let mut transmissions = Vec::new();
        let mut rows = stmt.query(params![transmission_type.as_str()])?;

        while let Some(row) = rows.next()? {
            transmissions.push(self.row_to_idea_transmission(row)?);
        }
        Ok(transmissions)
    }

    pub fn list_all_idea_transmissions(&self) -> Result<Vec<IdeaTransmission>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, idea, source_entity, target_entity, transmission_type, era_id, region_id, video_id, claim_id, notes, created_at FROM idea_transmissions ORDER BY idea, created_at DESC"
        )?;

        let mut transmissions = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            transmissions.push(self.row_to_idea_transmission(row)?);
        }
        Ok(transmissions)
    }

    pub fn delete_idea_transmission(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM idea_transmissions WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_idea_transmission(&self, row: &rusqlite::Row) -> Result<IdeaTransmission> {
        let transmission_type_str: String = row.get(4)?;
        let created_at: String = row.get(10)?;

        Ok(IdeaTransmission {
            id: row.get(0)?,
            idea: row.get(1)?,
            source_entity: row.get(2)?,
            target_entity: row.get(3)?,
            transmission_type: TransmissionType::from_str(&transmission_type_str).unwrap_or(TransmissionType::Horizontal),
            era_id: row.get(5)?,
            region_id: row.get(6)?,
            video_id: row.get(7)?,
            claim_id: row.get(8)?,
            notes: row.get(9)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // 8.4 Geopolitical Entity Operations

    pub fn create_geopolitical_entity(
        &self,
        name: &str,
        era_id: i64,
        position: SystemPosition,
        notes: Option<&str>,
    ) -> Result<GeopoliticalEntity> {
        let created_at = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO geopolitical_entities (name, era_id, position, notes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                name,
                era_id,
                position.as_str(),
                notes,
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(GeopoliticalEntity {
            id,
            name: name.to_string(),
            era_id,
            position,
            notes: notes.map(|s| s.to_string()),
            created_at,
        })
    }

    pub fn get_geopolitical_entity(&self, id: i64) -> Result<Option<GeopoliticalEntity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, era_id, position, notes, created_at FROM geopolitical_entities WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_geopolitical_entity(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_geopolitical_entity_by_name(&self, name: &str, era_id: i64) -> Result<Option<GeopoliticalEntity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, era_id, position, notes, created_at FROM geopolitical_entities WHERE name = ?1 AND era_id = ?2"
        )?;
        let mut rows = stmt.query(params![name, era_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_geopolitical_entity(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_geopolitical_entities_by_era(&self, era_id: i64) -> Result<Vec<GeopoliticalEntity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, era_id, position, notes, created_at FROM geopolitical_entities WHERE era_id = ?1 ORDER BY position, name"
        )?;

        let mut entities = Vec::new();
        let mut rows = stmt.query(params![era_id])?;

        while let Some(row) = rows.next()? {
            entities.push(self.row_to_geopolitical_entity(row)?);
        }
        Ok(entities)
    }

    pub fn list_geopolitical_entities_by_position(&self, position: SystemPosition) -> Result<Vec<GeopoliticalEntity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, era_id, position, notes, created_at FROM geopolitical_entities WHERE position = ?1 ORDER BY era_id, name"
        )?;

        let mut entities = Vec::new();
        let mut rows = stmt.query(params![position.as_str()])?;

        while let Some(row) = rows.next()? {
            entities.push(self.row_to_geopolitical_entity(row)?);
        }
        Ok(entities)
    }

    pub fn list_all_geopolitical_entities(&self) -> Result<Vec<GeopoliticalEntity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, era_id, position, notes, created_at FROM geopolitical_entities ORDER BY era_id, position, name"
        )?;

        let mut entities = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            entities.push(self.row_to_geopolitical_entity(row)?);
        }
        Ok(entities)
    }

    pub fn update_geopolitical_entity_position(&self, id: i64, position: SystemPosition) -> Result<bool> {
        let affected = self.conn.execute(
            "UPDATE geopolitical_entities SET position = ?1 WHERE id = ?2",
            params![position.as_str(), id],
        )?;
        Ok(affected > 0)
    }

    pub fn delete_geopolitical_entity(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM geopolitical_entities WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_geopolitical_entity(&self, row: &rusqlite::Row) -> Result<GeopoliticalEntity> {
        let position_str: String = row.get(3)?;
        let created_at: String = row.get(5)?;

        Ok(GeopoliticalEntity {
            id: row.get(0)?,
            name: row.get(1)?,
            era_id: row.get(2)?,
            position: SystemPosition::from_str(&position_str).unwrap_or(SystemPosition::Periphery),
            notes: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // Surplus Flow Operations

    pub fn create_surplus_flow(
        &self,
        from_entity_id: i64,
        to_entity_id: i64,
        commodity: &str,
        era_id: i64,
        video_id: Option<&str>,
        claim_id: Option<i64>,
        notes: Option<&str>,
    ) -> Result<SurplusFlow> {
        let created_at = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO surplus_flows (from_entity_id, to_entity_id, commodity, era_id, video_id, claim_id, notes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                from_entity_id,
                to_entity_id,
                commodity,
                era_id,
                video_id,
                claim_id,
                notes,
                created_at.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(SurplusFlow {
            id,
            from_entity_id,
            to_entity_id,
            commodity: commodity.to_string(),
            era_id,
            video_id: video_id.map(|s| s.to_string()),
            claim_id,
            notes: notes.map(|s| s.to_string()),
            created_at,
        })
    }

    pub fn get_surplus_flow(&self, id: i64) -> Result<Option<SurplusFlow>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_entity_id, to_entity_id, commodity, era_id, video_id, claim_id, notes, created_at FROM surplus_flows WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_surplus_flow(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_surplus_flows_for_entity(&self, entity_id: i64) -> Result<Vec<SurplusFlow>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_entity_id, to_entity_id, commodity, era_id, video_id, claim_id, notes, created_at FROM surplus_flows WHERE from_entity_id = ?1 OR to_entity_id = ?1 ORDER BY created_at DESC"
        )?;

        let mut flows = Vec::new();
        let mut rows = stmt.query(params![entity_id])?;

        while let Some(row) = rows.next()? {
            flows.push(self.row_to_surplus_flow(row)?);
        }
        Ok(flows)
    }

    pub fn list_surplus_flows_by_era(&self, era_id: i64) -> Result<Vec<SurplusFlow>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_entity_id, to_entity_id, commodity, era_id, video_id, claim_id, notes, created_at FROM surplus_flows WHERE era_id = ?1 ORDER BY commodity, created_at DESC"
        )?;

        let mut flows = Vec::new();
        let mut rows = stmt.query(params![era_id])?;

        while let Some(row) = rows.next()? {
            flows.push(self.row_to_surplus_flow(row)?);
        }
        Ok(flows)
    }

    pub fn list_all_surplus_flows(&self) -> Result<Vec<SurplusFlow>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_entity_id, to_entity_id, commodity, era_id, video_id, claim_id, notes, created_at FROM surplus_flows ORDER BY era_id, commodity, created_at DESC"
        )?;

        let mut flows = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            flows.push(self.row_to_surplus_flow(row)?);
        }
        Ok(flows)
    }

    pub fn delete_surplus_flow(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM surplus_flows WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_surplus_flow(&self, row: &rusqlite::Row) -> Result<SurplusFlow> {
        let created_at: String = row.get(8)?;

        Ok(SurplusFlow {
            id: row.get(0)?,
            from_entity_id: row.get(1)?,
            to_entity_id: row.get(2)?,
            commodity: row.get(3)?,
            era_id: row.get(4)?,
            video_id: row.get(5)?,
            claim_id: row.get(6)?,
            notes: row.get(7)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // Temporal Observation Operations

    pub fn create_temporal_observation(
        &self,
        claim_id: i64,
        timescale: BraudelTimescale,
        notes: Option<&str>,
    ) -> Result<TemporalObservation> {
        let created_at = Utc::now();
        self.conn.execute(
            "INSERT INTO temporal_observations (claim_id, timescale, notes, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![claim_id, timescale.as_str(), notes, created_at.to_rfc3339()],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(TemporalObservation {
            id,
            claim_id,
            timescale,
            notes: notes.map(|s| s.to_string()),
            created_at,
        })
    }

    pub fn get_temporal_observation(&self, id: i64) -> Result<Option<TemporalObservation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, claim_id, timescale, notes, created_at FROM temporal_observations WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_temporal_observation(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_temporal_observations_for_claim(&self, claim_id: i64) -> Result<Vec<TemporalObservation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, claim_id, timescale, notes, created_at FROM temporal_observations WHERE claim_id = ?1 ORDER BY timescale"
        )?;

        let mut observations = Vec::new();
        let mut rows = stmt.query(params![claim_id])?;

        while let Some(row) = rows.next()? {
            observations.push(self.row_to_temporal_observation(row)?);
        }
        Ok(observations)
    }

    pub fn list_temporal_observations_by_timescale(&self, timescale: BraudelTimescale) -> Result<Vec<TemporalObservation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, claim_id, timescale, notes, created_at FROM temporal_observations WHERE timescale = ?1 ORDER BY created_at DESC"
        )?;

        let mut observations = Vec::new();
        let mut rows = stmt.query(params![timescale.as_str()])?;

        while let Some(row) = rows.next()? {
            observations.push(self.row_to_temporal_observation(row)?);
        }
        Ok(observations)
    }

    pub fn list_all_temporal_observations(&self) -> Result<Vec<TemporalObservation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, claim_id, timescale, notes, created_at FROM temporal_observations ORDER BY timescale, created_at DESC"
        )?;

        let mut observations = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            observations.push(self.row_to_temporal_observation(row)?);
        }
        Ok(observations)
    }

    pub fn delete_temporal_observation(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM temporal_observations WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_temporal_observation(&self, row: &rusqlite::Row) -> Result<TemporalObservation> {
        let timescale_str: String = row.get(2)?;
        let created_at: String = row.get(4)?;

        Ok(TemporalObservation {
            id: row.get(0)?,
            claim_id: row.get(1)?,
            timescale: BraudelTimescale::from_str(&timescale_str).unwrap_or(BraudelTimescale::Event),
            notes: row.get(3)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        })
    }

    // Framework Statistics

    pub fn get_framework_stats(&self) -> Result<FrameworkStats> {
        let cyclical_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM cyclical_indicators", [], |row| row.get(0)
        )?;
        let causal_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM causal_relations", [], |row| row.get(0)
        )?;
        let transmission_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM idea_transmissions", [], |row| row.get(0)
        )?;
        let entity_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM geopolitical_entities", [], |row| row.get(0)
        )?;
        let flow_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM surplus_flows", [], |row| row.get(0)
        )?;
        let temporal_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM temporal_observations", [], |row| row.get(0)
        )?;

        Ok(FrameworkStats {
            cyclical_indicators: cyclical_count,
            causal_relations: causal_count,
            idea_transmissions: transmission_count,
            geopolitical_entities: entity_count,
            surplus_flows: flow_count,
            temporal_observations: temporal_count,
        })
    }

    // Phase 9: Synthesis & Pattern Detection

    // 9.1 Maps of Content

    pub fn create_moc(&self, title: &str, description: Option<&str>) -> Result<MapOfContent> {
        let now = Utc::now();
        self.conn.execute(
            "INSERT INTO mocs (title, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![title, description, now.to_rfc3339(), now.to_rfc3339()],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(MapOfContent {
            id,
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_moc(&self, id: i64) -> Result<Option<MapOfContent>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, created_at, updated_at FROM mocs WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_moc(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_moc_by_title(&self, title: &str) -> Result<Option<MapOfContent>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, created_at, updated_at FROM mocs WHERE title = ?1 COLLATE NOCASE"
        )?;
        let mut rows = stmt.query(params![title])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_moc(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_mocs(&self) -> Result<Vec<MapOfContent>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, created_at, updated_at FROM mocs ORDER BY title"
        )?;

        let mut mocs = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            mocs.push(self.row_to_moc(row)?);
        }
        Ok(mocs)
    }

    pub fn update_moc(&self, id: i64, title: Option<&str>, description: Option<&str>) -> Result<bool> {
        let now = Utc::now();
        let mut updates = vec!["updated_at = ?1"];
        let mut param_idx = 2;

        if title.is_some() {
            updates.push("title = ?2");
            param_idx = 3;
        }
        if description.is_some() {
            if param_idx == 2 {
                updates.push("description = ?2");
            } else {
                updates.push("description = ?3");
            }
        }

        let query = format!("UPDATE mocs SET {} WHERE id = ?{}", updates.join(", "), param_idx);

        let affected = if let (Some(t), Some(d)) = (title, description) {
            self.conn.execute(&query, params![now.to_rfc3339(), t, d, id])?
        } else if let Some(t) = title {
            self.conn.execute(&query, params![now.to_rfc3339(), t, id])?
        } else if let Some(d) = description {
            self.conn.execute(&query, params![now.to_rfc3339(), d, id])?
        } else {
            self.conn.execute("UPDATE mocs SET updated_at = ?1 WHERE id = ?2", params![now.to_rfc3339(), id])?
        };

        Ok(affected > 0)
    }

    pub fn delete_moc(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM mocs WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    pub fn add_claim_to_moc(&self, moc_id: i64, claim_id: i64, sort_order: i32) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "INSERT OR REPLACE INTO moc_claims (moc_id, claim_id, sort_order, added_at) VALUES (?1, ?2, ?3, ?4)",
            params![moc_id, claim_id, sort_order, now.to_rfc3339()],
        )?;
        // Update MOC's updated_at
        self.conn.execute(
            "UPDATE mocs SET updated_at = ?1 WHERE id = ?2",
            params![now.to_rfc3339(), moc_id],
        )?;
        Ok(())
    }

    pub fn remove_claim_from_moc(&self, moc_id: i64, claim_id: i64) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM moc_claims WHERE moc_id = ?1 AND claim_id = ?2",
            params![moc_id, claim_id],
        )?;
        Ok(affected > 0)
    }

    pub fn get_moc_claims(&self, moc_id: i64) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.text, c.video_id, c.timestamp, c.source_quote, c.category, c.confidence, c.created_at
            FROM claims c
            JOIN moc_claims mc ON mc.claim_id = c.id
            WHERE mc.moc_id = ?1
            ORDER BY mc.sort_order, c.created_at
            "#
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query(params![moc_id])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn add_sub_moc(&self, parent_id: i64, child_id: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO moc_hierarchy (parent_moc_id, child_moc_id) VALUES (?1, ?2)",
            params![parent_id, child_id],
        )?;
        Ok(())
    }

    pub fn get_sub_mocs(&self, moc_id: i64) -> Result<Vec<MapOfContent>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT m.id, m.title, m.description, m.created_at, m.updated_at
            FROM mocs m
            JOIN moc_hierarchy h ON h.child_moc_id = m.id
            WHERE h.parent_moc_id = ?1
            ORDER BY m.title
            "#
        )?;

        let mut mocs = Vec::new();
        let mut rows = stmt.query(params![moc_id])?;

        while let Some(row) = rows.next()? {
            mocs.push(self.row_to_moc(row)?);
        }
        Ok(mocs)
    }

    pub fn get_moc_with_claims(&self, moc_id: i64) -> Result<Option<MocWithClaims>> {
        let moc = match self.get_moc(moc_id)? {
            Some(m) => m,
            None => return Ok(None),
        };

        let claims = self.get_moc_claims(moc_id)?;
        let sub_mocs = self.get_sub_mocs(moc_id)?;

        Ok(Some(MocWithClaims { moc, claims, sub_mocs }))
    }

    pub fn get_moc_claim_count(&self, moc_id: i64) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM moc_claims WHERE moc_id = ?1",
            params![moc_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    fn row_to_moc(&self, row: &rusqlite::Row) -> Result<MapOfContent> {
        let created_at: String = row.get(3)?;
        let updated_at: String = row.get(4)?;

        Ok(MapOfContent {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
        })
    }

    // 9.2 Research Questions

    pub fn create_research_question(
        &self,
        question: &str,
        parent_id: Option<i64>,
        notes: Option<&str>,
    ) -> Result<ResearchQuestion> {
        let now = Utc::now();
        self.conn.execute(
            r#"
            INSERT INTO research_questions (question, status, parent_question_id, notes, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                question,
                QuestionStatus::Active.as_str(),
                parent_id,
                notes,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(ResearchQuestion {
            id,
            question: question.to_string(),
            status: QuestionStatus::Active,
            parent_question_id: parent_id,
            notes: notes.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_research_question(&self, id: i64) -> Result<Option<ResearchQuestion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, question, status, parent_question_id, notes, created_at, updated_at FROM research_questions WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_research_question(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_research_questions(&self, status: Option<QuestionStatus>) -> Result<Vec<ResearchQuestion>> {
        let questions = if let Some(s) = status {
            let mut stmt = self.conn.prepare(
                "SELECT id, question, status, parent_question_id, notes, created_at, updated_at FROM research_questions WHERE status = ?1 ORDER BY created_at DESC"
            )?;
            let mut rows = stmt.query(params![s.as_str()])?;
            let mut qs = Vec::new();
            while let Some(row) = rows.next()? {
                qs.push(self.row_to_research_question(row)?);
            }
            qs
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, question, status, parent_question_id, notes, created_at, updated_at FROM research_questions ORDER BY status, created_at DESC"
            )?;
            let mut rows = stmt.query([])?;
            let mut qs = Vec::new();
            while let Some(row) = rows.next()? {
                qs.push(self.row_to_research_question(row)?);
            }
            qs
        };
        Ok(questions)
    }

    pub fn update_question_status(&self, id: i64, status: QuestionStatus) -> Result<bool> {
        let now = Utc::now();
        let affected = self.conn.execute(
            "UPDATE research_questions SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![status.as_str(), now.to_rfc3339(), id],
        )?;
        Ok(affected > 0)
    }

    pub fn delete_research_question(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM research_questions WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    pub fn add_evidence_to_question(
        &self,
        question_id: i64,
        claim_id: Option<i64>,
        video_id: Option<&str>,
        relevance: Option<&str>,
    ) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "INSERT INTO question_evidence (question_id, claim_id, video_id, relevance, added_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![question_id, claim_id, video_id, relevance, now.to_rfc3339()],
        )?;
        // Update question's updated_at
        self.conn.execute(
            "UPDATE research_questions SET updated_at = ?1 WHERE id = ?2",
            params![now.to_rfc3339(), question_id],
        )?;
        Ok(())
    }

    pub fn get_question_evidence_claims(&self, question_id: i64) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.text, c.video_id, c.timestamp, c.source_quote, c.category, c.confidence, c.created_at
            FROM claims c
            JOIN question_evidence qe ON qe.claim_id = c.id
            WHERE qe.question_id = ?1
            ORDER BY qe.added_at DESC
            "#
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query(params![question_id])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn get_question_evidence_videos(&self, question_id: i64) -> Result<Vec<Video>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT v.id, v.url, v.title, v.channel, v.upload_date, v.description, v.added_at
            FROM videos v
            JOIN question_evidence qe ON qe.video_id = v.id
            WHERE qe.question_id = ?1
            ORDER BY qe.added_at DESC
            "#
        )?;

        let mut videos = Vec::new();
        let mut rows = stmt.query(params![question_id])?;

        while let Some(row) = rows.next()? {
            videos.push(self.row_to_video(row)?);
        }
        Ok(videos)
    }

    pub fn get_sub_questions(&self, parent_id: i64) -> Result<Vec<ResearchQuestion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, question, status, parent_question_id, notes, created_at, updated_at FROM research_questions WHERE parent_question_id = ?1 ORDER BY created_at"
        )?;

        let mut questions = Vec::new();
        let mut rows = stmt.query(params![parent_id])?;

        while let Some(row) = rows.next()? {
            questions.push(self.row_to_research_question(row)?);
        }
        Ok(questions)
    }

    pub fn get_question_with_evidence(&self, question_id: i64) -> Result<Option<QuestionWithEvidence>> {
        let question = match self.get_research_question(question_id)? {
            Some(q) => q,
            None => return Ok(None),
        };

        let claims = self.get_question_evidence_claims(question_id)?;
        let videos = self.get_question_evidence_videos(question_id)?;
        let sub_questions = self.get_sub_questions(question_id)?;

        Ok(Some(QuestionWithEvidence {
            question,
            claims,
            videos,
            sub_questions,
        }))
    }

    fn row_to_research_question(&self, row: &rusqlite::Row) -> Result<ResearchQuestion> {
        let status_str: String = row.get(2)?;
        let created_at: String = row.get(5)?;
        let updated_at: String = row.get(6)?;

        Ok(ResearchQuestion {
            id: row.get(0)?,
            question: row.get(1)?,
            status: QuestionStatus::from_str(&status_str).unwrap_or(QuestionStatus::Active),
            parent_question_id: row.get(3)?,
            notes: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
        })
    }

    // 9.3 Pattern Detection

    pub fn save_detected_pattern(
        &self,
        pattern_type: PatternType,
        description: &str,
        video_ids: &[String],
        claim_ids: &[i64],
        confidence: f32,
    ) -> Result<DetectedPattern> {
        let now = Utc::now();
        let video_ids_json = serde_json::to_string(video_ids)?;
        let claim_ids_json = serde_json::to_string(claim_ids)?;

        self.conn.execute(
            r#"
            INSERT INTO detected_patterns (pattern_type, description, video_ids_json, claim_ids_json, confidence, detected_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                pattern_type.as_str(),
                description,
                video_ids_json,
                claim_ids_json,
                confidence,
                now.to_rfc3339(),
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(DetectedPattern {
            id,
            pattern_type,
            description: description.to_string(),
            video_ids: video_ids.to_vec(),
            claim_ids: claim_ids.to_vec(),
            confidence,
            detected_at: now,
        })
    }

    pub fn get_detected_pattern(&self, id: i64) -> Result<Option<DetectedPattern>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, pattern_type, description, video_ids_json, claim_ids_json, confidence, detected_at FROM detected_patterns WHERE id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_detected_pattern(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_detected_patterns(&self, pattern_type: Option<PatternType>) -> Result<Vec<DetectedPattern>> {
        let patterns = if let Some(pt) = pattern_type {
            let mut stmt = self.conn.prepare(
                "SELECT id, pattern_type, description, video_ids_json, claim_ids_json, confidence, detected_at FROM detected_patterns WHERE pattern_type = ?1 ORDER BY detected_at DESC"
            )?;
            let mut rows = stmt.query(params![pt.as_str()])?;
            let mut ps = Vec::new();
            while let Some(row) = rows.next()? {
                ps.push(self.row_to_detected_pattern(row)?);
            }
            ps
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, pattern_type, description, video_ids_json, claim_ids_json, confidence, detected_at FROM detected_patterns ORDER BY detected_at DESC"
            )?;
            let mut rows = stmt.query([])?;
            let mut ps = Vec::new();
            while let Some(row) = rows.next()? {
                ps.push(self.row_to_detected_pattern(row)?);
            }
            ps
        };
        Ok(patterns)
    }

    pub fn delete_detected_pattern(&self, id: i64) -> Result<bool> {
        let affected = self.conn.execute("DELETE FROM detected_patterns WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    fn row_to_detected_pattern(&self, row: &rusqlite::Row) -> Result<DetectedPattern> {
        let pattern_type_str: String = row.get(1)?;
        let video_ids_json: String = row.get(3)?;
        let claim_ids_json: String = row.get(4)?;
        let detected_at: String = row.get(6)?;

        Ok(DetectedPattern {
            id: row.get(0)?,
            pattern_type: PatternType::from_str(&pattern_type_str).unwrap_or(PatternType::RecurringTheme),
            description: row.get(2)?,
            video_ids: serde_json::from_str(&video_ids_json)?,
            claim_ids: serde_json::from_str(&claim_ids_json)?,
            confidence: row.get(5)?,
            detected_at: DateTime::parse_from_rfc3339(&detected_at)?.with_timezone(&Utc),
        })
    }

    // 9.4 Review System

    pub fn record_claim_access(&self, claim_id: i64) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "INSERT OR REPLACE INTO claim_access (claim_id, last_accessed) VALUES (?1, ?2)",
            params![claim_id, now.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn get_stale_claims(&self, days: i64) -> Result<Vec<Claim>> {
        let cutoff = Utc::now() - chrono::Duration::days(days);

        // Get claims that either:
        // 1. Have never been accessed (not in claim_access table)
        // 2. Were last accessed before the cutoff
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.id, c.text, c.video_id, c.timestamp, c.source_quote, c.category, c.confidence, c.created_at
            FROM claims c
            LEFT JOIN claim_access ca ON ca.claim_id = c.id
            WHERE ca.claim_id IS NULL
               OR ca.last_accessed < ?1
            ORDER BY COALESCE(ca.last_accessed, c.created_at)
            "#
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query(params![cutoff.to_rfc3339()])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn get_orphan_claims(&self) -> Result<Vec<Claim>> {
        // Claims with fewer than 2 links
        self.get_unlinked_claims()
    }

    pub fn get_random_claims(&self, count: usize) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, video_id, timestamp, source_quote, category, confidence, created_at FROM claims ORDER BY RANDOM() LIMIT ?1"
        )?;

        let mut claims = Vec::new();
        let mut rows = stmt.query(params![count as i64])?;

        while let Some(row) = rows.next()? {
            claims.push(self.row_to_claim(row)?);
        }
        Ok(claims)
    }

    pub fn get_review_queue(&self, stale_days: i64, random_count: usize) -> Result<ReviewQueue> {
        let stale_claims = self.get_stale_claims(stale_days)?;
        let orphan_claims = self.get_orphan_claims()?;
        let random_suggestions = self.get_random_claims(random_count)?;

        Ok(ReviewQueue {
            stale_claims,
            orphan_claims,
            random_suggestions,
        })
    }

    // 9.5 Synthesis Statistics

    pub fn get_synthesis_stats(&self) -> Result<SynthesisStats> {
        let moc_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM mocs", [], |row| row.get(0)
        )?;

        let question_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM research_questions", [], |row| row.get(0)
        )?;

        let active_question_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM research_questions WHERE status = 'active'", [], |row| row.get(0)
        )?;

        let pattern_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM detected_patterns", [], |row| row.get(0)
        )?;

        let stale_count = self.get_stale_claims(30)?.len() as i64;
        let orphan_count = self.get_orphan_claims()?.len() as i64;

        Ok(SynthesisStats {
            mocs: moc_count,
            research_questions: question_count,
            active_questions: active_question_count,
            detected_patterns: pattern_count,
            stale_claims: stale_count,
            orphan_claims: orphan_count,
        })
    }
}

// Cosine similarity helper function
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
