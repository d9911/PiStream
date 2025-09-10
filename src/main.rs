use pi_stream_terminal::{Digits, FileWriter, PiComputationUseCase, PiStreamApp, UiPresenter};
use std::process;

#[tokio::main]
async fn main() {
    // Setup application with hexagonal architecture
    let (use_case, presenter, file_writer, config) = PiStreamApp::setup();

    // Show welcome message
    presenter.show_welcome();

    // Show available algorithms
    let algorithms = use_case.list_algorithms();
    presenter.show_algorithms(&algorithms);

    // Get algorithm selection
    let algorithm_input = presenter.prompt_algorithm_selection();
    let algorithm_choice: u8 = algorithm_input
        .trim()
        .parse()
        .unwrap_or(config.default_algorithm_choice);
    let algorithm = use_case.get_algorithm_by_choice(algorithm_choice);

    // Show ultra scale warning if needed
    if algorithm.is_ultra_scale() {
        presenter.show_ultra_scale_warning();
    }

    // Get digits input
    let digits_input = presenter.prompt_digits_input();
    let digits_value = if digits_input.is_empty() {
        // Use different default values based on algorithm
        if algorithm.is_ultra_scale() {
            100_000_000 // Default for ULTRA SCALE
        } else {
            config.default_digits
        }
    } else {
        match digits_input.parse() {
            Ok(n) if n > 0 => {
                // For Ultra Scale algorithm there are no limits
                if algorithm.is_ultra_scale() {
                    n
                } else if n <= 1_000_000_000 {
                    n
                } else {
                    presenter.show_warning("Слишком большое значение для выбранного алгоритма!");
                    presenter
                        .show_info("Используйте ULTRA SCALE (алгоритм 5) для триллионов знаков");
                    presenter.show_info("Используется значение по умолчанию: 1000");
                    1000
                }
            }
            _ => {
                presenter.show_warning("Неверный ввод. Используется значение по умолчанию: 1000");
                1000
            }
        }
    };

    let digits = match Digits::new(digits_value) {
        Ok(d) => d,
        Err(e) => {
            presenter.show_error(&e);
            process::exit(1);
        }
    };

    // Validate computation
    if let Err(e) = use_case.validate_computation(algorithm, digits) {
        presenter.show_error(&e);
        presenter.show_info(&format!(
            "Максимум для {}: {} знаков",
            algorithm.display_name(),
            algorithm.max_digits()
        ));
        presenter.show_info("Рекомендация: Используйте Binary Splitting для больших вычислений");
        process::exit(1);
    }

    // Show ultra scale confirmation if needed
    if algorithm.is_ultra_scale() && !presenter.prompt_ultra_scale_confirmation(digits.value()) {
        return;
    }

    // Show computation start
    presenter.show_computation_start(algorithm, digits.value());

    // Show resource estimation
    let estimate = use_case.estimate_resources(algorithm, digits);
    presenter.show_resource_estimation(&estimate);

    // Perform computation
    println!("⏰ Засекаем время...");
    match use_case.compute_pi(algorithm, digits).await {
        Ok(computation) => {
            // Show results
            presenter.show_results(&computation);

            // Show statistics
            if config.show_detailed_stats {
                presenter.show_statistics(&computation);
            }

            // Show achievements
            if config.show_achievements {
                presenter.show_achievement(digits.value());
            }

            // Offer to save file
            if config.enable_file_saving && digits.value() >= 1000 {
                if presenter.prompt_save_to_file() {
                    if let Some(result) = &computation.result {
                        let filename = file_writer
                            .generate_pi_filename(digits.value(), algorithm.display_name());

                        match file_writer.write_file(&filename, result).await {
                            Ok(_) => presenter.show_save_result(&filename, true, None),
                            Err(e) => presenter.show_save_result(&filename, false, Some(&e)),
                        }
                    }
                }
            }
        }
        Err(e) => {
            presenter.show_error(&format!("Произошла ошибка при вычислении: {}", e));
            presenter
                .show_info("Попробуйте уменьшить количество знаков или выберите другой алгоритм");
        }
    }
}
