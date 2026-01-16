use eframe::egui;
use egui_plot::Plot;
use crate::jssp::{generate_random_instance, JsspSolver, ScheduledOperation};
use std::collections::HashSet;

pub struct JsspApp {
    solver: Option<JsspSolver>,
    schedule: Vec<ScheduledOperation>,
    makespan: f64,
    num_jobs: usize,
    num_machines: usize,
    min_duration: f64,
    max_duration: f64,
    hidden_jobs: HashSet<usize>,
    show_export_dialog: bool,
}

impl Default for JsspApp {
    fn default() -> Self {
        Self {
            solver: None,
            schedule: Vec::new(),
            makespan: 0.0,
            num_jobs: 5,
            num_machines: 3,
            min_duration: 1.0,
            max_duration: 10.0,
            hidden_jobs: HashSet::new(),
            show_export_dialog: false,
        }
    }
}

impl eframe::App for JsspApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Configure better text rendering and sizing
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(24.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Monospace)),
        ].into();
        
        // Make UI elements more visible
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        ctx.set_style(style);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Job Shop Scheduling Problem - Greedy Solver");
            ui.separator();

            // Control panel
            ui.horizontal(|ui| {
                ui.label("Number of Jobs:");
                ui.add(egui::Slider::new(&mut self.num_jobs, 2..=40));
                
                ui.separator();
                
                ui.label("Number of Machines:");
                ui.add(egui::Slider::new(&mut self.num_machines, 2..=20));
            });

            ui.horizontal(|ui| {
                ui.label("Min Duration:");
                if ui.add(egui::Slider::new(&mut self.min_duration, 1.0..=50.0)).changed() {
                    // Ensure min is always less than max
                    if self.min_duration >= self.max_duration {
                        self.max_duration = self.min_duration + 1.0;
                    }
                }
                
                ui.separator();
                
                ui.label("Max Duration:");
                if ui.add(egui::Slider::new(&mut self.max_duration, 1.0..=100.0)).changed() {
                    // Ensure max is always greater than min
                    if self.max_duration <= self.min_duration {
                        self.min_duration = (self.max_duration - 1.0).max(1.0);
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.add_sized([180.0, 32.0], egui::Button::new("Generate Problem")).clicked() {
                    let jobs = generate_random_instance(
                        self.num_jobs,
                        self.num_machines,
                        self.min_duration,
                        self.max_duration,
                    );
                    self.solver = Some(JsspSolver::new(jobs, self.num_machines));
                    self.schedule.clear();
                    self.makespan = 0.0;
                    self.hidden_jobs.clear();
                }

                if ui.add_sized([180.0, 32.0], egui::Button::new("Solve Schedule")).clicked() {
                    if let Some(solver) = &self.solver {
                        self.schedule = solver.solve_greedy();
                        self.makespan = solver.calculate_makespan(&self.schedule);
                    }
                }

                if ui.add_sized([180.0, 32.0], egui::Button::new("Export Solution")).clicked() {
                    if !self.schedule.is_empty() {
                        self.show_export_dialog = true;
                    }
                }

                if ui.add_sized([120.0, 32.0], egui::Button::new("Clear All")).clicked() {
                    self.solver = None;
                    self.schedule.clear();
                    self.makespan = 0.0;
                    self.hidden_jobs.clear();
                }
            });

            ui.separator();

            // Display problem information
            if let Some(solver) = &self.solver {
                ui.label(format!(
                    "Problem: {} jobs, {} machines, {} total operations",
                    solver.jobs.len(),
                    solver.num_machines,
                    solver.jobs.iter().map(|j| j.operations.len()).sum::<usize>()
                ));

                if !self.schedule.is_empty() {
                    ui.colored_label(
                        egui::Color32::GREEN,
                        format!("✓ Solution found! Makespan: {:.2}", self.makespan)
                    );
                }
            } else {
                ui.colored_label(
                    egui::Color32::GRAY,
                    "No problem loaded. Click 'Generate Problem' to start."
                );
            }

            ui.separator();

            // Gantt Chart
            if !self.schedule.is_empty() {
                ui.heading("Gantt Chart (by Machine)");
                
                self.render_gantt_chart(ui);
            }
        });

        // Export dialog window
        if self.show_export_dialog {
            egui::Window::new("Export Solution")
                .collapsible(false)
                .resizable(false)
                .default_width(400.0)
                .show(ctx, |ui| {
                    ui.heading("Choose Export Format");
                    ui.add_space(10.0);
                    
                    ui.label("Select the format you want to export:");
                    ui.add_space(10.0);

                    if ui.add_sized([360.0, 30.0], egui::Button::new("JSON - Structured Data")).clicked() {
                        self.export_with_dialog("json");
                        self.show_export_dialog = false;
                    }
                    ui.small("Complete data with metadata for programmatic use");
                    ui.add_space(8.0);

                    if ui.add_sized([360.0, 30.0], egui::Button::new("CSV - Spreadsheet")).clicked() {
                        self.export_with_dialog("csv");
                        self.show_export_dialog = false;
                    }
                    ui.small("Table format compatible with Excel and analysis tools");
                    ui.add_space(8.0);

                    if ui.add_sized([360.0, 30.0], egui::Button::new("TXT - Summary Report")).clicked() {
                        self.export_with_dialog("txt");
                        self.show_export_dialog = false;
                    }
                    ui.small("Human-readable summary with formatted table");
                    ui.add_space(8.0);

                    if ui.add_sized([360.0, 30.0], egui::Button::new("ALL - Export All Formats")).clicked() {
                        self.export_with_dialog("all");
                        self.show_export_dialog = false;
                    }
                    ui.small("Save JSON, CSV, and TXT together to a folder");
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(5.0);
                    
                    if ui.add_sized([360.0, 30.0], egui::Button::new("Cancel")).clicked() {
                        self.show_export_dialog = false;
                    }
                });
        }
    }
}

impl JsspApp {
    fn render_gantt_chart(&mut self, ui: &mut egui::Ui) {
        let colors = [
            egui::Color32::from_rgb(255, 99, 71),    // Tomato
            egui::Color32::from_rgb(70, 130, 180),   // Steel Blue
            egui::Color32::from_rgb(60, 179, 113),   // Medium Sea Green
            egui::Color32::from_rgb(255, 165, 0),    // Orange
            egui::Color32::from_rgb(147, 112, 219),  // Medium Purple
            egui::Color32::from_rgb(255, 215, 0),    // Gold
            egui::Color32::from_rgb(220, 20, 60),    // Crimson
            egui::Color32::from_rgb(0, 191, 255),    // Deep Sky Blue
            egui::Color32::from_rgb(50, 205, 50),    // Lime Green
            egui::Color32::from_rgb(255, 105, 180),  // Hot Pink
            egui::Color32::from_rgb(138, 43, 226),   // Blue Violet
            egui::Color32::from_rgb(255, 140, 0),    // Dark Orange
            egui::Color32::from_rgb(72, 209, 204),   // Medium Turquoise
            egui::Color32::from_rgb(199, 21, 133),   // Medium Violet Red
            egui::Color32::from_rgb(0, 206, 209),    // Dark Turquoise
            egui::Color32::from_rgb(255, 69, 0),     // Red Orange
            egui::Color32::from_rgb(186, 85, 211),   // Medium Orchid
            egui::Color32::from_rgb(34, 139, 34),    // Forest Green
            egui::Color32::from_rgb(255, 20, 147),   // Deep Pink
            egui::Color32::from_rgb(30, 144, 255),   // Dodger Blue
        ];

        // Create custom legend with colored circles and clickable job names
        ui.horizontal(|ui| {
            ui.label("Jobs:");
            let unique_jobs: HashSet<usize> = self.schedule.iter().map(|op| op.job_id).collect();
            let mut sorted_jobs: Vec<usize> = unique_jobs.into_iter().collect();
            sorted_jobs.sort();
            
            for job_id in sorted_jobs {
                let color = colors[job_id % colors.len()];
                let is_hidden = self.hidden_jobs.contains(&job_id);
                
                ui.horizontal(|ui| {
                    // Draw colored circle
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(12.0, 12.0),
                        egui::Sense::click()
                    );
                    
                    if is_hidden {
                        ui.painter().circle_stroke(
                            rect.center(),
                            6.0,
                            egui::Stroke::new(2.0, color)
                        );
                    } else {
                        ui.painter().circle_filled(
                            rect.center(),
                            6.0,
                            color
                        );
                    }
                    
                    // Job label
                    let label_response = ui.selectable_label(false, format!("Job {}", job_id));
                    
                    // Toggle visibility on click
                    if response.clicked() || label_response.clicked() {
                        if is_hidden {
                            self.hidden_jobs.remove(&job_id);
                        } else {
                            self.hidden_jobs.insert(job_id);
                        }
                    }
                    
                    // Show tooltip
                    if response.hovered() || label_response.hovered() {
                        response.on_hover_text(format!("Click to {} Job {}", if is_hidden { "show" } else { "hide" }, job_id));
                    }
                });
            }
        });

        ui.add_space(5.0);

        let plot_response = Plot::new("gantt_chart")
            .height(400.0)
            .show_axes([true, true])
            .show_grid([true, true])  // Show grid for better readability
            .y_axis_label("Machine")
            .x_axis_label("Time (units)")
            .label_formatter(|name, value| {
                if !name.is_empty() {
                    format!("{}", name)
                } else {
                    format!("Time: {:.1}\nMachine: {:.0}", value.x, value.y)
                }
            })
            .allow_drag(true)  // Enable panning
            .allow_zoom(true)  // Enable zoom
            .allow_scroll(true)  // Enable scroll wheel zoom
            .show(ui, |plot_ui| {
                // Group operations by machine
                for machine_id in 0..self.num_machines {
                    let machine_ops: Vec<&ScheduledOperation> = self.schedule.iter()
                        .filter(|op| op.machine_id == machine_id)
                        .collect();

                    for op in machine_ops {
                        // Skip hidden jobs
                        if self.hidden_jobs.contains(&op.job_id) {
                            continue;
                        }

                        let color = colors[op.job_id % colors.len()];
                        
                        let y_pos = machine_id as f64;
                        let height = 0.8;
                        
                        // Draw operation as a rectangle
                        let points = vec![
                            [op.start_time, y_pos - height/2.0],
                            [op.end_time, y_pos - height/2.0],
                            [op.end_time, y_pos + height/2.0],
                            [op.start_time, y_pos + height/2.0],
                        ];
                        
                        plot_ui.polygon(
                            egui_plot::Polygon::new(points)
                                .fill_color(color)
                                .name(format!(
                                    "Job {} | Op {} | Machine {} | {:.1}->{:.1} ({:.1})",
                                    op.job_id,
                                    op.operation_id,
                                    op.machine_id,
                                    op.start_time,
                                    op.end_time,
                                    op.duration
                                ))
                        );

                        // Add text label - only show if block is wide enough
                        let block_width = op.end_time - op.start_time;
                        if block_width > 2.0 {  // Only show text if block is wide enough
                            let text_content = if block_width > 8.0 {
                                format!("Job {}", op.job_id)
                            } else {
                                format!("J{}", op.job_id)
                            };
                            
                            plot_ui.text(
                                egui_plot::Text::new(
                                    egui_plot::PlotPoint::new(
                                        (op.start_time + op.end_time) / 2.0,
                                        y_pos
                                    ),
                                    text_content
                                )
                                .color(egui::Color32::WHITE)
                                .name("")  // Empty name so text doesn't create duplicate tooltip
                            );
                        }
                    }
                }
            });

        // Show hover details in a separate area
        if let Some(pointer_pos) = plot_response.response.hover_pos() {
            let plot_pos = plot_response.transform.value_from_position(pointer_pos);
            // Find if we're hovering over any operation
            for op in &self.schedule {
                if self.hidden_jobs.contains(&op.job_id) {
                    continue;
                }
                
                let y_pos = op.machine_id as f64;
                let height = 0.8;
                
                // Check if pointer is inside this operation's rectangle
                if plot_pos.x >= op.start_time && plot_pos.x <= op.end_time
                    && plot_pos.y >= (y_pos - height/2.0) && plot_pos.y <= (y_pos + height/2.0) {
                    
                    plot_response.response.on_hover_ui(|ui| {
                        ui.set_max_width(250.0);
                        let color = colors[op.job_id % colors.len()];
                        ui.horizontal(|ui| {
                            ui.painter().circle_filled(
                                ui.cursor().center_top() + egui::vec2(6.0, 6.0),
                                5.0,
                                color
                            );
                            ui.add_space(15.0);
                            ui.heading(format!("Job {}", op.job_id));
                        });
                        ui.separator();
                        ui.label(format!("Operation: {}", op.operation_id));
                        ui.label(format!("Machine: {}", op.machine_id));
                        ui.label(format!("Start Time: {:.2}", op.start_time));
                        ui.label(format!("End Time: {:.2}", op.end_time));
                        ui.label(format!("Duration: {:.2}", op.duration));
                    });
                    break;
                }
            }
        }

        // Job information table
        ui.separator();
        ui.heading("Schedule Details");
        
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                egui::Grid::new("schedule_grid")
                    .striped(true)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Job");
                        ui.label("Operation");
                        ui.label("Machine");
                        ui.label("Start");
                        ui.label("End");
                        ui.label("Duration");
                        ui.end_row();

                        for op in &self.schedule {
                            ui.label(format!("{}", op.job_id));
                            ui.label(format!("{}", op.operation_id));
                            ui.label(format!("{}", op.machine_id));
                            ui.label(format!("{:.2}", op.start_time));
                            ui.label(format!("{:.2}", op.end_time));
                            ui.label(format!("{:.2}", op.duration));
                            ui.end_row();
                        }
                    });
            });
    }

    fn export_with_dialog(&self, format: &str) {
        use chrono::Local;
        use rfd::FileDialog;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        
        match format {
            "json" => {
                if let Some(path) = FileDialog::new()
                    .set_file_name(&format!("jssp_solution_{}.json", timestamp))
                    .add_filter("JSON", &["json"])
                    .save_file() 
                {
                    self.export_json(&path.to_string_lossy());
                }
            }
            "csv" => {
                if let Some(path) = FileDialog::new()
                    .set_file_name(&format!("jssp_solution_{}.csv", timestamp))
                    .add_filter("CSV", &["csv"])
                    .save_file()
                {
                    self.export_csv(&path.to_string_lossy());
                }
            }
            "txt" => {
                if let Some(path) = FileDialog::new()
                    .set_file_name(&format!("jssp_summary_{}.txt", timestamp))
                    .add_filter("Text", &["txt"])
                    .save_file()
                {
                    self.export_summary(&path.to_string_lossy());
                }
            }
            "all" => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    let dir_path = dir.to_string_lossy();
                    let json_path = format!("{}/jssp_solution_{}.json", dir_path, timestamp);
                    let csv_path = format!("{}/jssp_solution_{}.csv", dir_path, timestamp);
                    let txt_path = format!("{}/jssp_summary_{}.txt", dir_path, timestamp);
                    
                    self.export_json(&json_path);
                    self.export_csv(&csv_path);
                    self.export_summary(&txt_path);
                }
            }
            _ => {}
        }
    }

    fn export_json(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;
        use chrono::Local;

        match serde_json::to_string_pretty(&serde_json::json!({
            "metadata": {
                "timestamp": Local::now().to_rfc3339(),
                "num_jobs": self.num_jobs,
                "num_machines": self.num_machines,
                "makespan": self.makespan,
                "algorithm": "Greedy"
            },
            "schedule": self.schedule
        })) {
            Ok(json_content) => {
                if let Ok(mut file) = File::create(path) {
                    if file.write_all(json_content.as_bytes()).is_ok() {
                        println!("✓ Exported JSON to {}", path);
                    }
                }
            }
            Err(e) => println!("Failed to serialize JSON: {}", e),
        }
    }

    fn export_csv(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;

        if let Ok(mut file) = File::create(path) {
            let mut csv_content = String::from("Job,Operation,Machine,Start Time,End Time,Duration\n");
            for op in &self.schedule {
                csv_content.push_str(&format!(
                    "{},{},{},{:.2},{:.2},{:.2}\n",
                    op.job_id, op.operation_id, op.machine_id, 
                    op.start_time, op.end_time, op.duration
                ));
            }
            if file.write_all(csv_content.as_bytes()).is_ok() {
                println!("✓ Exported CSV to {}", path);
            }
        }
    }

    fn export_summary(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;
        use chrono::Local;

        if let Ok(mut file) = File::create(path) {
            let summary = format!(
                "JSSP Solution Summary\n\
                =====================\n\
                Timestamp: {}\n\
                Algorithm: Greedy\n\
                Number of Jobs: {}\n\
                Number of Machines: {}\n\
                Total Operations: {}\n\
                Makespan: {:.2}\n\
                \n\
                Schedule Details:\n\
                -----------------\n",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                self.num_jobs,
                self.num_machines,
                self.schedule.len(),
                self.makespan
            );
            
            let mut full_content = summary;
            full_content.push_str("Job | Op | Machine | Start  | End    | Duration\n");
            full_content.push_str("----+----+---------+--------+--------+---------\n");
            
            for op in &self.schedule {
                full_content.push_str(&format!(
                    "{:3} | {:2} | {:7} | {:6.2} | {:6.2} | {:6.2}\n",
                    op.job_id, op.operation_id, op.machine_id,
                    op.start_time, op.end_time, op.duration
                ));
            }
            
            if file.write_all(full_content.as_bytes()).is_ok() {
                println!("✓ Exported summary to {}", path);
            }
        }
    }
}
