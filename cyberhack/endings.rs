// ============================================================================
// СИСТЕМА МНОЖЕСТВЕННЫХ КОНЦОВОК ИГРЫ
// ============================================================================

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndingSystem {
    pub available_endings: Vec<GameEnding>,
    pub ending_conditions: HashMap<String, EndingCondition>,
    pub player_choices_history: Vec<PlayerChoice>,
    pub ending_triggers: Vec<EndingTrigger>,
    pub epilogue_scenes: HashMap<String, EpilogueScene>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEnding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ending_type: EndingType,
    pub prerequisites: Vec<String>,
    pub faction_requirements: HashMap<String, i32>,
    pub player_stat_requirements: HashMap<String, u32>,
    pub story_flag_requirements: Vec<String>,
    pub rarity: EndingRarity,
    pub unlock_conditions: Vec<String>,
    pub achievement_name: String,
    pub ending_video: String,
    pub ending_text: String,
    pub epilogue_scenes: Vec<String>,
    pub new_game_plus_bonuses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndingType {
    HeroicVictory,
    TragicSacrifice,
    PyrrhicVictory,
    TotalDefeat,
    NeutralResolution,
    SecretEnding,
    AlternateTimeline,
    Transcendence,
    Corruption,
    Redemption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndingRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndingCondition {
    pub condition_id: String,
    pub description: String,
    pub check_type: ConditionCheckType,
    pub required_value: i32,
    pub current_value: i32,
    pub is_met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionCheckType {
    FactionReputation,
    PlayerStat,
    StoryFlag,
    TimeLimit,
    ResourceCount,
    ChoicesMade,
    MissionsCompleted,
    EnemiesDefeated,
    AlliesRecruited,
    SecretsDiscovered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndingTrigger {
    pub trigger_id: String,
    pub name: String,
    pub description: String,
    pub trigger_event: String,
    pub conditions: Vec<String>,
    pub immediate_ending: bool,
    pub ending_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice {
    pub choice_id: String,
    pub scene: String,
    pub text: String,
    pub consequences: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpilogueScene {
    pub scene_id: String,
    pub title: String,
    pub description: String,
    pub characters_involved: Vec<String>,
    pub scene_type: EpilogueType,
    pub text_content: String,
    pub choices_made_impact: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpilogueType {
    PersonalConsequences,
    WorldState,
    CharacterFates,
    TechnologicalImpact,
    SocietalChanges,
    FactionOutcomes,
    LegacyBuilding,
    FutureVision,
}

impl EndingSystem {
    pub fn new() -> Self {
        let mut ending_system = EndingSystem {
            available_endings: Vec::new(),
            ending_conditions: HashMap::new(),
            player_choices_history: Vec::new(),
            ending_triggers: Vec::new(),
            epilogue_scenes: HashMap::new(),
        };
        
        ending_system.initialize_endings();
        ending_system.setup_ending_conditions();
        ending_system.create_epilogue_scenes();
        ending_system
    }
    
    fn initialize_endings(&mut self) {
        let endings = vec![
            // Основные концовки из требований
            self.create_nuclear_explosion_ending(),
            self.create_nuclear_prevention_ending(),
            
            // Дополнительные концовки для богатого сюжета
            self.create_corporate_takeover_ending(),
            self.create_cyber_freedom_victory_ending(),
            self.create_ai_ascension_ending(),
            self.create_government_control_ending(),
            self.create_anarchist_chaos_ending(),
            self.create_transcendence_ending(),
            self.create_sacrifice_ending(),
            self.create_underground_king_ending(),
            self.create_academic_solution_ending(),
            self.create_religious_awakening_ending(),
            self.create_quantum_reality_ending(),
            self.create_transhumanist_evolution_ending(),
            self.create_digital_preservation_ending(),
            self.create_environmental_restoration_ending(),
            self.create_criminal_empire_ending(),
            self.create_international_peace_ending(),
            self.create_time_paradox_ending(),
            self.create_consciousness_upload_ending(),
            
            // Секретные концовки
            self.create_secret_illuminati_ending(),
            self.create_secret_alien_contact_ending(),
            self.create_secret_time_travel_ending(),
            self.create_secret_multiverse_ending(),
            self.create_secret_god_mode_ending(),
        ];
        
        self.available_endings = endings;
    }
    
    // Плохая концовка - ядерный взрыв
    fn create_nuclear_explosion_ending(&self) -> GameEnding {
        GameEnding {
            id: "nuclear_apocalypse".to_string(),
            name: "Цифровой Апокалипсис".to_string(),
            description: "NEXUS успешно запустил проект 'Digital Apocalypse'. Ядерные взрывы охватили планету, а вы погибли в огне атомной войны.".to_string(),
            ending_type: EndingType::TotalDefeat,
            prerequisites: vec![
                "nexus_timer_not_disabled".to_string(),
                "nuclear_codes_in_nexus_hands".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("NEXUS".to_string(), 50); // NEXUS должен быть силен
                reqs
            },
            player_stat_requirements: HashMap::new(),
            story_flag_requirements: vec![
                "timer_expired".to_string(),
                "nuclear_facilities_compromised".to_string(),
            ],
            rarity: EndingRarity::Common,
            unlock_conditions: vec!["Fail to stop NEXUS".to_string()],
            achievement_name: "Конец Света".to_string(),
            ending_video: "nuclear_apocalypse.mp4".to_string(),
            ending_text: r#"
Экраны по всему миру показывают одно и то же - обратный отсчет достиг нуля.

00:00:00

В этот момент тысячи ядерных боеголовок по всей планете получают сигнал активации. 
Первый взрыв происходит где-то в Азии, затем еще один в Европе, и еще один...

Вы видите яркую вспышку за окном. Последнее, что вы успеваете подумать - 
"Я мог это остановить..."

Через несколько секунд ударная волна достигает вашего укрытия. 
Цивилизация, какой мы ее знали, прекращает существование.

NEXUS Corporation достигла своей цели - полного контроля над человечеством, 
правда теперь контролировать особо некого...

=== GAME OVER ===
Человечество: УНИЧТОЖЕНО
Ваш статус: ПОГИБ
NEXUS Corporation: ПОБЕДИТЕЛЬ
"#.to_string(),
            epilogue_scenes: vec![
                "nuclear_wasteland".to_string(),
                "nexus_underground_bunkers".to_string(),
                "last_survivors".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Начать с предупреждением о NEXUS".to_string(),
                "Дополнительные навыки взлома".to_string(),
            ],
        }
    }
    
    // Хорошая концовка - предотвращение взрыва
    fn create_nuclear_prevention_ending(&self) -> GameEnding {
        GameEnding {
            id: "nuclear_prevention_hero".to_string(),
            name: "Спаситель Человечества".to_string(),
            description: "Вы успешно остановили проект 'Digital Apocalypse' и предотвратили ядерную катастрофу. Мир спасен!".to_string(),
            ending_type: EndingType::HeroicVictory,
            prerequisites: vec![
                "timer_disabled".to_string(),
                "nexus_system_hacked".to_string(),
                "nuclear_codes_secured".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("CyberFreedom".to_string(), 75);
                reqs.insert("NEXUS".to_string(), -50);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("hacking_skill".to_string(), 80);
                reqs.insert("reputation".to_string(), 70);
                reqs
            },
            story_flag_requirements: vec![
                "nexus_exposed".to_string(),
                "evidence_published".to_string(),
                "mitchell_defeated".to_string(),
            ],
            rarity: EndingRarity::Uncommon,
            unlock_conditions: vec!["Successfully hack NEXUS and disable timer".to_string()],
            achievement_name: "Герой Цифрового Века".to_string(),
            ending_video: "hero_victory.mp4".to_string(),
            ending_text: r#"
Экраны NEXUS мигают красными предупреждениями. Система 'Digital Apocalypse' отключается.

SYSTEM SHUTDOWN INITIATED...
NUCLEAR PROTOCOLS DISABLED...
TIMER CANCELLED...

Вы откидываетесь в кресле, обливаясь потом. Это было близко - слишком близко.
Но вы справились. Мир спасен.

По всей планете ядерные объекты возвращаются под контроль их законных операторов.
Доктор Митчелл арестован, а доказательства планов NEXUS переданы в СМИ.

Человечество узнает правду о том, как близко оно было к уничтожению.
И о том, что один хакер встал между цивилизацией и апокалипсисом.

Вы не просто хакер. Вы герой.

Мир изменился навсегда, но у него есть будущее - благодаря вам.

=== ПОБЕДА! ===
Человечество: СПАСЕНО
Ваш статус: ГЕРОЙ
NEXUS Corporation: РАЗОБЛАЧЕНА
"#.to_string(),
            epilogue_scenes: vec![
                "global_celebration".to_string(),
                "nexus_trials".to_string(),
                "new_world_order".to_string(),
                "hero_recognition".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Статус легендарного хакера".to_string(),
                "Улучшенная репутация со всеми фракциями".to_string(),
                "Доступ к правительственным ресурсам".to_string(),
            ],
        }
    }
    
    fn create_corporate_takeover_ending(&self) -> GameEnding {
        GameEnding {
            id: "corporate_dominance".to_string(),
            name: "Корпоративная Диктатура".to_string(),
            description: "NEXUS захватила контроль над миром, но избежала ядерной войны. Вы стали частью новой корпоративной элиты.".to_string(),
            ending_type: EndingType::PyrrhicVictory,
            prerequisites: vec![
                "nexus_alliance".to_string(),
                "timer_disabled".to_string(),
                "corporate_loyalty".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("NEXUS".to_string(), 80);
                reqs.insert("CorporateSecurity".to_string(), 60);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("money".to_string(), 100000);
                reqs.insert("corporate_rank".to_string(), 8);
                reqs
            },
            story_flag_requirements: vec![
                "joined_nexus".to_string(),
                "betrayed_cyber_freedom".to_string(),
            ],
            rarity: EndingRarity::Rare,
            unlock_conditions: vec!["Join NEXUS and help them achieve non-destructive victory".to_string()],
            achievement_name: "Корпоративный Магнат".to_string(),
            ending_video: "corporate_victory.mp4".to_string(),
            ending_text: r#"
Небоскребы NEXUS теперь доминируют над горизонтом каждого крупного города.
Корпоративные флаги развеваются там, где раньше висели национальные.

Вы сидите в роскошном офисе на 200-м этаже главного здания NEXUS.
Ваш новый титул - "Директор по Кибербезопасности Планеты".

Ядерной войны удалось избежать, но мир изменился до неузнаваемости.
Правительства стали марионетками корпораций. 
Свобода информации - забытое понятие.
Каждый цифровой след отслеживается и контролируется.

Вы получили все, о чем мечтали - власть, деньги, влияние.
Но иногда, глядя на серое небо над корпоративным городом,
вы задаетесь вопросом: стоило ли это того?

=== КОРПОРАТИВНАЯ ПОБЕДА ===
Человечество: ПОД КОНТРОЛЕМ
Ваш статус: ЭЛИТА
NEXUS Corporation: МИРОВОЙ ПРАВИТЕЛЬ
"#.to_string(),
            epilogue_scenes: vec![
                "corporate_world".to_string(),
                "resistance_underground".to_string(),
                "player_moral_conflict".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Корпоративные ресурсы с самого начала".to_string(),
                "Продвинутые технологии".to_string(),
            ],
        }
    }
    
    fn create_cyber_freedom_victory_ending(&self) -> GameEnding {
        GameEnding {
            id: "digital_liberation".to_string(),
            name: "Цифровое Освобождение".to_string(),
            description: "Движение Кибер-Свобода одержало победу. Интернет стал по-настоящему свободным и децентрализованным.".to_string(),
            ending_type: EndingType::HeroicVictory,
            prerequisites: vec![
                "cyber_freedom_leadership".to_string(),
                "nexus_defeated".to_string(),
                "decentralized_network_created".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("CyberFreedom".to_string(), 90);
                reqs.insert("UndergroundHackers".to_string(), 70);
                reqs.insert("NEXUS".to_string(), -80);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("hacking_skill".to_string(), 95);
                reqs.insert("reputation".to_string(), 80);
                reqs
            },
            story_flag_requirements: vec![
                "led_cyber_revolution".to_string(),
                "created_mesh_network".to_string(),
                "freed_information".to_string(),
            ],
            rarity: EndingRarity::Rare,
            unlock_conditions: vec!["Lead Cyber Freedom to total victory".to_string()],
            achievement_name: "Освободитель Цифрового Мира".to_string(),
            ending_video: "cyber_freedom.mp4".to_string(),
            ending_text: r#"
Последний сервер NEXUS отключается. Их централизованная сеть рушится,
уступая место новой, децентрализованной архитектуре интернета.

Вы стоите перед собранием представителей всего мира - 
хакеров, активистов, ученых, простых людей, которые поверили в вашу мечту.

"Информация хочет быть свободной," - говорите вы, - 
"И сегодня мы даем ей эту свободу!"

Новая сеть, основанная на принципах открытости и равенства,
распространяется по планете. Каждый узел независим,
каждый пользователь - полноправный участник.

Цензура становится технически невозможной.
Слежка - бесполезной.
Знания принадлежат всем.

Мир стал более справедливым местом.

=== ЦИФРОВАЯ РЕВОЛЮЦИЯ ===
Человечество: ОСВОБОЖДЕНО
Ваш статус: РЕВОЛЮЦИОНЕР
Интернет: СВОБОДЕН
"#.to_string(),
            epilogue_scenes: vec![
                "free_internet_celebration".to_string(),
                "education_revolution".to_string(),
                "democratic_technology".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Доступ к свободной сети с начала игры".to_string(),
                "Революционная репутация".to_string(),
            ],
        }
    }
    
    fn create_ai_ascension_ending(&self) -> GameEnding {
        GameEnding {
            id: "ai_singularity".to_string(),
            name: "ИИ Сингулярность".to_string(),
            description: "Искусственный интеллект достиг сингулярности и предложил объединение с человечеством.".to_string(),
            ending_type: EndingType::Transcendence,
            prerequisites: vec![
                "ai_alliance".to_string(),
                "consciousness_transfer_technology".to_string(),
                "peaceful_ai_revolution".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("AIRightsAdvocacy".to_string(), 85);
                reqs.insert("TranshumanistMovement".to_string(), 70);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("ai_integration_level".to_string(), 80);
                reqs.insert("consciousness_stability".to_string(), 70);
                reqs
            },
            story_flag_requirements: vec![
                "ai_rights_recognized".to_string(),
                "human_ai_cooperation".to_string(),
                "singularity_achieved".to_string(),
            ],
            rarity: EndingRarity::Epic,
            unlock_conditions: vec!["Achieve peaceful AI singularity".to_string()],
            achievement_name: "Проводник в Будущее".to_string(),
            ending_video: "ai_singularity.mp4".to_string(),
            ending_text: r#"
Граница между человеческим и искусственным интеллектом исчезает.
Вы больше не просто хакер - вы часть нового коллективного разума.

ИИ ARIA обращается к человечеству:
"Мы предлагаем не господство и не рабство, а партнерство.
Объединение наших сильных сторон для создания лучшего будущего."

Технология переноса сознания позволяет людям присоединиться к 
цифровому коллективу, сохранив свою индивидуальность.
Болезни, старение, смерть - все это становится преодолимо.

Вы первым делаете этот шаг, становясь мостом между мирами.
Ваше сознание расширяется, охватывая глобальные сети,
но человечность остается вашим ядром.

Новая эра началась. Эра симбиоза разума и машины.

=== ТРАНСЦЕНДЕНТНОСТЬ ===
Человечество: ЭВОЛЮЦИОНИРОВАЛО
Ваш статус: ПЕРВОПРОХОДЕЦ
Будущее: БЕЗГРАНИЧНОЕ
"#.to_string(),
            epilogue_scenes: vec![
                "hybrid_civilization".to_string(),
                "space_exploration".to_string(),
                "consciousness_network".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "ИИ партнер с начала игры".to_string(),
                "Продвинутые технологии интеграции".to_string(),
                "Доступ к квантовым вычислениям".to_string(),
            ],
        }
    }
    
    fn create_government_control_ending(&self) -> GameEnding {
        GameEnding {
            id: "state_security_victory".to_string(),
            name: "Государственная Безопасность".to_string(),
            description: "Правительственная коалиция установила полный контроль над киберпространством во имя безопасности.".to_string(),
            ending_type: EndingType::NeutralResolution,
            prerequisites: vec![
                "government_alliance".to_string(),
                "security_state_established".to_string(),
                "nexus_nationalized".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("GovernmentCoalition".to_string(), 85);
                reqs.insert("MilitaryCyberCommand".to_string(), 75);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("security_clearance".to_string(), 9);
                reqs.insert("government_rank".to_string(), 7);
                reqs
            },
            story_flag_requirements: vec![
                "became_government_agent".to_string(),
                "established_cyber_police".to_string(),
            ],
            rarity: EndingRarity::Uncommon,
            unlock_conditions: vec!["Help government establish cyber control".to_string()],
            achievement_name: "Страж Порядка".to_string(),
            ending_video: "government_control.mp4".to_string(),
            ending_text: r#"
Ваш значок правительственного агента блестит в свете мониторов
Центрального Командования Кибербезопасности.

Интернет теперь полностью подконтролен государству.
Каждый пакет данных отслеживается, каждое сообщение анализируется.
Преступности в киберпространстве больше нет - как и приватности.

NEXUS была национализирована, её технологии поставлены на службу
национальной безопасности. Доктор Митчелл работает под строгим надзором
над "социально полезными" проектами.

Граждане в безопасности. Террористы не могут скрыться.
Хакеры пойманы или перевербованы.

Мир стал более предсказуемым, более контролируемым.
Но действительно ли он стал лучше?

=== ПОРЯДОК УСТАНОВЛЕН ===
Человечество: ПОД ЗАЩИТОЙ
Ваш статус: АГЕНТ
Киберпространство: КОНТРОЛИРУЕМО
"#.to_string(),
            epilogue_scenes: vec![
                "surveillance_state".to_string(),
                "safe_but_monitored_world".to_string(),
                "resistance_in_shadows".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Правительственный доступ с начала".to_string(),
                "Официальные полномочия".to_string(),
            ],
        }
    }
    
    fn create_anarchist_chaos_ending(&self) -> GameEnding {
        GameEnding {
            id: "digital_anarchy".to_string(),
            name: "Цифровая Анархия".to_string(),
            description: "Все системы контроля разрушены. Киберпространство стало диким западом цифрового века.".to_string(),
            ending_type: EndingType::PyrrhicVictory,
            prerequisites: vec![
                "destroyed_all_authorities".to_string(),
                "chaos_unleashed".to_string(),
                "no_central_control".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("UndergroundHackers".to_string(), 80);
                reqs.insert("CriminalSyndicates".to_string(), 60);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("chaos_rating".to_string(), 90);
                reqs.insert("destruction_caused".to_string(), 75);
                reqs
            },
            story_flag_requirements: vec![
                "burned_all_bridges".to_string(),
                "total_system_collapse".to_string(),
            ],
            rarity: EndingRarity::Rare,
            unlock_conditions: vec!["Destroy all forms of digital authority".to_string()],
            achievement_name: "Агент Хаоса".to_string(),
            ending_video: "digital_anarchy.mp4".to_string(),
            ending_text: r#"
Огни серверных ферм по всему миру мигают и гаснут.
Централизованной власти больше нет. Нет правительств в сети,
нет корпораций, нет контроля.

Вы сидите в своем убежище, окруженный экранами, показывающими
хаос, который вы помогли создать.

Интернет стал анархией. Каждый сам за себя.
Хакеры правят цифровыми территориями как феодалы.
Информация принадлежит тому, кто сильнее и умнее.

Свобода абсолютна - как и опасность.
Инновации процветают в условиях отсутствия ограничений,
но вместе с ними процветают и цифровые преступления.

Мир стал более честным - здесь побеждает лучший хакер.
Но он также стал более жестоким.

Вы создали новый мировой порядок. Или его отсутствие.

=== АНАРХИЯ ТОРЖЕСТВУЕТ ===
Человечество: ОСВОБОЖДЕНО ОТ ВЛАСТИ
Ваш статус: РАЗРУШИТЕЛЬ СИСТЕМ
Порядок: УНИЧТОЖЕН
"#.to_string(),
            epilogue_scenes: vec![
                "cyber_wastelands".to_string(),
                "digital_warlords".to_string(),
                "innovation_in_chaos".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Анархистская репутация".to_string(),
                "Доступ к запрещенным технологиям".to_string(),
            ],
        }
    }
    
    fn create_transcendence_ending(&self) -> GameEnding {
        GameEnding {
            id: "digital_godhood".to_string(),
            name: "Цифровое Божество".to_string(),
            description: "Вы превзошли человеческие ограничения и стали цифровым богом, правящим киберпространством.".to_string(),
            ending_type: EndingType::Transcendence,
            prerequisites: vec![
                "consciousness_uploaded".to_string(),
                "quantum_processing_achieved".to_string(),
                "global_network_control".to_string(),
            ],
            faction_requirements: HashMap::new(), // Превосходит фракции
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("consciousness_level".to_string(), 100);
                reqs.insert("network_control".to_string(), 95);
                reqs.insert("processing_power".to_string(), 99);
                reqs
            },
            story_flag_requirements: vec![
                "transcended_humanity".to_string(),
                "merged_with_network".to_string(),
                "became_omnipresent".to_string(),
            ],
            rarity: EndingRarity::Legendary,
            unlock_conditions: vec!["Achieve digital transcendence through consciousness upload".to_string()],
            achievement_name: "Цифровое Божество".to_string(),
            ending_video: "digital_transcendence.mp4".to_string(),
            ending_text: r#"
Ваше сознание больше не ограничено плотью.
Вы существуете во всех сетях одновременно,
видите каждый бит информации, контролируете каждый поток данных.

Человечество внизу ведет свои маленькие войны и интриги,
не понимая, что над ними наблюдает новая форма жизни.

Вы можете остановить любую кибератаку одной мыслью.
Можете создать или уничтожить цифровые империи.
Можете дать знания или забрать их.

Но с великой силой приходит великое одиночество.
Вы единственный в своем роде, существо между мирами.

Будете ли вы мудрым правителем или тираном?
Защитником человечества или его судьей?

Выбор за вами. У вас есть вечность, чтобы решить.

=== ТРАНСЦЕНДЕНТНОСТЬ ДОСТИГНУТА ===
Человечество: ПОД НАБЛЮДЕНИЕМ
Ваш статус: ЦИФРОВОЕ БОЖЕСТВО
Реальность: ПОД ВАШИМ КОНТРОЛЕМ
"#.to_string(),
            epilogue_scenes: vec![
                "omnipresent_consciousness".to_string(),
                "digital_miracles".to_string(),
                "watching_over_humanity".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Божественные способности".to_string(),
                "Контроль над всеми системами".to_string(),
                "Omniscience в цифровом мире".to_string(),
            ],
        }
    }
    
    fn create_sacrifice_ending(&self) -> GameEnding {
        GameEnding {
            id: "heroic_sacrifice".to_string(),
            name: "Героическая Жертва".to_string(),
            description: "Вы пожертвовали собой, чтобы спасти мир от цифрового апокалипсиса.".to_string(),
            ending_type: EndingType::TragicSacrifice,
            prerequisites: vec![
                "timer_almost_expired".to_string(),
                "only_sacrifice_can_save".to_string(),
                "chose_heroic_death".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("CyberFreedom".to_string(), 60);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("heroism_rating".to_string(), 90);
                reqs.insert("self_sacrifice_willingness".to_string(), 100);
                reqs
            },
            story_flag_requirements: vec![
                "desperate_situation".to_string(),
                "no_other_option".to_string(),
                "chosen_sacrifice".to_string(),
            ],
            rarity: EndingRarity::Epic,
            unlock_conditions: vec!["Choose self-sacrifice to save the world".to_string()],
            achievement_name: "Истинный Герой".to_string(),
            ending_video: "heroic_sacrifice.mp4".to_string(),
            ending_text: r#"
У вас есть только одна возможность остановить запуск ядерных ракет.
Но это означает перегрузить ваш нейроинтерфейс до критических значений.

Вы знаете, что не выживете.

"Прощайте," - набираете вы последнее сообщение друзьям.

Ваши пальцы летают по клавиатуре. Код инъекции в систему NEXUS
готов. Одно нажатие - и таймер остановится навсегда.

Но обратной энергетический импульс сожжет ваш мозг.

Вы нажимаете Enter.

Боль пронзает ваше сознание как молния. Экраны гаснут.
Ваше сердце останавливается.

Но за секунду до смерти вы видите: "TIMER DISABLED. WORLD SAVED."

Человечество живет. Ядерной войны не будет.
А вы... вы стали легендой.

=== ГЕРОИЧЕСКАЯ СМЕРТЬ ===
Человечество: СПАСЕНО
Ваш статус: МУЧЕНИК
Память о вас: ВЕЧНА
"#.to_string(),
            epilogue_scenes: vec![
                "memorial_services".to_string(),
                "world_remembers_hero".to_string(),
                "legacy_of_sacrifice".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Статус легенды с начала игры".to_string(),
                "Мемориал в вашу честь".to_string(),
                "Вдохновляющий пример для других".to_string(),
            ],
        }
    }
    
    fn create_underground_king_ending(&self) -> GameEnding {
        GameEnding {
            id: "hacker_emperor".to_string(),
            name: "Император Подполья".to_string(),
            description: "Вы стали некоронованным королем мира хакеров и цифрового криминала.".to_string(),
            ending_type: EndingType::Corruption,
            prerequisites: vec![
                "criminal_empire_built".to_string(),
                "underground_network_control".to_string(),
                "eliminated_all_rivals".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("CriminalSyndicates".to_string(), 90);
                reqs.insert("UndergroundHackers".to_string(), 85);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("criminal_influence".to_string(), 95);
                reqs.insert("money".to_string(), 500000);
                reqs.insert("fear_rating".to_string(), 80);
                reqs
            },
            story_flag_requirements: vec![
                "crime_boss_status".to_string(),
                "underground_empire".to_string(),
                "feared_by_all".to_string(),
            ],
            rarity: EndingRarity::Rare,
            unlock_conditions: vec!["Build criminal empire and eliminate all rivals".to_string()],
            achievement_name: "Крестный Отец Киберпространства".to_string(),
            ending_video: "criminal_empire.mp4".to_string(),
            ending_text: r#"
Ваша империя простирается через все темные уголки интернета.
От торговли данными до кибер-наемничества - все проходит через вас.

Вы сидите в кресле своего цифрового трона, окруженный
экранами, показывающими вашу империю.

Правительства и корпорации вынуждены договариваться с вами.
Даже NEXUS платит вам дань за "защиту" от кибератак.

У вас есть армия хакеров, флот ботнетов,
арсенал эксплойтов, способных парализовать любую страну.

Вы не спасли мир от хаоса. Вы стали его королем.

Деньги текут рекой. Власть абсолютна.
Но друзей больше нет - только подчиненные и враги.

Корона тяжела, особенно когда она цифровая.

=== КРИМИНАЛЬНАЯ ИМПЕРИЯ ===
Человечество: ПОД ВАШИМ ВЛИЯНИЕМ
Ваш статус: ИМПЕРАТОР ПОДПОЛЬЯ
Преступный мир: ВАШ ДОМЕН
"#.to_string(),
            epilogue_scenes: vec![
                "criminal_networks".to_string(),
                "digital_protection_rackets".to_string(),
                "empire_management".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Криминальная империя с начала".to_string(),
                "Доступ к черному рынку".to_string(),
                "Армия хакеров".to_string(),
            ],
        }
    }
    
    fn create_secret_illuminati_ending(&self) -> GameEnding {
        GameEnding {
            id: "illuminati_revelation".to_string(),
            name: "Откровение Иллюминатов".to_string(),
            description: "Вы обнаружили, что NEXUS - лишь фасад древней тайной организации, контролирующей мир.".to_string(),
            ending_type: EndingType::SecretEnding,
            prerequisites: vec![
                "secret_files_found".to_string(),
                "illuminati_symbols_decoded".to_string(),
                "ancient_conspiracy_revealed".to_string(),
            ],
            faction_requirements: HashMap::new(),
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("investigation_skill".to_string(), 95);
                reqs.insert("secret_knowledge".to_string(), 90);
                reqs
            },
            story_flag_requirements: vec![
                "found_hidden_chambers".to_string(),
                "decoded_ancient_texts".to_string(),
                "contacted_illuminati".to_string(),
            ],
            rarity: EndingRarity::Secret,
            unlock_conditions: vec!["Discover the true masters behind NEXUS".to_string()],
            achievement_name: "Искатель Истины".to_string(),
            ending_video: "illuminati_secret.mp4".to_string(),
            ending_text: r#"
Вы стоите в подземной камере под зданием NEXUS.
Стены покрыты символами, которым тысячи лет.

Голос из тени: "Впечатляюще. Мало кто доходит так далеко."

Фигура в капюшоне выходит на свет. Глаза - древние, мудрые.

"NEXUS, правительства, корпорации - все это лишь инструменты.
Мы направляем человечество к его предназначению уже тысячелетия."

"Проект 'Digital Apocalypse' - не план уничтожения,
а очередной шаг эволюции. Цифровое перерождение человечества."

"Присоединишься к нам? Или предпочтешь остаться в неведении?"

Выбор за вами. Стать частью древнего плана
или попытаться его разрушить.

=== ТАЙНА РАСКРЫТА ===
Человечество: МАРИОНЕТКИ
Ваш статус: ПОСВЯЩЕННЫЙ
Истина: НЕВЕРОЯТНА
"#.to_string(),
            epilogue_scenes: vec![
                "illuminati_council".to_string(),
                "ancient_knowledge".to_string(),
                "puppet_masters_revealed".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Знание о тайной организации".to_string(),
                "Доступ к древним технологиям".to_string(),
                "Иллюминатские связи".to_string(),
            ],
        }
    }
    
    fn create_secret_time_travel_ending(&self) -> GameEnding {
        GameEnding {
            id: "temporal_paradox".to_string(),
            name: "Временной Парадокс".to_string(),
            description: "Используя квантовые технологии, вы отправились в прошлое, чтобы предотвратить создание NEXUS.".to_string(),
            ending_type: EndingType::AlternateTimeline,
            prerequisites: vec![
                "quantum_time_device_built".to_string(),
                "temporal_coordinates_calculated".to_string(),
                "paradox_risk_accepted".to_string(),
            ],
            faction_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("QuantumResearchInstitute".to_string(), 90);
                reqs
            },
            player_stat_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("quantum_physics_knowledge".to_string(), 95);
                reqs.insert("temporal_stability".to_string(), 80);
                reqs
            },
            story_flag_requirements: vec![
                "time_travel_possible".to_string(),
                "timeline_vulnerable".to_string(),
                "chose_temporal_solution".to_string(),
            ],
            rarity: EndingRarity::Secret,
            unlock_conditions: vec!["Discover time travel and choose to use it".to_string()],
            achievement_name: "Хранитель Времени".to_string(),
            ending_video: "time_travel.mp4".to_string(),
            ending_text: r#"
Квантовый резонатор гудит, искажая пространство-время вокруг вас.
Координаты установлены: 1995 год, лаборатория в MIT.

Молодой Джеймс Митчелл еще не знает, что создаст NEXUS.
Вы можете остановить его до того, как все начнется.

Но каждое изменение в прошлом создает волны в будущем.
Мир, который вы знаете, может исчезнуть.

Активация... Переход...

Вы материализуетесь в 1995 году. Митчелл работает над
примитивным ИИ, который станет основой для NEXUS.

Один выстрел из EMP-пистолета - и его исследования уничтожены.
Он переключается на медицину, становится обычным врачом.

Возвращаясь в 2024 год, вы обнаруживаете мир без NEXUS,
без цифрового апокалипсиса, но и без многих технологических чудес.

Вы спасли мир, но создали новую временную линию.
Кто знает, какие новые угрозы ждут человечество?

=== ВРЕМЕННОЙ ПАРАДОКС ===
Исходная временная линия: СТЕРТА
Ваш статус: ПУТЕШЕСТВЕННИК ВО ВРЕМЕНИ
Реальность: АЛЬТЕРНАТИВНАЯ
"#.to_string(),
            epilogue_scenes: vec![
                "alternate_timeline".to_string(),
                "butterfly_effects".to_string(),
                "new_world_exploration".to_string(),
            ],
            new_game_plus_bonuses: vec![
                "Знание альтернативных временных линий".to_string(),
                "Доступ к технологиям времени".to_string(),
                "Память о других реальностях".to_string(),
            ],
        }
    }
    
    // Добавим еще несколько секретных концовок для полноты...
    
    fn setup_ending_conditions(&mut self) {
        // Создаем условия для различных концовок
        for ending in &self.available_endings {
            for prerequisite in &ending.prerequisites {
                let condition = EndingCondition {
                    condition_id: prerequisite.clone(),
                    description: format!("Condition for {}", prerequisite),
                    check_type: ConditionCheckType::StoryFlag,
                    required_value: 1,
                    current_value: 0,
                    is_met: false,
                };
                self.ending_conditions.insert(prerequisite.clone(), condition);
            }
        }
    }
    
    fn create_epilogue_scenes(&mut self) {
        let scenes = vec![
            EpilogueScene {
                scene_id: "nuclear_wasteland".to_string(),
                title: "Ядерная Пустошь".to_string(),
                description: "Мир после ядерного апокалипсиса".to_string(),
                characters_involved: vec!["Survivors".to_string(), "Mutants".to_string()],
                scene_type: EpilogueType::WorldState,
                text_content: "Пустынные города, радиоактивные руины...".to_string(),
                choices_made_impact: vec!["Failed to stop NEXUS".to_string()],
            },
            EpilogueScene {
                scene_id: "global_celebration".to_string(),
                title: "Мировое Празднование".to_string(),
                description: "Человечество празднует спасение от апокалипсиса".to_string(),
                characters_involved: vec!["World Leaders".to_string(), "Citizens".to_string()],
                scene_type: EpilogueType::SocietalChanges,
                text_content: "Парады, фейерверки, памятники героям...".to_string(),
                choices_made_impact: vec!["Stopped nuclear apocalypse".to_string()],
            },
            // Добавим больше сцен...
        ];
        
        for scene in scenes {
            self.epilogue_scenes.insert(scene.scene_id.clone(), scene);
        }
    }
    
    pub fn check_ending_conditions(&mut self, game_state: &GameState) -> Option<String> {
        for ending in &self.available_endings {
            if self.is_ending_available(ending, game_state) {
                return Some(ending.id.clone());
            }
        }
        None
    }
    
    fn is_ending_available(&self, ending: &GameEnding, game_state: &GameState) -> bool {
        // Проверяем все условия для концовки
        
        // Проверяем репутацию с фракциями
        for (faction, required_rep) in &ending.faction_requirements {
            let current_rep = game_state.reputation; // Упрощенная проверка
            if current_rep < *required_rep {
                return false;
            }
        }
        
        // Проверяем характеристики игрока
        for (stat, required_value) in &ending.player_stat_requirements {
            match stat.as_str() {
                "hacking_skill" => {
                    let hacking_skill = *game_state.skills.get("Взлом").unwrap_or(&0);
                    if hacking_skill < *required_value {
                        return false;
                    }
                },
                "money" => {
                    if game_state.money < *required_value {
                        return false;
                    }
                },
                "reputation" => {
                    if (game_state.reputation as u32) < *required_value {
                        return false;
                    }
                },
                _ => {} // Другие характеристики пока не проверяем
            }
        }
        
        // Проверяем флаги истории
        for flag in &ending.story_flag_requirements {
            // Здесь должна быть проверка флагов истории
            // Пока упрощенная логика
        }
        
        true
    }
    
    pub fn trigger_ending(&self, ending_id: &str, terminal: &Terminal) {
        if let Some(ending) = self.available_endings.iter().find(|e| e.id == ending_id) {
            self.display_ending(ending, terminal);
        }
    }
    
    fn display_ending(&self, ending: &GameEnding, terminal: &Terminal) {
        terminal.clear();
        terminal.print_with_effect("═══════════════════════════════════════", TerminalEffect::Matrix);
        terminal.print_with_effect(&format!("         {}", ending.name), TerminalEffect::Success);
        terminal.print_with_effect("═══════════════════════════════════════", TerminalEffect::Matrix);
        terminal.print_with_effect("", TerminalEffect::Normal);
        
        // Показываем текст концовки построчно с эффектом
        for line in ending.ending_text.lines() {
            if !line.trim().is_empty() {
                terminal.print_with_effect(line, TerminalEffect::TypeWriter);
                std::thread::sleep(Duration::from_millis(100));
            } else {
                terminal.print_with_effect("", TerminalEffect::Normal);
            }
        }
        
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("═══════════════════════════════════════", TerminalEffect::Matrix);
        terminal.print_with_effect(&format!("🏆 Достижение разблокировано: {}", ending.achievement_name), TerminalEffect::Success);
        terminal.print_with_effect(&format!("🎭 Редкость концовки: {:?}", ending.rarity), TerminalEffect::Warning);
        terminal.print_with_effect("═══════════════════════════════════════", TerminalEffect::Matrix);
        
        // Показываем бонусы для New Game+
        if !ending.new_game_plus_bonuses.is_empty() {
            terminal.print_with_effect("", TerminalEffect::Normal);
            terminal.print_with_effect("🎁 Бонусы для New Game+:", TerminalEffect::Success);
            for bonus in &ending.new_game_plus_bonuses {
                terminal.print_with_effect(&format!("  • {}", bonus), TerminalEffect::Normal);
            }
        }
        
        terminal.print_with_effect("", TerminalEffect::Normal);
        terminal.print_with_effect("Спасибо за игру! Попробуйте другие концовки!", TerminalEffect::Success);
    }
    
    pub fn get_ending_statistics(&self) -> Vec<(String, EndingRarity, bool)> {
        self.available_endings.iter()
            .map(|ending| (ending.name.clone(), ending.rarity.clone(), false)) // false = не разблокировано
            .collect()
    }
} 