# Engine Mate Stats TODO List

This project is a Rust-based replacement for [matetrack](https://github.com/vondele/matetrack). Below is a prioritized list of tasks to guide development. Please update this list as the project evolves.

## TODO (in order of importance)

1. **Implement UCI Engine Communication**
   - Launch and communicate with UCI chess engines
   - Send/receive UCI commands and parse responses
   - Required UCI commands to implement:
     - `uci` (initialize engine, get options)
     - `isready` (check engine readiness)
     - `setoption` (set engine options)
     - `ucinewgame` (signal start of new game)
     - `position` (set up position from FEN)
     - `go` (start analysis/search)
     - `stop` (stop analysis/search)
     - `quit` (terminate engine)
     - (Parse responses: `info`, `bestmove`, etc.)

2. **FEN Input and Management**
   - Read FEN positions from file (e.g., FENs.json)
   - Validate and manage FEN data

3. **Engine Result Parsing and Storage**
   - Parse engine output for mate scores and moves
   - Store results in a structured format (e.g., JSON, CSV)

4. **Analysis Orchestration**
   - Run analysis for all FENs with configurable engine options
   - Support multi-threaded or parallel analysis

5. **Result Aggregation and Statistics**
   - Aggregate mate statistics from engine results
   - Compute and display summary statistics (e.g., mate found, depth, time)

6. **Command-Line Interface (CLI)**
   - Provide a user-friendly CLI for configuration and execution
   - Support options for engine path, FEN file, output file, etc.

7. **Error Handling and Logging**
   - Robust error handling for engine crashes, timeouts, invalid FENs
   - Logging of analysis progress and issues

8. **Testing and Validation**
   - Unit and integration tests for all modules
   - Cross-validation with matetrack results

9. **Documentation**
   - Document code, usage, and configuration
   - Provide examples and troubleshooting tips

10. **Performance Optimization**
    - Profile and optimize for speed and memory usage
    - Support for large FEN sets and long-running analyses

11. **Optional: GUI or Web Interface**
    - (Future) Add a graphical or web-based interface for easier use

---

*Last updated: 2026-02-01*
