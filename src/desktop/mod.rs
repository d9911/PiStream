pub mod appearance;

use crate::{
    calculation::{self, Cancellation},
    Algorithm,
};
use appearance::{grouped, Palette, Theme};
use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, Stroke};
use std::{
    sync::{
        mpsc::{self, Receiver, TryRecvError},
        Arc,
    },
    time::{Duration, Instant},
};

const ALGORITHMS: [Algorithm; 4] = [
    Algorithm::ChudnovskyBinarySplitting,
    Algorithm::MachinFormula,
    Algorithm::BorweinQuadratic,
    Algorithm::ChudnovskyNaive,
];
fn title(a: Algorithm) -> &'static str {
    match a {
        Algorithm::ChudnovskyBinarySplitting => "Чудновский · Binary Splitting",
        Algorithm::MachinFormula => "Формула Мачина",
        Algorithm::BorweinQuadratic => "Гаусс—Лежандр · AGM",
        _ => "Чудновский · Классический",
    }
}
fn description(a: Algorithm) -> &'static str {
    match a {
        Algorithm::ChudnovskyBinarySplitting => {
            "Быстрый расчёт с рекурсивным разбиением ряда. Хороший выбор для большой точности."
        }
        Algorithm::MachinFormula => {
            "Классическая формула через арктангенсы. Удобна для сравнения алгоритмов."
        }
        Algorithm::BorweinQuadratic => {
            "Арифметико-геометрическое среднее. Точность удваивается с каждой итерацией."
        }
        _ => "Прямое суммирование ряда Чудновского. Простой метод для небольших расчётов.",
    }
}

#[derive(Clone)]
struct Record {
    algorithm: Algorithm,
    digits: usize,
    result: String,
    elapsed: Duration,
    time: String,
}
struct Session {
    rx: Receiver<Result<Record, String>>,
    cancel: Arc<Cancellation>,
    started: Instant,
}
impl Drop for Session {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Tab {
    Calculate,
    History,
}

pub struct App {
    theme: Theme,
    brand: egui::TextureHandle,
    tab: Tab,
    algorithm: Algorithm,
    digits: String,
    session: Option<Session>,
    current: Option<Arc<Record>>,
    history: Vec<Arc<Record>>,
    status: String,
    error: Option<String>,
    help: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = Theme::Light;
        theme.apply(&cc.egui_ctx);
        let icon = appearance::app_icon();
        let brand = cc.egui_ctx.load_texture(
            "pistream-brand",
            egui::ColorImage::from_rgba_unmultiplied(
                [icon.width as usize, icon.height as usize],
                &icon.rgba,
            ),
            egui::TextureOptions::LINEAR,
        );
        Self {
            theme,
            brand,
            tab: Tab::Calculate,
            algorithm: ALGORITHMS[0],
            digits: "1000".into(),
            session: None,
            current: None,
            history: Vec::new(),
            status: "Готово к вычислению".into(),
            error: None,
            help: false,
        }
    }
    fn start(&mut self, ctx: &egui::Context) {
        if self.session.is_some() {
            return;
        }
        let digits = match calculation::parse_digits(&self.digits) {
            Ok(d) => d,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };
        let algorithm = self.algorithm;
        let cancel = Arc::new(Cancellation::default());
        let token = Arc::clone(&cancel);
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();
        let started = Instant::now();
        let spawn = std::thread::Builder::new()
            .name("pistream-calculation".into())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    calculation::compute_default(algorithm, digits, &token)
                }))
                .map_err(|_| {
                    "Вычисление завершилось с ошибкой. Попробуйте меньшую точность.".to_string()
                })
                .and_then(|r| r)
                .map(|result| Record {
                    algorithm,
                    digits,
                    result,
                    elapsed: started.elapsed(),
                    time: chrono::Local::now().format("%H:%M:%S").to_string(),
                });
                let _ = tx.send(result);
                ctx.request_repaint();
            });
        match spawn {
            Ok(_) => {
                self.session = Some(Session {
                    rx,
                    cancel,
                    started,
                });
                self.error = None;
                self.status = "Вычисляю π…".into();
            }
            Err(e) => self.error = Some(format!("Не удалось запустить вычисление: {e}")),
        }
    }
    fn poll(&mut self) {
        let result = match self.session.as_ref().map(|s| s.rx.try_recv()) {
            Some(Ok(result)) => Some(result),
            Some(Err(TryRecvError::Disconnected)) => {
                Some(Err("Рабочий поток завершился без результата".into()))
            }
            _ => None,
        };
        if let Some(result) = result {
            let cancelled = self
                .session
                .as_ref()
                .is_some_and(|s| s.cancel.is_cancelled());
            self.session = None;
            if cancelled {
                self.status = "Остановлено. Можно начать новый расчёт.".into();
                return;
            }
            match result {
                Ok(record) => {
                    self.status = format!(
                        "Готово · {} знаков за {:.3} с",
                        grouped(record.digits),
                        record.elapsed.as_secs_f64()
                    );
                    let record = Arc::new(record);
                    self.current = Some(Arc::clone(&record));
                    self.history.insert(0, record);
                    self.history.truncate(12);
                    self.error = None;
                }
                Err(e) => {
                    self.status = "Не удалось завершить расчёт".into();
                    self.error = Some(e);
                }
            }
        }
    }
    fn stop(&mut self) {
        if let Some(s) = &self.session {
            s.cancel.cancel();
            self.status = "Останавливаю после текущей операции…".into();
        }
    }
    fn save(&mut self) {
        let Some(record) = &self.current else {
            return;
        };
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Сохранить число π")
            .set_file_name(format!("pi-{}-digits.txt", record.digits))
            .add_filter("Текстовый файл", &["txt"])
            .save_file()
        {
            match std::fs::write(&path, format!("{}\n", record.result)) {
                Ok(()) => self.status = format!("Сохранено: {}", path.display()),
                Err(e) => self.error = Some(format!("Не удалось сохранить файл: {e}")),
            }
        }
    }
    fn header(&mut self, root: &mut egui::Ui, ctx: &egui::Context, p: Palette) {
        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(p.canvas)
                    .inner_margin(egui::Margin::symmetric(28, 12)),
            )
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.image((self.brand.id(), egui::vec2(42.0, 42.0)));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("PiStreamTerminal").size(20.0).strong());
                        ui.label(
                            RichText::new("Число π. В вашей точности.")
                                .size(12.0)
                                .color(p.muted),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Справка").clicked() {
                            self.help = true;
                        }
                        let old = self.theme;
                        ui.selectable_value(&mut self.theme, Theme::Dark, "Тёмная");
                        ui.selectable_value(&mut self.theme, Theme::Light, "Светлая");
                        if old != self.theme {
                            self.theme.apply(ctx);
                        }
                    });
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    for (tab, label) in [(Tab::Calculate, "Вычисление"), (Tab::History, "История")]
                    {
                        let selected = self.tab == tab;
                        let response = ui.add(
                            egui::Button::new(RichText::new(label).strong())
                                .frame(false)
                                .selected(selected),
                        );
                        if selected {
                            ui.painter().line_segment(
                                [response.rect.left_bottom(), response.rect.right_bottom()],
                                Stroke::new(2.0, p.ink),
                            );
                        }
                        if response.clicked() {
                            self.tab = tab;
                        }
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new("Работает без интернета")
                                .size(12.0)
                                .color(p.muted),
                        );
                    });
                });
            });
    }
    fn settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, p: Palette) {
        egui::Frame::new()
            .fill(p.soft)
            .corner_radius(16)
            .inner_margin(20)
            .show(ui, |ui| {
                ui.set_min_width(252.0);
                ui.label(RichText::new("Настройки расчёта").size(18.0).strong());
                ui.add_space(8.0);
                let running = self.session.is_some();
                ui.add_enabled_ui(!running, |ui| {
                    ui.label(RichText::new("Алгоритм").strong());
                    egui::ComboBox::from_id_salt("algorithm")
                        .width(ui.available_width())
                        .selected_text(title(self.algorithm))
                        .show_ui(ui, |ui| {
                            for algorithm in ALGORITHMS {
                                if ui
                                    .selectable_value(
                                        &mut self.algorithm,
                                        algorithm,
                                        title(algorithm),
                                    )
                                    .on_hover_text(description(algorithm))
                                    .clicked()
                                {
                                    ui.close();
                                }
                            }
                        })
                        .response
                        .on_hover_text(description(self.algorithm));
                    ui.add_space(12.0);
                    ui.label(RichText::new("Знаков после запятой").strong());
                    let response = ui.add_sized(
                        [ui.available_width(), 44.0],
                        egui::TextEdit::singleline(&mut self.digits)
                            .font(FontId::proportional(26.0))
                            .margin(egui::vec2(12.0, 8.0))
                            .hint_text("1000"),
                    );
                    if response.changed() {
                        self.error = None;
                    }
                    ui.label(
                        RichText::new("От 1 до 1 000 000 знаков")
                            .size(12.0)
                            .color(p.muted),
                    );
                    ui.scope(|ui| {
                        ui.spacing_mut().button_padding = egui::vec2(8.0, 6.0);
                        ui.spacing_mut().item_spacing.x = 6.0;
                        ui.spacing_mut().interact_size.y = 28.0;
                        ui.horizontal_wrapped(|ui| {
                            for (value, label) in [
                                (100, "100"),
                                (1000, "1 тыс."),
                                (10_000, "10 тыс."),
                                (100_000, "100 тыс."),
                            ] {
                                if ui
                                    .selectable_label(
                                        self.digits == value.to_string(),
                                        RichText::new(label).size(12.0),
                                    )
                                    .clicked()
                                {
                                    self.digits = value.to_string();
                                    self.error = None;
                                }
                            }
                        });
                    });
                });
                ui.add_space(16.0);
                if running {
                    let stopping = self
                        .session
                        .as_ref()
                        .is_some_and(|s| s.cancel.is_cancelled());
                    if ui
                        .add_enabled(
                            !stopping,
                            egui::Button::new(if stopping {
                                "Останавливаю…"
                            } else {
                                "Остановить"
                            })
                            .min_size(egui::vec2(ui.available_width(), 44.0)),
                        )
                        .clicked()
                    {
                        self.stop();
                    }
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Вычисление…");
                    });
                } else if ui
                    .add(
                        egui::Button::new(
                            RichText::new("Вычислить π")
                                .color(Color32::WHITE)
                                .size(16.0)
                                .strong(),
                        )
                        .fill(p.primary_button)
                        .stroke(Stroke::NONE)
                        .min_size(egui::vec2(ui.available_width(), 44.0)),
                    )
                    .clicked()
                {
                    self.start(ctx);
                }
                if let Some(error) = &self.error {
                    ui.add_space(8.0);
                    ui.colored_label(p.error, error);
                }
                ui.add_space(8.0);
                ui.label(
                    RichText::new("GMP / MPFR · округление до выбранной точности")
                        .size(12.0)
                        .color(p.muted),
                );
            });
    }

    fn result(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, p: Palette) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Результат").size(20.0).strong());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add_enabled(self.current.is_some(), egui::Button::new("Сохранить .txt"))
                    .clicked()
                {
                    self.save();
                }
                if ui
                    .add_enabled(self.current.is_some(), egui::Button::new("Копировать"))
                    .clicked()
                {
                    if let Some(r) = &self.current {
                        ctx.copy_text(r.result.clone());
                        self.status = "Результат скопирован".into();
                    }
                }
            });
        });
        ui.add_space(12.0);
        if let Some(record) = &self.current {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{} знаков", grouped(record.digits))).strong());
                ui.label(
                    RichText::new(format!(
                        "· {:.3} с · {}",
                        record.elapsed.as_secs_f64(),
                        record.time
                    ))
                    .size(13.0)
                    .color(p.muted),
                );
            });
            ui.label(
                RichText::new(title(record.algorithm))
                    .size(13.0)
                    .color(p.muted),
            );
            ui.add_space(16.0);
            egui::Frame::new()
                .fill(p.soft)
                .corner_radius(14)
                .inner_margin(24)
                .show(ui, |ui| {
                    ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                    ui.label(RichText::new("π =").size(18.0).color(p.muted));
                    ui.add_space(8.0);
                    let count = record.result.len().min(2002);
                    let preview = &record.result[..count];
                    egui::ScrollArea::vertical()
                        .id_salt("result-digits")
                        .max_height(260.0)
                        .min_scrolled_height(180.0)
                        .show(ui, |ui| {
                            let mut job = egui::text::LayoutJob::default();
                            job.wrap.max_width = ui.available_width();
                            job.wrap.break_anywhere = true;
                            job.append(
                                preview,
                                0.0,
                                egui::TextFormat {
                                    font_id: FontId::monospace(21.0),
                                    color: p.ink,
                                    line_height: Some(32.0),
                                    ..Default::default()
                                },
                            );
                            ui.add(egui::Label::new(job).wrap().selectable(true));
                        });
                });
            ui.add_space(12.0);
            let note = if record.digits > 2000 {
                "Показаны первые 2 000 знаков. Копирование и сохранение включают полный результат."
            } else {
                "Можно выделить фрагмент числа или скопировать весь результат."
            };
            ui.label(RichText::new(note).size(12.0).color(p.muted));
        } else {
            egui::Frame::new()
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(16)
                .inner_margin(32)
                .show(ui, |ui| {
                    ui.set_min_width((ui.available_width() - 1.0).max(0.0));
                    ui.vertical_centered(|ui| {
                        ui.add_space(26.0);
                        ui.image((self.brand.id(), egui::vec2(88.0, 88.0)));
                        ui.label(
                            RichText::new("Здесь появится ваше число")
                                .size(20.0)
                                .strong(),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("Выберите точность и начните вычисление.").color(p.muted),
                        );
                        ui.add_space(22.0);
                        ui.label(
                            RichText::new("3.141592653589793238462643383279…")
                                .monospace()
                                .size(15.0)
                                .color(p.muted),
                        );
                        ui.add_space(24.0);
                    });
                });
        }
    }
    fn history(&mut self, ui: &mut egui::Ui, p: Palette) {
        ui.horizontal(|ui| {
            ui.heading("История вычислений");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add_enabled(
                        !self.history.is_empty(),
                        egui::Button::new("Очистить историю"),
                    )
                    .clicked()
                {
                    self.history.clear();
                }
            });
        });
        ui.label(RichText::new("Последние 12 результатов этой сессии. История сбрасывается при закрытии приложения.").color(p.muted));
        ui.add_space(20.0);
        if self.history.is_empty() {
            ui.label("Пока нет завершённых вычислений.");
        }
        let mut selected = None;
        for (index, record) in self.history.iter().enumerate() {
            egui::Frame::new()
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(12)
                .inner_margin(18)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("{} знаков", grouped(record.digits)))
                                    .size(18.0)
                                    .strong(),
                            );
                            ui.label(RichText::new(title(record.algorithm)).color(p.muted));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.button("Открыть").clicked() {
                                selected = Some(index);
                            }
                            ui.label(format!(
                                "{} · {:.3} с",
                                record.time,
                                record.elapsed.as_secs_f64()
                            ));
                        });
                    });
                });
            ui.add_space(4.0);
        }
        if let Some(index) = selected {
            self.current = Some(Arc::clone(&self.history[index]));
            self.tab = Tab::Calculate;
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        self.poll();
        let p = self.theme.palette();
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Enter)) {
            self.start(&ctx);
        }
        if self.session.is_some()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.stop();
        }
        self.header(root, &ctx, p);
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(p.canvas)
                    .inner_margin(egui::Margin::symmetric(28, 12)),
            )
            .show(root, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let text = if let Some(session) = &self.session {
                        format!(
                            "{} · {:.1} с",
                            self.status,
                            session.started.elapsed().as_secs_f64()
                        )
                    } else {
                        self.status.clone()
                    };
                    ui.label(RichText::new(text).size(12.0).color(p.muted));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!(
                                "v{} · ⌘/Ctrl+Enter — запуск",
                                env!("CARGO_PKG_VERSION")
                            ))
                            .size(12.0)
                            .color(p.muted),
                        );
                    });
                });
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(p.canvas)
                    .inner_margin(egui::Margin::symmetric(28, 16)),
            )
            .show(root, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("workspace")
                    .show(ui, |ui| {
                        if self.tab == Tab::History {
                            self.history(ui, p);
                            return;
                        }
                        ui.heading("Вычислите π");
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(
                                "Выберите алгоритм и точность. Остальное — дело математики.",
                            )
                            .color(p.muted),
                        );
                        ui.add_space(16.0);
                        if ui.available_width() >= 830.0 {
                            ui.horizontal_top(|ui| {
                                ui.allocate_ui_with_layout(
                                    egui::vec2(316.0, 0.0),
                                    Layout::top_down(Align::Min),
                                    |ui| {
                                        self.settings(ui, &ctx, p);
                                    },
                                );
                                ui.add_space(12.0);
                                ui.allocate_ui_with_layout(
                                    egui::vec2(ui.available_width(), 0.0),
                                    Layout::top_down(Align::Min),
                                    |ui| {
                                        self.result(ui, &ctx, p);
                                    },
                                );
                            });
                        } else {
                            self.settings(ui, &ctx, p);
                            ui.add_space(24.0);
                            self.result(ui, &ctx, p);
                        }
                    });
            });
        egui::Window::new("О PiStreamTerminal").open(&mut self.help).collapsible(false).resizable(false).show(&ctx,|ui|{
            ui.set_max_width(440.0);ui.label("Локальный калькулятор числа π с четырьмя алгоритмами и произвольной точностью.");ui.add_space(12.0);
            ui.label("1. Выберите алгоритм и количество знаков.\n2. Нажмите «Вычислить π» или ⌘/Ctrl+Enter.\n3. Скопируйте результат или сохраните его в .txt.");ui.add_space(12.0);
            ui.label("Отмена: кнопка «Остановить» или Esc. Проверка отмены выполняется между математическими операциями.");ui.add_space(12.0);
            ui.label("Расчёты выполняются на компьютере. Результаты и настройки не отправляются в сеть.");ui.add_space(12.0);
            ui.label("Режимы Ultra Scale и потоковая запись доступны отдельно: cargo run --bin pistream-cli --release.");
        });
        if self.session.is_some() {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
    }
}
