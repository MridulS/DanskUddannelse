use eframe::egui;
use rand::prelude::*;
use rand::{Rng, random};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Verb {
    infinitive: String,
    present: String,
    past: String,
    past_participle: String,
    english: String,
}

#[derive(Debug, Clone, Copy)]
enum PracticeMode {
    Translation,
    Conjugation,
}

#[derive(Debug, Clone, Copy)]
enum ConjugationForm {
    Present,
    Past,
    PastParticiple,
}

struct DanishVerbsApp {
    verbs: Vec<Verb>,
    current_verb_index: usize,
    practice_mode: PracticeMode,
    conjugation_form: ConjugationForm,
    user_answer: String,
    result_message: String,
    show_result: bool,
    fonts_loaded: bool,
    heading_font: Option<egui::FontId>,
    body_font: Option<egui::FontId>,
    accent_color: egui::Color32,
    background_color: egui::Color32,
    text_color: egui::Color32,
}

impl DanishVerbsApp {
    fn new() -> Self {
        let mut verbs = load_verbs();
        let mut rng = rand::rng();
        verbs.shuffle(&mut rng);

        Self {
            verbs,
            current_verb_index: 0,
            practice_mode: PracticeMode::Translation,
            conjugation_form: ConjugationForm::Present,
            user_answer: String::new(),
            result_message: String::new(),
            show_result: false,
            fonts_loaded: false,
            heading_font: None,
            body_font: None,
            accent_color: egui::Color32::from_rgb(66, 135, 245), // Blue
            background_color: egui::Color32::from_rgb(240, 240, 255), // Light blue-gray
            text_color: egui::Color32::from_rgb(40, 40, 60),     // Dark blue-gray
        }
    }

    fn load_fonts(&mut self, ctx: &egui::Context) {
        if !self.fonts_loaded {
            // Define custom fonts
            self.heading_font = Some(egui::FontId::proportional(32.0));
            self.body_font = Some(egui::FontId::proportional(20.0));
            self.fonts_loaded = true;

            // Configure global Visual settings
            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (egui::TextStyle::Heading, egui::FontId::proportional(32.0)),
                (egui::TextStyle::Body, egui::FontId::proportional(20.0)),
                (egui::TextStyle::Monospace, egui::FontId::monospace(18.0)),
                (egui::TextStyle::Button, egui::FontId::proportional(20.0)),
                (egui::TextStyle::Small, egui::FontId::proportional(16.0)),
            ]
            .into();
            ctx.set_style(style);
        }
    }

    fn next_verb(&mut self) {
        self.current_verb_index = (self.current_verb_index + 1) % self.verbs.len();
        self.user_answer.clear();
        self.result_message.clear();
        self.show_result = false;

        // Randomly select practice mode and conjugation form
        if random() {
            self.practice_mode = PracticeMode::Translation;
        } else {
            self.practice_mode = PracticeMode::Conjugation;
            let mut rng = rand::rng();
            let form = rng.random_range(0..=2);
            self.conjugation_form = match form {
                0 => ConjugationForm::Present,
                1 => ConjugationForm::Past,
                _ => ConjugationForm::PastParticiple,
            };
        }
    }

    fn check_answer(&mut self) {
        let current_verb = &self.verbs[self.current_verb_index];
        let correct_answer = match self.practice_mode {
            PracticeMode::Translation => current_verb.english.clone(),
            PracticeMode::Conjugation => match self.conjugation_form {
                ConjugationForm::Present => current_verb.present.clone(),
                ConjugationForm::Past => current_verb.past.clone(),
                ConjugationForm::PastParticiple => current_verb.past_participle.clone(),
            },
        };

        if self.user_answer.trim().to_lowercase() == correct_answer.trim().to_lowercase() {
            self.result_message = "Correct! 🎉".to_string();
        } else {
            self.result_message = format!("Incorrect. The correct answer is: {}", correct_answer);
        }
        self.show_result = true;
    }
}

impl eframe::App for DanishVerbsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load custom fonts if not done yet
        self.load_fonts(ctx);

        // Store font references for later use to avoid borrowing issues
        let heading_font = self.heading_font.clone();
        let body_font = self.body_font.clone();
        let accent_color = self.accent_color;
        let text_color = self.text_color;
        let background_color = self.background_color;

        // Get current verb info for display
        let current_verb = self.verbs[self.current_verb_index].clone();
        let practice_mode = self.practice_mode;
        let conjugation_form = self.conjugation_form;
        let show_result = self.show_result;
        let result_message = self.result_message.clone();

        // Set the background color
        let mut frame = egui::Frame::new();
        frame = frame.fill(background_color);
        frame = frame.inner_margin(20.0);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(text_color);

            // App title with styled heading
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Danish Verbs Practice")
                        .font(heading_font.as_ref().unwrap().clone())
                        .color(accent_color)
                        .strong(),
                ));
            });

            ui.add_space(30.0);

            let question_text = match practice_mode {
                PracticeMode::Translation => {
                    format!("Translate to English: {}", current_verb.infinitive)
                }
                PracticeMode::Conjugation => {
                    let form_name = match conjugation_form {
                        ConjugationForm::Present => "present tense",
                        ConjugationForm::Past => "past tense",
                        ConjugationForm::PastParticiple => "past participle",
                    };
                    format!("Conjugate '{}' in {}", current_verb.infinitive, form_name)
                }
            };

            // Display the question in a styled box
            ui.add(egui::Label::new(
                egui::RichText::new(question_text)
                    .font(body_font.as_ref().unwrap().clone())
                    .color(text_color)
                    .strong(),
            ));

            ui.add_space(20.0);

            // User input field
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Your answer:")
                        .font(body_font.as_ref().unwrap().clone())
                        .color(text_color),
                ));

                let response = ui.add_sized(
                    [ui.available_width() - 120.0, 40.0],
                    egui::TextEdit::singleline(&mut self.user_answer)
                        .font(body_font.as_ref().unwrap().clone())
                        .hint_text("Type your answer here"),
                );

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.check_answer();
                }
            });

            ui.add_space(20.0);

            // Buttons with improved styling
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 20.0;

                let check_button = ui.add_sized(
                    [150.0, 50.0],
                    egui::Button::new(
                        egui::RichText::new("Check")
                            .font(body_font.as_ref().unwrap().clone())
                            .color(egui::Color32::WHITE),
                    )
                    .fill(accent_color)
                    .corner_radius(8.0),
                );

                if check_button.clicked() {
                    self.check_answer();
                }

                let next_button = ui.add_sized(
                    [150.0, 50.0],
                    egui::Button::new(
                        egui::RichText::new("Next verb")
                            .font(body_font.as_ref().unwrap().clone())
                            .color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(76, 175, 80))
                    .corner_radius(8.0),
                );

                if next_button.clicked() {
                    self.next_verb();
                }
            });

            // Result message
            if show_result {
                ui.add_space(20.0);

                let text_color = if result_message.starts_with("Correct") {
                    egui::Color32::from_rgb(76, 175, 80) // Green
                } else {
                    egui::Color32::from_rgb(211, 47, 47) // Red
                };

                let result_text = egui::RichText::new(&result_message)
                    .font(body_font.as_ref().unwrap().clone())
                    .color(text_color)
                    .strong();

                ui.add(egui::Label::new(result_text));
            }

            ui.add_space(30.0);

            // Verb details section with improved styling
            let mut detail_frame = egui::Frame::new();
            detail_frame = detail_frame.fill(egui::Color32::from_rgb(230, 230, 250));
            detail_frame = detail_frame.stroke(egui::Stroke::new(1.0, accent_color));
            detail_frame = detail_frame.corner_radius(8.0);
            detail_frame = detail_frame.inner_margin(16.0);

            detail_frame.show(ui, |ui| {
                egui::CollapsingHeader::new(
                    egui::RichText::new("Verb details")
                        .font(body_font.as_ref().unwrap().clone())
                        .color(accent_color)
                        .strong(),
                )
                .default_open(false)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;

                    // Clone TextStyle for reuse
                    let verb_details_style = egui::TextStyle::Body;

                    ui.label(
                        egui::RichText::new(format!("Infinitive: {}", current_verb.infinitive))
                            .font(body_font.as_ref().unwrap().clone())
                            .text_style(verb_details_style.clone()),
                    );

                    ui.label(
                        egui::RichText::new(format!("Present: {}", current_verb.present))
                            .font(body_font.as_ref().unwrap().clone())
                            .text_style(verb_details_style.clone()),
                    );

                    ui.label(
                        egui::RichText::new(format!("Past: {}", current_verb.past))
                            .font(body_font.as_ref().unwrap().clone())
                            .text_style(verb_details_style.clone()),
                    );

                    ui.label(
                        egui::RichText::new(format!(
                            "Past participle: {}",
                            current_verb.past_participle
                        ))
                        .font(body_font.as_ref().unwrap().clone())
                        .text_style(verb_details_style.clone()),
                    );

                    ui.label(
                        egui::RichText::new(format!("English: {}", current_verb.english))
                            .font(body_font.as_ref().unwrap().clone())
                            .text_style(verb_details_style.clone()),
                    );
                });
            });
        });
    }
}

fn load_verbs() -> Vec<Verb> {
    let verbs_path = Path::new("src/verbs.json");

    match fs::read_to_string(verbs_path) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(verbs) => verbs,
            Err(e) => {
                eprintln!("Error parsing verb data: {}", e);
                vec![]
            }
        },
        Err(e) => {
            eprintln!("Error reading verb file: {}", e);
            vec![]
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 800.0])
            .with_min_inner_size([480.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Danish Verbs Practice",
        options,
        Box::new(|_cc| Ok(Box::new(DanishVerbsApp::new()))),
    )
}
