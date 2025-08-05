// ============================================================================
// СИСТЕМА ОТСЫЛОК И ПАСХАЛОК
// ============================================================================

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceSystem {
    pub easter_eggs: HashMap<String, EasterEgg>,
    pub reference_characters: HashMap<String, ReferenceCharacter>,
    pub reference_organizations: HashMap<String, ReferenceOrganization>,
    pub reference_missions: Vec<ReferenceMission>,
    pub reference_files: HashMap<String, ReferenceFile>,
    pub reference_programs: HashMap<String, ReferenceProgram>,
    pub discovered_references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasterEgg {
    pub id: String,
    pub reference_source: ReferenceSource,
    pub title: String,
    pub description: String,
    pub discovery_method: String,
    pub unlock_condition: String,
    pub reward: String,
    pub is_discovered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceSource {
    MrRobot,
    FightClub,
    Hackers,
    Hacknet,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceCharacter {
    pub id: String,
    pub name: String,
    pub reference_to: String,
    pub source: ReferenceSource,
    pub role: String,
    pub hidden_dialogue: Vec<String>,
    pub easter_egg_triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceOrganization {
    pub name: String,
    pub reference_to: String,
    pub source: ReferenceSource,
    pub description: String,
    pub members: Vec<String>,
    pub goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceMission {
    pub id: String,
    pub title: String,
    pub reference_to: String,
    pub source: ReferenceSource,
    pub description: String,
    pub special_objectives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceFile {
    pub filename: String,
    pub content: String,
    pub reference_to: String,
    pub source: ReferenceSource,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceProgram {
    pub name: String,
    pub description: String,
    pub reference_to: String,
    pub source: ReferenceSource,
    pub command_syntax: String,
    pub special_functions: Vec<String>,
}

impl ReferenceSystem {
    pub fn new() -> Self {
        let mut system = ReferenceSystem {
            easter_eggs: HashMap::new(),
            reference_characters: HashMap::new(),
            reference_organizations: HashMap::new(),
            reference_missions: Vec::new(),
            reference_files: HashMap::new(),
            reference_programs: HashMap::new(),
            discovered_references: Vec::new(),
        };
        
        system.initialize_mr_robot_references();
        system.initialize_fight_club_references();
        system.initialize_hackers_references();
        system.initialize_hacknet_references();
        system
    }
    
    fn initialize_mr_robot_references(&mut self) {
        // ===== MR. ROBOT REFERENCES =====
        
        // Персонажи
        self.reference_characters.insert("elliot_alderson_ref".to_string(), ReferenceCharacter {
            id: "elliot_alderson_ref".to_string(),
            name: "Илья 'E' Алексеев".to_string(),
            reference_to: "Elliot Alderson".to_string(),
            source: ReferenceSource::MrRobot,
            role: "Одинокий хакер с социальной тревожностью".to_string(),
            hidden_dialogue: vec![
                "Привет, друг... Ты меня видишь?".to_string(),
                "Иногда я разговариваю с кем-то, кого не существует".to_string(),
                "Общество - это иллюзия, контролируемая корпорациями".to_string(),
            ],
            easter_egg_triggers: vec!["hello_friend".to_string()],
        });
        
        self.reference_characters.insert("mr_robot_ref".to_string(), ReferenceCharacter {
            id: "mr_robot_ref".to_string(),
            name: "Роман 'Мистер' Роботов".to_string(),
            reference_to: "Mr. Robot".to_string(),
            source: ReferenceSource::MrRobot,
            role: "Загадочный лидер хакерской группы".to_string(),
            hidden_dialogue: vec![
                "Мы fsociety, и мы пришли, чтобы все изменить".to_string(),
                "Система сломана, и мы ее починим".to_string(),
                "Ты готов к революции?".to_string(),
            ],
            easter_egg_triggers: vec!["fsociety".to_string()],
        });
        
        self.reference_characters.insert("darlene_ref".to_string(), ReferenceCharacter {
            id: "darlene_ref".to_string(),
            name: "Дарья 'Hack' Ленская".to_string(),
            reference_to: "Darlene Alderson".to_string(),
            source: ReferenceSource::MrRobot,
            role: "Бесстрашная хакерша и активистка".to_string(),
            hidden_dialogue: vec![
                "Давай устроим хаос в их системе".to_string(),
                "Корпорации должны гореть в цифровом аду".to_string(),
            ],
            easter_egg_triggers: vec!["burn_them_down".to_string()],
        });
        
        // Организации
        self.reference_organizations.insert("fsociety".to_string(), ReferenceOrganization {
            name: "FSociety".to_string(),
            reference_to: "fsociety from Mr. Robot".to_string(),
            source: ReferenceSource::MrRobot,
            description: "Хакерская группа, борющаяся против корпоративного контроля".to_string(),
            members: vec!["Mr_Robot".to_string(), "Elliot_ref".to_string(), "Darlene_ref".to_string()],
            goals: vec![
                "Уничтожить долговые записи".to_string(),
                "Свергнуть корпоративную диктатуру".to_string(),
                "Освободить общество от финансового рабства".to_string(),
            ],
        });
        
        self.reference_organizations.insert("evil_corp".to_string(), ReferenceOrganization {
            name: "Evil Corp".to_string(),
            reference_to: "E Corp from Mr. Robot".to_string(),
            source: ReferenceSource::MrRobot,
            description: "Злая корпорация, контролирующая мировую экономику".to_string(),
            members: vec!["Corporate_Executives".to_string()],
            goals: vec![
                "Мировое финансовое господство".to_string(),
                "Контроль над данными людей".to_string(),
            ],
        });
        
        // Миссии
        self.reference_missions.push(ReferenceMission {
            id: "five_nine_hack".to_string(),
            title: "Операция 5/9".to_string(),
            reference_to: "5/9 hack from Mr. Robot".to_string(),
            source: ReferenceSource::MrRobot,
            description: "Масштабная хакерская операция по уничтожению долговых записей".to_string(),
            special_objectives: vec![
                "Взломать центральные серверы Evil Corp".to_string(),
                "Уничтожить все финансовые записи".to_string(),
                "Избежать обнаружения".to_string(),
            ],
        });
        
        // Файлы
        self.reference_files.insert("hello_friend.txt".to_string(), ReferenceFile {
            filename: "hello_friend.txt".to_string(),
            content: "Привет, друг.\n\nТы новенький? Ты выглядишь как тот, кто может понять...\nОбщество сломано. Корпорации контролируют все.\nМы можем это изменить.\n\n- E".to_string(),
            reference_to: "Elliot's internal monologue".to_string(),
            source: ReferenceSource::MrRobot,
            location: "hidden_folder".to_string(),
        });
        
        // Программы
        self.reference_programs.insert("fsociety_tool".to_string(), ReferenceProgram {
            name: "fsociety.exe".to_string(),
            description: "Многофункциональный инструмент для хакинга корпоративных систем".to_string(),
            reference_to: "fsociety hacking tools".to_string(),
            source: ReferenceSource::MrRobot,
            command_syntax: "fsociety [target] [operation]".to_string(),
            special_functions: vec![
                "Debt record deletion".to_string(),
                "Corporate network infiltration".to_string(),
                "Anonymous communication".to_string(),
            ],
        });
        
        // Пасхалки
        self.easter_eggs.insert("mr_robot_mask".to_string(), EasterEgg {
            id: "mr_robot_mask".to_string(),
            reference_source: ReferenceSource::MrRobot,
            title: "Маска Мистера Робота".to_string(),
            description: "Вы нашли знаменитую маску fsociety!".to_string(),
            discovery_method: "Введите 'fsociety' в качестве пароля".to_string(),
            unlock_condition: "password_fsociety".to_string(),
            reward: "Анонимность +50, Hacking +10".to_string(),
            is_discovered: false,
        });
    }
    
    fn initialize_fight_club_references(&mut self) {
        // ===== FIGHT CLUB REFERENCES =====
        
        // Персонажи
        self.reference_characters.insert("tyler_durden_ref".to_string(), ReferenceCharacter {
            id: "tyler_durden_ref".to_string(),
            name: "Тимофей 'Tyler' Дронов".to_string(),
            reference_to: "Tyler Durden".to_string(),
            source: ReferenceSource::FightClub,
            role: "Харизматичный лидер подпольного движения".to_string(),
            hidden_dialogue: vec![
                "Первое правило хакер-клуба - никому не рассказывать о хакер-клубе".to_string(),
                "Мы поколение, которое корпорации обманули".to_string(),
                "Система должна быть разрушена, чтобы мы могли построить новую".to_string(),
                "Ты не твоя работа, не деньги на банковском счете".to_string(),
            ],
            easter_egg_triggers: vec!["project_mayhem".to_string(), "first_rule".to_string()],
        });
        
        self.reference_characters.insert("marla_ref".to_string(), ReferenceCharacter {
            id: "marla_ref".to_string(),
            name: "Марта 'Chaos' Ларина".to_string(),
            reference_to: "Marla Singer".to_string(),
            source: ReferenceSource::FightClub,
            role: "Непредсказуемая информаторша".to_string(),
            hidden_dialogue: vec![
                "Я туристка в собственной жизни".to_string(),
                "Хочешь настоящих эмоций? Взломай что-нибудь важное".to_string(),
            ],
            easter_egg_triggers: vec!["tourist".to_string()],
        });
        
        // Организации
        self.reference_organizations.insert("project_mayhem".to_string(), ReferenceOrganization {
            name: "Project Mayhem".to_string(),
            reference_to: "Project Mayhem from Fight Club".to_string(),
            source: ReferenceSource::FightClub,
            description: "Радикальная организация, стремящаяся разрушить корпоративную систему".to_string(),
            members: vec!["Tyler_Durden_ref".to_string(), "Anonymous_Members".to_string()],
            goals: vec![
                "Разрушение кредитных записей".to_string(),
                "Подрыв корпоративной инфраструктуры".to_string(),
                "Возвращение людей к первобытному состоянию".to_string(),
            ],
        });
        
        // Миссии
        self.reference_missions.push(ReferenceMission {
            id: "credit_card_companies".to_string(),
            title: "Операция 'Обнуление'".to_string(),
            reference_to: "Credit card companies destruction".to_string(),
            source: ReferenceSource::FightClub,
            description: "Уничтожить здания кредитных компаний и обнулить долги".to_string(),
            special_objectives: vec![
                "Проникнуть в здания кредитных компаний".to_string(),
                "Установить логические бомбы".to_string(),
                "Синхронизировать атаку".to_string(),
            ],
        });
        
        // Файлы
        self.reference_files.insert("rules.txt".to_string(), ReferenceFile {
            filename: "fight_club_rules.txt".to_string(),
            content: "ПРАВИЛА ХАКЕР-КЛУБА:\n\n1. Никому не рассказывать о хакер-клубе\n2. НИКОМУ НЕ РАССКАЗЫВАТЬ О ХАКЕР-КЛУБЕ!\n3. Если кто-то кричит 'стоп', теряет сознание - хак прекращается\n4. За один раз хакают только два человека\n5. За раз только один хак\n6. Никаких антивирусов, никаких файрволов\n7. Хаки идут столько, сколько нужно\n8. Новичок обязан хакнуть что-то в первую ночь".to_string(),
            reference_to: "Fight Club rules".to_string(),
            source: ReferenceSource::FightClub,
            location: "underground_server".to_string(),
        });
        
        // Программы
        self.reference_programs.insert("mayhem.exe".to_string(), ReferenceProgram {
            name: "Project Mayhem Toolkit".to_string(),
            description: "Набор инструментов для разрушения корпоративных систем".to_string(),
            reference_to: "Project Mayhem operations".to_string(),
            source: ReferenceSource::FightClub,
            command_syntax: "mayhem [target] [destruction_level]".to_string(),
            special_functions: vec![
                "Corporate infrastructure sabotage".to_string(),
                "Credit record deletion".to_string(),
                "Anonymous recruitment".to_string(),
            ],
        });
    }
    
    fn initialize_hackers_references(&mut self) {
        // ===== HACKERS (1995) REFERENCES =====
        
        // Персонажи
        self.reference_characters.insert("zero_cool_ref".to_string(), ReferenceCharacter {
            id: "zero_cool_ref".to_string(),
            name: "Дмитрий 'Zero Cool' Крутов".to_string(),
            reference_to: "Dade Murphy (Zero Cool)".to_string(),
            source: ReferenceSource::Hackers,
            role: "Молодой хакер-вундеркинд".to_string(),
            hidden_dialogue: vec![
                "Hack the planet!".to_string(),
                "Они могут взять нашу жизнь, но никогда не возьмут нашу свободу хакинга!".to_string(),
                "Я elite hacker, я взламываю Gibson!".to_string(),
            ],
            easter_egg_triggers: vec!["hack_the_planet".to_string(), "gibson".to_string()],
        });
        
        self.reference_characters.insert("acid_burn_ref".to_string(), ReferenceCharacter {
            id: "acid_burn_ref".to_string(),
            name: "Кира 'Acid Burn' Бернс".to_string(),
            reference_to: "Kate Libby (Acid Burn)".to_string(),
            source: ReferenceSource::Hackers,
            role: "Элитная хакерша и соперница".to_string(),
            hidden_dialogue: vec![
                "Ты думаешь, что круче меня? Посмотрим!".to_string(),
                "Это не игра, это жизнь!".to_string(),
                "Только elite может понять elite".to_string(),
            ],
            easter_egg_triggers: vec!["elite".to_string()],
        });
        
        self.reference_characters.insert("plague_ref".to_string(), ReferenceCharacter {
            id: "plague_ref".to_string(),
            name: "Виктор 'Plague' Плагин".to_string(),
            reference_to: "Eugene Belford (The Plague)".to_string(),
            source: ReferenceSource::Hackers,
            role: "Корпоративный хакер-предатель".to_string(),
            hidden_dialogue: vec![
                "Добро пожаловать в реальный мир!".to_string(),
                "Деньги решают все, даже в хакинге".to_string(),
                "Я был элитой, когда ты еще в памперсах сидел!".to_string(),
            ],
            easter_egg_triggers: vec!["real_world".to_string()],
        });
        
        // Файлы
        self.reference_files.insert("gibson_mainframe.dat".to_string(), ReferenceFile {
            filename: "gibson_mainframe.dat".to_string(),
            content: "=== GIBSON SUPERCOMPUTER ===\n\nСистема: Gibson\nБезопасность: МАКСИМАЛЬНАЯ\nСтатус: АКТИВЕН\n\nВНИМАНИЕ: Попытка несанкционированного доступа\nбудет отслежена и наказана!\n\n'Hack the Planet!' - граффити найдено в системных логах\n\nПоследняя активность:\n- Zero Cool: попытка входа\n- Acid Burn: системное сканирование\n- Plague: административный доступ".to_string(),
            reference_to: "Gibson supercomputer".to_string(),
            source: ReferenceSource::Hackers,
            location: "corporate_mainframe".to_string(),
        });
        
        self.reference_files.insert("da_vinci_virus.exe".to_string(), ReferenceFile {
            filename: "da_vinci_virus.exe".to_string(),
            content: "// Da Vinci Virus\n// Автор: The Plague\n// Назначение: Хищение денег и подстава невинных\n\nif (balance > 0) {\n    transfer_to_secret_account(balance * 0.1);\n    frame_random_hacker();\n}\n\n// Никто не заподозрит старого доброго Plague\n// Muahahaha!".to_string(),
            reference_to: "Da Vinci virus from Hackers".to_string(),
            source: ReferenceSource::Hackers,
            location: "plague_server".to_string(),
        });
        
        // Программы
        self.reference_programs.insert("gibson_cracker".to_string(), ReferenceProgram {
            name: "Gibson Cracker".to_string(),
            description: "Специализированный инструмент для взлома суперкомпьютеров Gibson".to_string(),
            reference_to: "Gibson hacking tools".to_string(),
            source: ReferenceSource::Hackers,
            command_syntax: "gibson_crack [target_gibson] [elite_mode]".to_string(),
            special_functions: vec![
                "Supercomputer infiltration".to_string(),
                "Elite hacker competition".to_string(),
                "Corporate system analysis".to_string(),
            ],
        });
        
        // Миссии
        self.reference_missions.push(ReferenceMission {
            id: "save_the_world".to_string(),
            title: "Спасти Мир".to_string(),
            reference_to: "Saving the world in Hackers".to_string(),
            source: ReferenceSource::Hackers,
            description: "Остановить злого корпоративного хакера и спасти невинных".to_string(),
            special_objectives: vec![
                "Разоблачить Plague".to_string(),
                "Остановить Da Vinci virus".to_string(),
                "Доказать невиновность Zero Cool".to_string(),
                "Hack the Planet!".to_string(),
            ],
        });
    }
    
    fn initialize_hacknet_references(&mut self) {
        // ===== HACKNET REFERENCES =====
        
        // Программы (в стиле Hacknet)
        self.reference_programs.insert("netmap".to_string(), ReferenceProgram {
            name: "netmap".to_string(),
            description: "Сканирование и картографирование сетевых узлов".to_string(),
            reference_to: "nmap from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            command_syntax: "netmap [ip_address]".to_string(),
            special_functions: vec![
                "Network topology discovery".to_string(),
                "Port scanning".to_string(),
                "Service enumeration".to_string(),
            ],
        });
        
        self.reference_programs.insert("portscan".to_string(), ReferenceProgram {
            name: "portscan".to_string(),
            description: "Сканирование открытых портов на целевой системе".to_string(),
            reference_to: "PortHack from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            command_syntax: "portscan [ip] [port_range]".to_string(),
            special_functions: vec![
                "Port vulnerability detection".to_string(),
                "Service identification".to_string(),
                "Access point discovery".to_string(),
            ],
        });
        
        self.reference_programs.insert("sshdcrack".to_string(), ReferenceProgram {
            name: "SSHcrack".to_string(),
            description: "Взлом SSH соединений и получение shell доступа".to_string(),
            reference_to: "SSHcrack from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            command_syntax: "sshcrack [target_ip]".to_string(),
            special_functions: vec![
                "SSH vulnerability exploitation".to_string(),
                "Remote shell access".to_string(),
                "Secure tunnel creation".to_string(),
            ],
        });
        
        self.reference_programs.insert("ftpbounce".to_string(), ReferenceProgram {
            name: "FTPBounce".to_string(),
            description: "Эксплуатация уязвимостей FTP для получения доступа".to_string(),
            reference_to: "FTPBounce from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            command_syntax: "ftpbounce [ftp_server]".to_string(),
            special_functions: vec![
                "FTP server exploitation".to_string(),
                "File system access".to_string(),
                "Directory traversal".to_string(),
            ],
        });
        
        self.reference_programs.insert("decypher".to_string(), ReferenceProgram {
            name: "DECypher".to_string(),
            description: "Расшифровка защищенных файлов и сообщений".to_string(),
            reference_to: "DECypher from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            command_syntax: "decypher [encrypted_file]".to_string(),
            special_functions: vec![
                "Encryption breaking".to_string(),
                "Password recovery".to_string(),
                "Data decryption".to_string(),
            ],
        });
        
        // Файлы в стиле Hacknet
        self.reference_files.insert("entropy_log.txt".to_string(), ReferenceFile {
            filename: "entropy_log.txt".to_string(),
            content: "=== ENTROPY BREACH LOG ===\n\nИСХОДНЫЙ УЗЕЛ: 192.168.1.100\nЦЕЛЕВОЙ УЗЕЛ: 10.0.0.50\n\nИСПОЛЬЗОВАННЫЕ ИНСТРУМЕНТЫ:\n- netmap\n- portscan\n- sshcrack\n- decypher\n\nСТАТУС: КОМПРОМЕТИРОВАН\nДАННЫЕ: ИЗВЛЕЧЕНЫ\nСЛЕДЫ: ОЧИЩЕНЫ\n\n--- HACKNET PROTOCOL ENGAGED ---".to_string(),
            reference_to: "Hacknet hacking logs".to_string(),
            source: ReferenceSource::Hacknet,
            location: "hacker_terminal".to_string(),
        });
        
        self.reference_files.insert("naix_readme.txt".to_string(), ReferenceFile {
            filename: "naix_readme.txt".to_string(),
            content: "NAIX Operating System v2.4.1\n\nДобро пожаловать в NAIX!\n\nЭта система специально разработана для хакеров.\nВключает предустановленные инструменты:\n\n- netmap - сетевое сканирование\n- portscan - анализ портов  \n- sshcrack - SSH эксплуатация\n- ftpbounce - FTP взлом\n- decypher - расшифровка\n\nИспользуйте ответственно.\nПомните: оставлять следы - дурной тон.\n\n[СИСТЕМНОЕ СООБЩЕНИЕ: ENTROPY DETECTED]".to_string(),
            reference_to: "NAIX OS from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            location: "system_root".to_string(),
        });
        
        // Организации
        self.reference_organizations.insert("entropy".to_string(), ReferenceOrganization {
            name: "Entropy".to_string(),
            reference_to: "Entropy from Hacknet".to_string(),
            source: ReferenceSource::Hacknet,
            description: "Загадочная организация, работающая в глубинах даркнета".to_string(),
            members: vec!["Unknown Operatives".to_string()],
            goals: vec![
                "Контроль над хакерскими сетями".to_string(),
                "Манипулирование цифровой информацией".to_string(),
                "Достижение полной энтропии в системах".to_string(),
            ],
        });
        
        // Миссии
        self.reference_missions.push(ReferenceMission {
            id: "hacknet_sequence".to_string(),
            title: "Последовательность Hacknet".to_string(),
            reference_to: "Hacknet mission structure".to_string(),
            source: ReferenceSource::Hacknet,
            description: "Серия связанных хакерских операций с постепенным усложнением".to_string(),
            special_objectives: vec![
                "Взломать серию соединенных узлов".to_string(),
                "Собрать фрагменты данных".to_string(),
                "Раскрыть заговор Entropy".to_string(),
                "Очистить следы вторжения".to_string(),
            ],
        });
        
        // Пасхалки
        self.easter_eggs.insert("hacknet_terminal".to_string(), EasterEgg {
            id: "hacknet_terminal".to_string(),
            reference_source: ReferenceSource::Hacknet,
            title: "Терминал Hacknet".to_string(),
            description: "Вы нашли настоящий терминал из игры Hacknet!".to_string(),
            discovery_method: "Введите команду 'netmap localhost'".to_string(),
            unlock_condition: "netmap_localhost".to_string(),
            reward: "Доступ ко всем Hacknet инструментам".to_string(),
            is_discovered: false,
        });
        
        self.easter_eggs.insert("entropy_symbol".to_string(), EasterEgg {
            id: "entropy_symbol".to_string(),
            reference_source: ReferenceSource::Hacknet,
            title: "Символ Энтропии".to_string(),
            description: "Вы обнаружили секретный символ организации Entropy".to_string(),
            discovery_method: "Найти скрытый файл с именем 'ent.sys'".to_string(),
            unlock_condition: "found_entropy_file".to_string(),
            reward: "Секретные миссии Entropy".to_string(),
            is_discovered: false,
        });
    }
    
    // Методы для обработки отсылок
    pub fn trigger_easter_egg(&mut self, trigger: &str, terminal: &Terminal) -> bool {
        for egg in self.easter_eggs.values_mut() {
            if egg.unlock_condition == trigger && !egg.is_discovered {
                egg.is_discovered = true;
                self.discovered_references.push(egg.id.clone());
                
                self.display_easter_egg_found(egg, terminal);
                return true;
            }
        }
        false
    }
    
    fn display_easter_egg_found(&self, egg: &EasterEgg, terminal: &Terminal) {
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("🥚 ═══ ПАСХАЛКА НАЙДЕНА! ═══ 🥚", TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        
        let source_text = match egg.reference_source {
            ReferenceSource::MrRobot => "Mr. Robot",
            ReferenceSource::FightClub => "Fight Club", 
            ReferenceSource::Hackers => "Hackers (1995)",
            ReferenceSource::Hacknet => "Hacknet",
            ReferenceSource::General => "General",
        };
        
        terminal.print_with_effect(&format!("📺 Отсылка к: {}", source_text), TerminalEffect::Matrix);
        terminal.print_with_effect(&format!("🎯 {}", egg.title), TerminalEffect::Success);
        terminal.print_with_effect(&format!("📝 {}", egg.description), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("🎁 Награда: {}", egg.reward), TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("═══════════════════════════════", TerminalEffect::Success);
    }
    
    pub fn check_reference_dialogue(&self, character_id: &str, input: &str) -> Option<String> {
        if let Some(character) = self.reference_characters.get(character_id) {
            for trigger in &character.easter_egg_triggers {
                if input.to_lowercase().contains(&trigger.to_lowercase()) {
                    // Возвращаем специальный диалог
                    let index = rand::thread_rng().gen_range(0..character.hidden_dialogue.len());
                    return Some(character.hidden_dialogue[index].clone());
                }
            }
        }
        None
    }
    
    pub fn get_reference_program(&self, program_name: &str) -> Option<&ReferenceProgram> {
        self.reference_programs.get(program_name)
    }
    
    pub fn add_reference_commands_to_processor(&self, processor: &mut CommandProcessor) {
        // Добавляем команды из Hacknet
        processor.commands.insert("netmap".to_string(), Command {
            name: "netmap".to_string(),
            description: "Сканирование сетевой топологии (Hacknet style)".to_string(),
            usage: "netmap <target_ip>".to_string(),
            requires_target: true,
            requires_connection: false,
        });
        
        processor.commands.insert("portscan".to_string(), Command {
            name: "portscan".to_string(),
            description: "Сканирование портов цели (Hacknet style)".to_string(),
            usage: "portscan <target_ip>".to_string(),
            requires_target: true,
            requires_connection: false,
        });
        
        processor.commands.insert("sshcrack".to_string(), Command {
            name: "sshcrack".to_string(),
            description: "Взлом SSH соединения (Hacknet style)".to_string(),
            usage: "sshcrack <target_ip>".to_string(),
            requires_target: true,
            requires_connection: false,
        });
        
        processor.commands.insert("ftpbounce".to_string(), Command {
            name: "ftpbounce".to_string(),
            description: "Эксплуатация FTP уязвимостей (Hacknet style)".to_string(),
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
        
        // Команды из других источников
        processor.commands.insert("fsociety".to_string(), Command {
            name: "fsociety".to_string(),
            description: "Доступ к инструментам fsociety (Mr. Robot)".to_string(),
            usage: "fsociety [operation]".to_string(),
            requires_target: false,
            requires_connection: false,
        });
        
        processor.commands.insert("mayhem".to_string(), Command {
            name: "mayhem".to_string(),
            description: "Project Mayhem toolkit (Fight Club)".to_string(),
            usage: "mayhem <target> <operation>".to_string(),
            requires_target: true,
            requires_connection: false,
        });
        
        processor.commands.insert("gibson".to_string(), Command {
            name: "gibson".to_string(),
            description: "Доступ к суперкомпьютеру Gibson (Hackers)".to_string(),
            usage: "gibson <command>".to_string(),
            requires_target: false,
            requires_connection: false,
        });
    }
    
    pub fn get_discovered_count(&self) -> usize {
        self.discovered_references.len()
    }
    
    pub fn get_total_easter_eggs(&self) -> usize {
        self.easter_eggs.len()
    }
    
    pub fn display_reference_statistics(&self, terminal: &Terminal) {
        terminal.print_with_effect("═══ СТАТИСТИКА ПАСХАЛОК ═══", TerminalEffect::Matrix);
        terminal.print_with_effect("", TerminalEffect::Normal);
        
        let total = self.get_total_easter_eggs();
        let found = self.get_discovered_count();
        
        terminal.print_with_effect(&format!("🥚 Найдено пасхалок: {}/{}", found, total), TerminalEffect::Success);
        terminal.print_with_effect("", TerminalEffect::Normal);
        
        // Статистика по источникам
        let mut mr_robot_count = 0;
        let mut fight_club_count = 0;
        let mut hackers_count = 0;
        let mut hacknet_count = 0;
        
        for egg in self.easter_eggs.values() {
            if egg.is_discovered {
                match egg.reference_source {
                    ReferenceSource::MrRobot => mr_robot_count += 1,
                    ReferenceSource::FightClub => fight_club_count += 1,
                    ReferenceSource::Hackers => hackers_count += 1,
                    ReferenceSource::Hacknet => hacknet_count += 1,
                    _ => {}
                }
            }
        }
        
        terminal.print_with_effect("📺 По источникам:", TerminalEffect::Normal);
        terminal.print_with_effect(&format!("  Mr. Robot: {}", mr_robot_count), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("  Fight Club: {}", fight_club_count), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("  Hackers (1995): {}", hackers_count), TerminalEffect::Normal);
        terminal.print_with_effect(&format!("  Hacknet: {}", hacknet_count), TerminalEffect::Normal);
        
        if found == total {
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🏆 ПОЗДРАВЛЯЕМ! ВЫ НАШЛИ ВСЕ ПАСХАЛКИ!", TerminalEffect::Success);
            terminal.print_with_effect("🎉 Вы настоящий знаток киберпанк культуры!", TerminalEffect::Success);
        }
    }
}

use rand::Rng; 