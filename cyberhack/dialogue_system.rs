// ============================================================================
// РАСШИРЕННАЯ СИСТЕМА ДИАЛОГОВ И ПЕРСОНАЖЕЙ
// ============================================================================

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSystem {
    pub characters: HashMap<String, Character>,
    pub dialogue_trees: HashMap<String, DialogueTree>,
    pub conversation_history: Vec<ConversationRecord>,
    pub relationship_tracker: HashMap<String, i32>,
    pub active_conversations: HashMap<String, String>,
    pub story_variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub handle: String,
    pub faction: String,
    pub role: String,
    pub personality: CharacterPersonality,
    pub appearance: CharacterAppearance,
    pub background: CharacterBackground,
    pub relationships: HashMap<String, Relationship>,
    pub dialogue_state: DialogueState,
    pub available_services: Vec<CharacterService>,
    pub conversation_topics: Vec<String>,
    pub secrets: Vec<CharacterSecret>,
    pub goals: Vec<String>,
    pub fears: Vec<String>,
    pub current_mood: Mood,
    pub speaking_style: SpeakingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPersonality {
    pub openness: u32,        // 0-100
    pub conscientiousness: u32,
    pub extraversion: u32,
    pub agreeableness: u32,
    pub neuroticism: u32,
    pub intelligence: u32,
    pub morality: u32,
    pub ambition: u32,
    pub paranoia: u32,
    pub loyalty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAppearance {
    pub age: u32,
    pub gender: String,
    pub height: String,
    pub build: String,
    pub hair_color: String,
    pub eye_color: String,
    pub distinctive_features: Vec<String>,
    pub clothing_style: String,
    pub cybernetic_modifications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBackground {
    pub origin: String,
    pub education: String,
    pub career_history: Vec<String>,
    pub major_events: Vec<String>,
    pub family: Vec<String>,
    pub trauma: Vec<String>,
    pub achievements: Vec<String>,
    pub regrets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub character_id: String,
    pub relationship_type: RelationshipType,
    pub trust_level: i32,
    pub romantic_interest: bool,
    pub shared_history: Vec<String>,
    pub conflicts: Vec<String>,
    pub mutual_interests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Family,
    Friend,
    Ally,
    Rival,
    Enemy,
    Mentor,
    Student,
    Colleague,
    Stranger,
    RomanticInterest,
    FormerLover,
    Business,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueState {
    pub current_conversation: Option<String>,
    pub last_topic: String,
    pub conversation_count: u32,
    pub relationship_level: i32,
    pub known_secrets: Vec<String>,
    pub revealed_information: Vec<String>,
    pub trust_events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterService {
    pub service_id: String,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub requirements: Vec<String>,
    pub cooldown: Duration,
    pub last_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSecret {
    pub secret_id: String,
    pub content: String,
    pub revelation_requirements: Vec<String>,
    pub consequences_if_revealed: Vec<String>,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mood {
    Happy,
    Sad,
    Angry,
    Fearful,
    Confident,
    Paranoid,
    Excited,
    Depressed,
    Neutral,
    Suspicious,
    Friendly,
    Hostile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakingStyle {
    pub formality: u32,      // 0-100: casual to formal
    pub verbosity: u32,      // 0-100: terse to verbose
    pub directness: u32,     // 0-100: indirect to direct
    pub emotionality: u32,   // 0-100: cold to emotional
    pub slang_usage: u32,    // 0-100: proper to slang
    pub technical_jargon: u32, // 0-100: layman to technical
    pub catchphrases: Vec<String>,
    pub accent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    pub tree_id: String,
    pub character_id: String,
    pub root_node: String,
    pub nodes: HashMap<String, DialogueNode>,
    pub conditions: HashMap<String, DialogueCondition>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub node_id: String,
    pub speaker: String,
    pub text: String,
    pub text_variations: Vec<TextVariation>,
    pub options: Vec<DialogueOption>,
    pub conditions: Vec<String>,
    pub effects: Vec<DialogueEffect>,
    pub mood_impact: HashMap<String, i32>,
    pub relationship_impact: i32,
    pub information_revealed: Vec<String>,
    pub next_node: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextVariation {
    pub condition: String,
    pub text: String,
    pub mood_modifier: Mood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    pub option_id: String,
    pub text: String,
    pub requirements: Vec<String>,
    pub skill_check: Option<SkillCheck>,
    pub consequences: Vec<String>,
    pub target_node: String,
    pub relationship_change: i32,
    pub faction_reputation_change: HashMap<String, i32>,
    pub unlocks: Vec<String>,
    pub locks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCheck {
    pub skill: String,
    pub difficulty: u32,
    pub success_node: String,
    pub failure_node: String,
    pub critical_success_node: Option<String>,
    pub critical_failure_node: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEffect {
    pub effect_type: EffectType,
    pub target: String,
    pub value: i32,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    ChangeReputation,
    GiveMoney,
    TakeMoney,
    GiveItem,
    TakeItem,
    UnlockLocation,
    UnlockService,
    RevealSecret,
    ChangeRelationship,
    SetFlag,
    TriggerEvent,
    StartQuest,
    CompleteObjective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub target: String,
    pub operator: ComparisonOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Reputation,
    Money,
    Item,
    Skill,
    Flag,
    Relationship,
    QuestStatus,
    Time,
    Location,
    CharacterMood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Contains,
    NotContains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub conversation_id: String,
    pub character_id: String,
    pub timestamp: String,
    pub topics_discussed: Vec<String>,
    pub information_gained: Vec<String>,
    pub relationship_changes: i32,
    pub outcome: ConversationOutcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationOutcome {
    Successful,
    Failed,
    Neutral,
    Hostile,
    RomanceAdvanced,
    TrustGained,
    TrustLost,
    SecretRevealed,
    QuestReceived,
}

impl DialogueSystem {
    pub fn new() -> Self {
        let mut dialogue_system = DialogueSystem {
            characters: HashMap::new(),
            dialogue_trees: HashMap::new(),
            conversation_history: Vec::new(),
            relationship_tracker: HashMap::new(),
            active_conversations: HashMap::new(),
            story_variables: HashMap::new(),
        };
        
        dialogue_system.initialize_characters();
        dialogue_system.create_dialogue_trees();
        dialogue_system
    }
    
    fn initialize_characters(&mut self) {
        let characters = vec![
            self.create_shadow_character(),
            self.create_ghost_character(),
            self.create_dr_mitchell(),
            self.create_nexus_ceo(),
            self.create_government_director(),
            self.create_cyber_freedom_leader(),
            self.create_underground_kingpin(),
            self.create_ai_entity(),
            self.create_quantum_researcher(),
            self.create_corporate_spy(),
            self.create_street_contact(),
            self.create_mysterious_informant(),
            self.create_former_nexus_employee(),
            self.create_cyber_cafe_owner(),
            self.create_black_market_dealer(),
        ];
        
        for character in characters {
            self.characters.insert(character.id.clone(), character);
        }
    }
    
    fn create_shadow_character(&self) -> Character {
        Character {
            id: "shadow_hunter".to_string(),
            name: "Алексей 'Тень' Морозов".to_string(),
            handle: "shadow_hunter".to_string(),
            faction: "Кибер-повстанцы".to_string(),
            role: "Информационный брокер".to_string(),
            personality: CharacterPersonality {
                openness: 70,
                conscientiousness: 85,
                extraversion: 45,
                agreeableness: 60,
                neuroticism: 30,
                intelligence: 90,
                morality: 75,
                ambition: 65,
                paranoia: 80,
                loyalty: 85,
            },
            appearance: CharacterAppearance {
                age: 42,
                gender: "Мужской".to_string(),
                height: "180 см".to_string(),
                build: "Атлетическое".to_string(),
                hair_color: "Седеющие черные".to_string(),
                eye_color: "Карие".to_string(),
                distinctive_features: vec![
                    "Шрам на левой щеке".to_string(),
                    "Всегда носит темные очки".to_string(),
                ],
                clothing_style: "Темная практичная одежда".to_string(),
                cybernetic_modifications: vec![
                    "Кибернетический глаз для анализа данных".to_string(),
                ],
            },
            background: CharacterBackground {
                origin: "Бывший сотрудник ФСБ".to_string(),
                education: "Военная академия, факультет кибербезопасности".to_string(),
                career_history: vec![
                    "Агент ФСБ (2005-2020)".to_string(),
                    "Частный детектив (2020-2022)".to_string(),
                    "Информационный брокер (2022-настоящее время)".to_string(),
                ],
                major_events: vec![
                    "Разочарование в государственной системе".to_string(),
                    "Переход на сторону кибер-активистов".to_string(),
                    "Создание подпольной сети информаторов".to_string(),
                ],
                family: vec!["Бывшая жена Ирина (развод)".to_string(), "Дочь Анна (нет контакта)".to_string()],
                trauma: vec!["Предательство коллег в ФСБ".to_string()],
                achievements: vec!["Раскрыл несколько крупных корпоративных скандалов".to_string()],
                regrets: vec!["Потеря семьи из-за работы".to_string()],
            },
            relationships: HashMap::new(),
            dialogue_state: DialogueState {
                current_conversation: None,
                last_topic: "greeting".to_string(),
                conversation_count: 0,
                relationship_level: 25,
                known_secrets: vec![],
                revealed_information: vec![],
                trust_events: vec![],
            },
            available_services: vec![
                CharacterService {
                    service_id: "information_nexus".to_string(),
                    name: "Информация о NEXUS".to_string(),
                    description: "Подробные данные о структуре и планах NEXUS Corporation".to_string(),
                    cost: 5000,
                    requirements: vec!["trust_level >= 30".to_string()],
                    cooldown: Duration::from_secs(86400),
                    last_used: None,
                },
                CharacterService {
                    service_id: "network_map".to_string(),
                    name: "Карта сетей".to_string(),
                    description: "Схема корпоративных и правительственных сетей".to_string(),
                    cost: 3000,
                    requirements: vec!["completed_first_mission".to_string()],
                    cooldown: Duration::from_secs(43200),
                    last_used: None,
                },
            ],
            conversation_topics: vec![
                "nexus_corporation".to_string(),
                "government_corruption".to_string(),
                "cyber_freedom_movement".to_string(),
                "personal_history".to_string(),
                "family_loss".to_string(),
                "future_plans".to_string(),
            ],
            secrets: vec![
                CharacterSecret {
                    secret_id: "fsb_contact".to_string(),
                    content: "У меня до сих пор есть контакты в ФСБ, которые иногда помогают".to_string(),
                    revelation_requirements: vec!["trust_level >= 70".to_string()],
                    consequences_if_revealed: vec!["Access to government intelligence".to_string()],
                    value: 50,
                },
                CharacterSecret {
                    secret_id: "nexus_insider".to_string(),
                    content: "У меня есть источник внутри NEXUS на очень высоком уровне".to_string(),
                    revelation_requirements: vec!["trust_level >= 90".to_string(), "proved_loyalty".to_string()],
                    consequences_if_revealed: vec!["Access to NEXUS internal documents".to_string()],
                    value: 100,
                },
            ],
            goals: vec![
                "Остановить NEXUS".to_string(),
                "Защитить невинных людей".to_string(),
                "Восстановить отношения с дочерью".to_string(),
            ],
            fears: vec![
                "Предательство снова".to_string(),
                "Потеря еще большего количества людей".to_string(),
                "Что NEXUS узнает о его прошлом".to_string(),
            ],
            current_mood: Mood::Neutral,
            speaking_style: SpeakingStyle {
                formality: 70,
                verbosity: 50,
                directness: 80,
                emotionality: 40,
                slang_usage: 20,
                technical_jargon: 70,
                catchphrases: vec![
                    "Информация - это власть".to_string(),
                    "Доверяй, но проверяй".to_string(),
                ],
                accent: "Московский".to_string(),
            },
        }
    }
    
    fn create_ghost_character(&self) -> Character {
        Character {
            id: "ghost_in_shell".to_string(),
            name: "Анна 'Призрак' Волкова".to_string(),
            handle: "ghost_in_shell".to_string(),
            faction: "Хакеры-одиночки".to_string(),
            role: "Элитный хакер".to_string(),
            personality: CharacterPersonality {
                openness: 85,
                conscientiousness: 60,
                extraversion: 30,
                agreeableness: 40,
                neuroticism: 50,
                intelligence: 98,
                morality: 45,
                ambition: 80,
                paranoia: 90,
                loyalty: 35,
            },
            appearance: CharacterAppearance {
                age: 28,
                gender: "Женский".to_string(),
                height: "165 см".to_string(),
                build: "Стройное".to_string(),
                hair_color: "Платиновый блонд".to_string(),
                eye_color: "Ярко-зеленые (линзы)".to_string(),
                distinctive_features: vec![
                    "Многочисленные татуировки с QR-кодами".to_string(),
                    "Неоновые полоски в волосах".to_string(),
                ],
                clothing_style: "Кибер-панк стиль".to_string(),
                cybernetic_modifications: vec![
                    "Нейроинтерфейс прямого подключения".to_string(),
                    "Кибернетические пальцы для ускоренного набора".to_string(),
                ],
            },
            background: CharacterBackground {
                origin: "Воспитанница детского дома".to_string(),
                education: "Самообразование, онлайн курсы хакинга".to_string(),
                career_history: vec![
                    "Детское хакерство (2010-2014)".to_string(),
                    "Фриланс хакер (2014-2020)".to_string(),
                    "Элитный кибер-наемник (2020-настоящее время)".to_string(),
                ],
                major_events: vec![
                    "Первый взлом в 12 лет".to_string(),
                    "Взлом Пентагона в 16 лет".to_string(),
                    "Исчезновение из поля зрения ФБР".to_string(),
                ],
                family: vec!["Неизвестно (сирота)".to_string()],
                trauma: vec![
                    "Одиночество в детстве".to_string(),
                    "Предательство первой хакерской команды".to_string(),
                ],
                achievements: vec![
                    "Взломала 20 крупнейших корпораций".to_string(),
                    "Создала несколько революционных хакерских инструментов".to_string(),
                ],
                regrets: vec!["Никогда не имела настоящих друзей".to_string()],
            },
            relationships: HashMap::new(),
            dialogue_state: DialogueState {
                current_conversation: None,
                last_topic: "greeting".to_string(),
                conversation_count: 0,
                relationship_level: 10,
                known_secrets: vec![],
                revealed_information: vec![],
                trust_events: vec![],
            },
            available_services: vec![
                CharacterService {
                    service_id: "custom_exploit".to_string(),
                    name: "Создание эксплойта".to_string(),
                    description: "Персональный эксплойт для конкретной цели".to_string(),
                    cost: 15000,
                    requirements: vec!["reputation >= 50".to_string()],
                    cooldown: Duration::from_secs(172800),
                    last_used: None,
                },
                CharacterService {
                    service_id: "ghost_mode".to_string(),
                    name: "Режим призрака".to_string(),
                    description: "Временная невидимость в сети".to_string(),
                    cost: 8000,
                    requirements: vec!["trust_level >= 40".to_string()],
                    cooldown: Duration::from_secs(86400),
                    last_used: None,
                },
            ],
            conversation_topics: vec![
                "hacking_philosophy".to_string(),
                "past_exploits".to_string(),
                "trust_issues".to_string(),
                "technology_advancement".to_string(),
                "loneliness".to_string(),
                "future_vision".to_string(),
            ],
            secrets: vec![
                CharacterSecret {
                    secret_id: "real_identity".to_string(),
                    content: "Мое настоящее имя не Анна Волкова".to_string(),
                    revelation_requirements: vec!["trust_level >= 80".to_string(), "romantic_interest".to_string()],
                    consequences_if_revealed: vec!["Deep personal connection".to_string()],
                    value: 75,
                },
                CharacterSecret {
                    secret_id: "nexus_connection".to_string(),
                    content: "NEXUS однажды пытался меня завербовать, и я почти согласилась".to_string(),
                    revelation_requirements: vec!["trust_level >= 60".to_string()],
                    consequences_if_revealed: vec!["Knowledge of NEXUS recruitment methods".to_string()],
                    value: 60,
                },
            ],
            goals: vec![
                "Стать лучшим хакером в мире".to_string(),
                "Найти свою настоящую семью".to_string(),
                "Создать что-то значимое".to_string(),
            ],
            fears: vec![
                "Быть пойманной".to_string(),
                "Потерять свои способности".to_string(),
                "Довериться не тому человеку".to_string(),
            ],
            current_mood: Mood::Suspicious,
            speaking_style: SpeakingStyle {
                formality: 30,
                verbosity: 40,
                directness: 90,
                emotionality: 30,
                slang_usage: 70,
                technical_jargon: 95,
                catchphrases: vec![
                    "Код не лжет".to_string(),
                    "В цифрах я доверяю".to_string(),
                ],
                accent: "Нейтральный с техническими терминами".to_string(),
            },
        }
    }
    
    fn create_dr_mitchell(&self) -> Character {
        Character {
            id: "nuclear_prophet".to_string(),
            name: "Доктор Джеймс Митчелл".to_string(),
            handle: "nuclear_prophet".to_string(),
            faction: "NEXUS Corporation".to_string(),
            role: "Главный архитектор проекта 'Digital Apocalypse'".to_string(),
            personality: CharacterPersonality {
                openness: 50,
                conscientiousness: 95,
                extraversion: 70,
                agreeableness: 20,
                neuroticism: 40,
                intelligence: 98,
                morality: 15,
                ambition: 100,
                paranoia: 60,
                loyalty: 90,
            },
            appearance: CharacterAppearance {
                age: 55,
                gender: "Мужской".to_string(),
                height: "175 см".to_string(),
                build: "Худощавое".to_string(),
                hair_color: "Седые".to_string(),
                eye_color: "Холодные голубые".to_string(),
                distinctive_features: vec![
                    "Всегда идеально выбрит".to_string(),
                    "Дорогие костюмы".to_string(),
                    "Холодный взгляд".to_string(),
                ],
                clothing_style: "Элитный деловой стиль".to_string(),
                cybernetic_modifications: vec![
                    "Имплант для прямого управления системами NEXUS".to_string(),
                ],
            },
            background: CharacterBackground {
                origin: "Элитная семья физиков".to_string(),
                education: "MIT - ядерная физика, Стэнфорд - компьютерные науки".to_string(),
                career_history: vec![
                    "Исследователь ядерных технологий (1995-2005)".to_string(),
                    "Научный директор NEXUS (2005-2020)".to_string(),
                    "Главный архитектор Digital Apocalypse (2020-настоящее время)".to_string(),
                ],
                major_events: vec![
                    "Создание первого ИИ для управления ядерными реакторами".to_string(),
                    "Разработка концепции цифрового контроля ядерного оружия".to_string(),
                    "Становление главой проекта Digital Apocalypse".to_string(),
                ],
                family: vec!["Жена Маргарет (поддерживает)".to_string(), "Сын Роберт (отчужден)".to_string()],
                trauma: vec!["Потеря коллег в ядерной аварии".to_string()],
                achievements: vec![
                    "Революционные открытия в области ядерной физики".to_string(),
                    "Создание системы Digital Apocalypse".to_string(),
                ],
                regrets: vec!["Иногда сомневается в правильности своих методов".to_string()],
            },
            relationships: HashMap::new(),
            dialogue_state: DialogueState {
                current_conversation: None,
                last_topic: "greeting".to_string(),
                conversation_count: 0,
                relationship_level: -75,
                known_secrets: vec![],
                revealed_information: vec![],
                trust_events: vec![],
            },
            available_services: vec![], // Не предоставляет услуги
            conversation_topics: vec![
                "digital_apocalypse_project".to_string(),
                "nuclear_philosophy".to_string(),
                "world_salvation".to_string(),
                "nexus_vision".to_string(),
                "scientific_progress".to_string(),
                "necessary_sacrifices".to_string(),
            ],
            secrets: vec![
                CharacterSecret {
                    secret_id: "project_timeline".to_string(),
                    content: "Digital Apocalypse запустится автоматически через 72 часа, если его не остановить".to_string(),
                    revelation_requirements: vec!["extreme_interrogation".to_string()],
                    consequences_if_revealed: vec!["Knowledge of exact deadline".to_string()],
                    value: 200,
                },
                CharacterSecret {
                    secret_id: "kill_switch".to_string(),
                    content: "Существует аварийный код отключения, но он может убить его".to_string(),
                    revelation_requirements: vec!["near_death_experience".to_string()],
                    consequences_if_revealed: vec!["Access to emergency shutdown".to_string()],
                    value: 500,
                },
            ],
            goals: vec![
                "Запустить Digital Apocalypse".to_string(),
                "Спасти человечество от самого себя".to_string(),
                "Создать новый мировой порядок".to_string(),
            ],
            fears: vec![
                "Провал миссии".to_string(),
                "Что его поймут неправильно".to_string(),
                "Хаос после его смерти".to_string(),
            ],
            current_mood: Mood::Confident,
            speaking_style: SpeakingStyle {
                formality: 90,
                verbosity: 80,
                directness: 70,
                emotionality: 40,
                slang_usage: 5,
                technical_jargon: 85,
                catchphrases: vec![
                    "Иногда приходится разрушить, чтобы создать".to_string(),
                    "Наука требует жертв".to_string(),
                ],
                accent: "Образованный американский".to_string(),
            },
        }
    }
    
    fn create_dialogue_trees(&mut self) {
        // Создаем диалоговые деревья для каждого персонажа
        self.create_shadow_dialogue_tree();
        self.create_ghost_dialogue_tree();
        self.create_mitchell_dialogue_tree();
        // ... и так далее для всех персонажей
    }
    
    fn create_shadow_dialogue_tree(&mut self) {
        let mut nodes = HashMap::new();
        
        // Начальный узел приветствия
        nodes.insert("greeting".to_string(), DialogueNode {
            node_id: "greeting".to_string(),
            speaker: "shadow_hunter".to_string(),
            text: "Приветствую, товарищ. Я слышал о твоих подвигах. NEXUS становится все более агрессивным - нам нужна помощь.".to_string(),
            text_variations: vec![
                TextVariation {
                    condition: "first_meeting".to_string(),
                    text: "Ты тот самый хакер, о котором все говорят? Меня зовут Тень. Работаю с информацией.".to_string(),
                    mood_modifier: Mood::Suspicious,
                },
                TextVariation {
                    condition: "relationship >= 50".to_string(),
                    text: "Рад тебя видеть, друг. У меня есть свежая информация, которая может тебя заинтересовать.".to_string(),
                    mood_modifier: Mood::Friendly,
                },
            ],
            options: vec![
                DialogueOption {
                    option_id: "ask_about_nexus".to_string(),
                    text: "Расскажи мне о NEXUS".to_string(),
                    requirements: vec![],
                    skill_check: None,
                    consequences: vec![],
                    target_node: "nexus_info".to_string(),
                    relationship_change: 5,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec!["nexus_topic".to_string()],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "ask_personal".to_string(),
                    text: "Расскажи о себе".to_string(),
                    requirements: vec![],
                    skill_check: None,
                    consequences: vec![],
                    target_node: "personal_info".to_string(),
                    relationship_change: 10,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec!["personal_topic".to_string()],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "buy_information".to_string(),
                    text: "Мне нужна информация".to_string(),
                    requirements: vec!["money >= 1000".to_string()],
                    skill_check: None,
                    consequences: vec!["start_shop_interface".to_string()],
                    target_node: "shop_menu".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "leave".to_string(),
                    text: "До свидания".to_string(),
                    requirements: vec![],
                    skill_check: None,
                    consequences: vec!["end_conversation".to_string()],
                    target_node: "goodbye".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
            ],
            conditions: vec![],
            effects: vec![
                DialogueEffect {
                    effect_type: EffectType::RevealSecret,
                    target: "nexus_basic_info".to_string(),
                    value: 1,
                    duration: None,
                },
            ],
            mood_impact: HashMap::new(),
            relationship_impact: 0,
            information_revealed: vec!["NEXUS is more than a corporation".to_string()],
            next_node: None,
        });
        
        // Узел информации о NEXUS
        nodes.insert("nexus_info".to_string(), DialogueNode {
            node_id: "nexus_info".to_string(),
            speaker: "shadow_hunter".to_string(),
            text: "NEXUS - это не просто корпорация. Это сеть влияния, проникшая в правительства по всему миру. Их проект 'Digital Apocalypse' - это попытка захватить контроль над всеми ядерными объектами планеты.".to_string(),
            text_variations: vec![
                TextVariation {
                    condition: "relationship >= 70".to_string(),
                    text: "Слушай внимательно. NEXUS - это раковая опухоль на теле человечества. Они планируют использовать ядерное оружие для шантажа всего мира. У меня есть доказательства.".to_string(),
                    mood_modifier: Mood::Angry,
                },
            ],
            options: vec![
                DialogueOption {
                    option_id: "ask_details".to_string(),
                    text: "Расскажи подробнее о Digital Apocalypse".to_string(),
                    requirements: vec!["trust_level >= 30".to_string()],
                    skill_check: None,
                    consequences: vec!["reveal_major_plot".to_string()],
                    target_node: "digital_apocalypse_details".to_string(),
                    relationship_change: 5,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec!["apocalypse_knowledge".to_string()],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "ask_evidence".to_string(),
                    text: "Какие у тебя есть доказательства?".to_string(),
                    requirements: vec![],
                    skill_check: Some(SkillCheck {
                        skill: "Убеждение".to_string(),
                        difficulty: 40,
                        success_node: "evidence_shown".to_string(),
                        failure_node: "evidence_refused".to_string(),
                        critical_success_node: Some("evidence_plus_bonus".to_string()),
                        critical_failure_node: Some("trust_damaged".to_string()),
                    }),
                    consequences: vec![],
                    target_node: "evidence_check".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "back_to_main".to_string(),
                    text: "Поговорим о чем-то другом".to_string(),
                    requirements: vec![],
                    skill_check: None,
                    consequences: vec![],
                    target_node: "greeting".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
            ],
            conditions: vec![],
            effects: vec![],
            mood_impact: HashMap::new(),
            relationship_impact: 5,
            information_revealed: vec!["NEXUS is more than a corporation".to_string()],
            next_node: None,
        });
        
        // Создаем диалоговое дерево
        let dialogue_tree = DialogueTree {
            tree_id: "shadow_main".to_string(),
            character_id: "shadow_hunter".to_string(),
            root_node: "greeting".to_string(),
            nodes,
            conditions: HashMap::new(),
            variables: HashMap::new(),
        };
        
        self.dialogue_trees.insert("shadow_main".to_string(), dialogue_tree);
    }
    
    fn create_ghost_dialogue_tree(&mut self) {
        let mut nodes = HashMap::new();
        
        nodes.insert("greeting".to_string(), DialogueNode {
            node_id: "greeting".to_string(),
            speaker: "ghost_in_shell".to_string(),
            text: "Ты тот самый хакер, о котором все говорят? Интересно... Я 'Призрак'. Возможно, мы сможем сотрудничать.".to_string(),
            text_variations: vec![
                TextVariation {
                    condition: "relationship >= 60".to_string(),
                    text: "О, это ты. Я начинаю думать, что ты не так плох, как большинство. Что тебе нужно?".to_string(),
                    mood_modifier: Mood::Friendly,
                },
                TextVariation {
                    condition: "relationship <= -20".to_string(),
                    text: "Ты снова здесь? Я же говорила - я работаю одна. Не трать мое время.".to_string(),
                    mood_modifier: Mood::Hostile,
                },
            ],
            options: vec![
                DialogueOption {
                    option_id: "compliment_skills".to_string(),
                    text: "Твои навыки впечатляют".to_string(),
                    requirements: vec![],
                    skill_check: Some(SkillCheck {
                        skill: "Харизма".to_string(),
                        difficulty: 50,
                        success_node: "flattered".to_string(),
                        failure_node: "suspicious_compliment".to_string(),
                        critical_success_node: None,
                        critical_failure_node: None,
                    }),
                    consequences: vec![],
                    target_node: "compliment_check".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "ask_cooperation".to_string(),
                    text: "Хочешь поработать вместе?".to_string(),
                    requirements: vec!["relationship >= 40".to_string()],
                    skill_check: None,
                    consequences: vec!["unlock_partnership".to_string()],
                    target_node: "partnership_offer".to_string(),
                    relationship_change: 10,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec!["ghost_partnership".to_string()],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "challenge_skills".to_string(),
                    text: "Думаешь, ты лучший хакер?".to_string(),
                    requirements: vec!["hacking_skill >= 70".to_string()],
                    skill_check: Some(SkillCheck {
                        skill: "Взлом".to_string(),
                        difficulty: 80,
                        success_node: "impressed".to_string(),
                        failure_node: "unimpressed".to_string(),
                        critical_success_node: Some("very_impressed".to_string()),
                        critical_failure_node: Some("mocked".to_string()),
                    }),
                    consequences: vec![],
                    target_node: "skill_challenge".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
            ],
            conditions: vec![],
            effects: vec![],
            mood_impact: HashMap::new(),
            relationship_impact: 0,
            information_revealed: vec![],
            next_node: None,
        });
        
        let dialogue_tree = DialogueTree {
            tree_id: "ghost_main".to_string(),
            character_id: "ghost_in_shell".to_string(),
            root_node: "greeting".to_string(),
            nodes,
            conditions: HashMap::new(),
            variables: HashMap::new(),
        };
        
        self.dialogue_trees.insert("ghost_main".to_string(), dialogue_tree);
    }
    
    fn create_mitchell_dialogue_tree(&mut self) {
        let mut nodes = HashMap::new();
        
        nodes.insert("greeting".to_string(), DialogueNode {
            node_id: "greeting".to_string(),
            speaker: "nuclear_prophet".to_string(),
            text: "Так значит ты тот самый хакер, который досаждает нашей безопасности? Впечатляюще... но бесполезно.".to_string(),
            text_variations: vec![
                TextVariation {
                    condition: "final_confrontation".to_string(),
                    text: "Ты дошел до самого конца. Впечатляюще. Но ты опоздал - Digital Apocalypse уже не остановить.".to_string(),
                    mood_modifier: Mood::Confident,
                },
            ],
            options: vec![
                DialogueOption {
                    option_id: "demand_stop".to_string(),
                    text: "Останови этот безумный проект!".to_string(),
                    requirements: vec![],
                    skill_check: None,
                    consequences: vec![],
                    target_node: "project_justification".to_string(),
                    relationship_change: -5,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "try_reasoning".to_string(),
                    text: "Подумай о последствиях для человечества".to_string(),
                    requirements: vec![],
                    skill_check: Some(SkillCheck {
                        skill: "Убеждение".to_string(),
                        difficulty: 90,
                        success_node: "momentary_doubt".to_string(),
                        failure_node: "reasoning_rejected".to_string(),
                        critical_success_node: Some("deep_doubt_planted".to_string()),
                        critical_failure_node: Some("hardened_resolve".to_string()),
                    }),
                    consequences: vec![],
                    target_node: "reasoning_attempt".to_string(),
                    relationship_change: 0,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
                DialogueOption {
                    option_id: "threaten".to_string(),
                    text: "У меня есть способы заставить тебя".to_string(),
                    requirements: vec!["intimidation_skill >= 60".to_string()],
                    skill_check: Some(SkillCheck {
                        skill: "Запугивание".to_string(),
                        difficulty: 70,
                        success_node: "reveals_weakness".to_string(),
                        failure_node: "laughs_off_threat".to_string(),
                        critical_success_node: Some("breaks_under_pressure".to_string()),
                        critical_failure_node: Some("calls_security".to_string()),
                    }),
                    consequences: vec![],
                    target_node: "threat_response".to_string(),
                    relationship_change: -10,
                    faction_reputation_change: HashMap::new(),
                    unlocks: vec![],
                    locks: vec![],
                },
            ],
            conditions: vec![],
            effects: vec![],
            mood_impact: HashMap::new(),
            relationship_impact: 0,
            information_revealed: vec![],
            next_node: None,
        });
        
        let dialogue_tree = DialogueTree {
            tree_id: "mitchell_confrontation".to_string(),
            character_id: "nuclear_prophet".to_string(),
            root_node: "greeting".to_string(),
            nodes,
            conditions: HashMap::new(),
            variables: HashMap::new(),
        };
        
        self.dialogue_trees.insert("mitchell_confrontation".to_string(), dialogue_tree);
    }
    
    // Добавим методы для других персонажей...
    fn create_nexus_ceo(&self) -> Character {
        Character {
            id: "nexus_ceo".to_string(),
            name: "Маргарет Стерлинг".to_string(),
            handle: "iron_lady".to_string(),
            faction: "NEXUS Corporation".to_string(),
            role: "Генеральный директор NEXUS".to_string(),
            personality: CharacterPersonality {
                openness: 40,
                conscientiousness: 95,
                extraversion: 80,
                agreeableness: 25,
                neuroticism: 20,
                intelligence: 90,
                morality: 30,
                ambition: 100,
                paranoia: 70,
                loyalty: 85,
            },
            appearance: CharacterAppearance {
                age: 48,
                gender: "Женский".to_string(),
                height: "172 см".to_string(),
                build: "Элегантное".to_string(),
                hair_color: "Серебристо-серые".to_string(),
                eye_color: "Стальные серые".to_string(),
                distinctive_features: vec![
                    "Всегда безупречный макияж".to_string(),
                    "Дорогие украшения".to_string(),
                    "Властный взгляд".to_string(),
                ],
                clothing_style: "Люксовые деловые костюмы".to_string(),
                cybernetic_modifications: vec![
                    "Имплант для мониторинга корпоративных показателей".to_string(),
                ],
            },
            background: CharacterBackground {
                origin: "Династия промышленников".to_string(),
                education: "Гарвард - MBA, Wharton - финансы".to_string(),
                career_history: vec![
                    "Инвестиционный банкир (2000-2010)".to_string(),
                    "Вице-президент NEXUS (2010-2018)".to_string(),
                    "Генеральный директор NEXUS (2018-настоящее время)".to_string(),
                ],
                major_events: vec![
                    "Захват контроля в NEXUS через враждебное поглощение".to_string(),
                    "Запуск программы глобальной экспансии".to_string(),
                    "Одобрение проекта Digital Apocalypse".to_string(),
                ],
                family: vec!["Муж Ричард (тоже в бизнесе)".to_string(), "Дочь Элизабет (учится в Оксфорде)".to_string()],
                trauma: vec!["Потеря первого ребенка".to_string()],
                achievements: vec!["Превратила NEXUS в глобальную империю".to_string()],
                regrets: vec!["Мало времени проводит с семьей".to_string()],
            },
            relationships: HashMap::new(),
            dialogue_state: DialogueState {
                current_conversation: None,
                last_topic: "greeting".to_string(),
                conversation_count: 0,
                relationship_level: -50,
                known_secrets: vec![],
                revealed_information: vec![],
                trust_events: vec![],
            },
            available_services: vec![],
            conversation_topics: vec![
                "corporate_strategy".to_string(),
                "global_economy".to_string(),
                "nexus_future".to_string(),
                "business_philosophy".to_string(),
                "family_values".to_string(),
            ],
            secrets: vec![
                CharacterSecret {
                    secret_id: "board_opposition".to_string(),
                    content: "Половина совета директоров против Digital Apocalypse".to_string(),
                    revelation_requirements: vec!["extreme_pressure".to_string()],
                    consequences_if_revealed: vec!["Internal NEXUS conflict".to_string()],
                    value: 100,
                },
            ],
            goals: vec![
                "Обеспечить прибыльность NEXUS".to_string(),
                "Построить корпоративную империю".to_string(),
                "Обеспечить будущее дочери".to_string(),
            ],
            fears: vec![
                "Потерять контроль над компанией".to_string(),
                "Экономический крах".to_string(),
                "Угроза семье".to_string(),
            ],
            current_mood: Mood::Confident,
            speaking_style: SpeakingStyle {
                formality: 95,
                verbosity: 70,
                directness: 85,
                emotionality: 25,
                slang_usage: 0,
                technical_jargon: 60,
                catchphrases: vec![
                    "Время - деньги".to_string(),
                    "Результат превыше всего".to_string(),
                ],
                accent: "Британский элитный".to_string(),
            },
        }
    }
    
    // Здесь должны быть методы для создания всех остальных персонажей...
    // create_government_director, create_cyber_freedom_leader, и т.д.
    
    pub fn start_conversation(&mut self, character_id: &str, player_id: &str) -> Option<DialogueNode> {
        if let Some(dialogue_tree) = self.dialogue_trees.get(&format!("{}_main", character_id))
            .or_else(|| self.dialogue_trees.get(&format!("{}_confrontation", character_id))) {
            
            self.active_conversations.insert(player_id.to_string(), character_id.to_string());
            
            if let Some(root_node) = dialogue_tree.nodes.get(&dialogue_tree.root_node) {
                return Some(root_node.clone());
            }
        }
        None
    }
    
    pub fn process_dialogue_choice(&mut self, player_id: &str, choice_id: &str, game_state: &mut GameState) -> Option<DialogueNode> {
        if let Some(character_id) = self.active_conversations.get(player_id).cloned() {
            if let Some(dialogue_tree) = self.dialogue_trees.get(&format!("{}_main", character_id))
                .or_else(|| self.dialogue_trees.get(&format!("{}_confrontation", character_id))) {
                
                // Найти выбранную опцию и перейти к следующему узлу
                for node in dialogue_tree.nodes.values() {
                    for option in &node.options {
                        if option.option_id == choice_id {
                            // Проверить требования
                            if self.check_option_requirements(&option.requirements, game_state) {
                                // Применить эффекты выбора
                                self.apply_dialogue_effects(option, &character_id, game_state);
                                
                                // Вернуть следующий узел
                                return dialogue_tree.nodes.get(&option.target_node).cloned();
                            }
                        }
                    }
                }
            }
        }
        None
    }
    
    fn check_option_requirements(&self, requirements: &[String], game_state: &GameState) -> bool {
        for requirement in requirements {
            if !self.evaluate_requirement(requirement, game_state) {
                return false;
            }
        }
        true
    }
    
    fn evaluate_requirement(&self, requirement: &str, game_state: &GameState) -> bool {
        // Простая проверка требований
        if requirement.starts_with("money >= ") {
            if let Ok(amount) = requirement.replace("money >= ", "").parse::<u32>() {
                return game_state.money >= amount;
            }
        } else if requirement.starts_with("hacking_skill >= ") {
            if let Ok(level) = requirement.replace("hacking_skill >= ", "").parse::<u32>() {
                let skill = *game_state.skills.get("Взлом").unwrap_or(&0);
                return skill >= level;
            }
        }
        // Добавить больше проверок по мере необходимости
        true
    }
    
    fn apply_dialogue_effects(&mut self, option: &DialogueOption, character_id: &str, game_state: &mut GameState) {
        // Изменить отношения с персонажем
        let current_relationship = *self.relationship_tracker.get(character_id).unwrap_or(&0);
        self.relationship_tracker.insert(character_id.to_string(), current_relationship + option.relationship_change);
        
        // Изменить репутацию с фракциями
        for (faction, change) in &option.faction_reputation_change {
            // Здесь должна быть логика изменения репутации с фракциями
        }
        
        // Применить другие эффекты
        for effect in &option.consequences {
            self.apply_consequence(effect, game_state);
        }
    }
    
    fn apply_consequence(&self, consequence: &str, game_state: &mut GameState) {
        match consequence {
            "end_conversation" => {
                // Завершить разговор
            },
            "start_shop_interface" => {
                // Открыть магазин
            },
            "unlock_partnership" => {
                // Разблокировать партнерство
            },
            _ => {}
        }
    }
    
    pub fn get_character_relationship(&self, character_id: &str) -> i32 {
        *self.relationship_tracker.get(character_id).unwrap_or(&0)
    }
    
    pub fn update_character_mood(&mut self, character_id: &str, new_mood: Mood) {
        if let Some(character) = self.characters.get_mut(character_id) {
            character.current_mood = new_mood;
        }
    }
} 