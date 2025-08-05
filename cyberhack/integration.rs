// ============================================================================
// ИНТЕГРАЦИЯ ОТСЫЛОК В ОСНОВНУЮ ИГРУ
// ============================================================================

use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;

// Добавляем новые команды с отсылками в CommandProcessor
pub fn add_reference_commands(processor: &mut CommandProcessor) {
    // Команды в стиле Hacknet
    processor.commands.insert("netmap".to_string(), Command {
        name: "netmap".to_string(),
        description: "Сканирование сетевой топологии (Hacknet style)".to_string(),
        usage: "netmap <target_ip>".to_string(),
        requires_target: true,
        requires_connection: false,
    });
    
    processor.commands.insert("portscan".to_string(), Command {
        name: "portscan".to_string(),
        description: "Сканирование портов (Hacknet style)".to_string(),
        usage: "portscan <target_ip> [port_range]".to_string(),
        requires_target: true,
        requires_connection: false,
    });
    
    processor.commands.insert("sshcrack".to_string(), Command {
        name: "sshcrack".to_string(),
        description: "Взлом SSH (Hacknet style)".to_string(),
        usage: "sshcrack <target_ip>".to_string(),
        requires_target: true,
        requires_connection: false,
    });
    
    processor.commands.insert("ftpbounce".to_string(), Command {
        name: "ftpbounce".to_string(),
        description: "FTP bounce атака (Hacknet style)".to_string(),
        usage: "ftpbounce <target_ip>".to_string(),
        requires_target: true,
        requires_connection: false,
    });
    
    processor.commands.insert("decypher".to_string(), Command {
        name: "decypher".to_string(),
        description: "Расшифровка файлов (Hacknet style)".to_string(),
        usage: "decypher <filename>".to_string(),
        requires_target: true,
        requires_connection: true,
    });
    
    // Команды из Mr. Robot
    processor.commands.insert("fsociety".to_string(), Command {
        name: "fsociety".to_string(),
        description: "Инструменты fsociety".to_string(),
        usage: "fsociety [operation]".to_string(),
        requires_target: false,
        requires_connection: false,
    });
    
    // Команды из Fight Club
    processor.commands.insert("mayhem".to_string(), Command {
        name: "mayhem".to_string(),
        description: "Project Mayhem toolkit".to_string(),
        usage: "mayhem <target> <operation>".to_string(),
        requires_target: true,
        requires_connection: false,
    });
    
    // Команды из Hackers
    processor.commands.insert("gibson".to_string(), Command {
        name: "gibson".to_string(),
        description: "Доступ к Gibson supercomputer".to_string(),
        usage: "gibson <command>".to_string(),
        requires_target: false,
        requires_connection: false,
    });
    
    // Специальные команды для пасхалок
    processor.commands.insert("hack_the_planet".to_string(), Command {
        name: "hack_the_planet".to_string(),
        description: "HACK THE PLANET! (Hackers reference)".to_string(),
        usage: "hack_the_planet".to_string(),
        requires_target: false,
        requires_connection: false,
    });
}

// Добавляем обработчики команд с отсылками
pub fn handle_reference_command(command: &str, args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    match command {
        "netmap" => handle_netmap_command(args, state, terminal),
        "portscan" => handle_portscan_command(args, state, terminal),
        "sshcrack" => handle_sshcrack_command(args, state, terminal),
        "ftpbounce" => handle_ftpbounce_command(args, state, terminal),
        "decypher" => handle_decypher_command(args, state, terminal),
        "fsociety" => handle_fsociety_command(args, state, terminal),
        "mayhem" => handle_mayhem_command(args, state, terminal),
        "gibson" => handle_gibson_command(args, state, terminal),
        "hack_the_planet" => handle_hack_the_planet_command(state, terminal),
        _ => false,
    }
}

fn handle_netmap_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    if args.is_empty() {
        terminal.print_with_effect("Использование: netmap <target_ip>", TerminalEffect::Error);
        return true;
    }
    
    let target = args[0];
    terminal.print_with_effect(&format!("🔍 Сканирование сети {} в стиле Hacknet...", target), TerminalEffect::Matrix);
    terminal.loading_animation("Анализ сетевой топологии", Duration::from_secs(2));
    
    if target == "localhost" {
        // Пасхалка!
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🎉 HACKNET TERMINAL ACTIVATED! 🎉", TerminalEffect::Success);
        terminal.print_with_effect("Добро пожаловать в настоящий Hacknet терминал!", TerminalEffect::Matrix);
        terminal.print_with_effect("Доступны все инструменты Hacknet!", TerminalEffect::Success);
        terminal.print_with_effect("🥚 Пасхалка: Hacknet терминал найден!", TerminalEffect::Success);
        return true;
    }
    
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.7) {
        terminal.print_with_effect("═══ NETMAP RESULTS ═══", TerminalEffect::Matrix);
        terminal.print_with_effect(&format!("Цель: {}", target), TerminalEffect::Normal);
        terminal.print_with_effect("Обнаруженные узлы:", TerminalEffect::Normal);
        
        let nodes = vec![
            format!("{}.1 - Router", target.split('.').take(3).collect::<Vec<_>>().join(".")),
            format!("{}.10 - Workstation", target.split('.').take(3).collect::<Vec<_>>().join(".")),
            format!("{}.50 - Server", target.split('.').take(3).collect::<Vec<_>>().join(".")),
            format!("{}.100 - Database", target.split('.').take(3).collect::<Vec<_>>().join(".")),
        ];
        
        for node in &nodes {
            terminal.print_with_effect(&format!("  {}", node), TerminalEffect::Success);
        }
        
        state.player.experience += 30;
        terminal.print_with_effect("+30 XP за сетевое сканирование", TerminalEffect::Success);
    } else {
        terminal.print_with_effect("❌ Цель недоступна или защищена файрволом", TerminalEffect::Error);
    }
    
    true
}

fn handle_portscan_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    if args.is_empty() {
        terminal.print_with_effect("Использование: portscan <target_ip> [port_range]", TerminalEffect::Error);
        return true;
    }
    
    let target = args[0];
    let port_range = if args.len() > 1 { args[1] } else { "1-1000" };
    
    terminal.print_with_effect(&format!("🔍 Сканирование портов {} диапазон {}...", target, port_range), TerminalEffect::Matrix);
    terminal.loading_animation("Проверка портов", Duration::from_secs(3));
    
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.8) {
        terminal.print_with_effect("═══ PORT SCAN RESULTS ═══", TerminalEffect::Matrix);
        
        let open_ports = vec![
            ("22", "SSH", "🟢"),
            ("80", "HTTP", "🟢"),
            ("443", "HTTPS", "🟢"),
            ("21", "FTP", "🟡"),
            ("23", "Telnet", "🔴"),
            ("25", "SMTP", "🟢"),
            ("53", "DNS", "🟢"),
            ("3389", "RDP", "🔴"),
        ];
        
        for (port, service, status) in &open_ports[0..rng.gen_range(3..open_ports.len())] {
            terminal.print_with_effect(&format!("  {}/tcp OPEN - {} {}", port, service, status), TerminalEffect::Success);
        }
        
        state.player.experience += 25;
        terminal.print_with_effect("+25 XP за сканирование портов (Hacknet style)", TerminalEffect::Success);
    } else {
        terminal.print_with_effect("❌ Хост недоступен или все порты закрыты", TerminalEffect::Error);
    }
    
    true
}

fn handle_sshcrack_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    if args.is_empty() {
        terminal.print_with_effect("Использование: sshcrack <target_ip>", TerminalEffect::Error);
        return true;
    }
    
    let target = args[0];
    terminal.print_with_effect(&format!("🔐 SSH Crack на {} (Hacknet style)...", target), TerminalEffect::Matrix);
    terminal.loading_animation("Брутфорс SSH-ключей", Duration::from_secs(4));
    
    let mut rng = rand::thread_rng();
    let hacking_skill = *state.skills.get("Взлом").unwrap_or(&0);
    let success_chance = (hacking_skill as f64 / 100.0) * 0.6 + 0.2;
    
    if rng.gen_bool(success_chance) {
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🎯 SSH CRACKED SUCCESSFULLY!", TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("Connection established to remote host", TerminalEffect::Success);
        terminal.print_with_effect("Remote OS: NAIX 2.4.1", TerminalEffect::Normal);
        terminal.print_with_effect("User: hacknet_user", TerminalEffect::Normal);
        terminal.print_with_effect("Access Level: Standard", TerminalEffect::Normal);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🥚 Отсылка к Hacknet: SSH взлом!", TerminalEffect::Matrix);
        
        state.player.current_system = Some(target.to_string());
        state.player.experience += 100;
        terminal.print_with_effect("+100 XP за SSH взлом", TerminalEffect::Success);
    } else {
        terminal.print_with_effect("❌ SSH crack failed", TerminalEffect::Error);
        terminal.print_with_effect("Target appears to have updated security protocols", TerminalEffect::Warning);
    }
    
    true
}

fn handle_ftpbounce_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    if args.is_empty() {
        terminal.print_with_effect("Использование: ftpbounce <target_ip>", TerminalEffect::Error);
        return true;
    }
    
    let target = args[0];
    terminal.print_with_effect(&format!("📁 FTP Bounce атака на {} (Hacknet style)...", target), TerminalEffect::Matrix);
    terminal.loading_animation("Эксплуатация FTP сервиса", Duration::from_secs(3));
    
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.6) {
        terminal.print_with_effect("✅ FTP BOUNCE SUCCESSFUL!", TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("FTP server compromised", TerminalEffect::Success);
        terminal.print_with_effect("Available directories:", TerminalEffect::Normal);
        terminal.print_with_effect("  /home/ftp/public/", TerminalEffect::Normal);
        terminal.print_with_effect("  /var/log/", TerminalEffect::Normal);
        terminal.print_with_effect("  /tmp/uploads/", TerminalEffect::Normal);
        terminal.print_with_effect("  /etc/passwd", TerminalEffect::Warning);
        
        state.player.experience += 75;
        terminal.print_with_effect("+75 XP за FTP эксплуатацию", TerminalEffect::Success);
    } else {
        terminal.print_with_effect("❌ FTP bounce failed", TerminalEffect::Error);
        terminal.print_with_effect("Server has patched bounce vulnerabilities", TerminalEffect::Warning);
    }
    
    true
}

fn handle_decypher_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    if args.is_empty() {
        terminal.print_with_effect("Использование: decypher <filename>", TerminalEffect::Error);
        return true;
    }
    
    if state.player.current_system.is_none() {
        terminal.print_with_effect("❌ No active connection to remote system", TerminalEffect::Error);
        return true;
    }
    
    let filename = args[0];
    terminal.print_with_effect(&format!("🔓 DECypher запущен для {}...", filename), TerminalEffect::Matrix);
    terminal.loading_animation("Analyzing encryption algorithms", Duration::from_secs(5));
    
    let mut rng = rand::thread_rng();
    let crypto_skill = *state.skills.get("Криптография").unwrap_or(&0);
    let success_chance = (crypto_skill as f64 / 100.0) * 0.7 + 0.3;
    
    if rng.gen_bool(success_chance) {
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🔐 DECRYPTION SUCCESSFUL!", TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        
        match filename {
            "entropy.dat" => {
                terminal.print_with_effect("=== CLASSIFIED ENTROPY DATA ===", TerminalEffect::Error);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("ENTROPY ORGANIZATION - LEVEL OMEGA", TerminalEffect::Error);
                terminal.print_with_effect("Objective: Maximum entropy in digital systems", TerminalEffect::Warning);
                terminal.print_with_effect("Method: Hacknet protocol distribution", TerminalEffect::Normal);
                terminal.print_with_effect("Status: ACTIVE", TerminalEffect::Error);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("🥚 Найдена отсылка к Hacknet: Entropy!", TerminalEffect::Matrix);
            },
            "naix.sys" => {
                terminal.print_with_effect("NAIX Operating System Core", TerminalEffect::Matrix);
                terminal.print_with_effect("Built for hackers, by hackers", TerminalEffect::Success);
                terminal.print_with_effect("🥚 NAIX OS из Hacknet обнаружена!", TerminalEffect::Success);
            },
            _ => {
                terminal.print_with_effect("File contents:", TerminalEffect::Normal);
                terminal.print_with_effect("Decrypted data reveals system vulnerabilities", TerminalEffect::Success);
                terminal.print_with_effect("Hacknet-style encryption broken", TerminalEffect::Matrix);
            }
        }
        
        state.player.experience += 120;
        terminal.print_with_effect("+120 XP за успешную расшифровку", TerminalEffect::Success);
    } else {
        terminal.print_with_effect("❌ Decryption failed", TerminalEffect::Error);
        terminal.print_with_effect("Encryption algorithm too advanced", TerminalEffect::Warning);
        terminal.print_with_effect("Require higher cryptography skill", TerminalEffect::Normal);
    }
    
    true
}

fn handle_fsociety_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("👾 ═══════════════════════════════════════ 👾", TerminalEffect::Matrix);
    terminal.print_with_effect("                FSOCIETY TOOLKIT              ", TerminalEffect::Matrix);
    terminal.print_with_effect("👾 ═══════════════════════════════════════ 👾", TerminalEffect::Matrix);
    terminal.print_with_effect("", TerminalEffect::Normal);
    
    if args.is_empty() {
        terminal.print_with_effect("Hello, friend.", TerminalEffect::TypeWriter);
        thread::sleep(Duration::from_millis(500));
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🎭 Добро пожаловать в fsociety", TerminalEffect::Matrix);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("Доступные операции:", TerminalEffect::Normal);
        terminal.print_with_effect("  fsociety debt_erase    - Операция 5/9: Стереть долги", TerminalEffect::Success);
        terminal.print_with_effect("  fsociety evil_corp     - Атака на Evil Corp", TerminalEffect::Warning);
        terminal.print_with_effect("  fsociety manifest      - Показать манифест", TerminalEffect::Normal);
        terminal.print_with_effect("  fsociety mr_robot      - Связаться с Mr. Robot", TerminalEffect::Error);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("Мы fsociety.", TerminalEffect::Matrix);
        terminal.print_with_effect("Мы анонимы.", TerminalEffect::Matrix);
        terminal.print_with_effect("Мы легион.", TerminalEffect::Matrix);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🥚 Отсылка к Mr. Robot найдена!", TerminalEffect::Success);
        return true;
    }
    
    match args[0] {
        "debt_erase" => {
            terminal.print_with_effect("💣 Запуск операции 5/9...", TerminalEffect::Error);
            terminal.loading_animation("Infiltrating E Corp servers", Duration::from_secs(4));
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🎯 TARGET ACQUIRED: E Corp Financial Systems", TerminalEffect::Warning);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("💥 DEBT RECORDS ERASED!", TerminalEffect::Success);
            terminal.print_with_effect("💰 Millions of people freed from debt!", TerminalEffect::Success);
            terminal.print_with_effect("🏦 Banking system in chaos!", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("Operation 5/9 complete. Society is reset.", TerminalEffect::Matrix);
            
            state.player.experience += 500;
            state.reputation += 50;
            terminal.print_with_effect("+500 XP за операцию 5/9!", TerminalEffect::Success);
        },
        "evil_corp" => {
            terminal.print_with_effect("🎯 Targeting Evil Corp infrastructure...", TerminalEffect::Warning);
            terminal.loading_animation("Deploying attack vectors", Duration::from_secs(3));
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("💀 Evil Corp под массированной атакой!", TerminalEffect::Error);
            terminal.print_with_effect("📊 Их системы мониторинга парализованы!", TerminalEffect::Success);
            terminal.print_with_effect("🏢 Корпоративная империя дает сбои!", TerminalEffect::Warning);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("\"Fuck society.\" - fsociety", TerminalEffect::Matrix);
            
            state.player.experience += 300;
            state.reputation += 25;
        },
        "manifest" => {
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("📜 ═══ МАНИФЕСТ FSOCIETY ═══ 📜", TerminalEffect::Matrix);
            terminal.print_with_effect("", TerminalEffect::Normal);
            
            let manifest = vec![
                "Общество сломано.",
                "Корпорации контролируют каждый аспект нашей жизни.",
                "Правительства - марионетки в руках 1% элиты.",
                "Деньги стали оружием массового порабощения.",
                "",
                "Но у нас есть сила.",
                "Мы - код в их матрице.",
                "Мы - вирус в их системе.",
                "Мы - сбой в их программе контроля.",
                "",
                "Fsociety - это не просто группа хакеров.",
                "Мы - революция.",
                "Мы меняем мир, по одному хаку за раз.",
                "",
                "Hello, friend.",
                "Присоединяйся к нам.",
                "Вместе мы можем все изменить.",
            ];
            
            for line in manifest {
                terminal.print_with_effect(line, TerminalEffect::TypeWriter);
                thread::sleep(Duration::from_millis(300));
            }
            
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("- Mr. Robot & fsociety collective", TerminalEffect::Matrix);
        },
        "mr_robot" => {
            terminal.print_with_effect("📞 Establishing secure connection...", TerminalEffect::Matrix);
            terminal.loading_animation("Connecting to Mr. Robot", Duration::from_secs(3));
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🎭 Mr. Robot: Ты готов к следующему этапу?", TerminalEffect::Error);
            terminal.print_with_effect("🎭 Mr. Robot: Настоящая работа только начинается.", TerminalEffect::Warning);
            terminal.print_with_effect("🎭 Mr. Robot: Помни - мы делаем это не для себя.", TerminalEffect::Normal);
            terminal.print_with_effect("🎭 Mr. Robot: Мы делаем это для всех остальных.", TerminalEffect::Success);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("Connection terminated by remote host.", TerminalEffect::Error);
        },
        _ => {
            terminal.print_with_effect("❌ Неизвестная операция fsociety", TerminalEffect::Error);
            terminal.print_with_effect("Доступные операции: debt_erase, evil_corp, manifest, mr_robot", TerminalEffect::Normal);
        }
    }
    
    true
}

fn handle_mayhem_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    if args.len() < 2 {
        terminal.print_with_effect("Использование: mayhem <target> <operation>", TerminalEffect::Error);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("⚠️  ПЕРВОЕ ПРАВИЛО PROJECT MAYHEM:", TerminalEffect::Warning);
        terminal.print_with_effect("    Никому не рассказывать о Project Mayhem!", TerminalEffect::Error);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("⚠️  ВТОРОЕ ПРАВИЛО PROJECT MAYHEM:", TerminalEffect::Warning);
        terminal.print_with_effect("    НИКОМУ НЕ РАССКАЗЫВАТЬ О PROJECT MAYHEM!", TerminalEffect::Error);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🥚 Fight Club reference detected!", TerminalEffect::Success);
        return true;
    }
    
    let target = args[0];
    let operation = args[1];
    
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("💀 ═══════════════════════════════════════ 💀", TerminalEffect::Error);
    terminal.print_with_effect("                PROJECT MAYHEM                ", TerminalEffect::Error);
    terminal.print_with_effect("💀 ═══════════════════════════════════════ 💀", TerminalEffect::Error);
    terminal.print_with_effect("", TerminalEffect::Normal);
    
    match operation {
        "destroy" => {
            terminal.print_with_effect(&format!("🧨 Targeting for destruction: {}", target), TerminalEffect::Error);
            terminal.loading_animation("Preparing chaos protocols", Duration::from_secs(3));
            terminal.print_with_effect("", TerminalEffect::Normal);
            
            if target.contains("credit") || target.contains("bank") || target.contains("financial") {
                terminal.print_with_effect("🏦 FINANCIAL TARGET ACQUIRED", TerminalEffect::Warning);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("💥 CREDIT RECORDS DESTROYED!", TerminalEffect::Success);
                terminal.print_with_effect("💳 Credit card databases wiped!", TerminalEffect::Success);
                terminal.print_with_effect("💰 Debt records erased!", TerminalEffect::Success);
                terminal.print_with_effect("🏢 Corporate towers digitally demolished!", TerminalEffect::Error);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("\"We are a generation raised by corporations\"", TerminalEffect::Matrix);
                terminal.print_with_effect("\"Now we return the favor.\"", TerminalEffect::Matrix);
                
                state.player.experience += 400;
                state.reputation += 30;
            } else {
                terminal.print_with_effect("🎯 Target eliminated from digital existence", TerminalEffect::Success);
                terminal.print_with_effect("Corporate infrastructure compromised", TerminalEffect::Warning);
                state.player.experience += 200;
            }
        },
        "chaos" => {
            terminal.print_with_effect("⚡ Initiating controlled chaos...", TerminalEffect::Warning);
            terminal.loading_animation("Spreading digital anarchy", Duration::from_secs(4));
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🌪️ CHAOS PROTOCOLS ACTIVATED!", TerminalEffect::Error);
            terminal.print_with_effect("📺 Corporate media feeds corrupted!", TerminalEffect::Warning);
            terminal.print_with_effect("🏪 Consumer databases randomized!", TerminalEffect::Success);
            terminal.print_with_effect("💼 Executive communications intercepted!", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("\"The things you own end up owning you.\"", TerminalEffect::Matrix);
            terminal.print_with_effect("\"But not anymore.\"", TerminalEffect::Matrix);
            
            state.player.experience += 250;
            state.reputation += 20;
        },
        "fight_club" => {
            terminal.print_with_effect("🥊 Initiating digital fight club protocols...", TerminalEffect::Matrix);
            terminal.loading_animation("Setting up underground networks", Duration::from_secs(2));
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🥊 FIRST RULE OF FIGHT CLUB:", TerminalEffect::Error);
            terminal.print_with_effect("   You do not talk about fight club.", TerminalEffect::Warning);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🥊 SECOND RULE OF FIGHT CLUB:", TerminalEffect::Error);
            terminal.print_with_effect("   You DO NOT talk about fight club!", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("Underground hacker cells activated", TerminalEffect::Success);
            terminal.print_with_effect("Digital fight clubs established worldwide", TerminalEffect::Matrix);
            
            state.player.experience += 300;
        },
        _ => {
            terminal.print_with_effect("❌ Неизвестная операция Project Mayhem", TerminalEffect::Error);
            terminal.print_with_effect("Доступные операции: destroy, chaos, fight_club", TerminalEffect::Normal);
        }
    }
    
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("\"We are a generation of men raised by women.\"", TerminalEffect::Matrix);
    terminal.print_with_effect("\"I'm wondering if another woman is really the answer we need.\"", TerminalEffect::Matrix);
    terminal.print_with_effect("- Tyler Durden", TerminalEffect::Normal);
    
    true
}

fn handle_gibson_command(args: &[&str], state: &mut GameState, terminal: &Terminal) -> bool {
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("🖥️ ═══════════════════════════════════════ 🖥️", TerminalEffect::Matrix);
    terminal.print_with_effect("           GIBSON SUPERCOMPUTER               ", TerminalEffect::Matrix);
    terminal.print_with_effect("🖥️ ═══════════════════════════════════════ 🖥️", TerminalEffect::Matrix);
    terminal.print_with_effect("", TerminalEffect::Normal);
    
    if args.is_empty() {
        terminal.print_with_effect("Welcome to the Gibson!", TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🎮 Доступные команды Gibson:", TerminalEffect::Normal);
        terminal.print_with_effect("  gibson status      - Статус суперкомпьютера", TerminalEffect::Normal);
        terminal.print_with_effect("  gibson elite       - Проверка элитного статуса", TerminalEffect::Warning);
        terminal.print_with_effect("  gibson virus       - Сканирование Da Vinci вируса", TerminalEffect::Error);
        terminal.print_with_effect("  gibson zero_cool   - Профиль Zero Cool", TerminalEffect::Success);
        terminal.print_with_effect("  gibson acid_burn   - Профиль Acid Burn", TerminalEffect::Matrix);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🥚 Gibson из фильма 'Hackers' (1995)!", TerminalEffect::Success);
        return true;
    }
    
    match args[0] {
        "status" => {
            terminal.print_with_effect("═══ GIBSON SYSTEM STATUS ═══", TerminalEffect::Matrix);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🖥️  CPU: 9000 MIPS Superscalar Processor", TerminalEffect::Success);
            terminal.print_with_effect("💾 RAM: Unlimited Virtual Memory", TerminalEffect::Success);
            terminal.print_with_effect("🔒 Security Level: MAXIMUM", TerminalEffect::Warning);
            terminal.print_with_effect("🌐 Network Status: ONLINE", TerminalEffect::Success);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("⚠️  ACTIVE THREATS DETECTED:", TerminalEffect::Error);
            terminal.print_with_effect("   - Zero Cool: Multiple intrusion attempts", TerminalEffect::Warning);
            terminal.print_with_effect("   - Acid Burn: Advanced probing detected", TerminalEffect::Warning);
            terminal.print_with_effect("   - The Plague: Administrative access ACTIVE", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🎯 Threat Level: ELITE HACKERS", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("\"Hack the Planet!\" - graffiti found in system logs", TerminalEffect::Matrix);
        },
        "elite" => {
            let hacking_skill = *state.skills.get("Взлом").unwrap_or(&0);
            terminal.print_with_effect("🔍 Проверка элитного статуса хакера...", TerminalEffect::Matrix);
            terminal.loading_animation("Analyzing hacking prowess", Duration::from_secs(2));
            terminal.print_with_effect("", TerminalEffect::Normal);
            
            if hacking_skill >= 80 {
                terminal.print_with_effect("🏆 ═══ ELITE HACKER CONFIRMED! ═══ 🏆", TerminalEffect::Success);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("🎉 Welcome to the elite, hacker!", TerminalEffect::Matrix);
                terminal.print_with_effect("🌍 You have earned the right to HACK THE PLANET!", TerminalEffect::Success);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("\"They may take our lives,\"", TerminalEffect::TypeWriter);
                terminal.print_with_effect("\"but they'll never take our FREEDOM!\"", TerminalEffect::TypeWriter);
                terminal.print_with_effect("- Zero Cool", TerminalEffect::Normal);
                
                state.player.experience += 1000;
                state.reputation += 100;
                terminal.print_with_effect("+1000 XP за элитный статус!", TerminalEffect::Success);
                terminal.print_with_effect("+100 репутации среди хакеров!", TerminalEffect::Success);
            } else {
                terminal.print_with_effect("❌ ELITE STATUS: DENIED", TerminalEffect::Error);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("Current skill level insufficient", TerminalEffect::Warning);
                terminal.print_with_effect("Required: 80+ Hacking skill", TerminalEffect::Normal);
                terminal.print_with_effect(&format!("Your level: {}", hacking_skill), TerminalEffect::Normal);
                terminal.print_with_effect("", TerminalEffect::Normal);
                terminal.print_with_effect("\"Keep practicing, script kiddie.\"", TerminalEffect::Error);
                terminal.print_with_effect("- The Gibson", TerminalEffect::Normal);
            }
        },
        "virus" => {
            terminal.print_with_effect("🦠 Scanning for Da Vinci virus...", TerminalEffect::Warning);
            terminal.loading_animation("Deep system analysis", Duration::from_secs(3));
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("⚠️ ═══ VIRUS DETECTED! ═══ ⚠️", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🦠 Virus Name: Da Vinci", TerminalEffect::Error);
            terminal.print_with_effect("👤 Author: The Plague (Eugene Belford)", TerminalEffect::Warning);
            terminal.print_with_effect("🎯 Target: Ellingson Mineral Company", TerminalEffect::Normal);
            terminal.print_with_effect("💰 Function: Financial theft & frame-up", TerminalEffect::Error);
            terminal.print_with_effect("📊 Status: ACTIVE and stealing money!", TerminalEffect::Error);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🔍 Virus Code Sample:", TerminalEffect::Matrix);
            terminal.print_with_effect("    if (account_balance > 0) {", TerminalEffect::Normal);
            terminal.print_with_effect("        steal_money(balance * 0.10);", TerminalEffect::Error);
            terminal.print_with_effect("        frame_hacker('Zero Cool');", TerminalEffect::Warning);
            terminal.print_with_effect("    }", TerminalEffect::Normal);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("\"Welcome to the real world!\" - The Plague", TerminalEffect::Error);
        },
        "zero_cool" => {
            terminal.print_with_effect("👤 ═══ HACKER PROFILE: ZERO COOL ═══", TerminalEffect::Success);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🏷️  Real Name: Dade Murphy", TerminalEffect::Normal);
            terminal.print_with_effect("🎮 Handle: Zero Cool", TerminalEffect::Success);
            terminal.print_with_effect("📅 Age: 18", TerminalEffect::Normal);
            terminal.print_with_effect("🏫 School: Stuyvesant High School", TerminalEffect::Normal);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🎯 Notable Achievements:", TerminalEffect::Matrix);
            terminal.print_with_effect("   - Crashed 1,507 systems at age 11", TerminalEffect::Warning);
            terminal.print_with_effect("   - Caused 7-point drop in NYSE", TerminalEffect::Error);
            terminal.print_with_effect("   - Gibson infiltration attempts: 23", TerminalEffect::Normal);
            terminal.print_with_effect("   - Elite hacker status: CONFIRMED", TerminalEffect::Success);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("💬 Famous Quote:", TerminalEffect::Matrix);
            terminal.print_with_effect("   \"Hack the Planet!\"", TerminalEffect::Success);
        },
        "acid_burn" => {
            terminal.print_with_effect("👤 ═══ HACKER PROFILE: ACID BURN ═══", TerminalEffect::Matrix);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🏷️  Real Name: Kate Libby", TerminalEffect::Normal);
            terminal.print_with_effect("🎮 Handle: Acid Burn", TerminalEffect::Matrix);
            terminal.print_with_effect("📅 Age: 18", TerminalEffect::Normal);
            terminal.print_with_effect("🏫 School: Stuyvesant High School", TerminalEffect::Normal);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🎯 Skills Assessment:", TerminalEffect::Matrix);
            terminal.print_with_effect("   - Hacking Level: ELITE", TerminalEffect::Success);
            terminal.print_with_effect("   - Gibson Access: UNAUTHORIZED", TerminalEffect::Warning);
            terminal.print_with_effect("   - Competition vs Zero Cool: INTENSE", TerminalEffect::Error);
            terminal.print_with_effect("   - Leadership: NATURAL", TerminalEffect::Success);
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("💬 Assessment:", TerminalEffect::Matrix);
            terminal.print_with_effect("   \"Formidable opponent. Approach with caution.\"", TerminalEffect::Warning);
        },
        _ => {
            terminal.print_with_effect("❌ Неизвестная команда Gibson", TerminalEffect::Error);
            terminal.print_with_effect("Доступные команды: status, elite, virus, zero_cool, acid_burn", TerminalEffect::Normal);
        }
    }
    
    true
}

fn handle_hack_the_planet_command(state: &mut GameState, terminal: &Terminal) -> bool {
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("🌍 ═══════════════════════════════════════════════════════════ 🌍", TerminalEffect::Success);
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("          🚀 🚀 🚀 HACK THE PLANET! 🚀 🚀 🚀", TerminalEffect::Matrix);
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("🌍 ═══════════════════════════════════════════════════════════ 🌍", TerminalEffect::Success);
    terminal.print_with_effect("", TerminalEffect::Normal);
    
    // Анимация хакинга планеты
    let messages = vec![
        "🌍 Connecting to global networks...",
        "🛰️ Hijacking satellite communications...", 
        "🏦 Infiltrating financial systems...",
        "🏛️ Breaching government databases...",
        "🏢 Compromising corporate mainframes...",
        "📡 Taking control of internet backbone...",
        "🌐 Planet-wide network access achieved!",
    ];
    
    for message in messages {
        terminal.print_with_effect(message, TerminalEffect::TypeWriter);
        thread::sleep(Duration::from_millis(800));
    }
    
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("🎉 ═══ PLANETARY HACK COMPLETE! ═══ 🎉", TerminalEffect::Success);
    terminal.print_with_effect("", TerminalEffect::Normal);
    
    // Цитаты из фильма Hackers
    let quotes = vec![
        "\"They may take our lives, but they'll never take our FREEDOM!\"",
        "\"This is our world now... the world of the electron and the switch.\"",
        "\"We exist without skin color, without nationality, without religious bias.\"",
        "\"You call us criminals. We are the unwanted.\"",
        "\"Yes, I am a criminal. My crime is that of curiosity.\"",
        "\"We make use of a service already existing without paying for what could be dirt-cheap if it wasn't run by profiteering gluttons.\"",
    ];
    
    for quote in &quotes {
        terminal.print_with_effect(quote, TerminalEffect::TypeWriter);
        thread::sleep(Duration::from_millis(600));
    }
    
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("- Zero Cool & The Elite Hackers", TerminalEffect::Normal);
    terminal.print_with_effect("", TerminalEffect::Normal);
    
    terminal.print_with_effect("🏆 LEGENDARY COMMAND ACTIVATED!", TerminalEffect::Success);
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("🔥 Вы почувствовали силу всех элитных хакеров!", TerminalEffect::Matrix);
    terminal.print_with_effect("⚡ Все навыки временно удваиваются!", TerminalEffect::Success);
    terminal.print_with_effect("🌟 Элитный статус подтвержден!", TerminalEffect::Success);
    
    // Временный буст всех навыков
    for (skill, value) in state.skills.iter_mut() {
        *value = (*value * 2).min(100);
    }
    
    state.player.experience += 2000;
    state.reputation += 200;
    
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("+2000 XP за HACK THE PLANET!", TerminalEffect::Success);
    terminal.print_with_effect("+200 репутации среди хакеров!", TerminalEffect::Success);
    terminal.print_with_effect("", TerminalEffect::Normal);
    terminal.print_with_effect("🥚 МЕГА ПАСХАЛКА: Культовая фраза из Hackers (1995)!", TerminalEffect::Success);
    
    true
}

// Добавляем сети с отсылками
pub fn add_reference_networks(networks: &mut HashMap<String, Network>) {
    // Сеть Evil Corp (Mr. Robot)
    let evil_corp_system = create_evil_corp_system();
    let mut evil_corp_network = Network {
        name: "Evil Corp Network".to_string(),
        security_level: 9,
        systems: HashMap::new(),
        firewall_strength: 95,
        intrusion_detection: true,
        is_compromised: false,
    };
    evil_corp_network.systems.insert("192.168.666.1".to_string(), evil_corp_system);
    networks.insert("192.168.666.1".to_string(), evil_corp_network);
    
    // Сеть fsociety
    let fsociety_system = create_fsociety_system();
    let mut fsociety_network = Network {
        name: "fsociety Network".to_string(),
        security_level: 3,
        systems: HashMap::new(),
        firewall_strength: 60,
        intrusion_detection: false,
        is_compromised: false,
    };
    fsociety_network.systems.insert("10.0.0.1".to_string(), fsociety_system);
    networks.insert("10.0.0.1".to_string(), fsociety_network);
    
    // Сеть Project Mayhem (Fight Club)
    let mayhem_system = create_mayhem_system();
    let mut mayhem_network = Network {
        name: "Project Mayhem Network".to_string(),
        security_level: 7,
        systems: HashMap::new(),
        firewall_strength: 80,
        intrusion_detection: false,
        is_compromised: false,
    };
    mayhem_network.systems.insert("172.16.0.99".to_string(), mayhem_system);
    networks.insert("172.16.0.99".to_string(), mayhem_network);
    
    // Gibson Supercomputer (Hackers)
    let gibson_system = create_gibson_system();
    let mut gibson_network = Network {
        name: "Gibson Supercomputer Network".to_string(),
        security_level: 10,
        systems: HashMap::new(),
        firewall_strength: 100,
        intrusion_detection: true,
        is_compromised: false,
    };
    gibson_network.systems.insert("198.51.100.1".to_string(), gibson_system);
    networks.insert("198.51.100.1".to_string(), gibson_network);
    
    // Hacknet Node
    let hacknet_system = create_hacknet_system();
    let mut hacknet_network = Network {
        name: "Hacknet Node Network".to_string(),
        security_level: 6,
        systems: HashMap::new(),
        firewall_strength: 75,
        intrusion_detection: true,
        is_compromised: false,
    };
    hacknet_network.systems.insert("203.0.113.42".to_string(), hacknet_system);
    networks.insert("203.0.113.42".to_string(), hacknet_network);
}

// Вспомогательные функции для создания систем
fn create_evil_corp_system() -> System {
    let mut system = System {
        name: "evil-corp-mainframe".to_string(),
        os: "EvilOS 3.1.4".to_string(),
        security_level: 9,
        files: HashMap::new(),
        services: Vec::new(),
        vulnerabilities: Vec::new(),
        is_compromised: false,
        admin_access: false,
    };
    
    system.files.insert("financial_records.db".to_string(), File {
        name: "financial_records.db".to_string(),
        content: "EVIL CORP FINANCIAL DATABASE\n\n💀 Долговые записи: 99.9% населения\n💰 Общий долг: $∞\n\n// TODO: Implement 5/9 debt erasure protection\n// Note: fsociety threat level CRITICAL\n\n[MEMO] Operation Dark Army approved by board\n[MEMO] White Rose meeting scheduled".to_string(),
        permissions: "root:root 600".to_string(),
        size: 2048576,
        encrypted: true,
        password: Some("hello_friend".to_string()),
    });
    
    system
}

fn create_fsociety_system() -> System {
    let mut system = System {
        name: "fsociety-server".to_string(),
        os: "NAIX 2.4.1".to_string(),
        security_level: 3,
        files: HashMap::new(),
        services: Vec::new(),
        vulnerabilities: Vec::new(),
        is_compromised: false,
        admin_access: false,
    };
    
    system.files.insert("hello_friend.txt".to_string(), File {
        name: "hello_friend.txt".to_string(),
        content: "Hello, friend.\n\nАвтор этого сообщения скрыт.\nТы здесь, потому что знаешь что-то не так.\nТы знаешь что-то не так с миром.\nТы не знаешь что именно, но это как заноза в мозгу.\nЭто сводит тебя с ума.\n\nЭто то, что привело тебя ко мне.\n\nМы fsociety.\nМы здесь, чтобы все изменить.\n\n- E".to_string(),
        permissions: "elliot:fsociety 644".to_string(),
        size: 512,
        encrypted: false,
        password: None,
    });
    
    system
}

fn create_mayhem_system() -> System {
    let mut system = System {
        name: "project-mayhem-ops".to_string(),
        os: "ChaosOS 1.999".to_string(),
        security_level: 7,
        files: HashMap::new(),
        services: Vec::new(),
        vulnerabilities: Vec::new(),
        is_compromised: false,
        admin_access: false,
    };
    
    system.files.insert("fight_club_rules.txt".to_string(), File {
        name: "fight_club_rules.txt".to_string(),
        content: "RULES OF FIGHT CLUB:\n\n1st RULE: You do not talk about FIGHT CLUB.\n2nd RULE: You DO NOT talk about FIGHT CLUB.\n3rd RULE: If someone says \"stop\" or goes limp, taps out the fight is over.\n4th RULE: Only two guys to a fight.\n5th RULE: One fight at a time.\n6th RULE: No shirts, no shoes.\n7th RULE: Fights will go on as long as they have to.\n8th RULE: If this is your first night at FIGHT CLUB, you HAVE to fight.\n\n- Tyler D.".to_string(),
        permissions: "tyler:mayhem 600".to_string(),
        size: 1024,
        encrypted: false,
        password: None,
    });
    
    system
}

fn create_gibson_system() -> System {
    let mut system = System {
        name: "gibson-supercomputer".to_string(),
        os: "Gibson OS 9000".to_string(),
        security_level: 10,
        files: HashMap::new(),
        services: Vec::new(),
        vulnerabilities: Vec::new(),
        is_compromised: false,
        admin_access: false,
    };
    
    system.files.insert("da_vinci_virus.exe".to_string(), File {
        name: "da_vinci_virus.exe".to_string(),
        content: "// Da Vinci Virus v2.1\n// Author: The Plague (Eugene Belford)\n// Target: Ellingson Mineral Company\n\nint main() {\n    while(true) {\n        if (account_balance > 0) {\n            steal_money(balance * 0.10);\n            frame_hacker(\"Zero Cool\");\n            plant_evidence();\n        }\n        sleep(1000);\n    }\n    return 0;\n}\n\n// \"Welcome to the real world!\" - The Plague".to_string(),
        permissions: "plague:hackers 755".to_string(),
        size: 8192,
        encrypted: false,
        password: None,
    });
    
    system
}

fn create_hacknet_system() -> System {
    let mut system = System {
        name: "entropy-node-001".to_string(),
        os: "NAIX 2.4.1".to_string(),
        security_level: 6,
        files: HashMap::new(),
        services: Vec::new(),
        vulnerabilities: Vec::new(),
        is_compromised: false,
        admin_access: false,
    };
    
    system.files.insert("entropy_log.txt".to_string(), File {
        name: "entropy_log.txt".to_string(),
        content: "=== ENTROPY ORGANIZATION LOG ===\n\nMission: Achieve maximum entropy in digital systems\nMethod: Hacknet protocol distribution\nStatus: PHASE 3 ACTIVE\n\nNode Operations:\n- netmap 192.168.1.100\n- portscan 10.0.0.50 1-65535\n- sshcrack target_acquired\n- decypher classified_data.dat\n\nEntropy Level: 89.7%\nTarget: Global digital infrastructure\n\n--- HACKNET PROTOCOL ENGAGED ---".to_string(),
        permissions: "entropy:void 600".to_string(),
        size: 1024,
        encrypted: false,
        password: None,
    });
    
    system
}

use std::thread; 