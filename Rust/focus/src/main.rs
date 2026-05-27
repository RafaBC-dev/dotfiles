use chrono::{Local, Timelike};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{self, Write},
    process::{self, Command},
    time::{Duration, Instant},
};

// --- ESTRUCTURA DE DATOS ANALÍTICOS ---
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct FocusRecord {
    date: String,
    session_name: String,
    moment: String,
    duration_mins: u64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct FocusDB {
    records: Vec<FocusRecord>,
}

fn main() {
    let now = Local::now();
    let hour = now.hour();
    let date = now.format("%d-%m").to_string();

    let moment_of_day = match hour {
        6..=12 => "Morning",
        13..=20 => "Afternoon",
        _ => "Night",
    };

    let args: Vec<String> = env::args().collect();
    let mins_study: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(50);
    let mins_rest: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
    let session_name = args
        .get(3)
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "general".to_string());

    loop {
        send_notification(&format!("Focus: {}", session_name), "Time to work!");

        // Ejecutamos el timer. Nos devuelve los minutos reales completados
        let completed_mins = run_timer(
            mins_study,
            "FOCUS MODE",
            true,
            &session_name,
            moment_of_day,
            &date,
        );

        if completed_mins > 0 {
            save_record(&date, &session_name, moment_of_day, completed_mins);
        }

        send_notification("Rest Mode", "Time to take a break buddy!");

        run_timer(
            mins_rest,
            "REST MODE",
            false,
            &session_name,
            moment_of_day,
            &date,
        );
    }
}

// --- NÚCLEO DEL CRONÓMETRO Y TECLADO ---
fn run_timer(
    mins_to_run: u64,
    label: &str,
    is_focus: bool,
    session_name: &str,
    moment: &str,
    date: &str,
) -> u64 {
    let start_time_str = Local::now().format("%H:%M").to_string();
    let total_secs = mins_to_run * 60;

    let mut elapsed_secs = 0;
    let mut paused = false;
    let mut last_tick = Instant::now();

    // Activamos el control total del teclado
    enable_raw_mode().expect("Failed to enable raw mode");

    loop {
        if elapsed_secs >= total_secs {
            break;
        }

        // 1. Escucha no bloqueante del teclado (100ms)
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                // Capturar Ctrl + C
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    disable_raw_mode().unwrap();
                    let completed_mins = elapsed_secs / 60;

                    // Si interrumpimos a mitad del estudio, guardamos lo acumulado
                    if is_focus && completed_mins > 0 {
                        save_record(date, session_name, moment, completed_mins);
                    }

                    print_summary(date, session_name);
                    process::exit(0);
                }
                // Capturar barra espaciadora
                else if key.code == KeyCode::Char(' ') {
                    paused = !paused;
                    last_tick = Instant::now(); // Evita que cuente el tiempo que estuvo pausado
                }
            }
        }

        // 2. Lógica de tiempo (solo avanza si no está pausado)
        if !paused {
            let now = Instant::now();
            if now.duration_since(last_tick) >= Duration::from_secs(1) {
                elapsed_secs += 1;
                last_tick = now;
            }
        }

        // 3. Dibujado de interfaz
        draw_ui(
            elapsed_secs,
            total_secs,
            paused,
            label,
            session_name,
            moment,
            &start_time_str,
            is_focus,
        );
    }

    // Desactivamos el modo crudo al terminar la sesión natural
    disable_raw_mode().unwrap();
    elapsed_secs / 60
}

fn draw_ui(
    elapsed_secs: u64,
    total_secs: u64,
    paused: bool,
    label: &str,
    session_name: &str, // <-- ¡Esto faltaba!
    moment: &str,
    start_time: &str,
    is_focus: bool,
) {
    let remaining = total_secs - elapsed_secs;
    let m = remaining / 60;
    let s = remaining % 60;

    let (columns, _) = size().unwrap_or((40, 20));
    let bar_width = if columns > 10 {
        (columns - 6) as usize
    } else {
        10
    };

    let blocks = ((elapsed_secs as f64 / total_secs as f64) * bar_width as f64) as usize;
    let blocks = blocks.min(bar_width);

    let filled = "█".repeat(blocks);
    let empty = "░".repeat(bar_width - blocks);
    let color_start = if is_focus { "\x1B[34m" } else { "\x1B[32m" };
    let reset = "\x1B[0m";

    let header = format!(
        "[{}] | Session: {} | Started: {}",
        moment.to_uppercase(),
        session_name.to_uppercase(),
        start_time
    );
    let padding = " ".repeat(if columns as usize > header.len() {
        (columns as usize - header.len()) / 2
    } else {
        0
    });

    // En modo crudo, debemos usar \r\n explícitamente para los saltos de línea
    print!("\x1B[2J\x1B[1;1H"); // Limpiar pantalla
    print!("\r\n{}{}\r\n", padding, header);
    print!("{:-<width$}\r\n\r\n", "", width = columns as usize);

    // <-- ¡Faltaban estas dos líneas de cálculo de centrado!
    let timer_text = format!("{:02}:{:02} {}", m, s, label);
    let timer_padding = " ".repeat(if columns as usize > timer_text.len() {
        (columns as usize - timer_text.len()) / 2
    } else {
        0
    });

    if paused {
        print!(
            "{}\x1B[33m{:02}:{:02} [PAUSED]\x1B[0m\r\n",
            timer_padding, m, s
        );
    } else {
        print!("{}{:02}:{:02} {}\r\n", timer_padding, m, s, label);
    }

    print!("  [{}{}{}{}]\r\n", color_start, filled, empty, reset);
    io::stdout().flush().unwrap();
}

// --- BASE DE DATOS Y RESUMEN ---
fn load_db() -> FocusDB {
    match fs::read_to_string("focus_analytics.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => FocusDB::default(),
    }
}

fn save_record(date: &str, session_name: &str, moment: &str, duration_mins: u64) {
    let mut db = load_db();
    db.records.push(FocusRecord {
        date: date.to_string(),
        session_name: session_name.to_string(),
        moment: moment.to_string(),
        duration_mins,
    });

    if let Ok(json_string) = serde_json::to_string_pretty(&db) {
        let _ = fs::write("focus_analytics.json", json_string);
    }
}

fn print_summary(target_date: &str, current_session: &str) {
    let db = load_db();

    let mut total_today = 0;
    let mut session_today = 0;
    let mut session_historical = 0;

    let (mut m_mins, mut a_mins, mut n_mins) = (0, 0, 0);

    for record in &db.records {
        if record.session_name == current_session {
            session_historical += record.duration_mins;
        }

        if record.date == target_date {
            total_today += record.duration_mins;
            if record.session_name == current_session {
                session_today += record.duration_mins;
                match record.moment.as_str() {
                    "Morning" => m_mins += record.duration_mins,
                    "Afternoon" => a_mins += record.duration_mins,
                    "Night" => n_mins += record.duration_mins,
                    _ => {}
                }
            }
        }
    }

    // Limpiar pantalla por última vez antes de imprimir el resumen
    print!("\x1B[2J\x1B[1;1H");
    println!("=== DAILY SUMMARY [{}] ===", target_date);
    println!(
        "Total Focused Today (All projects): {}\n",
        format_time(total_today)
    );

    println!("Session: {}", current_session.to_uppercase());
    println!(
        "- Today: {} (Morning: {} | Afternoon: {} | Night: {})",
        format_time(session_today),
        format_time(m_mins),
        format_time(a_mins),
        format_time(n_mins)
    );
    println!("- Historical Total: {}", format_time(session_historical));
}

fn format_time(total_mins: u64) -> String {
    let hours = total_mins / 60;
    let mins = total_mins % 60;
    if hours > 0 {
        format!("{}h {}min", hours, mins)
    } else {
        format!("{}min", mins)
    }
}

fn send_notification(title: &str, message: &str) {
    let _ = Command::new("notify-send")
        .args([title, message, "-t", "2500"])
        .spawn();
}
