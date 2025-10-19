use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Instant;

/// Структура для потокового сохранения результатов вычисления π
pub struct StreamingPiSaver {
    sender: Sender<PiBlock>,
    handle: Option<thread::JoinHandle<()>>,
}

/// Блок данных π для сохранения
#[derive(Debug, Clone)]
pub struct PiBlock {
    pub block_number: usize,
    pub digits: String,
    pub start_position: usize,
    pub end_position: usize,
    pub timestamp: Instant,
}

/// Статус сохранения
#[derive(Debug)]
pub enum SaveStatus {
    Success,
    Error(String),
}

impl StreamingPiSaver {
    /// Создает новый потоковый сохранитель
    pub fn new(file_path: &str, total_digits: usize) -> Result<Self, String> {
        let (sender, receiver) = mpsc::channel();
        
        // Создаем файл с заголовком
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| format!("Не удалось создать файл {}: {}", file_path, e))?;
        
        // Записываем заголовок
        writeln!(file, "# Pi Computation Stream - {} digits", total_digits)
            .map_err(|e| format!("Ошибка записи заголовка: {}", e))?;
        writeln!(file, "# Format: block_number:start_pos-end_pos:digits")
            .map_err(|e| format!("Ошибка записи формата: {}", e))?;
        writeln!(file, "# Generated: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))
            .map_err(|e| format!("Ошибка записи времени: {}", e))?;
        writeln!(file, "")
            .map_err(|e| format!("Ошибка записи пустой строки: {}", e))?;
        
        // Запускаем поток сохранения
        let file_path_clone = file_path.to_string();
        let handle = thread::spawn(move || {
            Self::save_worker(receiver, &file_path_clone);
        });
        
        Ok(StreamingPiSaver {
            sender,
            handle: Some(handle),
        })
    }
    
    /// Отправляет блок данных для сохранения
    pub fn send_block(&self, block: PiBlock) -> Result<(), String> {
        self.sender.send(block)
            .map_err(|e| format!("Ошибка отправки блока: {}", e))
    }
    
    /// Завершает сохранение и закрывает поток
    pub fn finish(mut self) -> Result<SaveStatus, String> {
        // Отправляем сигнал завершения
        drop(self.sender);
        
        // Ждем завершения потока
        if let Some(handle) = self.handle.take() {
            handle.join()
                .map_err(|e| format!("Ошибка завершения потока: {:?}", e))?;
        }
        
        Ok(SaveStatus::Success)
    }
    
    /// Рабочий поток для сохранения
    fn save_worker(receiver: Receiver<PiBlock>, file_path: &str) {
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
        {
            Ok(f) => BufWriter::new(f),
            Err(e) => {
                eprintln!("❌ Ошибка открытия файла для записи: {}", e);
                return;
            }
        };
        
        let mut block_count = 0;
        let mut total_digits_saved = 0;
        let start_time = Instant::now();
        
        println!("💾 Поток сохранения запущен: {}", file_path);
        
        while let Ok(block) = receiver.recv() {
            match Self::write_block(&mut file, &block) {
                Ok(_) => {
                    block_count += 1;
                    total_digits_saved += block.digits.len();
                    
                    // Периодически выводим статистику
                    if block_count % 100 == 0 || block_count == 1 {
                        let elapsed = start_time.elapsed();
                        let speed = if elapsed.as_secs() > 0 {
                            total_digits_saved as f64 / elapsed.as_secs_f64()
                        } else {
                            0.0
                        };
                        
                        println!("💾 Сохранено блоков: {}, знаков: {}, скорость: {:.0} знаков/сек", 
                                block_count, total_digits_saved, speed);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Ошибка записи блока {}: {}", block.block_number, e);
                }
            }
        }
        
        // Финальная статистика
        let elapsed = start_time.elapsed();
        let speed = if elapsed.as_secs() > 0 {
            total_digits_saved as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };
        
        println!("✅ Поток сохранения завершен:");
        println!("   Блоков сохранено: {}", block_count);
        println!("   Знаков сохранено: {}", total_digits_saved);
        println!("   Время: {:.2}с", elapsed.as_secs_f64());
        println!("   Скорость записи: {:.0} знаков/сек", speed);
    }
    
    /// Записывает блок в файл
    fn write_block(file: &mut BufWriter<std::fs::File>, block: &PiBlock) -> Result<(), String> {
        // Формат: block_number:start_pos-end_pos:digits
        writeln!(file, "{}:{}:{}:{}", 
                block.block_number,
                block.start_position,
                block.end_position,
                block.digits)
            .map_err(|e| format!("Ошибка записи блока: {}", e))?;
        
        file.flush()
            .map_err(|e| format!("Ошибка сброса буфера: {}", e))?;
        
        Ok(())
    }
}

/// Утилиты для работы с потоковым сохранением
pub mod utils {
    use super::*;
    use std::fs;
    use std::path::Path;
    
    /// Проверяет существование файла с результатами
    pub fn check_existing_file(file_path: &str) -> Result<Option<usize>, String> {
        if !Path::new(file_path).exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Ошибка чтения файла: {}", e))?;
        
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Ok(None);
        }
        
        // Ищем последний блок
        let mut total_digits = 0;
        
        for line in lines {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            
            if let Some(parts) = parse_block_line(line) {
                total_digits = parts.end_position;
            }
        }
        
        Ok(Some(total_digits))
    }
    
    /// Парсит строку блока
    fn parse_block_line(line: &str) -> Option<PiBlock> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 4 {
            return None;
        }
        
        let block_number = parts[0].parse::<usize>().ok()?;
        let start_position = parts[1].parse::<usize>().ok()?;
        let end_position = parts[2].parse::<usize>().ok()?;
        let digits = parts[3].to_string();
        
        Some(PiBlock {
            block_number,
            digits,
            start_position,
            end_position,
            timestamp: Instant::now(),
        })
    }
    
    /// Объединяет все блоки в финальный результат
    pub fn merge_blocks(file_path: &str) -> Result<String, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Ошибка чтения файла: {}", e))?;
        
        let mut blocks: Vec<PiBlock> = Vec::new();
        
        for line in content.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            
            if let Some(block) = parse_block_line(line) {
                blocks.push(block);
            }
        }
        
        // Сортируем по номеру блока
        blocks.sort_by_key(|b| b.block_number);
        
        // Объединяем все цифры
        let mut result = String::new();
        for block in blocks {
            result.push_str(&block.digits);
        }
        
        Ok(result)
    }
    
    /// Создает читаемый файл π
    pub fn create_readable_pi_file(input_file: &str, output_file: &str) -> Result<(), String> {
        let digits = merge_blocks(input_file)?;
        
        let mut file = fs::File::create(output_file)
            .map_err(|e| format!("Ошибка создания файла: {}", e))?;
        
        use std::io::Write;
        // Просто записываем цифры с "3." в начале
        writeln!(file, "π = 3.{}", digits)
            .map_err(|e| format!("Ошибка записи: {}", e))?;
        
        Ok(())
    }
}
