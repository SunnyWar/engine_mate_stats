# Engine Mate Stats TODO List

This project is a Rust-based replacement for [matetrack](https://github.com/vondele/matetrack). Below is a prioritized list of tasks to guide development. Please update this list as the project evolves.

## TODO (in order of importance)

### 1. Core Functionality
1.4 Analysis Orchestration [ ]
   - [ ] Support multi-threaded or parallel analysis (map to --concurrency, --threads)

### 2. CLI and User Experience
2.1 Command-Line Interface (CLI) Parity [ ]
   - [ ] Support all matecheck.py CLI options:
      - [ ] --engine (engine binary path)
      - [ ] --nodes (nodes limit per position)
      - [ ] --depth (depth limit per position)
      - [ ] --time (time limit per position)
      - [ ] --mate (mate limit per position)
      - [ ] --hash (hash table size)
      - [ ] --threads (threads per position)
      - [ ] --syzygyPath (tablebase path)
      - [ ] --syzygy50MoveRule (50-move rule for Syzygy)
      - [ ] --maxTBscore (max TB win score)
      - [ ] --minTBscore (min TB win score)
      - [ ] --maxValidMate (max mate score)
      - [ ] --minValidMate (min mate score)
      - [ ] --concurrency (total threads)
      - [ ] --engineOpts (engine options as JSON)
      - [ ] --epdFile (input file(s))
      - [ ] --showAllIssues (show all unique UCI info lines with an issue)
      - [ ] --shortTBPVonly (only consider short PVs an issue)
      - [ ] --showAllStats (show nodes/depth stats)
      - [ ] --bench (cumulative stats)
      - [ ] --logFile (log engine output)
   - [ ] Print help/usage message matching matecheck.py

### 3. Reliability and Safety
3.1 Error Handling and Logging [ ]
   - [ ] Robust error handling for engine crashes, timeouts, invalid FENs
   - [ ] Logging of analysis progress and issues
   - [ ] Log engine output to file (--logFile)
   - [ ] Safety: Replace all instances of .unwrap() in UCI parsing with proper Error handling
3.2 Engine Reliability (Stability) [ ]
   - [ ] Implement result-based parsing: use match/if let and a custom UciParseError enum to skip bad lines instead of panicking
   - [ ] Use tokio::process::Command for async engine spawning and per-FEN timeouts; kill hung engines automatically
   - [ ] Durability: Implement Drop trait for Engine processes to prevent orphaned processes
   - [ ] Implement Drop for EngineInstance to ensure kill() is sent to engine process on error or scope exit

### 4. Performance and Concurrency
4.1 Fearless Concurrency (Speed) [ ]
   - [ ] Refactor: Replace sequential loop with Rayon parallel iterator
   - [ ] Implement parallel analysis using Rayon or Tokio (.par_iter() or async tasks)
   - [ ] Resource Management: Add a "Max Threads" CLI argument and implement a Semaphore
   - [ ] Use atomic aggregators (Arc<Mutex<Stats>> or AtomicU64) for thread-safe statistics
   - [ ] Implement a semaphore guard (tokio::sync::Semaphore) to limit active engine processes

### 5. Advanced Features and Statistics
5.1 Result Aggregation and Statistics [ ]
   - [ ] Show nodes and depth statistics for best mates found (--showAllStats)
   - [ ] Provide cumulative statistics for nodes searched and time used (--bench)
5.2 Advanced Statistics (Precision) [ ]
   - [ ] Use std::time::Instant for high-precision timing
   - [ ] Implement custom parser for UCI info strings to extract nps, hashfull, etc.

### 6. Portability and Distribution
6.1 Zero Dependencies (Portability) [ ]
   - [ ] Configure Cargo.toml for static linking (musl target for Linux)
   - [ ] Embed default configuration or FEN set using include_str!
6.2 CI/CD [ ]
   - [ ] Set up a GitHub Action to compile static binaries for Windows, Linux, and macOS

### 7. Testing and Documentation
7.1 Testing and Validation [ ]
   - [ ] Unit and integration tests for all modules
   - [ ] Cross-validation with matetrack results
7.2 Documentation [ ]
   - [ ] Document code, usage, and configuration
   - [ ] Provide examples and troubleshooting tips
   - [ ] Document all CLI options and their mapping to matecheck.py

### 8. Optional
8.1 Optional: GUI or Web Interface [ ]
   - [ ] (Future) Add a graphical or web-based interface for easier use
8.2 Optional: Graphing Capability [ ]
   - [ ] Implement graphing and visualization features similar to matetrack (e.g., result plots, statistics charts)

---

*Last updated: 2026-02-01*
