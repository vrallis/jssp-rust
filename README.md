# JSSP Scheduler - Job Shop Scheduling Problem Solver

A Rust GUI application for solving Job Shop Scheduling Problems (JSSP) using a greedy algorithm, with Gantt chart visualization.

## Features

- **Greedy Algorithm Solver**: Implements a greedy scheduling algorithm for JSSP
- **Interactive GUI**: Built with egui for a responsive user experience
- **Gantt Chart Visualization**: Visual representation of the schedule showing jobs across machines over time
- **Random Problem Generator**: Create random JSSP instances with configurable parameters
- **Real-time Scheduling**: Generate and solve problems on-the-fly

## What is JSSP?

The Job Shop Scheduling Problem involves scheduling a set of jobs on a set of machines, where:
- Each job consists of a sequence of operations
- Each operation must be processed on a specific machine
- Each operation has a duration
- Operations of the same job must be processed in order
- Each machine can process only one operation at a time

The goal is to minimize the **makespan** (total completion time).

## Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

1. **Configure Problem Size**:
   - Adjust the number of jobs (2-20)
   - Adjust the number of machines (2-10)
   - Set min/max operation durations

2. **Generate Problem**:
   - Click "🎲 Generate Random Problem" to create a new JSSP instance

3. **Solve**:
   - Click "▶️ Solve with Greedy Algorithm" to compute a solution

4. **View Results**:
   - The Gantt chart shows the schedule visually
   - Each color represents a different job
   - The x-axis shows time, y-axis shows machines
   - The schedule details table shows all operations with their timings

## Algorithm

The greedy algorithm used here schedules operations in the order they appear in each job:
- For each job, process operations sequentially
- Schedule each operation at the earliest time when both:
  - The required machine is available
  - The previous operation of the same job is complete

While this approach is simple and fast, it may not produce optimal solutions for complex instances.

## Dependencies

- `eframe`: GUI framework
- `egui`: Immediate mode GUI library
- `egui_plot`: Plotting widgets for egui
- `rand`: Random number generation
- `chrono`: Date and time utilities

## Future Enhancements

- Implement more sophisticated algorithms (simulated annealing, genetic algorithms)
- Add ability to load/save problem instances
- Export schedules to various formats
- Performance metrics and comparison tools
- Manual problem editing interface

## License

MIT
