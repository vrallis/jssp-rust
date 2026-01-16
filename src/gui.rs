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
        }
    }
}

impl eframe::App for JsspApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                if ui.button("🎲 Generate Random Problem").clicked() {
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

                if ui.button("▶️ Solve with Greedy Algorithm").clicked() {
                    if let Some(solver) = &self.solver {
                        self.schedule = solver.solve_greedy();
                        self.makespan = solver.calculate_makespan(&self.schedule);
                    }
                }

                if ui.button("🗑️ Clear").clicked() {
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
                    "No problem loaded. Click 'Generate Random Problem' to start."
                );
            }

            ui.separator();

            // Gantt Chart
            if !self.schedule.is_empty() {
                ui.heading("Gantt Chart (by Machine)");
                
                self.render_gantt_chart(ui);
            }
        });
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
}
