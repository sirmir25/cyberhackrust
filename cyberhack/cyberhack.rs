use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use rand::Rng;

// ============================================================================
// ОСНОВНЫЕ СТРУКТУРЫ ДАННЫХ
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub current_quest: Option<Quest>,
    pub completed_quests: Vec<String>,
    pub current_location: String,
    pub story_progress: u32,
    pub game_mode: GameMode,
    pub time_played: Duration,
    pub networks: HashMap<String, Network>,
    pub contacts: HashMap<String, Contact>,
    pub inventory: Vec<String>,
    pub skills: HashMap<String, u32>,
    pub reputation: i32,
    pub money: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMode {
    Story,
    Sandbox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub handle: String,
    pub level: u32,
    pub experience: u32,
    pub health: u32,
    pub max_health: u32,
    pub stress: u32,
    pub current_system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub rewards: Vec<String>,
    pub time_limit: Option<Duration>,
    pub difficulty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub description: String,
    pub completed: bool,
    pub target: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub security_level: u32,
    pub systems: HashMap<String, System>,
    pub firewall_strength: u32,
    pub intrusion_detection: bool,
    pub is_compromised: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub name: String,
    pub os: String,
    pub security_level: u32,
    pub files: HashMap<String, File>,
    pub services: Vec<Service>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub is_compromised: bool,
    pub admin_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub content: String,
    pub permissions: String,
    pub size: u32,
    pub encrypted: bool,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub port: u32,
    pub version: String,
    pub running: bool,
    pub vulnerable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub name: String,
    pub description: String,
    pub severity: u32,
    pub exploit_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub handle: String,
    pub relationship: i32,
    pub faction: String,
    pub services: Vec<String>,
    pub last_contact: String,
}

// ============================================================================
// ТЕРМИНАЛЬНЫЙ ИНТЕРФЕЙС
// ============================================================================

pub struct Terminal {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<String>,
    pub typing_speed: u64,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            width: 120,
            height: 30,
            buffer: Vec::new(),
            typing_speed: 30,
        }
    }

    pub fn clear(&self) {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush().unwrap();
    }

    pub fn print_with_effect(&self, text: &str, effect: TerminalEffect) {
        match effect {
            TerminalEffect::TypeWriter => self.typewriter_effect(text),
            TerminalEffect::Matrix => self.matrix_effect(text),
            TerminalEffect::Glitch => self.glitch_effect(text),
            TerminalEffect::Error => self.error_effect(text),
            TerminalEffect::Success => self.success_effect(text),
            TerminalEffect::Warning => self.warning_effect(text),
            TerminalEffect::Normal => println!("{}", text),
        }
    }

    fn typewriter_effect(&self, text: &str) {
        for ch in text.chars() {
            print!("{}", ch);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(self.typing_speed));
        }
        println!();
    }

    fn matrix_effect(&self, text: &str) {
        let green = "\x1b[32m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        print!("{}{}{}{}{}", green, bold, text, reset, reset);
        println!();
    }

    fn glitch_effect(&self, text: &str) {
        let mut rng = rand::thread_rng();
        let chars = ['█', '▓', '▒', '░', '▀', '▄', '▌', '▐'];
        
        for _ in 0..3 {
            for ch in text.chars() {
                if rng.gen_bool(0.1) {
                    print!("{}", chars[rng.gen_range(0..chars.len())]);
                } else {
                    print!("{}", ch);
                }
            }
            print!("\r");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        println!("{}", text);
    }

    fn error_effect(&self, text: &str) {
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        println!("{}{}[ERROR]{} {}", red, bold, reset, text);
    }

    fn success_effect(&self, text: &str) {
        let green = "\x1b[32m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        println!("{}{}[SUCCESS]{} {}", green, bold, reset, text);
    }

    fn warning_effect(&self, text: &str) {
        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        println!("{}{}[WARNING]{} {}", yellow, bold, reset, text);
    }

    pub fn display_ascii_art(&self, art: &str) {
        let cyan = "\x1b[36m";
        let reset = "\x1b[0m";
        
        println!("{}{}{}", cyan, art, reset);
    }

    pub fn display_hud(&self, state: &GameState) {
        let blue = "\x1b[34m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        println!("{}{}╭─────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮{}", blue, bold, reset);
        println!("{}{}│ CYBERHACK v2.1.3 │ User: {} ({}) │ Level: {} │ XP: {} │ Health: {}/{} │ Stress: {}% │ Rep: {} │{}", 
                 blue, bold, state.player.name, state.player.handle, state.player.level, 
                 state.player.experience, state.player.health, state.player.max_health, 
                 state.player.stress, state.reputation, reset);
        println!("{}{}│ Location: {} │ Mode: {:?} │ Money: {}₿ │ Time: {:02}:{:02}:{:02} │{}", 
                 blue, bold, state.current_location, state.game_mode, state.money,
                 state.time_played.as_secs() / 3600,
                 (state.time_played.as_secs() % 3600) / 60,
                 state.time_played.as_secs() % 60, reset);
        println!("{}{}╰─────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯{}", blue, bold, reset);
        println!();
    }

    pub fn prompt(&self, prompt_text: &str) -> String {
        let cyan = "\x1b[36m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        print!("{}{}>>{} {} ", cyan, bold, reset, prompt_text);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn loading_animation(&self, text: &str, duration: Duration) {
        let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let start = Instant::now();
        let mut frame_idx = 0;
        
        while start.elapsed() < duration {
            print!("\r{} {}", frames[frame_idx], text);
            io::stdout().flush().unwrap();
            frame_idx = (frame_idx + 1) % frames.len();
            thread::sleep(Duration::from_millis(100));
        }
        println!("\r✓ {}", text);
    }
}

#[derive(Debug, Clone)]
pub enum TerminalEffect {
    TypeWriter,
    Matrix,
    Glitch,
    Error,
    Success,
    Warning,
    Normal,
}

// ============================================================================
// СИСТЕМА КОМАНД
// ============================================================================

pub struct CommandProcessor {
    pub commands: HashMap<String, Command>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub requires_target: bool,
    pub requires_connection: bool,
}

impl CommandProcessor {
    pub fn new() -> Self {
        let mut processor = CommandProcessor {
            commands: HashMap::new(),
        };
        processor.register_commands();
        processor
    }

    fn register_commands(&mut self) {
        let commands = vec![
            ("help", "Показать список доступных команд", "help [команда]", false, false),
            ("scan", "Сканировать сеть на наличие систем", "scan <ip_range>", true, false),
            ("nmap", "Детальное сканирование портов", "nmap <target>", true, false),
            ("connect", "Подключиться к системе", "connect <ip>", true, false),
            ("disconnect", "Отключиться от системы", "disconnect", false, true),
            ("ls", "Показать файлы в директории", "ls [путь]", false, true),
            ("cat", "Показать содержимое файла", "cat <файл>", true, true),
            ("cd", "Изменить директорию", "cd <путь>", true, true),
            ("pwd", "Показать текущую директорию", "pwd", false, true),
            ("download", "Скачать файл", "download <файл>", true, true),
            ("upload", "Загрузить файл", "upload <файл>", true, true),
            ("exploit", "Использовать эксплойт", "exploit <уязвимость>", true, true),
            ("crack", "Взломать пароль", "crack <файл>", true, true),
            ("decrypt", "Расшифровать файл", "decrypt <файл> [ключ]", true, true),
            ("backdoor", "Установить бэкдор", "backdoor", false, true),
            ("rootkit", "Установить руткит", "rootkit", false, true),
            ("trace", "Отследить соединение", "trace", false, false),
            ("proxy", "Использовать прокси", "proxy <ip>", true, false),
            ("tor", "Подключиться через Tor", "tor", false, false),
            ("vpn", "Подключиться через VPN", "vpn <сервер>", true, false),
            ("social", "Социальная инженерия", "social <цель>", true, false),
            ("ddos", "DDoS атака", "ddos <цель>", true, false),
            ("mitm", "Man-in-the-middle атака", "mitm <цель1> <цель2>", true, false),
            ("keylog", "Установить кейлоггер", "keylog", false, true),
            ("screen", "Сделать скриншот", "screen", false, true),
            ("webcam", "Получить доступ к веб-камере", "webcam", false, true),
            ("mic", "Получить доступ к микрофону", "mic", false, true),
            ("mail", "Читать почту", "mail [пользователь]", false, true),
            ("db", "Доступ к базе данных", "db <запрос>", true, true),
            ("log", "Просмотр логов", "log [сервис]", false, true),
            ("ps", "Список процессов", "ps", false, true),
            ("kill", "Убить процесс", "kill <pid>", true, true),
            ("mount", "Монтировать диск", "mount <устройство>", true, true),
            ("format", "Форматировать диск", "format <устройство>", true, true),
            ("wipe", "Затереть данные", "wipe <файл>", true, true),
            ("recover", "Восстановить данные", "recover <файл>", true, true),
            ("analyze", "Анализ файла", "analyze <файл>", true, true),
            ("disasm", "Дизассемблирование", "disasm <файл>", true, true),
            ("debug", "Отладка программы", "debug <файл>", true, true),
            ("compile", "Компиляция кода", "compile <файл>", true, true),
            ("run", "Запуск программы", "run <файл>", true, true),
            ("status", "Статус системы", "status", false, false),
            ("inventory", "Инвентарь", "inventory", false, false),
            ("skills", "Навыки", "skills", false, false),
            ("contacts", "Контакты", "contacts", false, false),
            ("quest", "Текущий квест", "quest", false, false),
            ("save", "Сохранить игру", "save [слот]", false, false),
            ("load", "Загрузить игру", "load [слот]", false, false),
            ("quit", "Выход из игры", "quit", false, false),
            ("sandbox", "Переключиться в режим песочницы", "sandbox", false, false),
            ("story", "Переключиться в сюжетный режим", "story", false, false),
        ];

        for (name, desc, usage, requires_target, requires_connection) in commands {
            self.commands.insert(name.to_string(), Command {
                name: name.to_string(),
                description: desc.to_string(),
                usage: usage.to_string(),
                requires_target,
                requires_connection,
            });
        }
    }

    pub fn execute_command(&self, input: &str, state: &mut GameState, terminal: &Terminal) -> bool {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return true;
        }

        let command = parts[0].to_lowercase();
        let args: Vec<&str> = parts.iter().skip(1).cloned().collect();

        match command.as_str() {
            "help" => self.show_help(&args, terminal),
            "scan" => self.scan_network(&args, state, terminal),
            "nmap" => self.nmap_scan(&args, state, terminal),
            "connect" => self.connect_to_system(&args, state, terminal),
            "disconnect" => self.disconnect_from_system(state, terminal),
            "ls" => self.list_files(&args, state, terminal),
            "cat" => self.show_file_content(&args, state, terminal),
            "cd" => self.change_directory(&args, state, terminal),
            "pwd" => self.print_working_directory(state, terminal),
            "download" => self.download_file(&args, state, terminal),
            "upload" => self.upload_file(&args, state, terminal),
            "exploit" => self.exploit_vulnerability(&args, state, terminal),
            "crack" => self.crack_password(&args, state, terminal),
            "decrypt" => self.decrypt_file(&args, state, terminal),
            "backdoor" => self.install_backdoor(state, terminal),
            "rootkit" => self.install_rootkit(state, terminal),
            "trace" => self.trace_connection(state, terminal),
            "proxy" => self.use_proxy(&args, state, terminal),
            "tor" => self.use_tor(state, terminal),
            "vpn" => self.use_vpn(&args, state, terminal),
            "social" => self.social_engineering(&args, state, terminal),
            "ddos" => self.ddos_attack(&args, state, terminal),
            "mitm" => self.mitm_attack(&args, state, terminal),
            "keylog" => self.install_keylogger(state, terminal),
            "screen" => self.take_screenshot(state, terminal),
            "webcam" => self.access_webcam(state, terminal),
            "mic" => self.access_microphone(state, terminal),
            "mail" => self.read_mail(&args, state, terminal),
            "db" => self.database_query(&args, state, terminal),
            "log" => self.view_logs(&args, state, terminal),
            "ps" => self.list_processes(state, terminal),
            "kill" => self.kill_process(&args, state, terminal),
            "mount" => self.mount_device(&args, state, terminal),
            "format" => self.format_device(&args, state, terminal),
            "wipe" => self.wipe_data(&args, state, terminal),
            "recover" => self.recover_data(&args, state, terminal),
            "analyze" => self.analyze_file(&args, state, terminal),
            "disasm" => self.disassemble_file(&args, state, terminal),
            "debug" => self.debug_program(&args, state, terminal),
            "compile" => self.compile_code(&args, state, terminal),
            "run" => self.run_program(&args, state, terminal),
            "status" => self.show_status(state, terminal),
            "inventory" => self.show_inventory(state, terminal),
            "skills" => self.show_skills(state, terminal),
            "contacts" => self.show_contacts(state, terminal),
            "quest" => self.show_current_quest(state, terminal),
            "save" => self.save_game(&args, state, terminal),
            "load" => self.load_game(&args, state, terminal),
            "quit" => return false,
            "sandbox" => self.switch_to_sandbox(state, terminal),
            "story" => self.switch_to_story(state, terminal),
            _ => {
                terminal.print_with_effect(&format!("Неизвестная команда: {}", command), TerminalEffect::Error);
                terminal.print_with_effect("Используйте 'help' для списка команд", TerminalEffect::Normal);
            }
        }
        true
    }

    // Реализация команд будет здесь...
    fn show_help(&self, args: &[&str], terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("═══════════════════════════════════════════════════════════════════════════", TerminalEffect::Matrix);
            terminal.print_with_effect("                              СПРАВКА ПО КОМАНДАМ", TerminalEffect::Matrix);
            terminal.print_with_effect("═══════════════════════════════════════════════════════════════════════════", TerminalEffect::Matrix);
            terminal.print_with_effect("", TerminalEffect::Normal);
            
            let mut categories = HashMap::new();
            categories.insert("Сетевые команды", vec!["scan", "nmap", "connect", "disconnect", "trace", "proxy", "tor", "vpn"]);
            categories.insert("Файловые операции", vec!["ls", "cat", "cd", "pwd", "download", "upload", "wipe", "recover"]);
            categories.insert("Взлом и эксплуатация", vec!["exploit", "crack", "decrypt", "backdoor", "rootkit", "social", "ddos", "mitm"]);
            categories.insert("Мониторинг", vec!["keylog", "screen", "webcam", "mic", "mail", "log", "ps"]);
            categories.insert("Системные", vec!["kill", "mount", "format", "analyze", "disasm", "debug", "compile", "run"]);
            categories.insert("Игровые", vec!["status", "inventory", "skills", "contacts", "quest", "save", "load", "sandbox", "story"]);

            for (category, commands) in categories {
                terminal.print_with_effect(&format!("┌─ {} ─┐", category), TerminalEffect::Success);
                for cmd in commands {
                    if let Some(command) = self.commands.get(cmd) {
                        terminal.print_with_effect(&format!("  {:<15} - {}", command.name, command.description), TerminalEffect::Normal);
                    }
                }
                terminal.print_with_effect("", TerminalEffect::Normal);
            }
            
            terminal.print_with_effect("Используйте 'help <команда>' для детальной информации", TerminalEffect::Warning);
        } else {
            let cmd_name = args[0];
            if let Some(command) = self.commands.get(cmd_name) {
                terminal.print_with_effect(&format!("Команда: {}", command.name), TerminalEffect::Success);
                terminal.print_with_effect(&format!("Описание: {}", command.description), TerminalEffect::Normal);
                terminal.print_with_effect(&format!("Использование: {}", command.usage), TerminalEffect::Normal);
                if command.requires_connection {
                    terminal.print_with_effect("⚠ Требует подключения к системе", TerminalEffect::Warning);
                }
            } else {
                terminal.print_with_effect(&format!("Команда '{}' не найдена", cmd_name), TerminalEffect::Error);
            }
        }
    }

    fn scan_network(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: scan <ip_range>", TerminalEffect::Error);
            return;
        }

        let target = args[0];
        terminal.loading_animation(&format!("Сканирование сети {}...", target), Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        let host_count = rng.gen_range(3..12);
        
        terminal.print_with_effect("┌─────────────────────────────────────────────────────────┐", TerminalEffect::Matrix);
        terminal.print_with_effect("│                    РЕЗУЛЬТАТЫ СКАНИРОВАНИЯ             │", TerminalEffect::Matrix);
        terminal.print_with_effect("├─────────────────────────────────────────────────────────┤", TerminalEffect::Matrix);
        
        for i in 1..=host_count {
            let ip = format!("{}.{}", target, i);
            let status = if rng.gen_bool(0.7) { "ONLINE" } else { "OFFLINE" };
            let os = ["Windows", "Linux", "macOS", "FreeBSD", "Solaris"][rng.gen_range(0..5)];
            
            if status == "ONLINE" {
                terminal.print_with_effect(&format!("│ {:<15} │ {:<7} │ {:<10} │", ip, status, os), TerminalEffect::Success);
                
                // Добавляем систему в сеть если её нет
                if !state.networks.contains_key(&ip) {
                    let system = System {
                        name: format!("system_{}", i),
                        os: os.to_string(),
                        security_level: rng.gen_range(1..10),
                        files: generate_random_files(),
                        services: generate_random_services(),
                        vulnerabilities: generate_random_vulnerabilities(),
                        is_compromised: false,
                        admin_access: false,
                    };
                    
                    let mut systems = HashMap::new();
                    systems.insert(ip.clone(), system);
                    
                    let network = Network {
                        name: format!("Network_{}", target),
                        security_level: rng.gen_range(1..8),
                        systems,
                        firewall_strength: rng.gen_range(1..10),
                        intrusion_detection: rng.gen_bool(0.6),
                        is_compromised: false,
                    };
                    
                    state.networks.insert(ip, network);
                }
            } else {
                terminal.print_with_effect(&format!("│ {:<15} │ {:<7} │ {:<10} │", ip, status, "N/A"), TerminalEffect::Normal);
            }
        }
        
        terminal.print_with_effect("└─────────────────────────────────────────────────────────┘", TerminalEffect::Matrix);
        
        // Добавляем опыт
        state.player.experience += 10;
        terminal.print_with_effect("+10 XP за сканирование сети", TerminalEffect::Success);
    }

    fn nmap_scan(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: nmap <target>", TerminalEffect::Error);
            return;
        }

        let target = args[0];
        terminal.loading_animation(&format!("Сканирование портов {}...", target), Duration::from_secs(4));
        
        if let Some(network) = state.networks.get(target) {
            if let Some(system) = network.systems.get(target) {
                terminal.print_with_effect(&format!("═══ NMAP SCAN RESULTS FOR {} ═══", target), TerminalEffect::Matrix);
                terminal.print_with_effect(&format!("OS: {}", system.os), TerminalEffect::Normal);
                terminal.print_with_effect(&format!("Security Level: {}/10", system.security_level), TerminalEffect::Normal);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("ОТКРЫТЫЕ ПОРТЫ:", TerminalEffect::Success);
                
                for service in &system.services {
                    if service.running {
                        let status = if service.vulnerable { "VULNERABLE" } else { "SECURE" };
                        let color = if service.vulnerable { TerminalEffect::Warning } else { TerminalEffect::Success };
                        terminal.print_with_effect(
                            &format!("{:<6} {:<20} {:<10} [{}]", 
                                     service.port, service.name, service.version, status), 
                            color
                        );
                    }
                }
                
                if !system.vulnerabilities.is_empty() {
                    terminal.print_with_effect("", TerminalEffect::Normal);
                    terminal.print_with_effect("ОБНАРУЖЕННЫЕ УЯЗВИМОСТИ:", TerminalEffect::Warning);
                    for vuln in &system.vulnerabilities {
                        terminal.print_with_effect(
                            &format!("• {} (Серьезность: {}/10)", vuln.name, vuln.severity), 
                            TerminalEffect::Warning
                        );
                    }
                }
                
                state.player.experience += 25;
                terminal.print_with_effect("+25 XP за детальное сканирование", TerminalEffect::Success);
            } else {
                terminal.print_with_effect("Система не найдена в сети", TerminalEffect::Error);
            }
        } else {
            terminal.print_with_effect("Сначала просканируйте сеть командой 'scan'", TerminalEffect::Error);
        }
    }

    fn connect_to_system(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: connect <ip>", TerminalEffect::Error);
            return;
        }

        let target = args[0];
        
        if state.player.current_system.is_some() {
            terminal.print_with_effect("Сначала отключитесь от текущей системы", TerminalEffect::Warning);
            return;
        }

        if let Some(network) = state.networks.get(target) {
            if network.systems.contains_key(target) {
                terminal.loading_animation(&format!("Подключение к {}...", target), Duration::from_secs(2));
                
                let mut rng = rand::thread_rng();
                let success_chance = if network.firewall_strength > 5 { 0.6 } else { 0.8 };
                
                if rng.gen_bool(success_chance) {
                    state.player.current_system = Some(target.to_string());
                    state.current_location = format!("{}:/home/user", target);
                    terminal.print_with_effect(&format!("Успешное подключение к {}", target), TerminalEffect::Success);
                    terminal.print_with_effect(&format!("Текущая директория: /home/user"), TerminalEffect::Normal);
                    
                    state.player.experience += 15;
                    terminal.print_with_effect("+15 XP за подключение к системе", TerminalEffect::Success);
                } else {
                    terminal.print_with_effect("Подключение заблокировано файрволом", TerminalEffect::Error);
                    if network.intrusion_detection {
                        terminal.print_with_effect("⚠ ОБНАРУЖЕНА ПОПЫТКА ВТОРЖЕНИЯ!", TerminalEffect::Warning);
                        state.player.stress += 10;
                    }
                }
            } else {
                terminal.print_with_effect("Система недоступна", TerminalEffect::Error);
            }
        } else {
            terminal.print_with_effect("IP-адрес не найден. Сначала просканируйте сеть", TerminalEffect::Error);
        }
    }

    fn disconnect_from_system(&self, state: &mut GameState, terminal: &Terminal) {
        if let Some(system_ip) = &state.player.current_system {
            terminal.loading_animation(&format!("Отключение от {}...", system_ip), Duration::from_secs(1));
            state.player.current_system = None;
            state.current_location = "Local Terminal".to_string();
            terminal.print_with_effect("Отключение выполнено", TerminalEffect::Success);
        } else {
            terminal.print_with_effect("Нет активного подключения", TerminalEffect::Warning);
        }
    }

    // Остальные методы команд...
    fn list_files(&self, args: &[&str], state: &GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                terminal.print_with_effect("┌─────────────────────────────────────────────────────────┐", TerminalEffect::Matrix);
                terminal.print_with_effect("│                      СОДЕРЖИМОЕ ПАПКИ                  │", TerminalEffect::Matrix);
                terminal.print_with_effect("├─────────────────────────────────────────────────────────┤", TerminalEffect::Matrix);
                
                for (filename, file) in &system.files {
                    let lock_icon = if file.encrypted { "🔒" } else { "  " };
                    let size_str = format!("{} KB", file.size);
                    terminal.print_with_effect(
                        &format!("│ {} {:<30} {:<10} {} │", 
                                 lock_icon, filename, file.permissions, size_str), 
                        TerminalEffect::Normal
                    );
                }
                
                terminal.print_with_effect("└─────────────────────────────────────────────────────────┘", TerminalEffect::Matrix);
            }
        }
    }

    fn show_file_content(&self, args: &[&str], state: &GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: cat <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if let Some(file) = system.files.get(filename) {
                    if file.encrypted {
                        terminal.print_with_effect("Файл зашифрован. Используйте 'decrypt' для расшифровки", TerminalEffect::Warning);
                        return;
                    }
                    
                    terminal.print_with_effect(&format!("═══ {} ═══", filename), TerminalEffect::Matrix);
                    terminal.print_with_effect(&file.content, TerminalEffect::TypeWriter);
                    terminal.print_with_effect("═══ EOF ═══", TerminalEffect::Matrix);
                } else {
                    terminal.print_with_effect(&format!("Файл '{}' не найден", filename), TerminalEffect::Error);
                }
            }
        }
    }

    fn change_directory(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: cd <путь>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let path = args[0];
        let system_ip = state.player.current_system.as_ref().unwrap();
        let new_location = format!("{}:{}", system_ip, path);
        state.current_location = new_location;
        
        terminal.print_with_effect(&format!("Переход в директорию: {}", path), TerminalEffect::Success);
    }

    fn print_working_directory(&self, state: &GameState, terminal: &Terminal) {
        terminal.print_with_effect(&state.current_location, TerminalEffect::Normal);
    }

    // Остальные команды будут добавлены далее...
    
    fn show_status(&self, state: &GameState, terminal: &Terminal) {
        terminal.print_with_effect("╔═══════════════════════════════════════════════════════╗", TerminalEffect::Matrix);
        terminal.print_with_effect("║                   СТАТУС ИГРОКА                      ║", TerminalEffect::Matrix);
        terminal.print_with_effect("╠═══════════════════════════════════════════════════════╣", TerminalEffect::Matrix);
        terminal.print_with_effect(&format!("║ Имя: {:<45} ║", state.player.name), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Псевдоним: {:<40} ║", state.player.handle), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Уровень: {:<42} ║", state.player.level), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Опыт: {:<46} ║", state.player.experience), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Здоровье: {}/{:<35} ║", state.player.health, state.player.max_health), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Стресс: {}%{:<40} ║", state.player.stress, ""), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Репутация: {:<39} ║", state.reputation), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("║ Деньги: {}₿{:<38} ║", state.money, ""), TerminalEffect::Normal);
        terminal.print_with_effect("╚═══════════════════════════════════════════════════════╝", TerminalEffect::Matrix);
    }

    fn show_inventory(&self, state: &GameState, terminal: &Terminal) {
        terminal.print_with_effect("═══ ИНВЕНТАРЬ ═══", TerminalEffect::Matrix);
        if state.inventory.is_empty() {
            terminal.print_with_effect("Инвентарь пуст", TerminalEffect::Warning);
        } else {
            for (i, item) in state.inventory.iter().enumerate() {
                terminal.print_with_effect(&format!("{}. {}", i + 1, item), TerminalEffect::Normal);
            }
        }
    }

    fn show_skills(&self, state: &GameState, terminal: &Terminal) {
        terminal.print_with_effect("═══ НАВЫКИ ═══", TerminalEffect::Matrix);
        if state.skills.is_empty() {
            terminal.print_with_effect("Навыки не изучены", TerminalEffect::Warning);
        } else {
            for (skill, level) in &state.skills {
                let bar = "█".repeat(*level as usize / 10);
                terminal.print_with_effect(&format!("{:<20} [{:<10}] {}/100", skill, bar, level), TerminalEffect::Success);
            }
        }
    }

    fn show_contacts(&self, state: &GameState, terminal: &Terminal) {
        terminal.print_with_effect("═══ КОНТАКТЫ ═══", TerminalEffect::Matrix);
        if state.contacts.is_empty() {
            terminal.print_with_effect("Контактов нет", TerminalEffect::Warning);
        } else {
            for (handle, contact) in &state.contacts {
                let relationship_str = match contact.relationship {
                    r if r > 50 => "Союзник",
                    r if r > 0 => "Знакомый",
                    r if r == 0 => "Нейтральный",
                    _ => "Враг",
                };
                terminal.print_with_effect(
                    &format!("{} ({}) - {} - {}", 
                             contact.name, handle, contact.faction, relationship_str), 
                    TerminalEffect::Normal
                );
            }
        }
    }

    fn show_current_quest(&self, state: &GameState, terminal: &Terminal) {
        if let Some(quest) = &state.current_quest {
            terminal.print_with_effect("═══ ТЕКУЩИЙ КВЕСТ ═══", TerminalEffect::Matrix);
            terminal.print_with_effect(&format!("Название: {}", quest.title), TerminalEffect::Success);
            terminal.print_with_effect(&format!("Описание: {}", quest.description), TerminalEffect::Normal);
            terminal.print_with_effect(&format!("Сложность: {}/10", quest.difficulty), TerminalEffect::Warning);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("ЦЕЛИ:", TerminalEffect::Matrix);
            for (i, objective) in quest.objectives.iter().enumerate() {
                let status = if objective.completed { "✓" } else { "○" };
                terminal.print_with_effect(&format!("{}. {} {}", i + 1, status, objective.description), TerminalEffect::Normal);
            }
        } else {
            terminal.print_with_effect("Нет активных квестов", TerminalEffect::Warning);
        }
    }

    // Полная реализация команд хакинга
    fn download_file(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: download <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if let Some(file) = system.files.get(filename) {
                    if file.encrypted {
                        terminal.print_with_effect("Нельзя скачать зашифрованный файл. Сначала расшифруйте его.", TerminalEffect::Error);
                        return;
                    }
                    
                    terminal.loading_animation(&format!("Скачивание {}...", filename), Duration::from_secs(2));
                    
                    // Проверка на обнаружение
                    let mut rng = rand::thread_rng();
                    if network.intrusion_detection && rng.gen_bool(0.3) {
                        terminal.print_with_effect("⚠ ВНИМАНИЕ: Обнаружена подозрительная активность!", TerminalEffect::Warning);
                        state.player.stress += 15;
                    }
                    
                    state.inventory.push(format!("{} ({}KB)", filename, file.size));
                    terminal.print_with_effect(&format!("Файл {} успешно скачан", filename), TerminalEffect::Success);
                    state.player.experience += 20;
                    terminal.print_with_effect("+20 XP за скачивание файла", TerminalEffect::Success);
                } else {
                    terminal.print_with_effect(&format!("Файл '{}' не найден", filename), TerminalEffect::Error);
                }
            }
        }
    }

    fn upload_file(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: upload <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        
        // Проверяем наличие файла в инвентаре
        let has_file = state.inventory.iter().any(|item| item.contains(filename));
        if !has_file {
            terminal.print_with_effect("У вас нет такого файла для загрузки", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        terminal.loading_animation(&format!("Загрузка {}...", filename), Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.8) {
            terminal.print_with_effect(&format!("Файл {} успешно загружен на сервер", filename), TerminalEffect::Success);
            state.player.experience += 25;
            terminal.print_with_effect("+25 XP за загрузку файла", TerminalEffect::Success);
        } else {
            terminal.print_with_effect("Загрузка заблокирована системой безопасности", TerminalEffect::Error);
            state.player.stress += 10;
        }
    }

    fn exploit_vulnerability(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: exploit <уязвимость>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let vuln_name = args[0];
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get_mut(system_ip) {
            if let Some(system) = network.systems.get_mut(system_ip) {
                if let Some(vuln) = system.vulnerabilities.iter().find(|v| v.name.contains(vuln_name)) {
                    terminal.print_with_effect(&format!("Эксплуатация уязвимости: {}", vuln.name), TerminalEffect::Warning);
                    terminal.print_with_effect(&format!("Выполнение: {}", vuln.exploit_code), TerminalEffect::Matrix);
                    terminal.loading_animation("Эксплуатация уязвимости...", Duration::from_secs(4));
                    
                    let mut rng = rand::thread_rng();
                    let skill_bonus = *state.skills.get("Хакинг").unwrap_or(&0) as f64 / 100.0;
                    let success_chance = 0.6 + skill_bonus - (vuln.severity as f64 * 0.05);
                    
                    if rng.gen_bool(success_chance) {
                        terminal.print_with_effect("🎯 ЭКСПЛОЙТ УСПЕШЕН!", TerminalEffect::Success);
                        system.is_compromised = true;
                        
                        if vuln.severity >= 8 {
                            system.admin_access = true;
                            terminal.print_with_effect("🔓 ПОЛУЧЕНЫ ПРАВА АДМИНИСТРАТОРА!", TerminalEffect::Success);
                            state.player.experience += 100;
                            terminal.print_with_effect("+100 XP за получение root доступа", TerminalEffect::Success);
                        } else {
                            state.player.experience += 50;
                            terminal.print_with_effect("+50 XP за успешную эксплуатацию", TerminalEffect::Success);
                        }
                        
                        // Обновляем навык
                        let current_skill = *state.skills.get("Хакинг").unwrap_or(&0);
                        state.skills.insert("Хакинг".to_string(), current_skill + 5);
                        
                        // Проверяем цели квеста
                        if let Some(quest) = &mut state.current_quest {
                            for obj in &mut quest.objectives {
                                if obj.action == "exploit" && !obj.completed {
                                    obj.completed = true;
                                    terminal.print_with_effect("✓ Цель квеста выполнена!", TerminalEffect::Success);
                                    break;
                                }
                            }
                        }
                    } else {
                        terminal.print_with_effect("❌ Эксплойт не сработал", TerminalEffect::Error);
                        if network.intrusion_detection {
                            terminal.print_with_effect("🚨 ОБНАРУЖЕНО ВТОРЖЕНИЕ! Системы безопасности активированы!", TerminalEffect::Error);
                            state.player.stress += 25;
                        }
                    }
                } else {
                    terminal.print_with_effect(&format!("Уязвимость '{}' не найдена или не применима", vuln_name), TerminalEffect::Error);
                    terminal.print_with_effect("Используйте 'nmap' для поиска уязвимостей", TerminalEffect::Normal);
                }
            }
        }
    }

    fn crack_password(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: crack <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get_mut(system_ip) {
            if let Some(system) = network.systems.get_mut(system_ip) {
                if let Some(file) = system.files.get_mut(filename) {
                    if !file.encrypted {
                        terminal.print_with_effect("Файл не зашифрован", TerminalEffect::Warning);
                        return;
                    }
                    
                    terminal.print_with_effect("Запуск атаки по словарю...", TerminalEffect::Matrix);
                    terminal.loading_animation("Перебор паролей", Duration::from_secs(5));
                    
                    let mut rng = rand::thread_rng();
                    let skill_bonus = *state.skills.get("Криптография").unwrap_or(&0) as f64 / 100.0;
                    let success_chance = 0.4 + skill_bonus;
                    
                    if rng.gen_bool(success_chance) {
                        let password = file.password.as_ref().unwrap_or(&"unknown".to_string()).clone();
                        terminal.print_with_effect(&format!("🔓 ПАРОЛЬ ВЗЛОМАН: {}", password), TerminalEffect::Success);
                        file.encrypted = false;
                        state.player.experience += 75;
                        terminal.print_with_effect("+75 XP за взлом пароля", TerminalEffect::Success);
                        
                        // Обновляем навык
                        let current_skill = *state.skills.get("Криптография").unwrap_or(&0);
                        state.skills.insert("Криптография".to_string(), current_skill + 3);
                    } else {
                        terminal.print_with_effect("❌ Не удалось взломать пароль", TerminalEffect::Error);
                        terminal.print_with_effect("Попробуйте использовать более продвинутые методы", TerminalEffect::Normal);
                        state.player.stress += 5;
                    }
                } else {
                    terminal.print_with_effect(&format!("Файл '{}' не найден", filename), TerminalEffect::Error);
                }
            }
        }
    }

    fn decrypt_file(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: decrypt <файл> [ключ]", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        let key = if args.len() > 1 { Some(args[1]) } else { None };
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get_mut(system_ip) {
            if let Some(system) = network.systems.get_mut(system_ip) {
                if let Some(file) = system.files.get_mut(filename) {
                    if !file.encrypted {
                        terminal.print_with_effect("Файл уже расшифрован", TerminalEffect::Warning);
                        return;
                    }
                    
                    terminal.loading_animation("Расшифровка файла...", Duration::from_secs(3));
                    
                    let mut success = false;
                    
                    if let Some(provided_key) = key {
                        if let Some(correct_key) = &file.password {
                            if provided_key == correct_key {
                                success = true;
                                terminal.print_with_effect("✓ Ключ верен! Файл расшифрован", TerminalEffect::Success);
                            }
                        }
                    } else {
                        // Автоматическая попытка расшифровки
                        let mut rng = rand::thread_rng();
                        let skill_bonus = *state.skills.get("Криптография").unwrap_or(&0) as f64 / 100.0;
                        success = rng.gen_bool(0.3 + skill_bonus);
                    }
                    
                    if success {
                        file.encrypted = false;
                        terminal.print_with_effect(&format!("🔓 Файл {} успешно расшифрован", filename), TerminalEffect::Success);
                        state.player.experience += 60;
                        terminal.print_with_effect("+60 XP за расшифровку", TerminalEffect::Success);
                        
                        // Показываем содержимое если это важный файл
                        if filename.contains("nuclear") || filename.contains("secret") || filename.contains("codes") {
                            terminal.print_with_effect("", TerminalEffect::Normal);
                            terminal.print_with_effect("🔥 КРИТИЧЕСКИ ВАЖНАЯ ИНФОРМАЦИЯ ОБНАРУЖЕНА! 🔥", TerminalEffect::Error);
                            terminal.print_with_effect(&file.content, TerminalEffect::TypeWriter);
                        }
                    } else {
                        terminal.print_with_effect("❌ Не удалось расшифровать файл", TerminalEffect::Error);
                        if key.is_some() {
                            terminal.print_with_effect("Неверный ключ", TerminalEffect::Error);
                        } else {
                            terminal.print_with_effect("Требуется ключ шифрования", TerminalEffect::Error);
                        }
                    }
                } else {
                    terminal.print_with_effect(&format!("Файл '{}' не найден", filename), TerminalEffect::Error);
                }
            }
        }
    }

    fn install_backdoor(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get_mut(system_ip) {
            if let Some(system) = network.systems.get_mut(system_ip) {
                if !system.is_compromised {
                    terminal.print_with_effect("Система не скомпрометирована. Сначала используйте эксплойт", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect("Установка бэкдора...", TerminalEffect::Matrix);
                terminal.loading_animation("Внедрение в системные процессы", Duration::from_secs(4));
                
                let mut rng = rand::thread_rng();
                let skill_bonus = *state.skills.get("Хакинг").unwrap_or(&0) as f64 / 100.0;
                let success_chance = 0.7 + skill_bonus;
                
                if rng.gen_bool(success_chance) {
                    terminal.print_with_effect("🚪 БЭКДОР УСПЕШНО УСТАНОВЛЕН!", TerminalEffect::Success);
                    terminal.print_with_effect("Теперь вы можете получить доступ к системе в любое время", TerminalEffect::Success);
                    
                    // Добавляем бэкдор в инвентарь
                    state.inventory.push(format!("Backdoor to {}", system_ip));
                    state.player.experience += 80;
                    terminal.print_with_effect("+80 XP за установку бэкдора", TerminalEffect::Success);
                    
                    // Обновляем навык
                    let current_skill = *state.skills.get("Хакинг").unwrap_or(&0);
                    state.skills.insert("Хакинг".to_string(), current_skill + 4);
                    
                    // Проверяем цели квеста
                    if let Some(quest) = &mut state.current_quest {
                        for obj in &mut quest.objectives {
                            if obj.action == "backdoor" && !obj.completed {
                                obj.completed = true;
                                terminal.print_with_effect("✓ Цель квеста выполнена!", TerminalEffect::Success);
                                break;
                            }
                        }
                    }
                } else {
                    terminal.print_with_effect("❌ Установка бэкдора не удалась", TerminalEffect::Error);
                    if network.intrusion_detection {
                        terminal.print_with_effect("🚨 ОБНАРУЖЕНА ПОДОЗРИТЕЛЬНАЯ АКТИВНОСТЬ!", TerminalEffect::Warning);
                        state.player.stress += 20;
                    }
                }
            }
        }
    }

    fn install_rootkit(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get_mut(system_ip) {
            if let Some(system) = network.systems.get_mut(system_ip) {
                if !system.admin_access {
                    terminal.print_with_effect("Требуются права администратора", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect("Установка руткита...", TerminalEffect::Matrix);
                terminal.print_with_effect("Модификация системных файлов...", TerminalEffect::Warning);
                terminal.loading_animation("Скрытие следов присутствия", Duration::from_secs(6));
                
                let mut rng = rand::thread_rng();
                let skill_bonus = *state.skills.get("Хакинг").unwrap_or(&0) as f64 / 100.0;
                let success_chance = 0.8 + skill_bonus;
                
                if rng.gen_bool(success_chance) {
                    terminal.print_with_effect("👻 РУТКИТ УСПЕШНО УСТАНОВЛЕН!", TerminalEffect::Success);
                    terminal.print_with_effect("Система полностью под вашим контролем", TerminalEffect::Success);
                    terminal.print_with_effect("Ваше присутствие скрыто от системы мониторинга", TerminalEffect::Success);
                    
                    // Отключаем систему обнаружения вторжений
                    network.intrusion_detection = false;
                    
                    // Добавляем руткит в инвентарь
                    state.inventory.push(format!("Rootkit on {}", system_ip));
                    state.player.experience += 150;
                    terminal.print_with_effect("+150 XP за установку руткита", TerminalEffect::Success);
                    state.reputation += 25;
                    terminal.print_with_effect("+25 репутации в хакерском сообществе", TerminalEffect::Success);
                    
                    // Обновляем навыки
                    let current_skill = *state.skills.get("Хакинг").unwrap_or(&0);
                    state.skills.insert("Хакинг".to_string(), current_skill + 8);
                    let current_stealth = *state.skills.get("Анонимность").unwrap_or(&0);
                    state.skills.insert("Анонимность".to_string(), current_stealth + 5);
                } else {
                    terminal.print_with_effect("❌ Установка руткита не удалась", TerminalEffect::Error);
                    terminal.print_with_effect("Система обнаружила вредоносный код", TerminalEffect::Error);
                    state.player.stress += 30;
                }
            }
        }
    }

    fn trace_connection(&self, state: &mut GameState, terminal: &Terminal) {
        terminal.print_with_effect("Запуск трассировки соединения...", TerminalEffect::Matrix);
        terminal.loading_animation("Анализ маршрута пакетов", Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        let hops = rng.gen_range(8..15);
        
        terminal.print_with_effect("═══ TRACEROUTE РЕЗУЛЬТАТЫ ═══", TerminalEffect::Matrix);
        
        for i in 1..=hops {
            let delay = rng.gen_range(10..150);
            let ip_parts = (
                rng.gen_range(1..255),
                rng.gen_range(1..255),
                rng.gen_range(1..255),
                rng.gen_range(1..255)
            );
            
            terminal.print_with_effect(
                &format!("{:2}  {}.{}.{}.{}  {} ms", 
                         i, ip_parts.0, ip_parts.1, ip_parts.2, ip_parts.3, delay),
                TerminalEffect::Normal
            );
            thread::sleep(Duration::from_millis(200));
        }
        
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("Ваше местоположение:", TerminalEffect::Warning);
        
        if *state.skills.get("Анонимность").unwrap_or(&0) > 50 {
            terminal.print_with_effect("🔒 СКРЫТО - VPN/Tor активен", TerminalEffect::Success);
        } else {
            terminal.print_with_effect("🚨 ОБНАРУЖЕНО - требуется маскировка", TerminalEffect::Error);
            state.player.stress += 10;
        }
        
        state.player.experience += 15;
        terminal.print_with_effect("+15 XP за анализ сетевой активности", TerminalEffect::Success);
    }

    fn use_proxy(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: proxy <ip>", TerminalEffect::Error);
            return;
        }

        let proxy_ip = args[0];
        terminal.loading_animation(&format!("Подключение к прокси {}...", proxy_ip), Duration::from_secs(2));
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.8) {
            terminal.print_with_effect(&format!("✓ Подключение к прокси {} установлено", proxy_ip), TerminalEffect::Success);
            terminal.print_with_effect("Ваш IP-адрес теперь скрыт", TerminalEffect::Success);
            
            // Улучшаем анонимность
            let current_anon = *state.skills.get("Анонимность").unwrap_or(&0);
            state.skills.insert("Анонимность".to_string(), current_anon + 10);
            
            state.player.experience += 20;
            terminal.print_with_effect("+20 XP за использование прокси", TerminalEffect::Success);
            
            // Снижаем стресс
            if state.player.stress >= 10 {
                state.player.stress -= 10;
                terminal.print_with_effect("Уровень стресса снижен", TerminalEffect::Success);
            }
        } else {
            terminal.print_with_effect("❌ Прокси-сервер недоступен", TerminalEffect::Error);
            terminal.print_with_effect("Попробуйте другой сервер", TerminalEffect::Normal);
        }
    }

    fn use_tor(&self, state: &mut GameState, terminal: &Terminal) {
        terminal.print_with_effect("Подключение к сети Tor...", TerminalEffect::Matrix);
        terminal.loading_animation("Установка анонимного соединения", Duration::from_secs(4));
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.9) {
            terminal.print_with_effect("🧅 Подключение к Tor установлено", TerminalEffect::Success);
            terminal.print_with_effect("Максимальный уровень анонимности активирован", TerminalEffect::Success);
            
            // Значительно улучшаем анонимность
            let current_anon = *state.skills.get("Анонимность").unwrap_or(&0);
            state.skills.insert("Анонимность".to_string(), std::cmp::min(100, current_anon + 25));
            
            state.player.experience += 40;
            terminal.print_with_effect("+40 XP за использование Tor", TerminalEffect::Success);
            
            // Существенно снижаем стресс
            if state.player.stress >= 20 {
                state.player.stress -= 20;
                terminal.print_with_effect("Стресс значительно снижен", TerminalEffect::Success);
            }
        } else {
            terminal.print_with_effect("❌ Не удалось подключиться к сети Tor", TerminalEffect::Error);
            terminal.print_with_effect("Сеть может быть заблокирована", TerminalEffect::Warning);
        }
    }

    fn use_vpn(&self, _args: &[&str], _state: &mut GameState, terminal: &Terminal) {
        terminal.print_with_effect("Команда в разработке...", TerminalEffect::Warning);
    }

    fn social_engineering(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: social <цель>", TerminalEffect::Error);
            return;
        }

        let target = args[0];
        terminal.print_with_effect(&format!("Анализ социальных связей цели: {}", target), TerminalEffect::Matrix);
        terminal.loading_animation("Сбор информации из социальных сетей", Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        let social_skill = *state.skills.get("Социальная инженерия").unwrap_or(&0);
        let success_chance = (social_skill as f64 / 100.0) * 0.8 + 0.2;
        
        if rng.gen_bool(success_chance) {
            let info_types = vec![
                "Пароль от Wi-Fi домашней сети",
                "Девичья фамилия матери", 
                "Кличка домашнего животного",
                "Дата рождения супруга",
                "Название первой школы",
                "Любимый ресторан",
                "Модель первого автомобиля",
                "Номер рабочего телефона",
            ];
            
            let discovered_info = &info_types[rng.gen_range(0..info_types.len())];
            
            terminal.print_with_effect("🎯 СОЦИАЛЬНАЯ ИНЖЕНЕРИЯ УСПЕШНА!", TerminalEffect::Success);
            terminal.print_with_effect(&format!("Обнаруженная информация: {}", discovered_info), TerminalEffect::Success);
            
            // Добавляем информацию в инвентарь
            state.inventory.push(format!("Social Info: {} - {}", target, discovered_info));
            
            state.player.experience += 40;
            terminal.print_with_effect("+40 XP за социальную инженерию", TerminalEffect::Success);
            
            // Обновляем навык
            let current_skill = social_skill;
            state.skills.insert("Социальная инженерия".to_string(), current_skill + 3);
            
            // Эта информация может быть полезна для взлома паролей
            terminal.print_with_effect("💡 Эта информация может использоваться для подбора паролей", TerminalEffect::Normal);
            
        } else {
            terminal.print_with_effect("❌ Не удалось получить полезную информацию", TerminalEffect::Error);
            terminal.print_with_effect("Цель оказалась слишком осторожной", TerminalEffect::Normal);
            state.player.stress += 5;
        }
    }

    fn ddos_attack(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: ddos <цель>", TerminalEffect::Error);
            return;
        }

        let target = args[0];
        terminal.print_with_effect(&format!("Подготовка DDoS атаки на {}", target), TerminalEffect::Warning);
        terminal.print_with_effect("Мобилизация ботнета...", TerminalEffect::Matrix);
        terminal.loading_animation("Координация атаки", Duration::from_secs(5));
        
        let mut rng = rand::thread_rng();
        let network_skill = *state.skills.get("Сетевая безопасность").unwrap_or(&0);
        let success_chance = (network_skill as f64 / 100.0) * 0.7 + 0.3;
        
        if rng.gen_bool(success_chance) {
            let attack_power = rng.gen_range(100..1000);
            terminal.print_with_effect("🌊 DDoS АТАКА ЗАПУЩЕНА!", TerminalEffect::Warning);
            terminal.print_with_effect(&format!("Мощность атаки: {} Gbps", attack_power), TerminalEffect::Warning);
            
            // Симуляция атаки
            for i in 1..=5 {
                thread::sleep(Duration::from_millis(500));
                let current_load = rng.gen_range(60..100);
                terminal.print_with_effect(
                    &format!("Волна {}: Нагрузка на сервер {}%", i, current_load), 
                    TerminalEffect::Error
                );
            }
            
            terminal.print_with_effect("💥 СЕРВЕР ПЕРЕГРУЖЕН! Цель недоступна", TerminalEffect::Error);
            terminal.print_with_effect("Атака успешно завершена", TerminalEffect::Success);
            
            state.player.experience += 60;
            terminal.print_with_effect("+60 XP за DDoS атаку", TerminalEffect::Success);
            state.reputation -= 10; // DDoS считается "грязным" методом
            terminal.print_with_effect("-10 репутации (DDoS не приветствуется сообществом)", TerminalEffect::Warning);
            
            // Обновляем навык
            let current_skill = network_skill;
            state.skills.insert("Сетевая безопасность".to_string(), current_skill + 4);
            
            // Высокий стресс от такой атаки
            state.player.stress += 25;
            
        } else {
            terminal.print_with_effect("❌ DDoS атака отражена", TerminalEffect::Error);
            terminal.print_with_effect("Цель имеет мощную защиту от DDoS", TerminalEffect::Error);
            terminal.print_with_effect("🚨 ВАША АКТИВНОСТЬ МОЖЕТ БЫТЬ ОТСЛЕЖЕНА!", TerminalEffect::Warning);
            state.player.stress += 15;
        }
    }

    fn mitm_attack(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.len() < 2 {
            terminal.print_with_effect("Использование: mitm <цель1> <цель2>", TerminalEffect::Error);
            return;
        }

        let target1 = args[0];
        let target2 = args[1];
        
        terminal.print_with_effect(&format!("Подготовка MITM атаки между {} и {}", target1, target2), TerminalEffect::Matrix);
        terminal.print_with_effect("Перехват сетевого трафика...", TerminalEffect::Warning);
        terminal.loading_animation("Анализ пакетов", Duration::from_secs(4));
        
        let mut rng = rand::thread_rng();
        let network_skill = *state.skills.get("Сетевая безопасность").unwrap_or(&0);
        let crypto_skill = *state.skills.get("Криптография").unwrap_or(&0);
        let avg_skill = (network_skill + crypto_skill) / 2;
        let success_chance = (avg_skill as f64 / 100.0) * 0.6 + 0.2;
        
        if rng.gen_bool(success_chance) {
            terminal.print_with_effect("🕷️ MITM АТАКА УСПЕШНА!", TerminalEffect::Success);
            terminal.print_with_effect("Перехвачен сетевой трафик:", TerminalEffect::Success);
            
            // Генерируем перехваченные данные
            let intercepted_data = vec![
                "HTTP Basic Auth: admin:password123",
                "FTP Login: user@company.com:qwerty",
                "Email: 'Встреча в 15:00 в конференц-зале'",
                "Database Query: SELECT * FROM users WHERE role='admin'",
                "API Key: ak_1234567890abcdef",
                "Session Token: sess_abc123xyz789",
                "Credit Card: 4532-****-****-1234 (частично)",
                "VPN Credentials: vpnuser:strongpass456",
            ];
            
            let intercepted = &intercepted_data[rng.gen_range(0..intercepted_data.len())];
            
            terminal.print_with_effect(&format!("📡 Перехвачено: {}", intercepted), TerminalEffect::Warning);
            
            // Добавляем данные в инвентарь
            state.inventory.push(format!("MITM Data: {}", intercepted));
            
            state.player.experience += 80;
            terminal.print_with_effect("+80 XP за успешный MITM", TerminalEffect::Success);
            
            // Обновляем навыки
            let current_net = network_skill;
            let current_crypto = crypto_skill;
            state.skills.insert("Сетевая безопасность".to_string(), current_net + 5);
            state.skills.insert("Криптография".to_string(), current_crypto + 3);
            
            // MITM - серьезная атака, добавляет стресс
            state.player.stress += 20;
            
        } else {
            terminal.print_with_effect("❌ MITM атака не удалась", TerminalEffect::Error);
            
            if rng.gen_bool(0.5) {
                terminal.print_with_effect("Соединение использует сильное шифрование", TerminalEffect::Error);
            } else {
                terminal.print_with_effect("Обнаружена аномальная сетевая активность", TerminalEffect::Error);
                terminal.print_with_effect("🚨 РИСК РАЗОБЛАЧЕНИЯ ПОВЫШЕН!", TerminalEffect::Warning);
                state.player.stress += 30;
            }
        }
    }

    fn install_keylogger(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.is_compromised {
                    terminal.print_with_effect("Система не скомпрометирована", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect("Установка кейлоггера...", TerminalEffect::Matrix);
                terminal.loading_animation("Внедрение в клавиатурный буфер", Duration::from_secs(3));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.75) {
                    terminal.print_with_effect("⌨️ КЕЙЛОГГЕР УСТАНОВЛЕН!", TerminalEffect::Success);
                    terminal.print_with_effect("Захват нажатий клавиш активирован", TerminalEffect::Success);
                    
                    // Симуляция захваченных нажатий
                    thread::sleep(Duration::from_secs(2));
                    terminal.print_with_effect("📝 Захваченные нажатия:", TerminalEffect::Matrix);
                    
                    let keystrokes = vec![
                        "admin login: adminsecure2023",
                        "database password: DB_Pass!123",
                        "email: confidential_report.pdf",
                        "search: nuclear facility security",
                        "sudo su - root",
                        "bank transfer: $50000 to account 123456",
                    ];
                    
                    let captured = &keystrokes[rng.gen_range(0..keystrokes.len())];
                    terminal.print_with_effect(&format!("⌨️ {}", captured), TerminalEffect::Warning);
                    
                    // Добавляем в инвентарь
                    state.inventory.push(format!("Keylog: {}", captured));
                    state.player.experience += 70;
                    terminal.print_with_effect("+70 XP за установку кейлоггера", TerminalEffect::Success);
                    
                } else {
                    terminal.print_with_effect("❌ Антивирус заблокировал кейлоггер", TerminalEffect::Error);
                    state.player.stress += 10;
                }
            }
        }
    }

    fn take_screenshot(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.is_compromised {
                    terminal.print_with_effect("Система не скомпрометирована", TerminalEffect::Error);
                    return;
                }
                
                terminal.loading_animation("Захват экрана", Duration::from_secs(2));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.8) {
                    let screenshots = vec![
                        "Рабочий стол с папкой 'Секретные проекты'",
                        "Открытый документ 'План Цифрового Апокалипсиса'",
                        "Банковский интерфейс с переводом $1M",
                        "Email с темой 'RE: Nuclear Facility Access'",
                        "Диаграмма сетевой архитектуры NEXUS",
                        "Календарь с записью 'Launch Day - 72 hours'",
                        "Terminal с командой 'rm -rf /evidence'",
                        "Браузер на сайте darknet marketplace",
                    ];
                    
                    let captured_screen = &screenshots[rng.gen_range(0..screenshots.len())];
                    
                    terminal.print_with_effect("📸 СКРИНШОТ СДЕЛАН!", TerminalEffect::Success);
                    terminal.print_with_effect(&format!("Содержание: {}", captured_screen), TerminalEffect::Normal);
                    
                    // Добавляем в инвентарь
                    state.inventory.push(format!("Screenshot: {}", captured_screen));
                    state.player.experience += 30;
                    terminal.print_with_effect("+30 XP за скриншот", TerminalEffect::Success);
                    
                } else {
                    terminal.print_with_effect("❌ Экран заблокирован или пуст", TerminalEffect::Error);
                }
            }
        }
    }

    fn access_webcam(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.admin_access {
                    terminal.print_with_effect("Требуются права администратора", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect("Получение доступа к веб-камере...", TerminalEffect::Matrix);
                terminal.loading_animation("Активация камеры", Duration::from_secs(3));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.6) {
                    terminal.print_with_effect("📹 ДОСТУП К КАМЕРЕ ПОЛУЧЕН!", TerminalEffect::Success);
                    
                    let observations = vec![
                        "Пустой офис в ночное время",
                        "Сотрудник работает над секретным проектом",
                        "Встреча руководства за закрытыми дверями",
                        "Человек в костюме уничтожает документы",
                        "Охранник спит на рабочем месте",
                        "Подозрительная личность копирует файлы",
                        "Экстренная эвакуация сотрудников",
                        "Военные в здании компании",
                    ];
                    
                    let observation = &observations[rng.gen_range(0..observations.len())];
                    terminal.print_with_effect(&format!("👁️ Наблюдение: {}", observation), TerminalEffect::Warning);
                    
                    // Добавляем в инвентарь
                    state.inventory.push(format!("Webcam: {}", observation));
                    state.player.experience += 50;
                    terminal.print_with_effect("+50 XP за доступ к камере", TerminalEffect::Success);
                    
                    // Это серьезное нарушение приватности
                    state.player.stress += 15;
                    state.reputation -= 5;
                    
                } else {
                    terminal.print_with_effect("❌ Камера отключена или не найдена", TerminalEffect::Error);
                }
            }
        }
    }

    fn access_microphone(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.admin_access {
                    terminal.print_with_effect("Требуются права администратора", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect("Активация микрофона...", TerminalEffect::Matrix);
                terminal.loading_animation("Захват аудио", Duration::from_secs(4));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.7) {
                    terminal.print_with_effect("🎤 МИКРОФОН АКТИВИРОВАН!", TerminalEffect::Success);
                    
                    let audio_captures = vec![
                        "Разговор: 'Проект должен запуститься через 72 часа'",
                        "Телефонный звонок: 'Коды ядерных объектов готовы'",
                        "Шепот: 'Они узнали о нашем плане'",
                        "Диктовка: 'Уничтожить все доказательства'",
                        "Конференция: 'Digital Apocalypse - финальная фаза'",
                        "Переговоры: 'Хакер может нам помешать'",
                        "Сообщение: 'Активируйте протокол экстренной эвакуации'",
                        "Планерка: 'Через 3 дня мир изменится навсегда'",
                    ];
                    
                    let audio = &audio_captures[rng.gen_range(0..audio_captures.len())];
                    terminal.print_with_effect(&format!("🔊 Запись: {}", audio), TerminalEffect::Warning);
                    
                    // Добавляем в инвентарь
                    state.inventory.push(format!("Audio: {}", audio));
                    state.player.experience += 45;
                    terminal.print_with_effect("+45 XP за прослушку", TerminalEffect::Success);
                    
                    // Это серьезное нарушение приватности
                    state.player.stress += 12;
                    
                } else {
                    terminal.print_with_effect("❌ Микрофон недоступен или заблокирован", TerminalEffect::Error);
                }
            }
        }
    }

    fn read_mail(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.is_compromised {
                    terminal.print_with_effect("Система не скомпрометирована", TerminalEffect::Error);
                    return;
                }
                
                let username = if args.len() > 0 { args[0] } else { "current_user" };
                
                terminal.print_with_effect(&format!("Доступ к почте пользователя: {}", username), TerminalEffect::Matrix);
                terminal.loading_animation("Сканирование почтового ящика", Duration::from_secs(2));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.85) {
                    terminal.print_with_effect("📧 ДОСТУП К ПОЧТЕ ПОЛУЧЕН!", TerminalEffect::Success);
                    terminal.print_with_effect("", TerminalEffect::Normal);
                    
                    let emails = vec![
                        ("CEO NEXUS", "Re: Final preparations", "The nuclear control system is ready. Activation in 72 hours. Ensure all evidence is destroyed."),
                        ("Security Chief", "URGENT: Hacker detected", "Our systems show signs of intrusion. Initiate lockdown protocols immediately."),
                        ("Dr. Mitchell", "Project DA - Status", "Digital Apocalypse project entering final phase. World governments will bow to our demands."),
                        ("Unknown", "Meeting tomorrow", "Warehouse District, Building 7, 23:00. Bring the codes. Trust no one."),
                        ("Finance Dept", "Payment confirmation", "Transfer of $50,000,000 to offshore account completed. Operation funding secured."),
                        ("IT Support", "System vulnerabilities", "Found several security holes. Patch immediately or risk total system compromise."),
                        ("Agent Smith", "Target acquired", "The hacker has been identified. Eliminate before they can interfere with our plans."),
                    ];
                    
                    let email = &emails[rng.gen_range(0..emails.len())];
                    
                    terminal.print_with_effect(&format!("От: {}", email.0), TerminalEffect::Normal);
                    terminal.print_with_effect(&format!("Тема: {}", email.1), TerminalEffect::Normal);
                    terminal.print_with_effect(&format!("Содержание: {}", email.2), TerminalEffect::TypeWriter);
                    
                    // Добавляем в инвентарь
                    state.inventory.push(format!("Email: {} - {}", email.0, email.1));
                    state.player.experience += 35;
                    terminal.print_with_effect("+35 XP за чтение почты", TerminalEffect::Success);
                    
                } else {
                    terminal.print_with_effect("❌ Почтовый ящик зашифрован или пуст", TerminalEffect::Error);
                }
            }
        }
    }

    fn database_query(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: db <запрос>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let query = args.join(" ");
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.is_compromised {
                    terminal.print_with_effect("Система не скомпрометирована", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect(&format!("Выполнение SQL запроса: {}", query), TerminalEffect::Matrix);
                terminal.loading_animation("Обращение к базе данных", Duration::from_secs(2));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.7) {
                    terminal.print_with_effect("🗄️ ЗАПРОС ВЫПОЛНЕН УСПЕШНО!", TerminalEffect::Success);
                    terminal.print_with_effect("", TerminalEffect::Normal);
                    
                    // Генерируем результаты в зависимости от запроса
                    if query.contains("users") || query.contains("admin") {
                        terminal.print_with_effect("| ID | USERNAME    | ROLE        | ACCESS_LEVEL |", TerminalEffect::Matrix);
                        terminal.print_with_effect("|  1 | admin       | superadmin  | 10           |", TerminalEffect::Normal);
                        terminal.print_with_effect("|  2 | nexus_ceo   | admin       | 9            |", TerminalEffect::Normal);
                        terminal.print_with_effect("|  3 | dr_mitchell | scientist   | 8            |", TerminalEffect::Normal);
                        terminal.print_with_effect("|  4 | security    | guard       | 5            |", TerminalEffect::Normal);
                    } else if query.contains("nuclear") || query.contains("facility") {
                        terminal.print_with_effect("| FACILITY_ID | LOCATION        | STATUS     | CODES       |", TerminalEffect::Matrix);
                        terminal.print_with_effect("| 001         | Nevada_Plant    | ACTIVE     | NV_7789AA   |", TerminalEffect::Warning);
                        terminal.print_with_effect("| 002         | Texas_Reactor   | STANDBY    | TX_9922BC   |", TerminalEffect::Warning);
                        terminal.print_with_effect("| 003         | California_Hub  | EMERGENCY  | CA_1134DD   |", TerminalEffect::Error);
                    } else {
                        terminal.print_with_effect("Query executed successfully. 247 rows affected.", TerminalEffect::Normal);
                    }
                    
                    state.player.experience += 55;
                    terminal.print_with_effect("+55 XP за работу с базой данных", TerminalEffect::Success);
                    
                    // Обновляем навык программирования
                    let current_prog = *state.skills.get("Программирование").unwrap_or(&0);
                    state.skills.insert("Программирование".to_string(), current_prog + 3);
                    
                } else {
                    terminal.print_with_effect("❌ ОШИБКА SQL ЗАПРОСА", TerminalEffect::Error);
                    terminal.print_with_effect("Access denied: insufficient privileges", TerminalEffect::Error);
                }
            }
        }
    }

    fn view_logs(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let service = if args.len() > 0 { args[0] } else { "system" };
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.is_compromised {
                    terminal.print_with_effect("Система не скомпрометирована", TerminalEffect::Error);
                    return;
                }
                
                terminal.print_with_effect(&format!("Просмотр логов службы: {}", service), TerminalEffect::Matrix);
                terminal.loading_animation("Анализ логов", Duration::from_secs(2));
                
                terminal.print_with_effect(&format!("═══ {} LOGS ═══", service.to_uppercase()), TerminalEffect::Matrix);
                
                let log_entries = vec![
                    "[2024-01-15 14:23:11] INFO: User 'admin' logged in from 192.168.1.100",
                    "[2024-01-15 14:25:33] WARN: Multiple failed login attempts detected",
                    "[2024-01-15 14:30:15] ERROR: Unauthorized access attempt from unknown IP",
                    "[2024-01-15 14:35:42] INFO: File 'nuclear_codes.enc' accessed by user 'dr_mitchell'",
                    "[2024-01-15 14:40:18] CRIT: Security breach detected in sector 7",
                    "[2024-01-15 14:45:23] INFO: Backup systems activated",
                    "[2024-01-15 14:50:01] WARN: Unusual network traffic pattern observed",
                    "[2024-01-15 14:55:14] ERROR: Database connection lost",
                ];
                
                for entry in &log_entries {
                    terminal.print_with_effect(entry, TerminalEffect::Normal);
                    thread::sleep(Duration::from_millis(300));
                }
                
                state.player.experience += 25;
                terminal.print_with_effect("+25 XP за анализ логов", TerminalEffect::Success);
                
                // Обновляем навык форензики
                let current_forensics = *state.skills.get("Форензика").unwrap_or(&0);
                state.skills.insert("Форензика".to_string(), current_forensics + 2);
            }
        }
    }

    fn list_processes(&self, state: &mut GameState, terminal: &Terminal) {
        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                terminal.print_with_effect("Список запущенных процессов:", TerminalEffect::Matrix);
                terminal.print_with_effect("", TerminalEffect::Normal);
                
                terminal.print_with_effect("| PID  | USER     | PROCESS                    | CPU% | MEM% |", TerminalEffect::Matrix);
                terminal.print_with_effect("|------|----------|----------------------------|------|------|", TerminalEffect::Matrix);
                
                let processes = vec![
                    ("1234", "root", "nuclear_control_system", "45.2", "12.8"),
                    ("1567", "admin", "backup_manager", "23.1", "8.4"),
                    ("1890", "nexus", "surveillance_monitor", "15.7", "6.2"),
                    ("2001", "system", "firewall_daemon", "8.3", "4.1"),
                    ("2234", "dr_mitchell", "crypto_processor", "67.8", "18.9"),
                    ("2456", "security", "intrusion_detector", "12.4", "5.7"),
                    ("2678", "admin", "log_analyzer", "9.1", "3.3"),
                    ("2890", "guest", "temp_cleaner", "2.1", "1.2"),
                ];
                
                for (pid, user, process, cpu, mem) in &processes {
                    let color = if process.contains("nuclear") || process.contains("crypto") {
                        TerminalEffect::Warning
                    } else if process.contains("security") || process.contains("firewall") {
                        TerminalEffect::Error
                    } else {
                        TerminalEffect::Normal
                    };
                    
                    terminal.print_with_effect(
                        &format!("| {:<4} | {:<8} | {:<26} | {:<4} | {:<4} |", 
                                 pid, user, process, cpu, mem), 
                        color
                    );
                }
                
                state.player.experience += 15;
                terminal.print_with_effect("+15 XP за анализ процессов", TerminalEffect::Success);
            }
        }
    }

    fn kill_process(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: kill <pid>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let pid = args[0];
        let system_ip = state.player.current_system.as_ref().unwrap();
        
        if let Some(network) = state.networks.get(system_ip) {
            if let Some(system) = network.systems.get(system_ip) {
                if !system.admin_access {
                    terminal.print_with_effect("Требуются права администратора", TerminalEffect::Error);
                    return;
                }
                
                terminal.loading_animation(&format!("Завершение процесса {}...", pid), Duration::from_secs(1));
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.8) {
                    let process_type = match pid {
                        "1234" => "nuclear_control_system",
                        "1567" => "backup_manager", 
                        "1890" => "surveillance_monitor",
                        "2001" => "firewall_daemon",
                        "2234" => "crypto_processor",
                        "2456" => "intrusion_detector",
                        _ => "unknown_process",
                    };
                    
                    terminal.print_with_effect(&format!("💀 Процесс {} ({}) завершен", pid, process_type), TerminalEffect::Success);
                    
                    match process_type {
                        "nuclear_control_system" => {
                            terminal.print_with_effect("⚠️ КРИТИЧНО: Система управления ядерными объектами отключена!", TerminalEffect::Error);
                            state.player.experience += 200;
                            state.reputation += 50;
                        },
                        "firewall_daemon" => {
                            terminal.print_with_effect("🔥 Файрвол отключен! Система уязвима", TerminalEffect::Warning);
                            state.player.experience += 100;
                        },
                        "intrusion_detector" => {
                            terminal.print_with_effect("👻 Система обнаружения вторжений отключена", TerminalEffect::Success);
                            state.player.experience += 80;
                        },
                        _ => {
                            state.player.experience += 30;
                        }
                    }
                    
                    terminal.print_with_effect(&format!("+{} XP за завершение процесса", 
                        if process_type == "nuclear_control_system" { 200 } else { 30 }), TerminalEffect::Success);
                    
                } else {
                    terminal.print_with_effect("❌ Не удалось завершить процесс", TerminalEffect::Error);
                    terminal.print_with_effect("Процесс защищен системой безопасности", TerminalEffect::Error);
                }
            }
        }
    }

    fn mount_device(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: mount <устройство>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let device = args[0];
        terminal.loading_animation(&format!("Монтирование устройства {}...", device), Duration::from_secs(2));
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.75) {
            terminal.print_with_effect(&format!("📁 Устройство {} успешно смонтировано", device), TerminalEffect::Success);
            
            // Генерируем содержимое устройства
            let device_contents = vec![
                "Backup drives with encrypted corporate data",
                "USB with personal files and photos", 
                "External HDD containing surveillance footage",
                "Network drive with financial records",
                "Mobile device with contact lists",
                "Server backup with user databases",
                "Encrypted drive with nuclear facility blueprints",
                "Archive drive with deleted security logs",
            ];
            
            let content = &device_contents[rng.gen_range(0..device_contents.len())];
            terminal.print_with_effect(&format!("Содержимое: {}", content), TerminalEffect::Normal);
            
            // Добавляем доступ к новым файлам
            state.inventory.push(format!("Mounted: {} - {}", device, content));
            state.player.experience += 40;
            terminal.print_with_effect("+40 XP за монтирование устройства", TerminalEffect::Success);
            
        } else {
            terminal.print_with_effect("❌ Не удалось смонтировать устройство", TerminalEffect::Error);
            terminal.print_with_effect("Устройство повреждено или зашифровано", TerminalEffect::Error);
        }
    }

    fn format_device(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: format <устройство>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let device = args[0];
        terminal.print_with_effect(&format!("⚠️ ВНИМАНИЕ: Форматирование {} уничтожит ВСЕ данные!", device), TerminalEffect::Warning);
        terminal.loading_animation("Форматирование устройства", Duration::from_secs(4));
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.9) {
            terminal.print_with_effect(&format!("💾 Устройство {} отформатировано", device), TerminalEffect::Success);
            terminal.print_with_effect("Все данные безвозвратно уничтожены", TerminalEffect::Warning);
            
            // Если это было важное устройство, влияет на сюжет
            if device.contains("backup") || device.contains("evidence") {
                terminal.print_with_effect("🔥 КРИТИЧНО: Доказательства уничтожены!", TerminalEffect::Error);
                state.reputation -= 20;
                terminal.print_with_effect("-20 репутации за уничтожение доказательств", TerminalEffect::Warning);
            }
            
            state.player.experience += 25;
            terminal.print_with_effect("+25 XP за форматирование", TerminalEffect::Success);
            
        } else {
            terminal.print_with_effect("❌ Форматирование не удалось", TerminalEffect::Error);
            terminal.print_with_effect("Устройство защищено от записи", TerminalEffect::Error);
        }
    }

    fn wipe_data(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: wipe <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        terminal.print_with_effect(&format!("Безопасное удаление файла: {}", filename), TerminalEffect::Warning);
        terminal.loading_animation("Многократная перезапись данных", Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.85) {
            terminal.print_with_effect("🗑️ ФАЙЛ БЕЗВОЗВРАТНО УНИЧТОЖЕН", TerminalEffect::Error);
            terminal.print_with_effect("Данные перезаписаны случайными битами", TerminalEffect::Warning);
            terminal.print_with_effect("Восстановление невозможно", TerminalEffect::Error);
            
            // Особые эффекты для важных файлов
            if filename.contains("nuclear") || filename.contains("evidence") || filename.contains("secret") {
                terminal.print_with_effect("💥 КРИТИЧЕСКИЕ ДАННЫЕ УНИЧТОЖЕНЫ!", TerminalEffect::Error);
                state.player.experience += 150;
                state.reputation += 30;
                terminal.print_with_effect("Это может повлиять на ход событий...", TerminalEffect::Warning);
            } else {
                state.player.experience += 50;
            }
            
            terminal.print_with_effect(&format!("+{} XP за уничтожение данных", 
                if filename.contains("nuclear") { 150 } else { 50 }), TerminalEffect::Success);
            
        } else {
            terminal.print_with_effect("❌ Не удалось уничтожить файл", TerminalEffect::Error);
            terminal.print_with_effect("Файл защищен или используется системой", TerminalEffect::Error);
        }
    }

    fn recover_data(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: recover <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        terminal.print_with_effect(&format!("Попытка восстановления файла: {}", filename), TerminalEffect::Matrix);
        terminal.loading_animation("Анализ неразмеченного пространства", Duration::from_secs(4));
        
        let mut rng = rand::thread_rng();
        let forensics_skill = *state.skills.get("Форензика").unwrap_or(&0);
        let success_chance = (forensics_skill as f64 / 100.0) * 0.7 + 0.2;
        
        if rng.gen_bool(success_chance) {
            terminal.print_with_effect("🔍 ДАННЫЕ НАЙДЕНЫ!", TerminalEffect::Success);
            
            let recovery_quality = rng.gen_range(1..=100);
            let quality_text = match recovery_quality {
                90..=100 => "Полное восстановление",
                70..=89 => "Частичное восстановление", 
                50..=69 => "Фрагментарное восстановление",
                _ => "Минимальное восстановление",
            };
            
            terminal.print_with_effect(&format!("Качество восстановления: {} ({}%)", quality_text, recovery_quality), 
                                     TerminalEffect::Normal);
            
            // Восстанавливаем содержимое в зависимости от качества
            if recovery_quality >= 70 {
                let recovered_content = vec![
                    "Deleted email: 'Operation will commence at midnight'",
                    "Erased document: Nuclear facility access codes",
                    "Removed log: Suspicious network activity detected",
                    "Deleted photo: Meeting with unknown individuals",
                    "Wiped database: Employee personal information",
                    "Recovered backup: Financial transaction records",
                ];
                
                let content = &recovered_content[rng.gen_range(0..recovered_content.len())];
                terminal.print_with_effect(&format!("📄 Восстановлено: {}", content), TerminalEffect::Success);
                
                // Добавляем в инвентарь
                state.inventory.push(format!("Recovered: {}", content));
            }
            
            state.player.experience += (recovery_quality as u32 / 2) + 30;
            terminal.print_with_effect(&format!("+{} XP за восстановление данных", (recovery_quality as u32 / 2) + 30), 
                                     TerminalEffect::Success);
            
            // Обновляем навык форензики
            let current_forensics = forensics_skill;
            state.skills.insert("Форензика".to_string(), current_forensics + 5);
            
        } else {
            terminal.print_with_effect("❌ Восстановление не удалось", TerminalEffect::Error);
            terminal.print_with_effect("Данные перезаписаны или повреждены", TerminalEffect::Error);
            state.player.stress += 5;
        }
    }

    fn analyze_file(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: analyze <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        terminal.print_with_effect(&format!("Анализ файла: {}", filename), TerminalEffect::Matrix);
        terminal.loading_animation("Сканирование метаданных и структуры", Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        terminal.print_with_effect("═══ РЕЗУЛЬТАТЫ АНАЛИЗА ═══", TerminalEffect::Matrix);
        
        // Генерируем технические данные файла
        let file_size = rng.gen_range(1024..1048576);
        let creation_date = "2024-01-15 14:23:42";
        let modification_date = "2024-01-15 16:45:18";
        let file_type = if filename.ends_with(".exe") { "PE32 executable" }
                       else if filename.ends_with(".dll") { "Dynamic Link Library" }
                       else if filename.ends_with(".txt") { "Plain text document" }
                       else if filename.ends_with(".enc") { "Encrypted data" }
                       else { "Binary data" };
        
        terminal.print_with_effect(&format!("Размер файла: {} байт", file_size), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("Тип файла: {}", file_type), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("Создан: {}", creation_date), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("Изменен: {}", modification_date), TerminalEffect::Normal);
        
        // MD5 и SHA256 хеши
        let md5_hash = format!("{:x}", rng.gen::<u128>());
        let sha256_hash = format!("{:x}{:x}", rng.gen::<u128>(), rng.gen::<u128>());
        terminal.print_with_effect(&format!("MD5: {}", &md5_hash[0..32]), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("SHA256: {}", &sha256_hash[0..64]), TerminalEffect::Normal);
        
        // Поиск подозрительных паттернов
        if rng.gen_bool(0.6) {
            let suspicious_findings = vec![
                "Обнаружены строки, связанные с сетевыми соединениями",
                "Найдены ссылки на системные файлы",
                "Содержит зашифрованные данные",
                "Обнаружены следы обфускации кода",
                "Найдены подозрительные API вызовы",
                "Содержит скрытые метаданные",
                "Обнаружены признаки вредоносного ПО",
                "Найдены пароли в открытом виде",
            ];
            
            let finding = &suspicious_findings[rng.gen_range(0..suspicious_findings.len())];
            terminal.print_with_effect(&format!("⚠️ Подозрительно: {}", finding), TerminalEffect::Warning);
        }
        
        state.player.experience += 45;
        terminal.print_with_effect("+45 XP за анализ файла", TerminalEffect::Success);
        
        // Обновляем навык форензики
        let current_forensics = *state.skills.get("Форензика").unwrap_or(&0);
        state.skills.insert("Форензика".to_string(), current_forensics + 3);
    }

    fn disassemble_file(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: disasm <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        
        if !filename.ends_with(".exe") && !filename.ends_with(".dll") && !filename.ends_with(".bin") {
            terminal.print_with_effect("Файл не является исполняемым", TerminalEffect::Error);
            return;
        }
        
        terminal.print_with_effect(&format!("Дизассемблирование: {}", filename), TerminalEffect::Matrix);
        terminal.loading_animation("Анализ машинного кода", Duration::from_secs(4));
        
        terminal.print_with_effect("═══ ASSEMBLY CODE ═══", TerminalEffect::Matrix);
        
        // Генерируем псевдо-ассемблерный код
        let asm_instructions = vec![
            "mov eax, 0x401000",
            "call GetProcAddress",
            "push ebp",
            "mov ebp, esp",
            "sub esp, 0x40",
            "lea eax, [ebp-0x20]",
            "push eax",
            "call CreateFileA",
            "cmp eax, INVALID_HANDLE_VALUE",
            "je error_handler",
            "mov [ebp-4], eax",
            "push 0",
            "push offset network_payload",
            "call send_data",
            "add esp, 8",
            "mov esp, ebp",
            "pop ebp",
            "ret",
        ];
        
        for (i, instruction) in asm_instructions.iter().enumerate() {
            terminal.print_with_effect(&format!("{:08X}: {}", 0x401000 + (i * 4), instruction), TerminalEffect::Normal);
            thread::sleep(Duration::from_millis(100));
        }
        
        // Поиск интересных функций
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.7) {
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🔍 НАЙДЕНЫ ИНТЕРЕСНЫЕ ФУНКЦИИ:", TerminalEffect::Warning);
            
            let functions = vec![
                "decrypt_nuclear_codes() - расшифровка ядерных кодов",
                "establish_backdoor() - создание бэкдора",
                "steal_credentials() - кража учетных данных", 
                "disable_firewall() - отключение файрвола",
                "keylogger_main() - основная функция кейлоггера",
                "network_scan() - сканирование сети",
                "privilege_escalation() - повышение привилегий",
                "data_exfiltration() - кража данных",
            ];
            
            let found_function = &functions[rng.gen_range(0..functions.len())];
            terminal.print_with_effect(&format!("• {}", found_function), TerminalEffect::Warning);
        }
        
        state.player.experience += 75;
        terminal.print_with_effect("+75 XP за дизассемблирование", TerminalEffect::Success);
        
        // Обновляем навык обратной инженерии
        let current_re = *state.skills.get("Обратная инженерия").unwrap_or(&0);
        state.skills.insert("Обратная инженерия".to_string(), current_re + 6);
    }

    fn debug_program(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: debug <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        terminal.print_with_effect(&format!("Запуск отладчика для: {}", filename), TerminalEffect::Matrix);
        terminal.loading_animation("Загрузка символов отладки", Duration::from_secs(3));
        
        terminal.print_with_effect("═══ DEBUGGER SESSION ═══", TerminalEffect::Matrix);
        terminal.print_with_effect("gdb> attach process", TerminalEffect::Normal);
        terminal.print_with_effect("Breakpoint 1 at 0x401234", TerminalEffect::Success);
        
        let mut rng = rand::thread_rng();
        
        // Симуляция отладочной сессии
        let debug_output = vec![
            "Thread 1 hit breakpoint at main+0x12",
            "EAX=0x00401000 EBX=0x7C800000 ECX=0x00000000",
            "Reading memory at 0x401000: 'nuclear_access_key'",
            "Variable 'password' = 'admin12345'",
            "Call stack: main -> authenticate -> check_permissions",
            "Heap corruption detected at 0x00680000",
            "Exception: Access violation at 0x401234",
            "Buffer overflow in function parse_input()",
        ];
        
        for output in &debug_output[0..rng.gen_range(3..debug_output.len())] {
            terminal.print_with_effect(output, TerminalEffect::Normal);
            thread::sleep(Duration::from_millis(500));
        }
        
        // Поиск уязвимостей через отладку
        if rng.gen_bool(0.6) {
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🐛 НАЙДЕНА УЯЗВИМОСТЬ!", TerminalEffect::Error);
            
            let vulnerabilities = vec![
                "Buffer overflow в функции input_handler",
                "Format string vulnerability в log_message",
                "Integer overflow в calculate_size",
                "Use-after-free в memory_manager",
                "Race condition в thread_handler",
                "SQL injection в database_query",
                "Path traversal в file_loader",
                "Command injection в system_call",
            ];
            
            let vuln = &vulnerabilities[rng.gen_range(0..vulnerabilities.len())];
            terminal.print_with_effect(&format!("Уязвимость: {}", vuln), TerminalEffect::Warning);
            
            // Добавляем информацию об уязвимости в инвентарь
            state.inventory.push(format!("Debug Info: {}", vuln));
        }
        
        state.player.experience += 90;
        terminal.print_with_effect("+90 XP за отладку программы", TerminalEffect::Success);
        
        // Обновляем навыки
        let current_re = *state.skills.get("Обратная инженерия").unwrap_or(&0);
        let current_prog = *state.skills.get("Программирование").unwrap_or(&0);
        state.skills.insert("Обратная инженерия".to_string(), current_re + 5);
        state.skills.insert("Программирование".to_string(), current_prog + 4);
    }

    fn compile_code(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: compile <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        
        if !filename.ends_with(".c") && !filename.ends_with(".cpp") && !filename.ends_with(".py") && !filename.ends_with(".rs") {
            terminal.print_with_effect("Файл не является исходным кодом", TerminalEffect::Error);
            return;
        }
        
        let compiler = if filename.ends_with(".c") { "gcc" }
                      else if filename.ends_with(".cpp") { "g++" }
                      else if filename.ends_with(".py") { "python" }
                      else if filename.ends_with(".rs") { "rustc" }
                      else { "unknown" };
        
        terminal.print_with_effect(&format!("Компиляция {} с помощью {}...", filename, compiler), TerminalEffect::Matrix);
        terminal.loading_animation("Анализ синтаксиса и генерация кода", Duration::from_secs(3));
        
        let mut rng = rand::thread_rng();
        let programming_skill = *state.skills.get("Программирование").unwrap_or(&0);
        let success_chance = (programming_skill as f64 / 100.0) * 0.8 + 0.4;
        
        if rng.gen_bool(success_chance) {
            terminal.print_with_effect("✅ КОМПИЛЯЦИЯ УСПЕШНА!", TerminalEffect::Success);
            
            let output_file = filename.replace(".c", ".exe")
                                    .replace(".cpp", ".exe")
                                    .replace(".py", ".pyc")
                                    .replace(".rs", ".exe");
            
            terminal.print_with_effect(&format!("Создан исполняемый файл: {}", output_file), TerminalEffect::Success);
            
            // Определяем тип программы
            let program_types = vec![
                "Exploit payload - эксплойт для повышения привилегий",
                "Network scanner - сканер сетевых уязвимостей", 
                "Password cracker - взломщик паролей",
                "Keylogger - перехватчик нажатий клавиш",
                "Backdoor client - клиент бэкдора",
                "Data exfiltrator - программа кражи данных",
                "System monitor - монитор системных процессов",
                "Encryption tool - инструмент шифрования",
            ];
            
            let program_type = &program_types[rng.gen_range(0..program_types.len())];
            terminal.print_with_effect(&format!("Тип программы: {}", program_type), TerminalEffect::Normal);
            
            // Добавляем скомпилированную программу в инвентарь
            state.inventory.push(format!("Compiled: {} - {}", output_file, program_type));
            
            state.player.experience += 65;
            terminal.print_with_effect("+65 XP за компиляцию", TerminalEffect::Success);
            
            // Обновляем навык программирования
            let current_prog = programming_skill;
            state.skills.insert("Программирование".to_string(), current_prog + 5);
            
        } else {
            terminal.print_with_effect("❌ ОШИБКА КОМПИЛЯЦИИ", TerminalEffect::Error);
            
            let errors = vec![
                "syntax error: expected ';' before '}' token",
                "undefined reference to 'main'",
                "error: incompatible types in assignment",
                "warning: implicit declaration of function",
                "error: 'variable' was not declared in this scope",
                "fatal error: header file not found",
                "error: multiple definition of 'function'",
                "linker error: unresolved external symbol",
            ];
            
            let error = &errors[rng.gen_range(0..errors.len())];
            terminal.print_with_effect(&format!("Ошибка: {}", error), TerminalEffect::Error);
            
            state.player.stress += 5;
        }
    }

    fn run_program(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        if args.is_empty() {
            terminal.print_with_effect("Использование: run <файл>", TerminalEffect::Error);
            return;
        }

        if state.player.current_system.is_none() {
            terminal.print_with_effect("Нет подключения к удаленной системе", TerminalEffect::Error);
            return;
        }

        let filename = args[0];
        
        terminal.print_with_effect(&format!("Запуск программы: {}", filename), TerminalEffect::Matrix);
        terminal.loading_animation("Инициализация процесса", Duration::from_secs(2));
        
        let mut rng = rand::thread_rng();
        
        // Различные сценарии выполнения программ
        if filename.contains("exploit") || filename.contains("payload") {
            if rng.gen_bool(0.6) {
                terminal.print_with_effect("🎯 ЭКСПЛОЙТ ВЫПОЛНЕН!", TerminalEffect::Success);
                terminal.print_with_effect("Получены повышенные привилегии", TerminalEffect::Success);
                state.player.experience += 100;
            } else {
                terminal.print_with_effect("❌ Эксплойт заблокирован антивирусом", TerminalEffect::Error);
            }
        } else if filename.contains("scanner") || filename.contains("nmap") {
            terminal.print_with_effect("🔍 Сканирование сети...", TerminalEffect::Matrix);
            terminal.print_with_effect("Найдено 5 активных хостов", TerminalEffect::Success);
            terminal.print_with_effect("Обнаружены открытые порты: 22, 80, 443, 3389", TerminalEffect::Normal);
            state.player.experience += 50;
        } else if filename.contains("keylog") {
            terminal.print_with_effect("⌨️ Кейлоггер активирован", TerminalEffect::Warning);
            terminal.print_with_effect("Захват нажатий клавиш начат", TerminalEffect::Warning);
            state.player.experience += 70;
        } else if filename.contains("backdoor") {
            terminal.print_with_effect("🚪 Бэкдор установлен", TerminalEffect::Success);
            terminal.print_with_effect("Удаленный доступ активирован", TerminalEffect::Success);
            state.player.experience += 120;
        } else {
            // Обычная программа
            terminal.print_with_effect("Program started successfully", TerminalEffect::Success);
            terminal.print_with_effect("Process ID: 2048", TerminalEffect::Normal);
            
            if rng.gen_bool(0.7) {
                terminal.print_with_effect("Program executed without errors", TerminalEffect::Success);
                state.player.experience += 30;
            } else {
                terminal.print_with_effect("Runtime error: segmentation fault", TerminalEffect::Error);
                terminal.print_with_effect("Core dumped", TerminalEffect::Error);
            }
        }
        
        terminal.print_with_effect(&format!("+{} XP за запуск программы", 
            if filename.contains("exploit") { 100 } else { 30 }), TerminalEffect::Success);
    }

    fn save_game(&self, args: &[&str], state: &GameState, terminal: &Terminal) {
        let slot = if args.len() > 0 { 
            args[0].parse::<u32>().unwrap_or(1) 
        } else { 
            1 
        };
        
        terminal.loading_animation("Сохранение игры", Duration::from_secs(2));
        
        match save_system::save_game(state, slot) {
            Ok(_) => {
                terminal.print_with_effect(&format!("💾 Игра сохранена в слот {}", slot), TerminalEffect::Success);
            },
            Err(e) => {
                terminal.print_with_effect(&format!("❌ Ошибка сохранения: {}", e), TerminalEffect::Error);
            }
        }
    }

    fn load_game(&self, args: &[&str], state: &mut GameState, terminal: &Terminal) {
        let slot = if args.len() > 0 { 
            args[0].parse::<u32>().unwrap_or(1) 
        } else { 
            1 
        };
        
        terminal.loading_animation("Загрузка игры", Duration::from_secs(2));
        
        match save_system::load_game(slot) {
            Ok(loaded_state) => {
                *state = loaded_state;
                terminal.print_with_effect(&format!("📁 Игра загружена из слота {}", slot), TerminalEffect::Success);
            },
            Err(e) => {
                terminal.print_with_effect(&format!("❌ Ошибка загрузки: {}", e), TerminalEffect::Error);
            }
        }
    }

    fn switch_to_sandbox(&self, state: &mut GameState, terminal: &Terminal) {
        state.game_mode = GameMode::Sandbox;
        terminal.print_with_effect("Переключение в режим песочницы", TerminalEffect::Success);
        terminal.print_with_effect("Теперь вы можете свободно исследовать и взламывать системы", TerminalEffect::Normal);
    }

    fn switch_to_story(&self, state: &mut GameState, terminal: &Terminal) {
        state.game_mode = GameMode::Story;
        terminal.print_with_effect("Переключение в сюжетный режим", TerminalEffect::Success);
        terminal.print_with_effect("Продолжайте основной сюжет", TerminalEffect::Normal);
    }
}

// ============================================================================
// ГЕНЕРАЦИЯ КОНТЕНТА
// ============================================================================

fn generate_random_files() -> HashMap<String, File> {
    let mut files = HashMap::new();
    let mut rng = rand::thread_rng();
    
    let file_templates = vec![
        ("system.log", "System boot log\n[OK] Loading kernel modules\n[OK] Starting services\n[WARN] SSH failed login attempts detected", false),
        ("passwords.txt", "admin:admin123\nuser:password\nroot:toor\ntest:test123", true),
        ("email.txt", "From: boss@company.com\nTo: employee@company.com\nSubject: Confidential Project\n\nThe nuclear facility security codes are...", false),
        ("database.sql", "CREATE TABLE users (id INT, username VARCHAR(50), password VARCHAR(100));\nINSERT INTO users VALUES (1, 'admin', 'hashed_password');", false),
        ("source.cpp", "#include <iostream>\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}", false),
        ("config.ini", "[Database]\nhost=localhost\nport=3306\nusername=admin\npassword=secret123", true),
        ("nuclear_codes.enc", "U2FsdGVkX1+vupppZksvRf5pq5g5XjFRIipRkwB0K1Y96Qsv2Lm+31cmzaAILwyt", true),
        ("budget.xls", "Q1: $2,500,000\nQ2: $3,100,000\nQ3: $2,800,000\nQ4: $4,200,000", false),
    ];
    
    for (name, content, encrypted) in file_templates {
        files.insert(name.to_string(), File {
            name: name.to_string(),
            content: content.to_string(),
            permissions: if encrypted { "rw-------" } else { "rw-r--r--" }.to_string(),
            size: rng.gen_range(1..1000),
            encrypted,
            password: if encrypted { Some("password123".to_string()) } else { None },
        });
    }
    
    files
}

fn generate_random_services() -> Vec<Service> {
    let mut services = Vec::new();
    let mut rng = rand::thread_rng();
    
    let service_templates = vec![
        ("SSH", 22, "OpenSSH 7.4", true),
        ("HTTP", 80, "Apache 2.4.6", true),
        ("HTTPS", 443, "Apache 2.4.6", true),
        ("FTP", 21, "vsftpd 3.0.2", false),
        ("Telnet", 23, "GNU inetutils", false),
        ("SMTP", 25, "Postfix 3.3.1", true),
        ("DNS", 53, "BIND 9.11.4", true),
        ("MySQL", 3306, "MySQL 5.7.28", true),
        ("PostgreSQL", 5432, "PostgreSQL 11.5", true),
        ("RDP", 3389, "Microsoft RDP", false),
    ];
    
    for (name, port, version, running) in service_templates {
        if rng.gen_bool(0.7) { // 70% вероятность что сервис запущен
            services.push(Service {
                name: name.to_string(),
                port,
                version: version.to_string(),
                running,
                vulnerable: rng.gen_bool(0.3), // 30% вероятность уязвимости
            });
        }
    }
    
    services
}

fn generate_random_vulnerabilities() -> Vec<Vulnerability> {
    let mut vulnerabilities = Vec::new();
    let mut rng = rand::thread_rng();
    
    let vuln_templates = vec![
        ("CVE-2019-14287", "Sudo privilege escalation", 8, "sudo -u#-1 /bin/bash"),
        ("CVE-2017-0144", "EternalBlue SMB vulnerability", 9, "ms17-010_exploit.py"),
        ("CVE-2014-0160", "Heartbleed OpenSSL vulnerability", 7, "heartbleed_exploit.py"),
        ("CVE-2012-2982", "SQL Injection in login form", 6, "sqlmap -u target --dbs"),
        ("CVE-2018-1111", "DHCP client RCE", 8, "dhcp_exploit.sh"),
        ("Buffer Overflow", "Stack-based buffer overflow", 7, "python overflow.py"),
        ("Weak Passwords", "Default or weak passwords", 5, "hydra -l admin -P passwords.txt ssh://target"),
        ("Directory Traversal", "Path traversal vulnerability", 6, "../../../etc/passwd"),
    ];
    
    for (name, desc, severity, exploit) in vuln_templates {
        if rng.gen_bool(0.4) { // 40% вероятность наличия уязвимости
            vulnerabilities.push(Vulnerability {
                name: name.to_string(),
                description: desc.to_string(),
                severity,
                exploit_code: exploit.to_string(),
            });
        }
    }
    
    vulnerabilities
}

// ============================================================================
// ОСНОВНАЯ ИГРА
// ============================================================================

pub struct Game {
    pub state: GameState,
    pub terminal: Terminal,
    pub command_processor: CommandProcessor,
    pub start_time: Instant,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            state: GameState {
                player: Player {
                    name: String::new(),
                    handle: String::new(),
                    level: 1,
                    experience: 0,
                    health: 100,
                    max_health: 100,
                    stress: 0,
                    current_system: None,
                },
                current_quest: None,
                completed_quests: Vec::new(),
                current_location: "Local Terminal".to_string(),
                story_progress: 0,
                game_mode: GameMode::Story,
                time_played: Duration::new(0, 0),
                networks: HashMap::new(),
                contacts: HashMap::new(),
                inventory: Vec::new(),
                skills: HashMap::new(),
                reputation: 0,
                money: 1000,
            },
            terminal: Terminal::new(),
            command_processor: CommandProcessor::new(),
            start_time: Instant::now(),
        };
        
        game.initialize_skills();
        game.initialize_contacts();
        game
    }

    fn initialize_skills(&mut self) {
        let skills = vec![
            ("Хакинг", 10),
            ("Социальная инженерия", 5),
            ("Криптография", 8),
            ("Сетевая безопасность", 12),
            ("Программирование", 15),
            ("Анонимность", 7),
            ("Форензика", 3),
            ("Обратная инженерия", 6),
        ];
        
        for (skill, level) in skills {
            self.state.skills.insert(skill.to_string(), level);
        }
    }

    fn initialize_contacts(&mut self) {
        let contacts = vec![
            ("Тень", Contact {
                name: "Алексей 'Тень' Морозов".to_string(),
                handle: "shadow_hunter".to_string(),
                relationship: 25,
                faction: "Кибер-повстанцы".to_string(),
                services: vec!["Информация".to_string(), "Эксплойты".to_string()],
                last_contact: "Вчера".to_string(),
            }),
            ("Призрак", Contact {
                name: "Анна 'Призрак' Волкова".to_string(),
                handle: "ghost_in_shell".to_string(),
                relationship: 15,
                faction: "Хакеры-одиночки".to_string(),
                services: vec!["Взлом".to_string(), "Анонимность".to_string()],
                last_contact: "3 дня назад".to_string(),
            }),
        ];
        
        for (handle, contact) in contacts {
            self.state.contacts.insert(handle.to_string(), contact);
        }
    }

    pub fn run(&mut self) {
        self.show_intro();
        self.character_creation();
        self.show_initial_briefing();
        
        // Основной игровой цикл
        loop {
            self.state.time_played = self.start_time.elapsed();
            self.terminal.display_hud(&self.state);
            
            let input = self.terminal.prompt("root@cyberhack:~#");
            
            if !self.command_processor.execute_command(&input, &mut self.state, &self.terminal) {
                break;
            }
            
            self.update_game_state();
        }
        
        self.show_outro();
    }

    fn show_intro(&self) {
        self.terminal.clear();
        
        let ascii_art = r#"
   ▄████████ ▄██   ▄   ▀█████████▄     ▄████████    ▄████████    ▄█    █▄       ▄████████  ▄████████    ▄█   ▄█▄ 
  ███    ███ ███   ██▄   ███    ███   ███    ███   ███    ███   ███    ███     ███    ███ ███    ███   ███ ▄███▀ 
  ███    █▀  ███▄▄▄███   ███    ███   ███    █▀    ███    ███   ███    ███     ███    ███ ███    █▀    ███▐██▀   
  ███         ▀▀▀▀▀▀███   ▄███▄▄▄██▀  ▄███▄▄▄      ▄███▄▄▄▄██▀  ▄███▄▄▄▄███▄▄   ███    ███ ███         ▄█████▀    
▀███████████   ▄██   ███  ▀▀███▀▀▀██▄ ▀▀███▀▀▀     ▀▀███▀▀▀▀▀   ▀▀███▀▀▀▀███▀  ▀███████████ ███        ▀▀█████▄    
         ███   ███   ███    ███    ██▄   ███    █▄  ▀███████████   ███    ███     ███    ███ ███    █▄    ███▐██▄   
   ▄█    ███   ███   ███    ███    ███   ███    ███   ███    ███   ███    ███     ███    ███ ███    ███   ███ ▀███▄ 
 ▄████████▀     ▀█████▀   ▄█████████▀    ██████████   ███    ███   ███    █▀      ███    █▀  ████████▀    ███   ▀█▀ 
                                                     ███    ███                                           ▀         
"#;
        
        self.terminal.display_ascii_art(ascii_art);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        self.terminal.print_with_effect("═══════════════════════════════════════════════════════════════════════════", TerminalEffect::Matrix);
        self.terminal.print_with_effect("                        Добро пожаловать в CYBERHACK", TerminalEffect::Matrix);
        self.terminal.print_with_effect("                   Консольная хакерская игра v2.1.3", TerminalEffect::Matrix);
        self.terminal.print_with_effect("═══════════════════════════════════════════════════════════════════════════", TerminalEffect::Matrix);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        
        let intro_text = "В недалеком будущем мир изменился. Корпорации правят всем, а обычные люди\n\
                         стали бессильными пешками в их играх. Но есть те, кто сопротивляется...\n\
                         \n\
                         Вы - хакер, владеющий искусством проникновения в цифровые системы.\n\
                         Ваша миссия: раскрыть заговор мегакорпорации NEXUS, которая планирует\n\
                         запустить проект 'ЦИФРОВОЙ АПОКАЛИПСИС' - систему тотального контроля\n\
                         над человечеством через ядерную угрозу.\n\
                         \n\
                         У вас есть 72 часа до активации системы. Выбор за вами:\n\
                         - Предотвратить катастрофу и спасти человечество\n\
                         - Или погибнуть в огне ядерного взрыва...\n\
                         \n\
                         Добро пожаловать в мир киберпанка. Добро пожаловать в CYBERHACK.";
        
        self.terminal.print_with_effect(intro_text, TerminalEffect::TypeWriter);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        self.terminal.prompt("Нажмите Enter для продолжения...");
    }

    fn character_creation(&mut self) {
        self.terminal.clear();
        self.terminal.print_with_effect("═══ СОЗДАНИЕ ПЕРСОНАЖА ═══", TerminalEffect::Matrix);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        
        // Ввод имени
        loop {
            let name = self.terminal.prompt("Введите ваше настоящее имя:");
            if !name.trim().is_empty() {
                self.state.player.name = name.trim().to_string();
                break;
            }
            self.terminal.print_with_effect("Имя не может быть пустым!", TerminalEffect::Error);
        }
        
        // Ввод псевдонима
        loop {
            let handle = self.terminal.prompt("Введите ваш хакерский псевдоним:");
            if !handle.trim().is_empty() {
                self.state.player.handle = handle.trim().to_string();
                break;
            }
            self.terminal.print_with_effect("Псевдоним не может быть пустым!", TerminalEffect::Error);
        }
        
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        self.terminal.print_with_effect(&format!("Отлично, {}! Добро пожаловать в игру.", self.state.player.name), TerminalEffect::Success);
        self.terminal.print_with_effect(&format!("Ваш псевдоним: {}", self.state.player.handle), TerminalEffect::Success);
        
        self.terminal.prompt("Нажмите Enter для продолжения...");
    }

    fn show_initial_briefing(&mut self) {
        self.terminal.clear();
        self.terminal.print_with_effect("═══ НАЧАЛЬНЫЙ БРИФИНГ ═══", TerminalEffect::Matrix);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        
        let briefing = format!(
            "Агент {}, вас приветствует подпольная организация 'Кибер-Свобода'.\n\
            \n\
            Мы перехватили секретные сообщения корпорации NEXUS. Они планируют\n\
            активировать систему 'ЦИФРОВОЙ АПОКАЛИПСИС' через 72 часа.\n\
            \n\
            Эта система позволит им:\n\
            - Взять под контроль все ядерные объекты мира\n\
            - Шантажировать правительства угрозой ядерного удара\n\
            - Установить диктатуру корпорации над всем человечеством\n\
            \n\
            ВАША МИССИЯ:\n\
            1. Проникнуть в сеть NEXUS Corporation\n\
            2. Найти коды доступа к системе управления\n\
            3. Отключить систему до её активации\n\
            \n\
            В вашем распоряжении:\n\
            - Терминал с набором хакерских инструментов\n\
            - Контакты в подполье\n\
            - Начальный капитал: 1000₿\n\
            \n\
            Помните: время играет против вас. Каждая минута на счету!\n\
            \n\
            Используйте команду 'help' для получения списка доступных команд.\n\
            Удачи, {}. Судьба мира в ваших руках.",
            self.state.player.handle, self.state.player.handle
        );
        
        self.terminal.print_with_effect(&briefing, TerminalEffect::TypeWriter);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        
        // Первый квест
        let first_quest = Quest {
            id: "mission_start".to_string(),
            title: "Начало миссии".to_string(),
            description: "Подготовиться к проникновению в сеть NEXUS".to_string(),
            objectives: vec![
                Objective {
                    description: "Просканировать локальную сеть".to_string(),
                    completed: false,
                    target: "192.168.1.0/24".to_string(),
                    action: "scan".to_string(),
                },
                Objective {
                    description: "Подключиться к любой системе".to_string(),
                    completed: false,
                    target: "any".to_string(),
                    action: "connect".to_string(),
                },
                Objective {
                    description: "Изучить файловую систему".to_string(),
                    completed: false,
                    target: "any".to_string(),
                    action: "ls".to_string(),
                },
            ],
            rewards: vec!["500 XP".to_string(), "Новые контакты".to_string()],
            time_limit: Some(Duration::from_secs(3600)), // 1 час
            difficulty: 1,
        };
        
        self.state.current_quest = Some(first_quest);
        
        self.terminal.prompt("Нажмите Enter для начала миссии...");
    }

    fn update_game_state(&mut self) {
        // Обновление уровня игрока
        let required_xp = self.state.player.level * 100;
        if self.state.player.experience >= required_xp {
            self.state.player.level += 1;
            self.state.player.experience -= required_xp;
            self.state.player.max_health += 10;
            self.state.player.health = self.state.player.max_health;
            
            self.terminal.print_with_effect(&format!("🎉 УРОВЕНЬ ПОВЫШЕН! Теперь вы {}го уровня!", self.state.player.level), TerminalEffect::Success);
            self.terminal.print_with_effect(&format!("Максимальное здоровье увеличено до {}", self.state.player.max_health), TerminalEffect::Success);
        }
        
        // Проверка завершения квестов
        if let Some(quest) = &mut self.state.current_quest {
            let all_completed = quest.objectives.iter().all(|obj| obj.completed);
            if all_completed {
                self.terminal.print_with_effect("🎉 КВЕСТ ЗАВЕРШЕН!", TerminalEffect::Success);
                self.terminal.print_with_effect(&format!("Квест '{}' выполнен!", quest.title), TerminalEffect::Success);
                
                for reward in &quest.rewards {
                    self.terminal.print_with_effect(&format!("Награда: {}", reward), TerminalEffect::Success);
                }
                
                self.state.completed_quests.push(quest.id.clone());
                self.state.current_quest = None;
                self.state.story_progress += 1;
                
                // Выдача следующего квеста
                self.assign_next_quest();
            }
        }
        
        // Управление стрессом
        if self.state.player.stress > 80 {
            self.terminal.print_with_effect("⚠ ВЫСОКИЙ УРОВЕНЬ СТРЕССА! Требуется отдых.", TerminalEffect::Warning);
        }
        
        // Проверка здоровья
        if self.state.player.health <= 20 {
            self.terminal.print_with_effect("⚠ КРИТИЧЕСКОЕ СОСТОЯНИЕ! Здоровье на исходе.", TerminalEffect::Error);
        }
    }

    fn assign_next_quest(&mut self) {
        match self.state.story_progress {
            1 => {
                let quest = Quest {
                    id: "find_nexus_network".to_string(),
                    title: "Поиск сети NEXUS".to_string(),
                    description: "Найти и проникнуть в корпоративную сеть NEXUS Corporation".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Найти IP-адрес сети NEXUS".to_string(),
                            completed: false,
                            target: "nexus.corp".to_string(),
                            action: "scan".to_string(),
                        },
                        Objective {
                            description: "Провести nmap сканирование".to_string(),
                            completed: false,
                            target: "nexus.corp".to_string(),
                            action: "nmap".to_string(),
                        },
                    ],
                    rewards: vec!["1000 XP".to_string(), "Доступ к корпоративной сети".to_string()],
                    time_limit: Some(Duration::from_secs(7200)), // 2 часа
                    difficulty: 3,
                };
                self.state.current_quest = Some(quest);
            },
            2 => {
                let quest = Quest {
                    id: "penetrate_nexus".to_string(),
                    title: "Проникновение в NEXUS".to_string(),
                    description: "Взломать защиту и получить доступ к внутренним системам NEXUS".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Эксплуатировать найденную уязвимость".to_string(),
                            completed: false,
                            target: "nexus_server".to_string(),
                            action: "exploit".to_string(),
                        },
                        Objective {
                            description: "Получить права администратора".to_string(),
                            completed: false,
                            target: "nexus_server".to_string(),
                            action: "backdoor".to_string(),
                        },
                    ],
                    rewards: vec!["1500 XP".to_string(), "Доступ к секретным файлам".to_string()],
                    time_limit: Some(Duration::from_secs(5400)), // 1.5 часа
                    difficulty: 5,
                };
                self.state.current_quest = Some(quest);
            },
            _ => {
                // Дальнейшие квесты будут добавлены
                self.terminal.print_with_effect("Сюжет продолжается... (В разработке)", TerminalEffect::Warning);
            }
        }
    }

    fn show_outro(&self) {
        self.terminal.clear();
        self.terminal.print_with_effect("═══════════════════════════════════════════════════════════════════════════", TerminalEffect::Matrix);
        self.terminal.print_with_effect("                                ИГРА ЗАВЕРШЕНА", TerminalEffect::Matrix);
        self.terminal.print_with_effect("═══════════════════════════════════════════════════════════════════════════", TerminalEffect::Matrix);
        self.terminal.print_with_effect("", TerminalEffect::Normal);
        
        let outro_text = format!(
            "Спасибо за игру в CYBERHACK!\n\
            \n\
            Статистика вашей игры:\n\
            • Имя: {}\n\
            • Псевдоним: {}\n\
            • Уровень: {}\n\
            • Опыт: {}\n\
            • Время игры: {:02}:{:02}:{:02}\n\
            • Завершенных квестов: {}\n\
            • Репутация: {}\n\
            \n\
            До свидания, хакер. Мир киберпанка ждет вашего возвращения...",
            self.state.player.name,
            self.state.player.handle,
            self.state.player.level,
            self.state.player.experience,
            self.state.time_played.as_secs() / 3600,
            (self.state.time_played.as_secs() % 3600) / 60,
            self.state.time_played.as_secs() % 60,
            self.state.completed_quests.len(),
            self.state.reputation
        );
        
        self.terminal.print_with_effect(&outro_text, TerminalEffect::TypeWriter);
    }
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

fn main() {
    let mut game = Game::new();
    game.run();
}

// ============================================================================
// ДОПОЛНИТЕЛЬНЫЕ МОДУЛИ (добавятся далее)
// ============================================================================

// Модуль для системы сохранений
mod save_system {
    use super::*;
    use std::fs;
    use std::path::Path;
    
    pub fn save_game(state: &GameState, slot: u32) -> Result<(), Box<dyn std::error::Error>> {
        let save_dir = "saves";
        if !Path::new(save_dir).exists() {
            fs::create_dir(save_dir)?;
        }
        
        let save_file = format!("{}/save_{}.json", save_dir, slot);
        let serialized = serde_json::to_string_pretty(state)?;
        fs::write(save_file, serialized)?;
        
        Ok(())
    }
    
    pub fn load_game(slot: u32) -> Result<GameState, Box<dyn std::error::Error>> {
        let save_file = format!("saves/save_{}.json", slot);
        let content = fs::read_to_string(save_file)?;
        let state: GameState = serde_json::from_str(&content)?;
        
        Ok(state)
    }
    
    pub fn list_saves() -> Vec<u32> {
        let mut saves = Vec::new();
        if let Ok(entries) = fs::read_dir("saves") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("save_") && name.ends_with(".json") {
                        if let Ok(slot) = name.replace("save_", "").replace(".json", "").parse::<u32>() {
                            saves.push(slot);
                        }
                    }
                }
            }
        }
        saves.sort();
        saves
    }
}

// Модуль для AI противников
mod ai_system {
    use super::*;
    
    pub struct AIOpponent {
        pub name: String,
        pub skill_level: u32,
        pub aggression: u32,
        pub patience: u32,
        pub current_action: Option<String>,
    }
    
    impl AIOpponent {
        pub fn new(name: String, skill_level: u32) -> Self {
            AIOpponent {
                name,
                skill_level,
                aggression: rand::thread_rng().gen_range(1..10),
                patience: rand::thread_rng().gen_range(1..10),
                current_action: None,
            }
        }
        
        pub fn react_to_intrusion(&mut self, player_action: &str) -> String {
            match player_action {
                "scan" => {
                    if self.aggression > 7 {
                        "Немедленная блокировка IP".to_string()
                    } else {
                        "Мониторинг активности".to_string()
                    }
                },
                "connect" => {
                    if self.skill_level > 5 {
                        "Активация ловушки".to_string()
                    } else {
                        "Логирование попытки входа".to_string()
                    }
                },
                _ => "Стандартная защитная реакция".to_string(),
            }
        }
    }
}

// ============================================================================
// РАСШИРЕННАЯ СИСТЕМА КВЕСТОВ И СЮЖЕТА
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryManager {
    pub current_chapter: u32,
    pub story_flags: HashMap<String, bool>,
    pub character_relationships: HashMap<String, i32>,
    pub faction_standing: HashMap<String, i32>,
    pub time_pressure: Duration,
    pub world_state: WorldState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub nexus_alert_level: u32,
    pub nuclear_facilities_controlled: u32,
    pub total_nuclear_facilities: u32,
    pub government_awareness: u32,
    pub media_attention: u32,
    pub cyber_freedom_support: u32,
    pub global_panic_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionObjective {
    pub id: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub target_data: String,
    pub completion_condition: String,
    pub reward_xp: u32,
    pub reward_money: u32,
    pub reward_items: Vec<String>,
    pub failure_penalty: String,
    pub time_limit: Option<Duration>,
    pub difficulty_rating: u32,
    pub prerequisites: Vec<String>,
    pub completion_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    Infiltrate,
    Steal,
    Destroy,
    Rescue,
    Sabotage,
    Information,
    Eliminate,
    Protect,
    Survive,
    Negotiate,
}

impl StoryManager {
    pub fn new() -> Self {
        let mut story_flags = HashMap::new();
        story_flags.insert("game_started".to_string(), true);
        story_flags.insert("nexus_aware".to_string(), false);
        story_flags.insert("nuclear_codes_found".to_string(), false);
        story_flags.insert("cyber_freedom_contacted".to_string(), false);
        
        let mut faction_standing = HashMap::new();
        faction_standing.insert("Cyber Freedom".to_string(), 50);
        faction_standing.insert("NEXUS Corporation".to_string(), -100);
        faction_standing.insert("Government".to_string(), 0);
        faction_standing.insert("Underground Hackers".to_string(), 25);
        faction_standing.insert("Corporate Security".to_string(), -50);
        
        StoryManager {
            current_chapter: 1,
            story_flags,
            character_relationships: HashMap::new(),
            faction_standing,
            time_pressure: Duration::from_secs(259200), // 72 hours
            world_state: WorldState {
                nexus_alert_level: 1,
                nuclear_facilities_controlled: 0,
                total_nuclear_facilities: 15,
                government_awareness: 10,
                media_attention: 0,
                cyber_freedom_support: 30,
                global_panic_level: 0,
            },
        }
    }
    
    pub fn advance_story(&mut self, event: &str, terminal: &Terminal) {
        match event {
            "nexus_network_infiltrated" => {
                self.story_flags.insert("nexus_infiltrated".to_string(), true);
                self.world_state.nexus_alert_level += 2;
                terminal.print_with_effect("🚨 NEXUS обнаружил вторжение! Уровень тревоги повышен", TerminalEffect::Warning);
            },
            "nuclear_codes_accessed" => {
                self.story_flags.insert("nuclear_codes_found".to_string(), true);
                self.world_state.nuclear_facilities_controlled += 5;
                terminal.print_with_effect("💥 Получен доступ к ядерным кодам!", TerminalEffect::Error);
            },
            "media_leak" => {
                self.world_state.media_attention += 30;
                self.world_state.government_awareness += 20;
                terminal.print_with_effect("📺 СМИ сообщают о кибератаке на NEXUS", TerminalEffect::Warning);
            },
            "cyber_freedom_betrayal" => {
                self.faction_standing.insert("Cyber Freedom".to_string(), -50);
                terminal.print_with_effect("💔 Кибер-Свобода предала вас!", TerminalEffect::Error);
            },
            _ => {}
        }
        
        // Обновляем давление времени
        if self.time_pressure.as_secs() > 3600 {
            self.time_pressure = Duration::from_secs(self.time_pressure.as_secs() - 1800);
        }
    }
    
    pub fn get_current_objectives(&self) -> Vec<MissionObjective> {
        match self.current_chapter {
            1 => self.chapter_1_objectives(),
            2 => self.chapter_2_objectives(),
            3 => self.chapter_3_objectives(),
            4 => self.chapter_4_objectives(),
            5 => self.chapter_5_objectives(),
            _ => self.final_chapter_objectives(),
        }
    }
    
    fn chapter_1_objectives(&self) -> Vec<MissionObjective> {
        vec![
            MissionObjective {
                id: "tutorial_scan".to_string(),
                description: "Просканировать локальную сеть для поиска целей".to_string(),
                objective_type: ObjectiveType::Information,
                target_data: "192.168.1.0/24".to_string(),
                completion_condition: "scan_completed".to_string(),
                reward_xp: 100,
                reward_money: 500,
                reward_items: vec!["Basic Network Map".to_string()],
                failure_penalty: "none".to_string(),
                time_limit: Some(Duration::from_secs(1800)),
                difficulty_rating: 1,
                prerequisites: vec![],
                completion_flags: vec!["first_scan_done".to_string()],
            },
            MissionObjective {
                id: "first_infiltration".to_string(),
                description: "Проникнуть в любую систему для получения опыта".to_string(),
                objective_type: ObjectiveType::Infiltrate,
                target_data: "any_system".to_string(),
                completion_condition: "system_compromised".to_string(),
                reward_xp: 200,
                reward_money: 1000,
                reward_items: vec!["Basic Hacking Tools".to_string()],
                failure_penalty: "stress +20".to_string(),
                time_limit: Some(Duration::from_secs(3600)),
                difficulty_rating: 2,
                prerequisites: vec!["first_scan_done".to_string()],
                completion_flags: vec!["first_hack_done".to_string()],
            },
        ]
    }
    
    fn chapter_2_objectives(&self) -> Vec<MissionObjective> {
        vec![
            MissionObjective {
                id: "nexus_recon".to_string(),
                description: "Найти сеть NEXUS Corporation и провести разведку".to_string(),
                objective_type: ObjectiveType::Information,
                target_data: "nexus.corp".to_string(),
                completion_condition: "nexus_located".to_string(),
                reward_xp: 500,
                reward_money: 2500,
                reward_items: vec!["NEXUS Network Map".to_string(), "Corporate Directory".to_string()],
                failure_penalty: "nexus_alert +1".to_string(),
                time_limit: Some(Duration::from_secs(7200)),
                difficulty_rating: 4,
                prerequisites: vec!["first_hack_done".to_string()],
                completion_flags: vec!["nexus_discovered".to_string()],
            },
            MissionObjective {
                id: "employee_social_eng".to_string(),
                description: "Использовать социальную инженерию против сотрудников NEXUS".to_string(),
                objective_type: ObjectiveType::Information,
                target_data: "nexus_employees".to_string(),
                completion_condition: "social_info_gathered".to_string(),
                reward_xp: 300,
                reward_money: 1500,
                reward_items: vec!["Employee Passwords".to_string(), "Internal Contacts".to_string()],
                failure_penalty: "reputation -10".to_string(),
                time_limit: Some(Duration::from_secs(5400)),
                difficulty_rating: 3,
                prerequisites: vec!["nexus_discovered".to_string()],
                completion_flags: vec!["social_engineering_success".to_string()],
            },
        ]
    }
    
    fn chapter_3_objectives(&self) -> Vec<MissionObjective> {
        vec![
            MissionObjective {
                id: "nexus_infiltration".to_string(),
                description: "Проникнуть в корпоративную сеть NEXUS".to_string(),
                objective_type: ObjectiveType::Infiltrate,
                target_data: "nexus_internal_network".to_string(),
                completion_condition: "nexus_network_breached".to_string(),
                reward_xp: 800,
                reward_money: 5000,
                reward_items: vec!["NEXUS Access Tokens".to_string(), "Internal Documentation".to_string()],
                failure_penalty: "nexus_alert +3, stress +40".to_string(),
                time_limit: Some(Duration::from_secs(10800)),
                difficulty_rating: 6,
                prerequisites: vec!["social_engineering_success".to_string()],
                completion_flags: vec!["nexus_network_infiltrated".to_string()],
            },
            MissionObjective {
                id: "find_nuclear_project".to_string(),
                description: "Найти информацию о проекте 'Цифровой Апокалипсис'".to_string(),
                objective_type: ObjectiveType::Information,
                target_data: "digital_apocalypse_files".to_string(),
                completion_condition: "project_files_found".to_string(),
                reward_xp: 1000,
                reward_money: 7500,
                reward_items: vec!["Project Blueprints".to_string(), "Nuclear Facility List".to_string()],
                failure_penalty: "time_pressure -2h".to_string(),
                time_limit: Some(Duration::from_secs(14400)),
                difficulty_rating: 7,
                prerequisites: vec!["nexus_network_infiltrated".to_string()],
                completion_flags: vec!["project_discovered".to_string()],
            },
        ]
    }
    
    fn chapter_4_objectives(&self) -> Vec<MissionObjective> {
        vec![
            MissionObjective {
                id: "nuclear_facility_hack".to_string(),
                description: "Взломать системы управления ядерными объектами".to_string(),
                objective_type: ObjectiveType::Sabotage,
                target_data: "nuclear_control_systems".to_string(),
                completion_condition: "nuclear_systems_compromised".to_string(),
                reward_xp: 1500,
                reward_money: 10000,
                reward_items: vec!["Nuclear Access Codes".to_string(), "Facility Blueprints".to_string()],
                failure_penalty: "nuclear_meltdown_risk".to_string(),
                time_limit: Some(Duration::from_secs(18000)),
                difficulty_rating: 9,
                prerequisites: vec!["project_discovered".to_string()],
                completion_flags: vec!["nuclear_facilities_hacked".to_string()],
            },
            MissionObjective {
                id: "disable_doomsday_timer".to_string(),
                description: "Отключить таймер системы 'Цифровой Апокалипсис'".to_string(),
                objective_type: ObjectiveType::Sabotage,
                target_data: "doomsday_timer".to_string(),
                completion_condition: "timer_disabled".to_string(),
                reward_xp: 2000,
                reward_money: 15000,
                reward_items: vec!["Master Override Codes".to_string()],
                failure_penalty: "game_over_bad_ending".to_string(),
                time_limit: Some(Duration::from_secs(3600)),
                difficulty_rating: 10,
                prerequisites: vec!["nuclear_facilities_hacked".to_string()],
                completion_flags: vec!["timer_disabled".to_string()],
            },
        ]
    }
    
    fn chapter_5_objectives(&self) -> Vec<MissionObjective> {
        vec![
            MissionObjective {
                id: "expose_nexus".to_string(),
                description: "Обнародовать доказательства планов NEXUS".to_string(),
                objective_type: ObjectiveType::Information,
                target_data: "evidence_package".to_string(),
                completion_condition: "evidence_published".to_string(),
                reward_xp: 2500,
                reward_money: 20000,
                reward_items: vec!["Media Contacts".to_string(), "Legal Protection".to_string()],
                failure_penalty: "nexus_coverup".to_string(),
                time_limit: Some(Duration::from_secs(7200)),
                difficulty_rating: 8,
                prerequisites: vec!["timer_disabled".to_string()],
                completion_flags: vec!["nexus_exposed".to_string()],
            },
            MissionObjective {
                id: "escape_nexus_hunters".to_string(),
                description: "Избежать охотников NEXUS и скрыться".to_string(),
                objective_type: ObjectiveType::Survive,
                target_data: "self".to_string(),
                completion_condition: "escaped_safely".to_string(),
                reward_xp: 3000,
                reward_money: 25000,
                reward_items: vec!["New Identity".to_string(), "Safe House".to_string()],
                failure_penalty: "captured_bad_ending".to_string(),
                time_limit: Some(Duration::from_secs(10800)),
                difficulty_rating: 9,
                prerequisites: vec!["nexus_exposed".to_string()],
                completion_flags: vec!["escaped_nexus".to_string()],
            },
        ]
    }
    
    fn final_chapter_objectives(&self) -> Vec<MissionObjective> {
        vec![
            MissionObjective {
                id: "final_choice".to_string(),
                description: "Принять финальное решение о судьбе мира".to_string(),
                objective_type: ObjectiveType::Negotiate,
                target_data: "world_fate".to_string(),
                completion_condition: "choice_made".to_string(),
                reward_xp: 5000,
                reward_money: 50000,
                reward_items: vec!["Legacy Item".to_string()],
                failure_penalty: "none".to_string(),
                time_limit: None,
                difficulty_rating: 10,
                prerequisites: vec!["escaped_nexus".to_string()],
                completion_flags: vec!["game_completed".to_string()],
            },
        ]
    }
}

// ============================================================================
// РАСШИРЕННАЯ СИСТЕМА ПЕРСОНАЖЕЙ
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub handle: String,
    pub faction: String,
    pub role: String,
    pub personality: Personality,
    pub skills: HashMap<String, u32>,
    pub trust_level: i32,
    pub availability: bool,
    pub location: String,
    pub backstory: String,
    pub services: Vec<Service>,
    pub current_mission: Option<String>,
    pub dialogue_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub aggression: u32,
    pub loyalty: u32,
    pub greed: u32,
    pub caution: u32,
    pub intelligence: u32,
    pub morality: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOffering {
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub reputation_required: i32,
    pub faction_standing_required: HashMap<String, i32>,
    pub one_time_use: bool,
    pub availability_condition: String,
}

impl Character {
    pub fn new_shadow() -> Self {
        Character {
            name: "Алексей 'Тень' Морозов".to_string(),
            handle: "shadow_hunter".to_string(),
            faction: "Кибер-повстанцы".to_string(),
            role: "Информационный брокер".to_string(),
            personality: Personality {
                aggression: 30,
                loyalty: 80,
                greed: 40,
                caution: 90,
                intelligence: 85,
                morality: 70,
            },
            skills: {
                let mut skills = HashMap::new();
                skills.insert("Information Gathering".to_string(), 95);
                skills.insert("Network Security".to_string(), 80);
                skills.insert("Social Engineering".to_string(), 75);
                skills
            },
            trust_level: 25,
            availability: true,
            location: "Underground Safe House".to_string(),
            backstory: "Бывший сотрудник ФСБ, перешедший на сторону хакеров после разочарования в государственной системе. Специализируется на сборе компромата на крупные корпорации.".to_string(),
            services: vec![],
            current_mission: None,
            dialogue_history: vec![],
        }
    }
    
    pub fn new_ghost() -> Self {
        Character {
            name: "Анна 'Призрак' Волкова".to_string(),
            handle: "ghost_in_shell".to_string(),
            faction: "Хакеры-одиночки".to_string(),
            role: "Специалист по проникновению".to_string(),
            personality: Personality {
                aggression: 60,
                loyalty: 50,
                greed: 70,
                caution: 85,
                intelligence: 90,
                morality: 40,
            },
            skills: {
                let mut skills = HashMap::new();
                skills.insert("Advanced Hacking".to_string(), 98);
                skills.insert("Stealth Operations".to_string(), 90);
                skills.insert("Cryptography".to_string(), 85);
                skills
            },
            trust_level: 15,
            availability: true,
            location: "Mobile - Location Unknown".to_string(),
            backstory: "Легендарный хакер, чья личность остается загадкой. Говорят, она взламывала системы Пентагона в возрасте 16 лет. Работает только за очень большие деньги.".to_string(),
            services: vec![],
            current_mission: None,
            dialogue_history: vec![],
        }
    }
    
    pub fn new_dr_mitchell() -> Self {
        Character {
            name: "Доктор Джеймс Митчелл".to_string(),
            handle: "nuclear_prophet".to_string(),
            faction: "NEXUS Corporation".to_string(),
            role: "Главный архитектор проекта 'Цифровой Апокалипсис'".to_string(),
            personality: Personality {
                aggression: 90,
                loyalty: 95,
                greed: 60,
                caution: 70,
                intelligence: 98,
                morality: 10,
            },
            skills: {
                let mut skills = HashMap::new();
                skills.insert("Nuclear Engineering".to_string(), 100);
                skills.insert("AI Development".to_string(), 95);
                skills.insert("Strategic Planning".to_string(), 90);
                skills
            },
            trust_level: -100,
            availability: false,
            location: "NEXUS Headquarters - Executive Floor".to_string(),
            backstory: "Бывший ученый-ядерщик, одержимый идеей установления 'нового мирового порядка' через технологическое превосходство. Считает, что только NEXUS может спасти человечество от самоуничтожения.".to_string(),
            services: vec![],
            current_mission: Some("Oversee Digital Apocalypse Project".to_string()),
            dialogue_history: vec![],
        }
    }
    
    pub fn interact(&mut self, player_action: &str, state: &GameState) -> String {
        self.dialogue_history.push(player_action.to_string());
        
        match self.handle.as_str() {
            "shadow_hunter" => self.shadow_dialogue(player_action, state),
            "ghost_in_shell" => self.ghost_dialogue(player_action, state),
            "nuclear_prophet" => self.mitchell_dialogue(player_action, state),
            _ => "Персонаж не отвечает...".to_string(),
        }
    }
    
    fn shadow_dialogue(&self, action: &str, state: &GameState) -> String {
        match action {
            "greet" => "Приветствую, товарищ. Я слышал о твоих подвигах. NEXUS становится все более агрессивным - нам нужна помощь.".to_string(),
            "info_nexus" => "NEXUS - это не просто корпорация. Это сеть влияния, проникшая в правительства по всему миру. Их проект 'Цифровой Апокалипсис' - это попытка захватить контроль над всеми ядерными объектами планеты.".to_string(),
            "help_mission" => "Я могу предоставить тебе карты их сети, пароли сотрудников и информацию о слабых местах в их защите. Но это будет стоить 5000₿.".to_string(),
            "buy_info" => {
                if state.money >= 5000 {
                    "Сделка! Вот что мне удалось узнать о NEXUS... *передает зашифрованные данные*".to_string()
                } else {
                    "У тебя недостаточно средств. Информация стоит дорого в наше время.".to_string()
                }
            },
            _ => "О чем ты хочешь поговорить? Я могу рассказать о NEXUS или помочь с миссией.".to_string(),
        }
    }
    
    fn ghost_dialogue(&self, action: &str, state: &GameState) -> String {
        match action {
            "greet" => "Ты тот самый хакер, о котором все говорят? Интересно... Я 'Призрак'. Возможно, мы сможем сотрудничать.".to_string(),
            "hire_services" => "Мои услуги недешевы. Я могу взломать любую систему, но это будет стоить 15000₿. Гарантирую результат.".to_string(),
            "nexus_intel" => "NEXUS пытался нанять меня год назад. Я отказалась, но успела изучить их инфраструктуру. У меня есть эксклюзивная информация.".to_string(),
            "team_up" => {
                if state.reputation >= 50 {
                    "Твоя репутация впечатляет. Я согласна на временное партнерство. Вместе мы сможем проникнуть туда, куда одному не попасть.".to_string()
                } else {
                    "Ты еще не заслужил моего доверия. Повысь свою репутацию в сообществе, тогда поговорим.".to_string()
                }
            },
            _ => "Говори быстрее. Время - деньги, а мое время очень дорого стоит.".to_string(),
        }
    }
    
    fn mitchell_dialogue(&self, action: &str, _state: &GameState) -> String {
        match action {
            "greet" => "Так значит ты тот самый хакер, который досаждает нашей безопасности? Впечатляюще... но бесполезно.".to_string(),
            "stop_project" => "Остановить проект? Ты не понимаешь! Цифровой Апокалипсис - это не оружие, это спасение! Только абсолютная власть может принести мир.".to_string(),
            "negotiate" => "Переговоры? Интересно... Присоединись к нам, и я гарантирую тебе место в новом мире. Откажешься - и ты станешь пеплом вместе с остальными.".to_string(),
            "threaten" => "Угрожаешь мне? *смеется* У меня есть коды к каждой ядерной боеголовке на планете. Одно слово от меня - и твой город превратится в радиоактивную пустыню.".to_string(),
            _ => "Ты тратишь мое время. У меня есть мир, который нужно спасти... или уничтожить.".to_string(),
        }
    }
}

// ============================================================================
// СИСТЕМА МИНИ-ИГР И ГОЛОВОЛОМОК
// ============================================================================

#[derive(Debug, Clone)]
pub struct MiniGame {
    pub name: String,
    pub description: String,
    pub difficulty: u32,
    pub time_limit: Option<Duration>,
    pub reward_xp: u32,
    pub reward_money: u32,
}

pub struct MiniGameManager;

impl MiniGameManager {
    pub fn play_password_crack(terminal: &Terminal, difficulty: u32) -> bool {
        terminal.print_with_effect("=== ВЗЛОМ ПАРОЛЯ ===", TerminalEffect::Matrix);
        terminal.print_with_effect("Угадайте пароль по подсказкам:", TerminalEffect::Normal);
        
        let passwords = vec![
            ("admin123", vec!["5 цифр в конце", "начинается с 'admin'", "без спецсимволов"]),
            ("P@ssw0rd", vec!["8 символов", "содержит @ и 0", "начинается с P"]),
            ("nuclear2024", vec!["содержит год", "тема: атомная энергия", "11 символов"]),
            ("NEXUS_corp", vec!["название компании", "содержит подчеркивание", "9 символов"]),
        ];
        
        let (correct_password, hints) = &passwords[rand::thread_rng().gen_range(0..passwords.len())];
        
        for (i, hint) in hints.iter().enumerate() {
            terminal.print_with_effect(&format!("Подсказка {}: {}", i + 1, hint), TerminalEffect::Normal);
        }
        
        for attempt in 1..=3 {
            let guess = terminal.prompt(&format!("Попытка {}/3 - Введите пароль:"));
            
            if guess.to_lowercase() == correct_password.to_lowercase() {
                terminal.print_with_effect("🔓 ПАРОЛЬ ВЗЛОМАН!", TerminalEffect::Success);
                return true;
            } else {
                terminal.print_with_effect(&format!("❌ Неверно! Осталось попыток: {}", 3 - attempt), TerminalEffect::Error);
            }
        }
        
        terminal.print_with_effect(&format!("💀 Не удалось взломать. Правильный пароль: {}", correct_password), TerminalEffect::Error);
        false
    }
    
    pub fn play_circuit_puzzle(terminal: &Terminal) -> bool {
        terminal.print_with_effect("=== ЭЛЕКТРОННАЯ СХЕМА ===", TerminalEffect::Matrix);
        terminal.print_with_effect("Восстановите последовательность для активации:", TerminalEffect::Normal);
        
        let sequence = vec!["A", "B", "C", "D"];
        let mut shuffled = sequence.clone();
        shuffled.shuffle(&mut rand::thread_rng());
        
        terminal.print_with_effect("Доступные компоненты:", TerminalEffect::Normal);
        for (i, component) in shuffled.iter().enumerate() {
            terminal.print_with_effect(&format!("{}: {}", i + 1, component), TerminalEffect::Normal);
        }
        
        terminal.print_with_effect("Подсказка: Последовательность должна быть A -> B -> C -> D", TerminalEffect::Warning);
        
        let answer = terminal.prompt("Введите последовательность (например: 1,2,3,4):");
        let parts: Vec<&str> = answer.split(',').collect();
        
        if parts.len() == 4 {
            let mut player_sequence = Vec::new();
            for part in parts {
                if let Ok(index) = part.trim().parse::<usize>() {
                    if index > 0 && index <= 4 {
                        player_sequence.push(shuffled[index - 1]);
                    }
                }
            }
            
            if player_sequence == sequence {
                terminal.print_with_effect("⚡ СХЕМА АКТИВИРОВАНА!", TerminalEffect::Success);
                return true;
            }
        }
        
        terminal.print_with_effect("❌ Неверная последовательность! Схема заблокирована.", TerminalEffect::Error);
        false
    }
    
    pub fn play_memory_game(terminal: &Terminal) -> bool {
        terminal.print_with_effect("=== ВОССТАНОВЛЕНИЕ ПАМЯТИ ===", TerminalEffect::Matrix);
        terminal.print_with_effect("Запомните последовательность и повторите:", TerminalEffect::Normal);
        
        let symbols = vec!["🔴", "🔵", "🟢", "🟡", "🟣", "🟠"];
        let mut sequence = Vec::new();
        let length = rand::thread_rng().gen_range(4..8);
        
        for _ in 0..length {
            sequence.push(symbols[rand::thread_rng().gen_range(0..symbols.len())]);
        }
        
        // Показываем последовательность
        for symbol in &sequence {
            print!("{} ", symbol);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(800));
        }
        
        // Очищаем
        terminal.print_with_effect("\n\nПоследовательность скрыта!", TerminalEffect::Warning);
        
        let answer = terminal.prompt("Введите последовательность (скопируйте символы):");
        
        let player_sequence: Vec<&str> = answer.split_whitespace().collect();
        let original_sequence: Vec<&str> = sequence.iter().map(|s| s.as_str()).collect();
        
        if player_sequence == original_sequence {
            terminal.print_with_effect("🧠 ПАМЯТЬ ВОССТАНОВЛЕНА!", TerminalEffect::Success);
            true
        } else {
            terminal.print_with_effect("❌ Неверная последовательность!", TerminalEffect::Error);
            false
        }
    }
    
    pub fn play_firewall_maze(terminal: &Terminal) -> bool {
        terminal.print_with_effect("=== ЛАБИРИНТ ФАЙРВОЛА ===", TerminalEffect::Matrix);
        terminal.print_with_effect("Найдите путь через защиту:", TerminalEffect::Normal);
        
        let maze = vec![
            "┌─────────────┐",
            "│S..│.......│.│",
            "│.█.│.█████.│.│", 
            "│...│.......│.│",
            "│.█████.███.│.│",
            "│.......│...│.│",
            "│.█████.│.█.│.│",
            "│.......│...│E│",
            "└─────────────┘",
        ];
        
        for line in &maze {
            terminal.print_with_effect(line, TerminalEffect::Matrix);
        }
        
        terminal.print_with_effect("S - старт, E - выход, █ - стена, . - путь", TerminalEffect::Normal);
        
        let path = terminal.prompt("Введите путь (например: right,down,right,up,right,down):");
        let moves: Vec<&str> = path.split(',').collect();
        
        // Простая проверка пути (в реальной игре была бы более сложная логика)
        let valid_moves = vec!["right", "down", "right", "down", "right", "down", "right"];
        let moves_lower: Vec<String> = moves.iter().map(|m| m.trim().to_lowercase()).collect();
        
        if moves_lower.len() >= 6 && moves_lower.iter().all(|m| ["right", "down", "up", "left"].contains(&m.as_str())) {
            terminal.print_with_effect("🔓 ФАЙРВОЛ ОБОЙДЕН!", TerminalEffect::Success);
            true
        } else {
            terminal.print_with_effect("❌ Тупик! Файрвол заблокировал доступ.", TerminalEffect::Error);
            false
        }
    }
}

// ============================================================================
// РАСШИРЕННАЯ ЭКОНОМИЧЕСКАЯ СИСТЕМА
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyManager {
    pub black_market: BlackMarket,
    pub contracts: Vec<Contract>,
    pub investments: HashMap<String, Investment>,
    pub cryptocurrency_rates: HashMap<String, f64>,
    pub market_volatility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackMarket {
    pub items: Vec<MarketItem>,
    pub reputation_requirement: i32,
    pub last_refresh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketItem {
    pub name: String,
    pub description: String,
    pub price: u32,
    pub category: ItemCategory,
    pub rarity: Rarity,
    pub reputation_required: i32,
    pub effects: HashMap<String, i32>,
    pub one_time_use: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    Software,
    Hardware,
    Information,
    Service,
    Weapon,
    Defense,
    Utility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub client: String,
    pub title: String,
    pub description: String,
    pub payment: u32,
    pub deadline: Duration,
    pub difficulty: u32,
    pub requirements: Vec<String>,
    pub completion_conditions: Vec<String>,
    pub penalty_for_failure: u32,
    pub reputation_reward: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Investment {
    pub name: String,
    pub initial_amount: u32,
    pub current_value: u32,
    pub return_rate: f64,
    pub risk_level: u32,
    pub maturity_time: Duration,
}

impl EconomyManager {
    pub fn new() -> Self {
        EconomyManager {
            black_market: BlackMarket::new(),
            contracts: Vec::new(),
            investments: HashMap::new(),
            cryptocurrency_rates: {
                let mut rates = HashMap::new();
                rates.insert("Bitcoin".to_string(), 45000.0);
                rates.insert("Ethereum".to_string(), 2800.0);
                rates.insert("DarkCoin".to_string(), 150.0);
                rates.insert("CyberCoin".to_string(), 25.0);
                rates
            },
            market_volatility: 0.1,
        }
    }
    
    pub fn generate_contracts(&mut self) -> Vec<Contract> {
        let mut contracts = Vec::new();
        
        let contract_templates = vec![
            ("Корпоративный шпионаж", "Взломать конкурента и украсть бизнес-планы", 15000, 5),
            ("Очистка репутации", "Удалить компромат из интернета", 8000, 3),
            ("Месть бывшему", "Взломать социальные сети и опубликовать переписку", 5000, 2),
            ("Финансовое мошенничество", "Перевести деньги с корпоративных счетов", 25000, 8),
            ("Промышленный саботаж", "Вывести из строя производственные системы", 20000, 7),
            ("Политический хактивизм", "Взломать правительственные сайты", 12000, 6),
            ("Освобождение данных", "Опубликовать секретные документы", 18000, 7),
            ("Защита свидетеля", "Обеспечить цифровую анонимность", 10000, 4),
        ];
        
        for (i, (title, desc, payment, difficulty)) in contract_templates.iter().enumerate() {
            contracts.push(Contract {
                id: format!("contract_{}", i),
                client: format!("Anonymous Client #{}", i + 1),
                title: title.to_string(),
                description: desc.to_string(),
                payment: *payment,
                deadline: Duration::from_secs(86400 * (difficulty + 1) as u64), // 1-9 дней
                difficulty: *difficulty,
                requirements: vec![
                    format!("Hacking skill >= {}", difficulty * 10),
                    format!("Reputation >= {}", difficulty * 5),
                ],
                completion_conditions: vec![
                    "Target system compromised".to_string(),
                    "Evidence collected".to_string(),
                ],
                penalty_for_failure: payment / 4,
                reputation_reward: *difficulty as i32 * 5,
            });
        }
        
        contracts
    }
}

impl BlackMarket {
    pub fn new() -> Self {
        BlackMarket {
            items: Self::generate_items(),
            reputation_requirement: 25,
            last_refresh: "2024-01-15".to_string(),
        }
    }
    
    fn generate_items() -> Vec<MarketItem> {
        vec![
            MarketItem {
                name: "Advanced Port Scanner".to_string(),
                description: "Сканер портов нового поколения с обходом файрволов".to_string(),
                price: 2500,
                category: ItemCategory::Software,
                rarity: Rarity::Uncommon,
                reputation_required: 20,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("scan_success_rate".to_string(), 25);
                    effects
                },
                one_time_use: false,
            },
            MarketItem {
                name: "Neural Password Cracker".to_string(),
                description: "ИИ-система для взлома паролей любой сложности".to_string(),
                price: 15000,
                category: ItemCategory::Software,
                rarity: Rarity::Epic,
                reputation_required: 75,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("password_crack_rate".to_string(), 50);
                    effects
                },
                one_time_use: false,
            },
            MarketItem {
                name: "Ghost VPN Network".to_string(),
                description: "Анонимная сеть с узлами по всему миру".to_string(),
                price: 5000,
                category: ItemCategory::Service,
                rarity: Rarity::Rare,
                reputation_required: 40,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("anonymity".to_string(), 40);
                    effects.insert("stress_reduction".to_string(), 20);
                    effects
                },
                one_time_use: false,
            },
            MarketItem {
                name: "Quantum Encryption Key".to_string(),
                description: "Квантовый ключ шифрования, неподдающийся взлому".to_string(),
                price: 50000,
                category: ItemCategory::Defense,
                rarity: Rarity::Legendary,
                reputation_required: 100,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("data_protection".to_string(), 100);
                    effects
                },
                one_time_use: true,
            },
            MarketItem {
                name: "Social Engineering Toolkit".to_string(),
                description: "Набор инструментов для манипуляции людьми".to_string(),
                price: 8000,
                category: ItemCategory::Software,
                rarity: Rarity::Rare,
                reputation_required: 50,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("social_engineering_bonus".to_string(), 30);
                    effects
                },
                one_time_use: false,
            },
            MarketItem {
                name: "Hardware Keylogger".to_string(),
                description: "Микроскопическое устройство для перехвата нажатий".to_string(),
                price: 1200,
                category: ItemCategory::Hardware,
                rarity: Rarity::Common,
                reputation_required: 10,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("keylog_efficiency".to_string(), 20);
                    effects
                },
                one_time_use: true,
            },
            MarketItem {
                name: "Corporate Intel Package".to_string(),
                description: "Досье на топ-менеджмент крупных корпораций".to_string(),
                price: 12000,
                category: ItemCategory::Information,
                rarity: Rarity::Rare,
                reputation_required: 60,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("corporate_mission_bonus".to_string(), 25);
                    effects
                },
                one_time_use: true,
            },
            MarketItem {
                name: "Zero-Day Exploit Collection".to_string(),
                description: "Коллекция неизвестных уязвимостей".to_string(),
                price: 35000,
                category: ItemCategory::Software,
                rarity: Rarity::Legendary,
                reputation_required: 90,
                effects: {
                    let mut effects = HashMap::new();
                    effects.insert("exploit_success_rate".to_string(), 60);
                    effects
                },
                one_time_use: false,
            },
        ]
    }
}

// ============================================================================
// СИСТЕМА БОЕВЫХ ДЕЙСТВИЙ И КИБЕР-ВОЙНЫ
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSystem {
    pub player_combat_stats: CombatStats,
    pub active_opponents: Vec<CyberOpponent>,
    pub battle_log: Vec<String>,
    pub current_battle: Option<Battle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    pub attack_power: u32,
    pub defense_rating: u32,
    pub speed: u32,
    pub stealth: u32,
    pub counter_attack_chance: f64,
    pub critical_hit_chance: f64,
    pub equipment_bonuses: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberOpponent {
    pub name: String,
    pub opponent_type: OpponentType,
    pub health: u32,
    pub max_health: u32,
    pub attack_power: u32,
    pub defense: u32,
    pub special_abilities: Vec<SpecialAbility>,
    pub ai_behavior: AIBehavior,
    pub loot_table: Vec<String>,
    pub experience_reward: u32,
    pub money_reward: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpponentType {
    SecurityAgent,
    CorporateHacker,
    GovernmentCyberSoldier,
    AIDefenseSystem,
    MercenaryHacker,
    CyberCriminal,
    CorruptedAI,
    NEXUSElite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialAbility {
    pub name: String,
    pub description: String,
    pub damage: u32,
    pub cooldown_turns: u32,
    pub current_cooldown: u32,
    pub effect_type: AbilityEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityEffect {
    Damage,
    Stun,
    Poison,
    Heal,
    BuffAttack,
    BuffDefense,
    Debuff,
    SystemCrash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIBehavior {
    Aggressive,
    Defensive,
    Tactical,
    Berserker,
    Support,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battle {
    pub battle_id: String,
    pub battle_type: BattleType,
    pub turn_counter: u32,
    pub battle_state: BattleState,
    pub environment_effects: Vec<EnvironmentEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleType {
    RandomEncounter,
    StoryBoss,
    ArenaFight,
    DefensiveBattle,
    EscapeSequence,
    NetworkWar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleState {
    PlayerTurn,
    OpponentTurn,
    BattleWon,
    BattleLost,
    EscapeSuccessful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentEffect {
    pub name: String,
    pub description: String,
    pub effect_on_player: i32,
    pub effect_on_opponents: i32,
    pub duration: u32,
}

impl CombatSystem {
    pub fn new() -> Self {
        CombatSystem {
            player_combat_stats: CombatStats::default(),
            active_opponents: Vec::new(),
            battle_log: Vec::new(),
            current_battle: None,
        }
    }
    
    pub fn initiate_battle(&mut self, opponent_type: OpponentType, terminal: &Terminal) -> bool {
        let opponent = self.create_opponent(opponent_type);
        self.active_opponents = vec![opponent];
        
        let battle = Battle {
            battle_id: format!("battle_{}", rand::thread_rng().gen::<u32>()),
            battle_type: BattleType::RandomEncounter,
            turn_counter: 0,
            battle_state: BattleState::PlayerTurn,
            environment_effects: Vec::new(),
        };
        
        self.current_battle = Some(battle);
        self.display_battle_start(terminal);
        
        while let Some(ref mut battle) = self.current_battle {
            match battle.battle_state {
                BattleState::PlayerTurn => {
                    if !self.player_turn(terminal) {
                        battle.battle_state = BattleState::BattleLost;
                        break;
                    }
                },
                BattleState::OpponentTurn => {
                    self.opponent_turn(terminal);
                },
                BattleState::BattleWon => {
                    self.display_victory(terminal);
                    return true;
                },
                BattleState::BattleLost => {
                    self.display_defeat(terminal);
                    return false;
                },
                BattleState::EscapeSuccessful => {
                    self.display_escape(terminal);
                    return true;
                },
            }
            battle.turn_counter += 1;
        }
        
        false
    }
    
    fn create_opponent(&self, opponent_type: OpponentType) -> CyberOpponent {
        let mut rng = rand::thread_rng();
        
        match opponent_type {
            OpponentType::SecurityAgent => CyberOpponent {
                name: "Агент безопасности NEXUS".to_string(),
                opponent_type,
                health: 150,
                max_health: 150,
                attack_power: 25,
                defense: 20,
                special_abilities: vec![
                    SpecialAbility {
                        name: "Кибер-атака".to_string(),
                        description: "Мощная атака на системы противника".to_string(),
                        damage: 40,
                        cooldown_turns: 3,
                        current_cooldown: 0,
                        effect_type: AbilityEffect::Damage,
                    }
                ],
                ai_behavior: AIBehavior::Tactical,
                loot_table: vec!["Security Keycard".to_string(), "Encrypted Data".to_string()],
                experience_reward: 150,
                money_reward: 1000,
            },
            OpponentType::CorporateHacker => CyberOpponent {
                name: "Корпоративный хакер".to_string(),
                opponent_type,
                health: 100,
                max_health: 100,
                attack_power: 35,
                defense: 15,
                special_abilities: vec![
                    SpecialAbility {
                        name: "Virus Upload".to_string(),
                        description: "Загружает вирус, наносящий урон со временем".to_string(),
                        damage: 15,
                        cooldown_turns: 4,
                        current_cooldown: 0,
                        effect_type: AbilityEffect::Poison,
                    }
                ],
                ai_behavior: AIBehavior::Aggressive,
                loot_table: vec!["Hacking Tools".to_string(), "Corporate Secrets".to_string()],
                experience_reward: 200,
                money_reward: 1500,
            },
            OpponentType::AIDefenseSystem => CyberOpponent {
                name: "ИИ система защиты".to_string(),
                opponent_type,
                health: 300,
                max_health: 300,
                attack_power: 45,
                defense: 35,
                special_abilities: vec![
                    SpecialAbility {
                        name: "System Overload".to_string(),
                        description: "Перегружает системы противника".to_string(),
                        damage: 60,
                        cooldown_turns: 5,
                        current_cooldown: 0,
                        effect_type: AbilityEffect::SystemCrash,
                    },
                    SpecialAbility {
                        name: "Self-Repair".to_string(),
                        description: "Восстанавливает здоровье".to_string(),
                        damage: 0,
                        cooldown_turns: 6,
                        current_cooldown: 0,
                        effect_type: AbilityEffect::Heal,
                    }
                ],
                ai_behavior: AIBehavior::Adaptive,
                loot_table: vec!["AI Core".to_string(), "Advanced Algorithms".to_string()],
                experience_reward: 500,
                money_reward: 3000,
            },
            _ => CyberOpponent {
                name: "Неизвестный противник".to_string(),
                opponent_type,
                health: 50,
                max_health: 50,
                attack_power: 15,
                defense: 10,
                special_abilities: Vec::new(),
                ai_behavior: AIBehavior::Aggressive,
                loot_table: vec!["Basic Data".to_string()],
                experience_reward: 50,
                money_reward: 100,
            },
        }
    }
    
    fn display_battle_start(&self, terminal: &Terminal) {
        terminal.print_with_effect("⚔️ ═══ КИБЕР-БОЙ НАЧИНАЕТСЯ ═══ ⚔️", TerminalEffect::Error);
        if let Some(opponent) = self.active_opponents.first() {
            terminal.print_with_effect(&format!("Противник: {}", opponent.name), TerminalEffect::Warning);
            terminal.print_with_effect(&format!("Здоровье: {}/{}", opponent.health, opponent.max_health), TerminalEffect::Normal);
        }
    }
    
    fn player_turn(&mut self, terminal: &Terminal) -> bool {
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("=== ВАШ ХОД ===", TerminalEffect::Matrix);
        terminal.print_with_effect("1. Атака", TerminalEffect::Normal);
        terminal.print_with_effect("2. Защита", TerminalEffect::Normal);
        terminal.print_with_effect("3. Специальная способность", TerminalEffect::Normal);
        terminal.print_with_effect("4. Попытка побега", TerminalEffect::Normal);
        
        let choice = terminal.prompt("Выберите действие (1-4):");
        
        match choice.as_str() {
            "1" => self.player_attack(terminal),
            "2" => self.player_defend(terminal),
            "3" => self.player_special_attack(terminal),
            "4" => self.attempt_escape(terminal),
            _ => {
                terminal.print_with_effect("Неверный выбор! Ход пропущен.", TerminalEffect::Error);
            }
        }
        
        if let Some(ref mut battle) = self.current_battle {
            if self.active_opponents.iter().all(|o| o.health == 0) {
                battle.battle_state = BattleState::BattleWon;
            } else {
                battle.battle_state = BattleState::OpponentTurn;
            }
        }
        
        true
    }
    
    fn player_attack(&mut self, terminal: &Terminal) {
        let damage = self.player_combat_stats.attack_power;
        let mut rng = rand::thread_rng();
        
        if let Some(opponent) = self.active_opponents.first_mut() {
            let actual_damage = if damage > opponent.defense {
                damage - opponent.defense
            } else {
                1
            };
            
            let critical_hit = rng.gen_bool(self.player_combat_stats.critical_hit_chance);
            let final_damage = if critical_hit { actual_damage * 2 } else { actual_damage };
            
            opponent.health = opponent.health.saturating_sub(final_damage);
            
            if critical_hit {
                terminal.print_with_effect(&format!("💥 КРИТИЧЕСКИЙ УДАР! Нанесено {} урона!", final_damage), TerminalEffect::Success);
            } else {
                terminal.print_with_effect(&format!("⚔️ Нанесено {} урона", final_damage), TerminalEffect::Normal);
            }
            
            if opponent.health == 0 {
                terminal.print_with_effect(&format!("💀 {} повержен!", opponent.name), TerminalEffect::Success);
            }
        }
    }
    
    fn player_defend(&mut self, terminal: &Terminal) {
        self.player_combat_stats.defense_rating *= 2;
        terminal.print_with_effect("🛡️ Вы приняли защитную позицию. Защита удвоена на этот ход.", TerminalEffect::Success);
    }
    
    fn player_special_attack(&mut self, terminal: &Terminal) {
        // Реализация специальных атак игрока
        terminal.print_with_effect("⚡ Выполнена специальная кибер-атака!", TerminalEffect::Matrix);
        
        if let Some(opponent) = self.active_opponents.first_mut() {
            let damage = self.player_combat_stats.attack_power * 2;
            opponent.health = opponent.health.saturating_sub(damage);
            terminal.print_with_effect(&format!("💥 Нанесено {} урона специальной атакой!", damage), TerminalEffect::Success);
        }
    }
    
    fn attempt_escape(&mut self, terminal: &Terminal) {
        let mut rng = rand::thread_rng();
        let escape_chance = self.player_combat_stats.speed as f64 / 100.0;
        
        if rng.gen_bool(escape_chance) {
            terminal.print_with_effect("🏃 Успешный побег из боя!", TerminalEffect::Success);
            if let Some(ref mut battle) = self.current_battle {
                battle.battle_state = BattleState::EscapeSuccessful;
            }
        } else {
            terminal.print_with_effect("❌ Побег не удался! Противник блокирует выход.", TerminalEffect::Error);
        }
    }
    
    fn opponent_turn(&mut self, terminal: &Terminal) {
        terminal.print_with_effect("=== ХОД ПРОТИВНИКА ===", TerminalEffect::Warning);
        
        if let Some(opponent) = self.active_opponents.first_mut() {
            let mut rng = rand::thread_rng();
            
            // ИИ выбирает действие на основе поведения
            let action = match opponent.ai_behavior {
                AIBehavior::Aggressive => "attack",
                AIBehavior::Defensive => if opponent.health < opponent.max_health / 2 { "heal" } else { "attack" },
                AIBehavior::Tactical => if rng.gen_bool(0.7) { "special" } else { "attack" },
                AIBehavior::Adaptive => if rng.gen_bool(0.5) { "special" } else { "attack" },
                _ => "attack",
            };
            
            match action {
                "attack" => {
                    let damage = opponent.attack_power.saturating_sub(self.player_combat_stats.defense_rating / 2);
                    terminal.print_with_effect(&format!("💀 {} атакует и наносит {} урона!", opponent.name, damage), TerminalEffect::Error);
                    // В реальной игре здесь бы уменьшалось здоровье игрока
                },
                "special" => {
                    if let Some(ability) = opponent.special_abilities.first_mut() {
                        if ability.current_cooldown == 0 {
                            terminal.print_with_effect(&format!("⚡ {} использует '{}'!", opponent.name, ability.name), TerminalEffect::Warning);
                            ability.current_cooldown = ability.cooldown_turns;
                        } else {
                            terminal.print_with_effect(&format!("{} атакует обычной атакой", opponent.name), TerminalEffect::Normal);
                        }
                    }
                },
                "heal" => {
                    let heal_amount = 30;
                    opponent.health = std::cmp::min(opponent.max_health, opponent.health + heal_amount);
                    terminal.print_with_effect(&format!("💚 {} восстанавливает {} здоровья", opponent.name, heal_amount), TerminalEffect::Warning);
                },
                _ => {}
            }
            
            // Уменьшаем кулдауны способностей
            for ability in &mut opponent.special_abilities {
                if ability.current_cooldown > 0 {
                    ability.current_cooldown -= 1;
                }
            }
        }
        
        if let Some(ref mut battle) = self.current_battle {
            battle.battle_state = BattleState::PlayerTurn;
        }
        
        // Сброс временных модификаторов защиты
        self.player_combat_stats.defense_rating /= 2;
    }
    
    fn display_victory(&self, terminal: &Terminal) {
        terminal.print_with_effect("🎉 ═══ ПОБЕДА! ═══ 🎉", TerminalEffect::Success);
        
        if let Some(opponent) = self.active_opponents.first() {
            terminal.print_with_effect(&format!("Получено {} опыта", opponent.experience_reward), TerminalEffect::Success);
            terminal.print_with_effect(&format!("Получено {}₿", opponent.money_reward), TerminalEffect::Success);
            
            if !opponent.loot_table.is_empty() {
                let loot = &opponent.loot_table[rand::thread_rng().gen_range(0..opponent.loot_table.len())];
                terminal.print_with_effect(&format!("Найден предмет: {}", loot), TerminalEffect::Success);
            }
        }
    }
    
    fn display_defeat(&self, terminal: &Terminal) {
        terminal.print_with_effect("💀 ═══ ПОРАЖЕНИЕ ═══ 💀", TerminalEffect::Error);
        terminal.print_with_effect("Ваши системы взломаны! Требуется перезагрузка...", TerminalEffect::Error);
    }
    
    fn display_escape(&self, terminal: &Terminal) {
        terminal.print_with_effect("🏃 ═══ УСПЕШНЫЙ ПОБЕГ ═══ 🏃", TerminalEffect::Success);
        terminal.print_with_effect("Вы успешно скрылись от противника", TerminalEffect::Success);
    }
}

impl Default for CombatStats {
    fn default() -> Self {
        CombatStats {
            attack_power: 20,
            defense_rating: 15,
            speed: 25,
            stealth: 20,
            counter_attack_chance: 0.1,
            critical_hit_chance: 0.15,
            equipment_bonuses: HashMap::new(),
        }
    }
}

// ============================================================================
// СИСТЕМА КРАФТИНГА И МОДИФИКАЦИИ ОБОРУДОВАНИЯ
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingSystem {
    pub recipes: HashMap<String, Recipe>,
    pub player_materials: HashMap<String, u32>,
    pub crafting_stations: Vec<CraftingStation>,
    pub upgrade_paths: HashMap<String, Vec<Upgrade>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub category: CraftingCategory,
    pub required_materials: HashMap<String, u32>,
    pub required_skill_level: u32,
    pub required_station: Option<String>,
    pub crafting_time: Duration,
    pub result_item: String,
    pub result_quantity: u32,
    pub experience_gain: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingCategory {
    Software,
    Hardware,
    Weapon,
    Defense,
    Utility,
    Consumable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingStation {
    pub name: String,
    pub description: String,
    pub station_type: StationType,
    pub available_recipes: Vec<String>,
    pub upgrade_level: u32,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StationType {
    BasicWorkbench,
    AdvancedLab,
    QuantumForge,
    BioLab,
    DataCenter,
    HardwareStation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub required_materials: HashMap<String, u32>,
    pub stat_bonuses: HashMap<String, i32>,
    pub special_effects: Vec<String>,
    pub upgrade_cost: u32,
}

impl CraftingSystem {
    pub fn new() -> Self {
        let mut recipes = HashMap::new();
        let crafting_recipes = Self::create_base_recipes();
        
        for recipe in crafting_recipes {
            recipes.insert(recipe.name.clone(), recipe);
        }
        
        CraftingSystem {
            recipes,
            player_materials: HashMap::new(),
            crafting_stations: Self::create_crafting_stations(),
            upgrade_paths: HashMap::new(),
        }
    }
    
    fn create_base_recipes() -> Vec<Recipe> {
        vec![
            Recipe {
                name: "Basic Virus".to_string(),
                description: "Простой вирус для базовых атак".to_string(),
                category: CraftingCategory::Software,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("Code Fragments".to_string(), 5);
                    materials.insert("Malware Core".to_string(), 1);
                    materials
                },
                required_skill_level: 1,
                required_station: Some("BasicWorkbench".to_string()),
                crafting_time: Duration::from_secs(300),
                result_item: "Basic Attack Virus".to_string(),
                result_quantity: 1,
                experience_gain: 50,
            },
            Recipe {
                name: "Advanced Encryption Tool".to_string(),
                description: "Инструмент для продвинутого шифрования".to_string(),
                category: CraftingCategory::Software,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("Quantum Bits".to_string(), 10);
                    materials.insert("Encryption Algorithm".to_string(), 3);
                    materials.insert("Processing Units".to_string(), 2);
                    materials
                },
                required_skill_level: 5,
                required_station: Some("AdvancedLab".to_string()),
                crafting_time: Duration::from_secs(1200),
                result_item: "Quantum Encryptor".to_string(),
                result_quantity: 1,
                experience_gain: 200,
            },
            Recipe {
                name: "Neural Interface".to_string(),
                description: "Прямой интерфейс мозг-компьютер".to_string(),
                category: CraftingCategory::Hardware,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("Biocompatible Circuits".to_string(), 5);
                    materials.insert("Neural Mesh".to_string(), 1);
                    materials.insert("Quantum Processors".to_string(), 2);
                    materials.insert("Bio-gel".to_string(), 3);
                    materials
                },
                required_skill_level: 8,
                required_station: Some("BioLab".to_string()),
                crafting_time: Duration::from_secs(3600),
                result_item: "Advanced Neural Interface".to_string(),
                result_quantity: 1,
                experience_gain: 500,
            },
            Recipe {
                name: "Stealth Cloak Software".to_string(),
                description: "Программа для скрытия в сети".to_string(),
                category: CraftingCategory::Defense,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("Stealth Algorithms".to_string(), 8);
                    materials.insert("Proxy Chains".to_string(), 5);
                    materials.insert("Anonymity Protocols".to_string(), 3);
                    materials
                },
                required_skill_level: 6,
                required_station: Some("DataCenter".to_string()),
                crafting_time: Duration::from_secs(900),
                result_item: "Ghost Protocol Suite".to_string(),
                result_quantity: 1,
                experience_gain: 300,
            },
            Recipe {
                name: "Data Mining Bot".to_string(),
                description: "Автономный бот для сбора информации".to_string(),
                category: CraftingCategory::Utility,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("AI Core".to_string(), 1);
                    materials.insert("Search Algorithms".to_string(), 10);
                    materials.insert("Network Crawlers".to_string(), 5);
                    materials.insert("Data Filters".to_string(), 3);
                    materials
                },
                required_skill_level: 7,
                required_station: Some("AdvancedLab".to_string()),
                crafting_time: Duration::from_secs(1800),
                result_item: "Autonomous Data Miner".to_string(),
                result_quantity: 1,
                experience_gain: 400,
            },
            Recipe {
                name: "System Restoration Kit".to_string(),
                description: "Набор для восстановления поврежденных систем".to_string(),
                category: CraftingCategory::Consumable,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("Repair Nanobots".to_string(), 15);
                    materials.insert("System Patches".to_string(), 8);
                    materials.insert("Recovery Scripts".to_string(), 5);
                    materials
                },
                required_skill_level: 4,
                required_station: Some("BasicWorkbench".to_string()),
                crafting_time: Duration::from_secs(600),
                result_item: "Emergency Repair Kit".to_string(),
                result_quantity: 3,
                experience_gain: 150,
            },
            Recipe {
                name: "Quantum Firewall".to_string(),
                description: "Неприступная защита на квантовом уровне".to_string(),
                category: CraftingCategory::Defense,
                required_materials: {
                    let mut materials = HashMap::new();
                    materials.insert("Quantum Entanglement Cores".to_string(), 3);
                    materials.insert("Advanced Cryptographic Keys".to_string(), 5);
                    materials.insert("Defense Matrices".to_string(), 10);
                    materials.insert("Quantum Memory".to_string(), 2);
                    materials
                },
                required_skill_level: 10,
                required_station: Some("QuantumForge".to_string()),
                crafting_time: Duration::from_secs(7200),
                result_item: "Impenetrable Quantum Firewall".to_string(),
                result_quantity: 1,
                experience_gain: 1000,
            },
        ]
    }
    
    fn create_crafting_stations() -> Vec<CraftingStation> {
        vec![
            CraftingStation {
                name: "Basic Workbench".to_string(),
                description: "Простой верстак для базового крафтинга".to_string(),
                station_type: StationType::BasicWorkbench,
                available_recipes: vec!["Basic Virus".to_string(), "System Restoration Kit".to_string()],
                upgrade_level: 1,
                location: "Player Hideout".to_string(),
            },
            CraftingStation {
                name: "Advanced Laboratory".to_string(),
                description: "Продвинутая лаборатория с высокотехнологичным оборудованием".to_string(),
                station_type: StationType::AdvancedLab,
                available_recipes: vec!["Advanced Encryption Tool".to_string(), "Data Mining Bot".to_string()],
                upgrade_level: 1,
                location: "Underground Tech Hub".to_string(),
            },
            CraftingStation {
                name: "Quantum Forge".to_string(),
                description: "Экспериментальная установка для квантового крафтинга".to_string(),
                station_type: StationType::QuantumForge,
                available_recipes: vec!["Quantum Firewall".to_string()],
                upgrade_level: 1,
                location: "Secret Research Facility".to_string(),
            },
            CraftingStation {
                name: "Bio-Laboratory".to_string(),
                description: "Лаборатория для создания био-технологий".to_string(),
                station_type: StationType::BioLab,
                available_recipes: vec!["Neural Interface".to_string()],
                upgrade_level: 1,
                location: "Abandoned Medical Center".to_string(),
            },
            CraftingStation {
                name: "Data Processing Center".to_string(),
                description: "Центр обработки данных для создания программного обеспечения".to_string(),
                station_type: StationType::DataCenter,
                available_recipes: vec!["Stealth Cloak Software".to_string()],
                upgrade_level: 1,
                location: "Corporate District".to_string(),
            },
        ]
    }
    
    pub fn craft_item(&mut self, recipe_name: &str, player_skills: &HashMap<String, u32>, terminal: &Terminal) -> bool {
        if let Some(recipe) = self.recipes.get(recipe_name) {
            // Проверяем уровень навыка
            let programming_skill = *player_skills.get("Программирование").unwrap_or(&0);
            if programming_skill < recipe.required_skill_level {
                terminal.print_with_effect(&format!("Недостаточный уровень навыка! Требуется: {}, у вас: {}", 
                    recipe.required_skill_level, programming_skill), TerminalEffect::Error);
                return false;
            }
            
            // Проверяем материалы
            for (material, required_amount) in &recipe.required_materials {
                let available = *self.player_materials.get(material).unwrap_or(&0);
                if available < *required_amount {
                    terminal.print_with_effect(&format!("Недостаточно материала '{}'. Требуется: {}, у вас: {}", 
                        material, required_amount, available), TerminalEffect::Error);
                    return false;
                }
            }
            
            // Проверяем станцию крафтинга
            if let Some(required_station) = &recipe.required_station {
                let station_available = self.crafting_stations.iter()
                    .any(|s| s.station_type == match required_station.as_str() {
                        "BasicWorkbench" => StationType::BasicWorkbench,
                        "AdvancedLab" => StationType::AdvancedLab,
                        "QuantumForge" => StationType::QuantumForge,
                        "BioLab" => StationType::BioLab,
                        "DataCenter" => StationType::DataCenter,
                        _ => StationType::BasicWorkbench,
                    });
                
                if !station_available {
                    terminal.print_with_effect(&format!("Требуется станция крафтинга: {}", required_station), TerminalEffect::Error);
                    return false;
                }
            }
            
            // Начинаем крафтинг
            terminal.print_with_effect(&format!("Начинаем создание: {}", recipe.name), TerminalEffect::Matrix);
            terminal.loading_animation("Процесс крафтинга", recipe.crafting_time);
            
            // Списываем материалы
            for (material, amount) in &recipe.required_materials {
                let current = *self.player_materials.get(material).unwrap_or(&0);
                self.player_materials.insert(material.clone(), current - amount);
            }
            
            terminal.print_with_effect(&format!("✅ Создан предмет: {} x{}", recipe.result_item, recipe.result_quantity), TerminalEffect::Success);
            terminal.print_with_effect(&format!("Получено {} опыта крафтинга", recipe.experience_gain), TerminalEffect::Success);
            
            true
        } else {
            terminal.print_with_effect("Рецепт не найден!", TerminalEffect::Error);
            false
        }
    }
    
    pub fn add_material(&mut self, material: String, amount: u32) {
        let current = *self.player_materials.get(&material).unwrap_or(&0);
        self.player_materials.insert(material, current + amount);
    }
    
    pub fn display_recipes(&self, terminal: &Terminal) {
        terminal.print_with_effect("═══ ДОСТУПНЫЕ РЕЦЕПТЫ ═══", TerminalEffect::Matrix);
        
        for (name, recipe) in &self.recipes {
            terminal.print_with_effect(&format!("📋 {}", name), TerminalEffect::Success);
            terminal.print_with_effect(&format!("   Описание: {}", recipe.description), TerminalEffect::Normal);
            terminal.print_with_effect(&format!("   Требуемый навык: {}", recipe.required_skill_level), TerminalEffect::Warning);
            terminal.print_with_effect("   Материалы:", TerminalEffect::Normal);
            
            for (material, amount) in &recipe.required_materials {
                let available = *self.player_materials.get(material).unwrap_or(&0);
                let color = if available >= *amount { TerminalEffect::Success } else { TerminalEffect::Error };
                terminal.print_with_effect(&format!("     • {} x{} (у вас: {})", material, amount, available), color);
            }
            
            if let Some(station) = &recipe.required_station {
                terminal.print_with_effect(&format!("   Станция: {}", station), TerminalEffect::Warning);
            }
            
            terminal.print_with_effect("", TerminalEffect::Normal);
        }
    }
    
    pub fn display_materials(&self, terminal: &Terminal) {
        terminal.print_with_effect("═══ МАТЕРИАЛЫ ═══", TerminalEffect::Matrix);
        
        if self.player_materials.is_empty() {
            terminal.print_with_effect("Материалов нет", TerminalEffect::Warning);
        } else {
            for (material, amount) in &self.player_materials {
                terminal.print_with_effect(&format!("• {} x{}", material, amount), TerminalEffect::Normal);
            }
        }
    }
}

// ============================================================================
// РАСШИРЕННАЯ СИСТЕМА ЛОКАЦИЙ И ИССЛЕДОВАНИЯ
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub locations: HashMap<String, Location>,
    pub current_location: String,
    pub discovered_locations: HashSet<String>,
    pub travel_history: Vec<String>,
    pub fast_travel_points: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub location_type: LocationType,
    pub connected_locations: Vec<String>,
    pub available_services: Vec<LocationService>,
    pub npcs: Vec<String>,
    pub loot_spawns: Vec<LootSpawn>,
    pub security_level: u32,
    pub discovery_requirements: Vec<String>,
    pub special_events: Vec<String>,
    pub background_music: String,
    pub environment_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    SafeHouse,
    CorporateBuilding,
    UndergroundTunnel,
    AbandonedWarehouse,
    GovernmentFacility,
    ResearchLab,
    DataCenter,
    BlackMarket,
    CyberCafe,
    Hospital,
    University,
    Airport,
    NuclearFacility,
    Space,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationService {
    pub service_type: ServiceType,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub availability_condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Shop,
    Upgrade,
    Healing,
    Information,
    Training,
    Hacking,
    Storage,
    Transportation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootSpawn {
    pub item_name: String,
    pub spawn_chance: f64,
    pub respawn_time: Duration,
    pub last_collected: Option<String>,
}

use std::collections::HashSet;

impl WorldMap {
    pub fn new() -> Self {
        let mut world_map = WorldMap {
            locations: HashMap::new(),
            current_location: "player_hideout".to_string(),
            discovered_locations: HashSet::new(),
            travel_history: Vec::new(),
            fast_travel_points: HashSet::new(),
        };
        
        world_map.create_world_locations();
        world_map.discovered_locations.insert("player_hideout".to_string());
        world_map.fast_travel_points.insert("player_hideout".to_string());
        
        world_map
    }
    
    fn create_world_locations(&mut self) {
        let locations = vec![
            // Основные локации
            self.create_player_hideout(),
            self.create_nexus_headquarters(),
            self.create_underground_market(),
            self.create_cyber_freedom_base(),
            self.create_government_facility(),
            self.create_research_lab(),
            self.create_nuclear_facility(),
            self.create_data_center(),
            self.create_abandoned_warehouse(),
            self.create_cyber_cafe(),
            self.create_university(),
            self.create_hospital(),
            self.create_airport(),
            
            // Дополнительные локации
            self.create_corporate_district(),
            self.create_slums(),
            self.create_industrial_zone(),
            self.create_tech_hub(),
            self.create_financial_center(),
            self.create_media_center(),
            self.create_space_station(),
            self.create_deep_web(),
            self.create_virtual_reality(),
        ];
        
        for location in locations {
            self.locations.insert(location.name.clone(), location);
        }
    }
    
    fn create_player_hideout(&self) -> Location {
        Location {
            name: "player_hideout".to_string(),
            description: "Ваше секретное убежище в заброшенном здании. Здесь вы можете планировать операции, улучшать оборудование и отдыхать.".to_string(),
            location_type: LocationType::SafeHouse,
            connected_locations: vec!["underground_market".to_string(), "cyber_cafe".to_string(), "slums".to_string()],
            available_services: vec![
                LocationService {
                    service_type: ServiceType::Healing,
                    name: "Отдых".to_string(),
                    description: "Восстановить здоровье и снизить стресс".to_string(),
                    cost: 0,
                    availability_condition: "always".to_string(),
                },
                LocationService {
                    service_type: ServiceType::Storage,
                    name: "Сейф".to_string(),
                    description: "Сохранить предметы и деньги".to_string(),
                    cost: 0,
                    availability_condition: "always".to_string(),
                },
                LocationService {
                    service_type: ServiceType::Upgrade,
                    name: "Верстак".to_string(),
                    description: "Улучшить оборудование".to_string(),
                    cost: 0,
                    availability_condition: "always".to_string(),
                },
            ],
            npcs: vec!["AI_Assistant".to_string()],
            loot_spawns: vec![
                LootSpawn {
                    item_name: "Energy Drink".to_string(),
                    spawn_chance: 0.3,
                    respawn_