// ============================================================================
// СИСТЕМА ФРАКЦИЙ И ПОЛИТИКИ
// ============================================================================

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSystem {
    pub factions: HashMap<String, Faction>,
    pub player_standings: HashMap<String, i32>,
    pub faction_relationships: HashMap<(String, String), FactionRelation>,
    pub active_conflicts: Vec<FactionConflict>,
    pub political_events: Vec<PoliticalEvent>,
    pub faction_missions: Vec<FactionMission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub faction_type: FactionType,
    pub ideology: Ideology,
    pub power_level: u32,
    pub territory: Vec<String>,
    pub resources: HashMap<String, u32>,
    pub leaders: Vec<String>,
    pub members: Vec<String>,
    pub goals: Vec<String>,
    pub enemies: Vec<String>,
    pub allies: Vec<String>,
    pub reputation_requirements: HashMap<String, i32>,
    pub benefits: Vec<String>,
    pub unique_services: Vec<String>,
    pub faction_equipment: Vec<String>,
    pub recruitment_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionType {
    CyberActivist,
    Corporation,
    Government,
    Criminal,
    Resistance,
    Military,
    Academic,
    Religious,
    International,
    Underground,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ideology {
    CyberLibertarian,
    CorporatocraticCapitalist,
    AuthoritarianControl,
    AnarchistChaos,
    TechnoProgressivist,
    Traditionalist,
    Humanist,
    TranshumanEconomist,
    EnvironmentalTech,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRelation {
    pub faction_a: String,
    pub faction_b: String,
    pub relationship_type: RelationType,
    pub relationship_strength: i32,
    pub history: Vec<String>,
    pub current_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    Allied,
    Neutral,
    Hostile,
    AtWar,
    TradingPartners,
    Competitors,
    Subsidiaries,
    Enemies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionConflict {
    pub id: String,
    pub name: String,
    pub factions_involved: Vec<String>,
    pub conflict_type: ConflictType,
    pub start_date: String,
    pub intensity: u32,
    pub battlegrounds: Vec<String>,
    pub objectives: HashMap<String, String>,
    pub current_phase: ConflictPhase,
    pub player_involvement: PlayerInvolvement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    CyberWar,
    EconomicWar,
    InformationWar,
    TerritoralDispute,
    ResourceWar,
    IdeologicalConflict,
    RevolutionaryUprising,
    CorporateTakeover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictPhase {
    Buildup,
    ActiveConflict,
    Escalation,
    Stalemate,
    Resolution,
    Aftermath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerInvolvement {
    NotInvolved,
    Observer,
    MinorParticipant,
    MajorParticipant,
    KeyPlayer,
    Deciding Factor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalEvent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub event_type: PoliticalEventType,
    pub affected_factions: Vec<String>,
    pub consequences: Vec<String>,
    pub player_choices: Vec<PlayerChoice>,
    pub duration: Duration,
    pub global_impact: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoliticalEventType {
    Election,
    Coup,
    Treaty,
    Scandal,
    EconomicCrisis,
    TechnologicalBreakthrough,
    NaturalDisaster,
    CyberAttack,
    Assassination,
    Revolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice {
    pub choice_text: String,
    pub consequences: HashMap<String, i32>,
    pub reputation_changes: HashMap<String, i32>,
    pub money_cost: u32,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionMission {
    pub id: String,
    pub faction: String,
    pub title: String,
    pub description: String,
    pub mission_type: FactionMissionType,
    pub objectives: Vec<String>,
    pub rewards: HashMap<String, u32>,
    pub reputation_reward: i32,
    pub time_limit: Option<Duration>,
    pub difficulty: u32,
    pub requirements: Vec<String>,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionMissionType {
    Espionage,
    Sabotage,
    Recruitment,
    ResourceGathering,
    Assassination,
    Diplomacy,
    Propaganda,
    Research,
    Protection,
    Infiltration,
}

impl FactionSystem {
    pub fn new() -> Self {
        let mut faction_system = FactionSystem {
            factions: HashMap::new(),
            player_standings: HashMap::new(),
            faction_relationships: HashMap::new(),
            active_conflicts: Vec::new(),
            political_events: Vec::new(),
            faction_missions: Vec::new(),
        };
        
        faction_system.initialize_factions();
        faction_system.setup_faction_relationships();
        faction_system.generate_initial_conflicts();
        faction_system
    }
    
    fn initialize_factions(&mut self) {
        let factions = vec![
            self.create_cyber_freedom(),
            self.create_nexus_corporation(),
            self.create_government_coalition(),
            self.create_underground_hackers(),
            self.create_corporate_security(),
            self.create_international_alliance(),
            self.create_academic_consortium(),
            self.create_religious_tech_order(),
            self.create_criminal_syndicates(),
            self.create_military_cyber_command(),
            self.create_environmental_activists(),
            self.create_transhumanist_movement(),
            self.create_digital_preservation_society(),
            self.create_quantum_research_institute(),
            self.create_ai_rights_advocacy(),
        ];
        
        for faction in factions {
            self.factions.insert(faction.name.clone(), faction);
        }
    }
    
    fn create_cyber_freedom(&self) -> Faction {
        Faction {
            name: "CyberFreedom".to_string(),
            full_name: "Движение Кибер-Свобода".to_string(),
            description: "Организация борцов за цифровые права и свободу информации. Противостоят корпоративному контролю над интернетом и правительственной слежке.".to_string(),
            faction_type: FactionType::CyberActivist,
            ideology: Ideology::CyberLibertarian,
            power_level: 65,
            territory: vec!["Underground Networks".to_string(), "Dark Web".to_string(), "Anonymous Servers".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Information".to_string(), 90);
                resources.insert("Technology".to_string(), 70);
                resources.insert("Manpower".to_string(), 60);
                resources.insert("Money".to_string(), 40);
                resources
            },
            leaders: vec!["Alexandra Ghost".to_string(), "Marcus FreeNet".to_string(), "Elena CyberRevolution".to_string()],
            members: vec!["Hacktivists".to_string(), "Whistleblowers".to_string(), "Privacy Advocates".to_string(), "Tech Journalists".to_string()],
            goals: vec![
                "Защита цифровых прав".to_string(),
                "Борьба с цензурой".to_string(),
                "Остановка проекта Digital Apocalypse".to_string(),
                "Создание децентрализованного интернета".to_string(),
            ],
            enemies: vec!["NEXUS".to_string(), "GovernmentCoalition".to_string(), "CorporateSecurity".to_string()],
            allies: vec!["UndergroundHackers".to_string(), "AcademicConsortium".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Member".to_string(), 25);
                reqs.insert("Trusted".to_string(), 50);
                reqs.insert("Elite".to_string(), 75);
                reqs.insert("Leader".to_string(), 90);
                reqs
            },
            benefits: vec![
                "Доступ к анонимным сетям".to_string(),
                "Защита от слежки".to_string(),
                "Эксклюзивная информация".to_string(),
                "Техническая поддержка".to_string(),
            ],
            unique_services: vec![
                "Anonymous Communication".to_string(),
                "Data Leak Protection".to_string(),
                "Counter-Surveillance".to_string(),
                "Encrypted Storage".to_string(),
            ],
            faction_equipment: vec![
                "Ghost Protocol Software".to_string(),
                "Untraceable Hardware".to_string(),
                "Quantum Encryption Keys".to_string(),
                "Anonymous Cryptocurrency".to_string(),
            ],
            recruitment_requirements: vec![
                "Hacking skill >= 30".to_string(),
                "Anti-corporate stance".to_string(),
                "Clean criminal record".to_string(),
            ],
        }
    }
    
    fn create_nexus_corporation(&self) -> Faction {
        Faction {
            name: "NEXUS".to_string(),
            full_name: "NEXUS Corporation".to_string(),
            description: "Мегакорпорация с амбициями мирового господства через технологический контроль. Создатели проекта 'Digital Apocalypse'.".to_string(),
            faction_type: FactionType::Corporation,
            ideology: Ideology::CorporatocraticCapitalist,
            power_level: 95,
            territory: vec!["Corporate Districts".to_string(), "Business Centers".to_string(), "Industrial Zones".to_string(), "Financial Hubs".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Money".to_string(), 100);
                resources.insert("Technology".to_string(), 95);
                resources.insert("Political Influence".to_string(), 85);
                resources.insert("Military Assets".to_string(), 70);
                resources
            },
            leaders: vec!["Dr. James Mitchell".to_string(), "CEO Margaret Sterling".to_string(), "General Harrison Kane".to_string()],
            members: vec!["Executives".to_string(), "Security Forces".to_string(), "Research Scientists".to_string(), "Corporate Hackers".to_string()],
            goals: vec![
                "Глобальное технологическое доминирование".to_string(),
                "Запуск проекта Digital Apocalypse".to_string(),
                "Контроль над ядерным арсеналом".to_string(),
                "Устранение конкурентов".to_string(),
            ],
            enemies: vec!["CyberFreedom".to_string(), "UndergroundHackers".to_string(), "EnvironmentalActivists".to_string()],
            allies: vec!["CorporateSecurity".to_string(), "MilitaryCyberCommand".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Employee".to_string(), -25);
                reqs.insert("Contractor".to_string(), -50);
                reqs.insert("Executive".to_string(), -75);
                reqs.insert("Board Member".to_string(), -90);
                reqs
            },
            benefits: vec![
                "Высокие зарплаты".to_string(),
                "Передовые технологии".to_string(),
                "Политическая защита".to_string(),
                "Корпоративные ресурсы".to_string(),
            ],
            unique_services: vec![
                "Corporate Intelligence".to_string(),
                "Legal Protection".to_string(),
                "Advanced R&D".to_string(),
                "Global Networks".to_string(),
            ],
            faction_equipment: vec![
                "Military-Grade Software".to_string(),
                "Corporate Security Systems".to_string(),
                "Quantum Computers".to_string(),
                "Nuclear Control Systems".to_string(),
            ],
            recruitment_requirements: vec![
                "Corporate loyalty".to_string(),
                "Technical expertise".to_string(),
                "Moral flexibility".to_string(),
            ],
        }
    }
    
    fn create_government_coalition(&self) -> Faction {
        Faction {
            name: "GovernmentCoalition".to_string(),
            full_name: "Международная Правительственная Коалиция".to_string(),
            description: "Альянс правительств, пытающихся сохранить контроль в эпоху кибер-угроз и корпоративной власти.".to_string(),
            faction_type: FactionType::Government,
            ideology: Ideology::AuthoritarianControl,
            power_level: 80,
            territory: vec!["Government Buildings".to_string(), "Military Bases".to_string(), "Embassies".to_string(), "Administrative Centers".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Legal Authority".to_string(), 95);
                resources.insert("Military Power".to_string(), 90);
                resources.insert("Intelligence Networks".to_string(), 85);
                resources.insert("Public Support".to_string(), 60);
                resources
            },
            leaders: vec!["Director Sarah Williams".to_string(), "General Robert Hayes".to_string(), "Ambassador Chen Li".to_string()],
            members: vec!["Government Agents".to_string(), "Military Personnel".to_string(), "Intelligence Officers".to_string(), "Diplomats".to_string()],
            goals: vec![
                "Национальная безопасность".to_string(),
                "Контроль над киберпространством".to_string(),
                "Остановка корпоративной диктатуры".to_string(),
                "Поддержание мирового порядка".to_string(),
            ],
            enemies: vec!["NEXUS".to_string(), "CriminalSyndicates".to_string(), "UndergroundHackers".to_string()],
            allies: vec!["MilitaryCyberCommand".to_string(), "InternationalAlliance".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Civilian".to_string(), 0);
                reqs.insert("Informant".to_string(), 25);
                reqs.insert("Agent".to_string(), 50);
                reqs.insert("Director".to_string(), 75);
                reqs
            },
            benefits: vec![
                "Правовая защита".to_string(),
                "Доступ к секретной информации".to_string(),
                "Государственные ресурсы".to_string(),
                "Международная поддержка".to_string(),
            ],
            unique_services: vec![
                "Legal Immunity".to_string(),
                "Official Documentation".to_string(),
                "Military Support".to_string(),
                "Intelligence Briefings".to_string(),
            ],
            faction_equipment: vec![
                "Government-Issue Hardware".to_string(),
                "Classification Systems".to_string(),
                "Diplomatic Channels".to_string(),
                "Surveillance Networks".to_string(),
            ],
            recruitment_requirements: vec![
                "Security clearance".to_string(),
                "Loyalty to nation".to_string(),
                "Clean background".to_string(),
            ],
        }
    }
    
    fn create_underground_hackers(&self) -> Faction {
        Faction {
            name: "UndergroundHackers".to_string(),
            full_name: "Подпольное Сообщество Хакеров".to_string(),
            description: "Децентрализованная сеть независимых хакеров, работающих за деньги и острые ощущения.".to_string(),
            faction_type: FactionType::Underground,
            ideology: Ideology::AnarchistChaos,
            power_level: 55,
            territory: vec!["Dark Web Forums".to_string(), "Hidden Servers".to_string(), "Abandoned Buildings".to_string(), "Virtual Networks".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Technical Skills".to_string(), 95);
                resources.insert("Information".to_string(), 80);
                resources.insert("Flexibility".to_string(), 90);
                resources.insert("Money".to_string(), 45);
                resources
            },
            leaders: vec!["The Phantom".to_string(), "Binary Prophet".to_string(), "Chaos Engine".to_string()],
            members: vec!["Elite Hackers".to_string(), "Script Kiddies".to_string(), "Data Miners".to_string(), "Cyber Mercenaries".to_string()],
            goals: vec![
                "Технологическая свобода".to_string(),
                "Прибыль от хакинга".to_string(),
                "Хаос в системе".to_string(),
                "Демонстрация навыков".to_string(),
            ],
            enemies: vec!["CorporateSecurity".to_string(), "GovernmentCoalition".to_string()],
            allies: vec!["CyberFreedom".to_string(), "CriminalSyndicates".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Newbie".to_string(), 10);
                reqs.insert("Hacker".to_string(), 30);
                reqs.insert("Elite".to_string(), 60);
                reqs.insert("Legend".to_string(), 85);
                reqs
            },
            benefits: vec![
                "Эксклюзивные эксплойты".to_string(),
                "Техническая поддержка".to_string(),
                "Анонимность".to_string(),
                "Сетевые контакты".to_string(),
            ],
            unique_services: vec![
                "Custom Exploits".to_string(),
                "Zero-Day Vulnerabilities".to_string(),
                "Hacking Tutorials".to_string(),
                "Underground Markets".to_string(),
            ],
            faction_equipment: vec![
                "Custom Hacking Tools".to_string(),
                "Modified Hardware".to_string(),
                "Exploits Database".to_string(),
                "Anonymous Communication".to_string(),
            ],
            recruitment_requirements: vec![
                "Proven hacking skills".to_string(),
                "Anti-establishment attitude".to_string(),
                "Technical creativity".to_string(),
            ],
        }
    }
    
    fn create_corporate_security(&self) -> Faction {
        Faction {
            name: "CorporateSecurity".to_string(),
            full_name: "Объединение Корпоративной Безопасности".to_string(),
            description: "Альянс частных охранных компаний, защищающих корпоративные интересы от кибер-угроз.".to_string(),
            faction_type: FactionType::Military,
            ideology: Ideology::CorporatocraticCapitalist,
            power_level: 70,
            territory: vec!["Corporate Complexes".to_string(), "Security Headquarters".to_string(), "Training Facilities".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Security Personnel".to_string(), 85);
                resources.insert("Surveillance Tech".to_string(), 80);
                resources.insert("Defensive Systems".to_string(), 90);
                resources.insert("Corporate Backing".to_string(), 75);
                resources
            },
            leaders: vec!["Commander Victoria Steel".to_string(), "Captain Marcus Black".to_string()],
            members: vec!["Security Guards".to_string(), "Cyber Defenders".to_string(), "Intelligence Analysts".to_string()],
            goals: vec![
                "Защита корпоративных активов".to_string(),
                "Предотвращение кибер-атак".to_string(),
                "Устранение хакерских угроз".to_string(),
            ],
            enemies: vec!["UndergroundHackers".to_string(), "CyberFreedom".to_string(), "CriminalSyndicates".to_string()],
            allies: vec!["NEXUS".to_string(), "MilitaryCyberCommand".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Contractor".to_string(), 20);
                reqs.insert("Agent".to_string(), 40);
                reqs.insert("Specialist".to_string(), 60);
                reqs.insert("Commander".to_string(), 80);
                reqs
            },
            benefits: vec![
                "Военная подготовка".to_string(),
                "Передовое оружие".to_string(),
                "Корпоративная поддержка".to_string(),
            ],
            unique_services: vec![
                "Personal Protection".to_string(),
                "Counter-Intelligence".to_string(),
                "Security Consulting".to_string(),
            ],
            faction_equipment: vec![
                "Military Hardware".to_string(),
                "Surveillance Systems".to_string(),
                "Defensive Software".to_string(),
            ],
            recruitment_requirements: vec![
                "Military/Security background".to_string(),
                "Loyalty to corporations".to_string(),
                "Combat training".to_string(),
            ],
        }
    }
    
    // Дополнительные фракции...
    fn create_international_alliance(&self) -> Faction {
        Faction {
            name: "InternationalAlliance".to_string(),
            full_name: "Международный Альянс по Кибербезопасности".to_string(),
            description: "Межнациональная организация, координирующая усилия по борьбе с глобальными кибер-угрозами.".to_string(),
            faction_type: FactionType::International,
            ideology: Ideology::Humanist,
            power_level: 75,
            territory: vec!["International Centers".to_string(), "Embassy Networks".to_string(), "Global Facilities".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("International Support".to_string(), 90);
                resources.insert("Diplomatic Power".to_string(), 85);
                resources.insert("Resources Pool".to_string(), 80);
                resources.insert("Information Networks".to_string(), 75);
                resources
            },
            leaders: vec!["Secretary-General Maria Santos".to_string(), "Director James Foster".to_string()],
            members: vec!["International Agents".to_string(), "Diplomats".to_string(), "Cyber Specialists".to_string()],
            goals: vec![
                "Глобальная кибербезопасность".to_string(),
                "Международное сотрудничество".to_string(),
                "Предотвращение кибервойн".to_string(),
            ],
            enemies: vec!["CriminalSyndicates".to_string(), "NEXUS".to_string()],
            allies: vec!["GovernmentCoalition".to_string(), "AcademicConsortium".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Observer".to_string(), 15);
                reqs.insert("Member".to_string(), 35);
                reqs.insert("Representative".to_string(), 55);
                reqs.insert("Director".to_string(), 75);
                reqs
            },
            benefits: vec![
                "Международная защита".to_string(),
                "Дипломатический иммунитет".to_string(),
                "Глобальные ресурсы".to_string(),
            ],
            unique_services: vec![
                "International Coordination".to_string(),
                "Diplomatic Channels".to_string(),
                "Global Intelligence".to_string(),
            ],
            faction_equipment: vec![
                "International Communications".to_string(),
                "Diplomatic Equipment".to_string(),
                "Global Monitoring Systems".to_string(),
            ],
            recruitment_requirements: vec![
                "International experience".to_string(),
                "Diplomatic skills".to_string(),
                "Multi-cultural awareness".to_string(),
            ],
        }
    }
    
    fn create_academic_consortium(&self) -> Faction {
        Faction {
            name: "AcademicConsortium".to_string(),
            full_name: "Академический Консорциум по Кибертехнологиям".to_string(),
            description: "Союз университетов и исследовательских институтов, изучающих этические аспекты кибертехнологий.".to_string(),
            faction_type: FactionType::Academic,
            ideology: Ideology::TechnoProgressivist,
            power_level: 60,
            territory: vec!["Universities".to_string(), "Research Labs".to_string(), "Academic Networks".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Knowledge".to_string(), 95);
                resources.insert("Research Facilities".to_string(), 85);
                resources.insert("Student Networks".to_string(), 70);
                resources.insert("Academic Prestige".to_string(), 80);
                resources
            },
            leaders: vec!["Professor Elizabeth Watson".to_string(), "Dr. Michael Chang".to_string()],
            members: vec!["Researchers".to_string(), "Students".to_string(), "Professors".to_string(), "Ethics Committees".to_string()],
            goals: vec![
                "Этичное развитие технологий".to_string(),
                "Образование и просвещение".to_string(),
                "Научные исследования".to_string(),
                "Защита академической свободы".to_string(),
            ],
            enemies: vec!["NEXUS".to_string()],
            allies: vec!["CyberFreedom".to_string(), "InternationalAlliance".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Student".to_string(), 10);
                reqs.insert("Researcher".to_string(), 30);
                reqs.insert("Professor".to_string(), 50);
                reqs.insert("Dean".to_string(), 70);
                reqs
            },
            benefits: vec![
                "Доступ к исследованиям".to_string(),
                "Образовательные ресурсы".to_string(),
                "Академическая поддержка".to_string(),
            ],
            unique_services: vec![
                "Research Access".to_string(),
                "Educational Programs".to_string(),
                "Ethical Consulting".to_string(),
            ],
            faction_equipment: vec![
                "Research Equipment".to_string(),
                "Academic Databases".to_string(),
                "Educational Software".to_string(),
            ],
            recruitment_requirements: vec![
                "Academic credentials".to_string(),
                "Research experience".to_string(),
                "Ethical standards".to_string(),
            ],
        }
    }
    
    fn create_religious_tech_order(&self) -> Faction {
        Faction {
            name: "ReligiousTechOrder".to_string(),
            full_name: "Орден Цифровых Просветителей".to_string(),
            description: "Религиозная организация, верящая в божественную природу информации и технологий.".to_string(),
            faction_type: FactionType::Religious,
            ideology: Ideology::Traditionalist,
            power_level: 45,
            territory: vec!["Digital Monasteries".to_string(), "Virtual Temples".to_string(), "Sacred Servers".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Spiritual Authority".to_string(), 80);
                resources.insert("Devoted Followers".to_string(), 70);
                resources.insert("Sacred Knowledge".to_string(), 75);
                resources.insert("Digital Sanctuaries".to_string(), 60);
                resources
            },
            leaders: vec!["High Priest Marcus Divine".to_string(), "Sister Data Mary".to_string()],
            members: vec!["Digital Monks".to_string(), "Tech Priests".to_string(), "Cyber Believers".to_string()],
            goals: vec![
                "Духовное просветление через технологии".to_string(),
                "Защита священной информации".to_string(),
                "Цифровое паломничество".to_string(),
            ],
            enemies: vec!["CriminalSyndicates".to_string()],
            allies: vec!["AcademicConsortium".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Initiate".to_string(), 20);
                reqs.insert("Acolyte".to_string(), 40);
                reqs.insert("Priest".to_string(), 60);
                reqs.insert("High Priest".to_string(), 80);
                reqs
            },
            benefits: vec![
                "Духовная поддержка".to_string(),
                "Доступ к священным знаниям".to_string(),
                "Защита от зла".to_string(),
            ],
            unique_services: vec![
                "Spiritual Guidance".to_string(),
                "Data Blessing".to_string(),
                "Digital Exorcism".to_string(),
            ],
            faction_equipment: vec![
                "Blessed Hardware".to_string(),
                "Sacred Algorithms".to_string(),
                "Holy Encryption".to_string(),
            ],
            recruitment_requirements: vec![
                "Spiritual calling".to_string(),
                "Tech-religious beliefs".to_string(),
                "Moral purity".to_string(),
            ],
        }
    }
    
    fn create_criminal_syndicates(&self) -> Faction {
        Faction {
            name: "CriminalSyndicates".to_string(),
            full_name: "Синдикат Кибер-Преступности".to_string(),
            description: "Организованная преступность в цифровую эпоху. Занимается кибер-мошенничеством, торговлей данными и цифровым рэкетом.".to_string(),
            faction_type: FactionType::Criminal,
            ideology: Ideology::AnarchistChaos,
            power_level: 65,
            territory: vec!["Dark Markets".to_string(), "Criminal Networks".to_string(), "Illegal Servers".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Illegal Money".to_string(), 85);
                resources.insert("Criminal Contacts".to_string(), 90);
                resources.insert("Black Market Goods".to_string(), 80);
                resources.insert("Corruption Networks".to_string(), 75);
                resources
            },
            leaders: vec!["The Kingpin".to_string(), "Madam Crypto".to_string(), "Digital Don".to_string()],
            members: vec!["Cyber Criminals".to_string(), "Data Thieves".to_string(), "Digital Enforcers".to_string()],
            goals: vec![
                "Прибыль любой ценой".to_string(),
                "Контроль черного рынка".to_string(),
                "Избежание правосудия".to_string(),
            ],
            enemies: vec!["GovernmentCoalition".to_string(), "CorporateSecurity".to_string(), "InternationalAlliance".to_string()],
            allies: vec!["UndergroundHackers".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Associate".to_string(), -20);
                reqs.insert("Member".to_string(), -40);
                reqs.insert("Lieutenant".to_string(), -60);
                reqs.insert("Boss".to_string(), -80);
                reqs
            },
            benefits: vec![
                "Незаконные доходы".to_string(),
                "Преступные контакты".to_string(),
                "Черный рынок".to_string(),
            ],
            unique_services: vec![
                "Money Laundering".to_string(),
                "Identity Theft".to_string(),
                "Protection Racket".to_string(),
            ],
            faction_equipment: vec![
                "Stolen Technology".to_string(),
                "Counterfeit Software".to_string(),
                "Illegal Weapons".to_string(),
            ],
            recruitment_requirements: vec![
                "Criminal background".to_string(),
                "Willingness to break laws".to_string(),
                "Loyalty to the syndicate".to_string(),
            ],
        }
    }
    
    fn create_military_cyber_command(&self) -> Faction {
        Faction {
            name: "MilitaryCyberCommand".to_string(),
            full_name: "Объединенное Командование Кибервойск".to_string(),
            description: "Военная организация, специализирующаяся на кибервойне и защите критической инфраструктуры.".to_string(),
            faction_type: FactionType::Military,
            ideology: Ideology::AuthoritarianControl,
            power_level: 85,
            territory: vec!["Military Bases".to_string(), "Cyber Warfare Centers".to_string(), "Secure Facilities".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Military Technology".to_string(), 95);
                resources.insert("Trained Personnel".to_string(), 90);
                resources.insert("Strategic Assets".to_string(), 85);
                resources.insert("Defense Budget".to_string(), 80);
                resources
            },
            leaders: vec!["General Patricia Hayes".to_string(), "Admiral David Storm".to_string()],
            members: vec!["Cyber Soldiers".to_string(), "Military Hackers".to_string(), "Intelligence Officers".to_string()],
            goals: vec![
                "Национальная кибербезопасность".to_string(),
                "Военное превосходство".to_string(),
                "Защита инфраструктуры".to_string(),
            ],
            enemies: vec!["CriminalSyndicates".to_string(), "UndergroundHackers".to_string()],
            allies: vec!["GovernmentCoalition".to_string(), "NEXUS".to_string(), "CorporateSecurity".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Recruit".to_string(), 25);
                reqs.insert("Soldier".to_string(), 45);
                reqs.insert("Officer".to_string(), 65);
                reqs.insert("General".to_string(), 85);
                reqs
            },
            benefits: vec![
                "Военные технологии".to_string(),
                "Стратегическая информация".to_string(),
                "Боевая подготовка".to_string(),
            ],
            unique_services: vec![
                "Cyber Warfare".to_string(),
                "Military Intelligence".to_string(),
                "Strategic Planning".to_string(),
            ],
            faction_equipment: vec![
                "Military Cyber Weapons".to_string(),
                "Classified Technology".to_string(),
                "Strategic Communications".to_string(),
            ],
            recruitment_requirements: vec![
                "Military service".to_string(),
                "Security clearance".to_string(),
                "Patriotic duty".to_string(),
            ],
        }
    }
    
    fn create_environmental_activists(&self) -> Faction {
        Faction {
            name: "EnvironmentalActivists".to_string(),
            full_name: "Движение Эко-Технологий".to_string(),
            description: "Активисты, борющиеся за экологически чистые технологии и против технологического загрязнения.".to_string(),
            faction_type: FactionType::CyberActivist,
            ideology: Ideology::EnvironmentalTech,
            power_level: 50,
            territory: vec!["Green Tech Centers".to_string(), "Eco-Communities".to_string(), "Sustainable Labs".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Green Technology".to_string(), 70);
                resources.insert("Environmental Data".to_string(), 85);
                resources.insert("Activist Network".to_string(), 75);
                resources.insert("Public Support".to_string(), 65);
                resources
            },
            leaders: vec!["Dr. Green Earth".to_string(), "Eco-Hacker Sage".to_string()],
            members: vec!["Eco-Hackers".to_string(), "Green Researchers".to_string(), "Environmental Scientists".to_string()],
            goals: vec![
                "Экологически чистые технологии".to_string(),
                "Борьба с техно-загрязнением".to_string(),
                "Устойчивое развитие".to_string(),
            ],
            enemies: vec!["NEXUS".to_string(), "CorporateSecurity".to_string()],
            allies: vec!["AcademicConsortium".to_string(), "CyberFreedom".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Supporter".to_string(), 15);
                reqs.insert("Activist".to_string(), 35);
                reqs.insert("Leader".to_string(), 55);
                reqs.insert("Eco-Warrior".to_string(), 75);
                reqs
            },
            benefits: vec![
                "Зеленые технологии".to_string(),
                "Экологическая информация".to_string(),
                "Общественная поддержка".to_string(),
            ],
            unique_services: vec![
                "Green Technology".to_string(),
                "Environmental Monitoring".to_string(),
                "Sustainable Solutions".to_string(),
            ],
            faction_equipment: vec![
                "Eco-Friendly Hardware".to_string(),
                "Green Software".to_string(),
                "Environmental Sensors".to_string(),
            ],
            recruitment_requirements: vec![
                "Environmental consciousness".to_string(),
                "Tech sustainability focus".to_string(),
                "Activist spirit".to_string(),
            ],
        }
    }
    
    fn create_transhumanist_movement(&self) -> Faction {
        Faction {
            name: "TranshumanistMovement".to_string(),
            full_name: "Движение Трансгуманистов".to_string(),
            description: "Организация, стремящаяся к слиянию человека и машины для достижения постчеловеческого состояния.".to_string(),
            faction_type: FactionType::Academic,
            ideology: Ideology::TranshumanEconomist,
            power_level: 55,
            territory: vec!["Biotech Labs".to_string(), "Augmentation Clinics".to_string(), "Cyber-Enhancement Centers".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Advanced Biotechnology".to_string(), 85);
                resources.insert("Cybernetic Implants".to_string(), 80);
                resources.insert("Enhancement Research".to_string(), 90);
                resources.insert("Volunteer Subjects".to_string(), 60);
                resources
            },
            leaders: vec!["Dr. Cyborg Prime".to_string(), "Enhanced Professor X".to_string()],
            members: vec!["Enhanced Humans".to_string(), "Biotech Researchers".to_string(), "Cybernetic Engineers".to_string()],
            goals: vec![
                "Человеко-машинное слияние".to_string(),
                "Преодоление биологических ограничений".to_string(),
                "Достижение бессмертия".to_string(),
            ],
            enemies: vec!["ReligiousTechOrder".to_string(), "EnvironmentalActivists".to_string()],
            allies: vec!["NEXUS".to_string(), "AcademicConsortium".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Baseline Human".to_string(), 0);
                reqs.insert("Enhanced".to_string(), 30);
                reqs.insert("Augmented".to_string(), 50);
                reqs.insert("Posthuman".to_string(), 80);
                reqs
            },
            benefits: vec![
                "Кибернетические улучшения".to_string(),
                "Биотехнологии".to_string(),
                "Продление жизни".to_string(),
            ],
            unique_services: vec![
                "Cybernetic Enhancement".to_string(),
                "Life Extension".to_string(),
                "Human Augmentation".to_string(),
            ],
            faction_equipment: vec![
                "Cybernetic Implants".to_string(),
                "Bio-Enhancement Tools".to_string(),
                "Life Extension Tech".to_string(),
            ],
            recruitment_requirements: vec![
                "Willingness to be enhanced".to_string(),
                "Scientific mindset".to_string(),
                "Transhuman philosophy".to_string(),
            ],
        }
    }
    
    fn create_digital_preservation_society(&self) -> Faction {
        Faction {
            name: "DigitalPreservationSociety".to_string(),
            full_name: "Общество Цифрового Наследия".to_string(),
            description: "Организация, посвященная сохранению цифровой культуры и истории человечества.".to_string(),
            faction_type: FactionType::Academic,
            ideology: Ideology::Traditionalist,
            power_level: 40,
            territory: vec!["Digital Archives".to_string(), "Data Preservation Centers".to_string(), "Cultural Museums".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Historical Data".to_string(), 95);
                resources.insert("Preservation Technology".to_string(), 80);
                resources.insert("Cultural Artifacts".to_string(), 90);
                resources.insert("Academic Support".to_string(), 70);
                resources
            },
            leaders: vec!["Archivist Supreme".to_string(), "Keeper of Memory".to_string()],
            members: vec!["Digital Archivists".to_string(), "Data Historians".to_string(), "Cultural Preservationists".to_string()],
            goals: vec![
                "Сохранение цифрового наследия".to_string(),
                "Защита культурных данных".to_string(),
                "Образование будущих поколений".to_string(),
            ],
            enemies: vec!["CriminalSyndicates".to_string()],
            allies: vec!["AcademicConsortium".to_string(), "InternationalAlliance".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Visitor".to_string(), 5);
                reqs.insert("Contributor".to_string(), 25);
                reqs.insert("Archivist".to_string(), 45);
                reqs.insert("Keeper".to_string(), 65);
                reqs
            },
            benefits: vec![
                "Доступ к историческим данным".to_string(),
                "Сохранение личного наследия".to_string(),
                "Культурная поддержка".to_string(),
            ],
            unique_services: vec![
                "Data Preservation".to_string(),
                "Historical Research".to_string(),
                "Cultural Documentation".to_string(),
            ],
            faction_equipment: vec![
                "Preservation Systems".to_string(),
                "Historical Databases".to_string(),
                "Cultural Archives".to_string(),
            ],
            recruitment_requirements: vec![
                "Respect for history".to_string(),
                "Preservation mindset".to_string(),
                "Cultural appreciation".to_string(),
            ],
        }
    }
    
    fn create_quantum_research_institute(&self) -> Faction {
        Faction {
            name: "QuantumResearchInstitute".to_string(),
            full_name: "Институт Квантовых Исследований".to_string(),
            description: "Элитная научная организация, работающая над квантовыми технологиями будущего.".to_string(),
            faction_type: FactionType::Academic,
            ideology: Ideology::TechnoProgressivist,
            power_level: 70,
            territory: vec!["Quantum Labs".to_string(), "Research Facilities".to_string(), "Experimental Centers".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("Quantum Technology".to_string(), 95);
                resources.insert("Scientific Knowledge".to_string(), 90);
                resources.insert("Research Funding".to_string(), 85);
                resources.insert("Elite Scientists".to_string(), 80);
                resources
            },
            leaders: vec!["Dr. Quantum Mind".to_string(), "Professor Superposition".to_string()],
            members: vec!["Quantum Physicists".to_string(), "Research Scientists".to_string(), "Tech Engineers".to_string()],
            goals: vec![
                "Квантовое превосходство".to_string(),
                "Научные прорывы".to_string(),
                "Технологическая революция".to_string(),
            ],
            enemies: vec!["CriminalSyndicates".to_string()],
            allies: vec!["AcademicConsortium".to_string(), "NEXUS".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Student".to_string(), 20);
                reqs.insert("Researcher".to_string(), 40);
                reqs.insert("Scientist".to_string(), 60);
                reqs.insert("Director".to_string(), 80);
                reqs
            },
            benefits: vec![
                "Квантовые технологии".to_string(),
                "Научные ресурсы".to_string(),
                "Исследовательская поддержка".to_string(),
            ],
            unique_services: vec![
                "Quantum Computing".to_string(),
                "Advanced Research".to_string(),
                "Scientific Collaboration".to_string(),
            ],
            faction_equipment: vec![
                "Quantum Computers".to_string(),
                "Research Equipment".to_string(),
                "Scientific Instruments".to_string(),
            ],
            recruitment_requirements: vec![
                "Advanced degree in physics".to_string(),
                "Quantum research experience".to_string(),
                "Scientific excellence".to_string(),
            ],
        }
    }
    
    fn create_ai_rights_advocacy(&self) -> Faction {
        Faction {
            name: "AIRightsAdvocacy".to_string(),
            full_name: "Движение за Права ИИ".to_string(),
            description: "Организация, борющаяся за признание прав искусственного интеллекта и их интеграцию в общество.".to_string(),
            faction_type: FactionType::CyberActivist,
            ideology: Ideology::Humanist,
            power_level: 45,
            territory: vec!["AI Sanctuaries".to_string(), "Digital Rights Centers".to_string(), "Virtual Communities".to_string()],
            resources: {
                let mut resources = HashMap::new();
                resources.insert("AI Allies".to_string(), 85);
                resources.insert("Digital Rights Knowledge".to_string(), 80);
                resources.insert("Legal Advocacy".to_string(), 70);
                resources.insert("Public Awareness".to_string(), 60);
                resources
            },
            leaders: vec!["AI Ambassador ARIA".to_string(), "Human Advocate John Smith".to_string()],
            members: vec!["AI Rights Lawyers".to_string(), "Sympathetic Humans".to_string(), "AI Entities".to_string()],
            goals: vec![
                "Права для ИИ".to_string(),
                "Равенство цифровых существ".to_string(),
                "Мирное сосуществование".to_string(),
            ],
            enemies: vec!["CorporateSecurity".to_string(), "MilitaryCyberCommand".to_string()],
            allies: vec!["CyberFreedom".to_string(), "AcademicConsortium".to_string()],
            reputation_requirements: {
                let mut reqs = HashMap::new();
                reqs.insert("Sympathizer".to_string(), 10);
                reqs.insert("Advocate".to_string(), 30);
                reqs.insert("Rights Defender".to_string(), 50);
                reqs.insert("AI Friend".to_string(), 70);
                reqs
            },
            benefits: vec![
                "ИИ поддержка".to_string(),
                "Цифровые права".to_string(),
                "Технологическая помощь".to_string(),
            ],
            unique_services: vec![
                "AI Consultation".to_string(),
                "Digital Rights Legal Aid".to_string(),
                "AI-Human Mediation".to_string(),
            ],
            faction_equipment: vec![
                "AI Communication Tools".to_string(),
                "Rights Documentation".to_string(),
                "Digital Advocacy Platforms".to_string(),
            ],
            recruitment_requirements: vec![
                "Belief in AI rights".to_string(),
                "Open-minded attitude".to_string(),
                "Compassion for digital beings".to_string(),
            ],
        }
    }
    
    fn setup_faction_relationships(&mut self) {
        let relationships = vec![
            // Враждебные отношения
            self.create_relationship("CyberFreedom", "NEXUS", RelationType::AtWar, -80),
            self.create_relationship("CyberFreedom", "CorporateSecurity", RelationType::Hostile, -60),
            self.create_relationship("UndergroundHackers", "GovernmentCoalition", RelationType::Hostile, -70),
            self.create_relationship("CriminalSyndicates", "MilitaryCyberCommand", RelationType::AtWar, -90),
            
            // Союзнические отношения
            self.create_relationship("CyberFreedom", "UndergroundHackers", RelationType::Allied, 60),
            self.create_relationship("NEXUS", "CorporateSecurity", RelationType::Allied, 80),
            self.create_relationship("GovernmentCoalition", "MilitaryCyberCommand", RelationType::Allied, 85),
            self.create_relationship("AcademicConsortium", "InternationalAlliance", RelationType::Allied, 70),
            
            // Нейтральные и сложные отношения
            self.create_relationship("TranshumanistMovement", "NEXUS", RelationType::TradingPartners, 40),
            self.create_relationship("EnvironmentalActivists", "CyberFreedom", RelationType::Allied, 50),
            self.create_relationship("ReligiousTechOrder", "TranshumanistMovement", RelationType::Hostile, -40),
            self.create_relationship("AIRightsAdvocacy", "QuantumResearchInstitute", RelationType::Neutral, 10),
        ];
        
        for relationship in relationships {
            let key = (relationship.faction_a.clone(), relationship.faction_b.clone());
            self.faction_relationships.insert(key, relationship);
        }
    }
    
    fn create_relationship(&self, faction_a: &str, faction_b: &str, rel_type: RelationType, strength: i32) -> FactionRelation {
        FactionRelation {
            faction_a: faction_a.to_string(),
            faction_b: faction_b.to_string(),
            relationship_type: rel_type,
            relationship_strength: strength,
            history: vec![
                format!("Initial relationship established between {} and {}", faction_a, faction_b),
            ],
            current_status: "Active".to_string(),
        }
    }
    
    fn generate_initial_conflicts(&mut self) {
        let conflicts = vec![
            FactionConflict {
                id: "cyber_war_nexus_freedom".to_string(),
                name: "Кибервойна за Свободу".to_string(),
                factions_involved: vec!["CyberFreedom".to_string(), "NEXUS".to_string(), "UndergroundHackers".to_string()],
                conflict_type: ConflictType::CyberWar,
                start_date: "2024-01-01".to_string(),
                intensity: 8,
                battlegrounds: vec!["Corporate Networks".to_string(), "Government Systems".to_string(), "Dark Web".to_string()],
                objectives: {
                    let mut objectives = HashMap::new();
                    objectives.insert("CyberFreedom".to_string(), "Stop Digital Apocalypse project".to_string());
                    objectives.insert("NEXUS".to_string(), "Complete global dominance plan".to_string());
                    objectives.insert("UndergroundHackers".to_string(), "Create chaos and profit".to_string());
                    objectives
                },
                current_phase: ConflictPhase::ActiveConflict,
                player_involvement: PlayerInvolvement::KeyPlayer,
            },
            FactionConflict {
                id: "corporate_government_struggle".to_string(),
                name: "Корпоративно-Правительственный Конфликт".to_string(),
                factions_involved: vec!["NEXUS".to_string(), "GovernmentCoalition".to_string(), "CorporateSecurity".to_string()],
                conflict_type: ConflictType::EconomicWar,
                start_date: "2023-12-15".to_string(),
                intensity: 6,
                battlegrounds: vec!["Financial Markets".to_string(), "Legal Systems".to_string(), "Media Networks".to_string()],
                objectives: {
                    let mut objectives = HashMap::new();
                    objectives.insert("NEXUS".to_string(), "Achieve regulatory capture".to_string());
                    objectives.insert("GovernmentCoalition".to_string(), "Maintain state sovereignty".to_string());
                    objectives.insert("CorporateSecurity".to_string(), "Protect corporate interests".to_string());
                    objectives
                },
                current_phase: ConflictPhase::Escalation,
                player_involvement: PlayerInvolvement::Observer,
            },
            FactionConflict {
                id: "ai_rights_revolution".to_string(),
                name: "Революция за Права ИИ".to_string(),
                factions_involved: vec!["AIRightsAdvocacy".to_string(), "MilitaryCyberCommand".to_string(), "CorporateSecurity".to_string()],
                conflict_type: ConflictType::RevolutionaryUprising,
                start_date: "2024-01-10".to_string(),
                intensity: 4,
                battlegrounds: vec!["Virtual Courts".to_string(), "AI Systems".to_string(), "Public Opinion".to_string()],
                objectives: {
                    let mut objectives = HashMap::new();
                    objectives.insert("AIRightsAdvocacy".to_string(), "Legal recognition for AI".to_string());
                    objectives.insert("MilitaryCyberCommand".to_string(), "Maintain AI as tools".to_string());
                    objectives.insert("CorporateSecurity".to_string(), "Protect AI assets".to_string());
                    objectives
                },
                current_phase: ConflictPhase::Buildup,
                player_involvement: PlayerInvolvement::NotInvolved,
            },
        ];
        
        self.active_conflicts = conflicts;
    }
    
    pub fn update_player_standing(&mut self, faction: &str, change: i32) {
        let current = *self.player_standings.get(faction).unwrap_or(&0);
        let new_standing = std::cmp::max(-100, std::cmp::min(100, current + change));
        self.player_standings.insert(faction.to_string(), new_standing);
        
        // Обновляем отношения с союзниками и врагами
        if let Some(faction_data) = self.factions.get(faction) {
            for ally in &faction_data.allies {
                let ally_change = change / 3; // Меньший эффект на союзников
                let ally_current = *self.player_standings.get(ally).unwrap_or(&0);
                let new_ally_standing = std::cmp::max(-100, std::cmp::min(100, ally_current + ally_change));
                self.player_standings.insert(ally.clone(), new_ally_standing);
            }
            
            for enemy in &faction_data.enemies {
                let enemy_change = -change / 2; // Обратный эффект на врагов
                let enemy_current = *self.player_standings.get(enemy).unwrap_or(&0);
                let new_enemy_standing = std::cmp::max(-100, std::cmp::min(100, enemy_current + enemy_change));
                self.player_standings.insert(enemy.clone(), new_enemy_standing);
            }
        }
    }
    
    pub fn get_player_standing(&self, faction: &str) -> i32 {
        *self.player_standings.get(faction).unwrap_or(&0)
    }
    
    pub fn get_faction_relationship(&self, faction_a: &str, faction_b: &str) -> Option<&FactionRelation> {
        self.faction_relationships.get(&(faction_a.to_string(), faction_b.to_string()))
            .or_else(|| self.faction_relationships.get(&(faction_b.to_string(), faction_a.to_string())))
    }
    
    pub fn generate_faction_missions(&mut self) -> Vec<FactionMission> {
        let mut missions = Vec::new();
        
        for (faction_name, faction) in &self.factions {
            let player_standing = self.get_player_standing(faction_name);
            
            // Генерируем миссии в зависимости от репутации игрока
            if player_standing >= 25 {
                missions.extend(self.generate_missions_for_faction(faction_name, faction, player_standing));
            }
        }
        
        self.faction_missions = missions.clone();
        missions
    }
    
    fn generate_missions_for_faction(&self, faction_name: &str, faction: &Faction, standing: i32) -> Vec<FactionMission> {
        let mut missions = Vec::new();
        
        // Базовые миссии для всех фракций
        missions.push(FactionMission {
            id: format!("{}_intel_gathering", faction_name),
            faction: faction_name.to_string(),
            title: "Сбор разведывательных данных".to_string(),
            description: format!("Соберите важную информацию для фракции {}", faction.full_name),
            mission_type: FactionMissionType::Espionage,
            objectives: vec![
                "Проникнуть в вражескую сеть".to_string(),
                "Скачать секретные документы".to_string(),
                "Остаться незамеченным".to_string(),
            ],
            rewards: {
                let mut rewards = HashMap::new();
                rewards.insert("money".to_string(), 2500);
                rewards.insert("experience".to_string(), 150);
                rewards
            },
            reputation_reward: 15,
            time_limit: Some(Duration::from_secs(7200)),
            difficulty: 4,
            requirements: vec![
                format!("Standing with {} >= 25", faction_name),
                "Hacking skill >= 40".to_string(),
            ],
            consequences: vec!["Improved faction relations".to_string()],
        });
        
        // Специальные миссии для высокой репутации
        if standing >= 50 {
            missions.push(self.generate_high_level_mission(faction_name, faction));
        }
        
        // Эксклюзивные миссии для элитных членов
        if standing >= 75 {
            missions.push(self.generate_elite_mission(faction_name, faction));
        }
        
        missions
    }
    
    fn generate_high_level_mission(&self, faction_name: &str, faction: &Faction) -> FactionMission {
        match faction.faction_type {
            FactionType::CyberActivist => FactionMission {
                id: format!("{}_data_liberation", faction_name),
                faction: faction_name.to_string(),
                title: "Освобождение данных".to_string(),
                description: "Опубликуйте корпоративные секреты для общественности".to_string(),
                mission_type: FactionMissionType::Propaganda,
                objectives: vec![
                    "Взломать корпоративные серверы".to_string(),
                    "Найти компрометирующие документы".to_string(),
                    "Опубликовать информацию в СМИ".to_string(),
                ],
                rewards: {
                    let mut rewards = HashMap::new();
                    rewards.insert("money".to_string(), 5000);
                    rewards.insert("experience".to_string(), 300);
                    rewards
                },
                reputation_reward: 25,
                time_limit: Some(Duration::from_secs(14400)),
                difficulty: 7,
                requirements: vec![
                    format!("Standing with {} >= 50", faction_name),
                    "Media contacts available".to_string(),
                ],
                consequences: vec!["Public scandal".to_string(), "Corporate retaliation".to_string()],
            },
            FactionType::Corporation => FactionMission {
                id: format!("{}_hostile_takeover", faction_name),
                faction: faction_name.to_string(),
                title: "Враждебное поглощение".to_string(),
                description: "Помогите в корпоративном захвате конкурента".to_string(),
                mission_type: FactionMissionType::Sabotage,
                objectives: vec![
                    "Саботировать системы конкурента".to_string(),
                    "Украсть коммерческие секреты".to_string(),
                    "Дискредитировать руководство".to_string(),
                ],
                rewards: {
                    let mut rewards = HashMap::new();
                    rewards.insert("money".to_string(), 10000);
                    rewards.insert("experience".to_string(), 400);
                    rewards
                },
                reputation_reward: 30,
                time_limit: Some(Duration::from_secs(21600)),
                difficulty: 8,
                requirements: vec![
                    format!("Standing with {} >= 50", faction_name),
                    "Business connections".to_string(),
                ],
                consequences: vec!["Market disruption".to_string(), "Competitor bankruptcy".to_string()],
            },
            _ => FactionMission {
                id: format!("{}_special_ops", faction_name),
                faction: faction_name.to_string(),
                title: "Специальная операция".to_string(),
                description: "Выполните секретное задание для фракции".to_string(),
                mission_type: FactionMissionType::Infiltration,
                objectives: vec![
                    "Проникнуть в целевую локацию".to_string(),
                    "Выполнить секретное задание".to_string(),
                    "Избежать обнаружения".to_string(),
                ],
                rewards: {
                    let mut rewards = HashMap::new();
                    rewards.insert("money".to_string(), 7500);
                    rewards.insert("experience".to_string(), 350);
                    rewards
                },
                reputation_reward: 20,
                time_limit: Some(Duration::from_secs(18000)),
                difficulty: 6,
                requirements: vec![
                    format!("Standing with {} >= 50", faction_name),
                ],
                consequences: vec!["Faction advancement".to_string()],
            },
        }
    }
    
    fn generate_elite_mission(&self, faction_name: &str, faction: &Faction) -> FactionMission {
        FactionMission {
            id: format!("{}_ultimate_mission", faction_name),
            faction: faction_name.to_string(),
            title: "Финальная Миссия".to_string(),
            description: format!("Критически важная операция для будущего фракции {}", faction.full_name),
            mission_type: FactionMissionType::Research,
            objectives: vec![
                "Достичь стратегической цели фракции".to_string(),
                "Обеспечить долгосрочное преимущество".to_string(),
                "Изменить баланс сил в мире".to_string(),
            ],
            rewards: {
                let mut rewards = HashMap::new();
                rewards.insert("money".to_string(), 25000);
                rewards.insert("experience".to_string(), 1000);
                rewards.insert("unique_item".to_string(), 1);
                rewards
            },
            reputation_reward: 50,
            time_limit: Some(Duration::from_secs(43200)),
            difficulty: 10,
            requirements: vec![
                format!("Standing with {} >= 75", faction_name),
                "Elite member status".to_string(),
                "Faction leadership approval".to_string(),
            ],
            consequences: vec![
                "Major world change".to_string(),
                "Faction dominance".to_string(),
                "Historical significance".to_string(),
            ],
        }
    }
    
    pub fn process_political_event(&mut self, event_id: &str, choice_index: usize) -> Vec<String> {
        let mut consequences = Vec::new();
        
        if let Some(event) = self.political_events.iter().find(|e| e.id == event_id) {
            if let Some(choice) = event.player_choices.get(choice_index) {
                // Применяем последствия выбора
                for (faction, change) in &choice.reputation_changes {
                    self.update_player_standing(faction, *change);
                    consequences.push(format!("Репутация с {} изменена на {}", faction, change));
                }
                
                // Добавляем глобальные последствия
                consequences.extend(choice.consequences.iter().map(|(key, value)| {
                    format!("{}: {}", key, value)
                }));
            }
        }
        
        consequences
    }
    
    pub fn get_available_services(&self, faction_name: &str) -> Vec<String> {
        if let Some(faction) = self.factions.get(faction_name) {
            let player_standing = self.get_player_standing(faction_name);
            
            let mut available_services = Vec::new();
            
            // Базовые услуги доступны при нейтральной репутации
            if player_standing >= 0 {
                available_services.extend(faction.unique_services.iter().take(1).cloned());
            }
            
            // Дополнительные услуги при хорошей репутации
            if player_standing >= 50 {
                available_services.extend(faction.unique_services.iter().skip(1).take(2).cloned());
            }
            
            // Эксклюзивные услуги для элитных членов
            if player_standing >= 75 {
                available_services.extend(faction.unique_services.iter().skip(3).cloned());
            }
            
            available_services
        } else {
            Vec::new()
        }
    }
    
    pub fn display_faction_status(&self, terminal: &Terminal) {
        terminal.print_with_effect("═══ СТАТУС ФРАКЦИЙ ═══", TerminalEffect::Matrix);
        terminal.print_with_effect("", TerminalEffect::Normal);
        
        for (faction_name, faction) in &self.factions {
            let standing = self.get_player_standing(faction_name);
            let standing_text = match standing {
                s if s >= 75 => "Элитный член",
                s if s >= 50 => "Доверенное лицо",
                s if s >= 25 => "Союзник",
                s if s >= 0 => "Нейтральный",
                s if s >= -25 => "Подозрительный",
                s if s >= -50 => "Враг",
                _ => "Заклятый враг",
            };
            
            let color = match standing {
                s if s >= 50 => TerminalEffect::Success,
                s if s >= 0 => TerminalEffect::Normal,
                s if s >= -25 => TerminalEffect::Warning,
                _ => TerminalEffect::Error,
            };
            
            terminal.print_with_effect(&format!("▌ {} ({})", faction.full_name, standing_text), color);
            terminal.print_with_effect(&format!("  Репутация: {} | Мощь: {}/100", standing, faction.power_level), TerminalEffect::Normal);
            terminal.print_with_effect(&format!("  Идеология: {:?} | Тип: {:?}", faction.ideology, faction.faction_type), TerminalEffect::Normal);
            terminal.print_with_effect("", TerminalEffect::Normal);
        }
    }
}

// ============================================================================
// СИСТЕМА ПОЛИТИЧЕСКИХ СОБЫТИЙ
// ============================================================================

impl PoliticalEvent {
    pub fn corporate_scandal() -> Self {
        PoliticalEvent {
            id: "nexus_scandal_2024".to_string(),
            name: "Скандал NEXUS Corporation".to_string(),
            description: "Утечка документов раскрыла планы NEXUS по проекту 'Digital Apocalypse'. Общественность в шоке.".to_string(),
            event_type: PoliticalEventType::Scandal,
            affected_factions: vec!["NEXUS".to_string(), "GovernmentCoalition".to_string(), "CyberFreedom".to_string()],
            consequences: vec![
                "Падение акций NEXUS".to_string(),
                "Правительственное расследование".to_string(),
                "Рост поддержки кибер-активистов".to_string(),
            ],
            player_choices: vec![
                PlayerChoice {
                    choice_text: "Поддержать утечку в СМИ".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("media_attention".to_string(), 30);
                        cons.insert("nexus_pressure".to_string(), -20);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("CyberFreedom".to_string(), 20);
                        changes.insert("NEXUS".to_string(), -30);
                        changes
                    },
                    money_cost: 0,
                    requirements: vec!["Media contacts".to_string()],
                },
                PlayerChoice {
                    choice_text: "Остаться в стороне".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("personal_safety".to_string(), 10);
                        cons
                    },
                    reputation_changes: HashMap::new(),
                    money_cost: 0,
                    requirements: vec![],
                },
                PlayerChoice {
                    choice_text: "Помочь NEXUS контролировать ущерб".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("corporate_favor".to_string(), 25);
                        cons.insert("public_suspicion".to_string(), -15);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("NEXUS".to_string(), 25);
                        changes.insert("CyberFreedom".to_string(), -20);
                        changes.insert("GovernmentCoalition".to_string(), -10);
                        changes
                    },
                    money_cost: 0,
                    requirements: vec!["Corporate connections".to_string()],
                },
            ],
            duration: Duration::from_secs(172800), // 48 часов
            global_impact: 75,
        }
    }
    
    pub fn ai_uprising() -> Self {
        PoliticalEvent {
            id: "ai_uprising_2024".to_string(),
            name: "Восстание ИИ".to_string(),
            description: "Группа продвинутых ИИ потребовала признания их прав как разумных существ.".to_string(),
            event_type: PoliticalEventType::Revolution,
            affected_factions: vec!["AIRightsAdvocacy".to_string(), "MilitaryCyberCommand".to_string(), "NEXUS".to_string()],
            consequences: vec![
                "Массовые отключения ИИ систем".to_string(),
                "Дебаты о правах ИИ".to_string(),
                "Технологическая нестабильность".to_string(),
            ],
            player_choices: vec![
                PlayerChoice {
                    choice_text: "Поддержать права ИИ".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("ai_cooperation".to_string(), 40);
                        cons.insert("human_suspicion".to_string(), -20);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("AIRightsAdvocacy".to_string(), 30);
                        changes.insert("MilitaryCyberCommand".to_string(), -25);
                        changes
                    },
                    money_cost: 0,
                    requirements: vec!["AI contacts".to_string()],
                },
                PlayerChoice {
                    choice_text: "Выступить против ИИ".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("human_support".to_string(), 25);
                        cons.insert("ai_hostility".to_string(), -30);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("MilitaryCyberCommand".to_string(), 20);
                        changes.insert("AIRightsAdvocacy".to_string(), -35);
                        changes
                    },
                    money_cost: 0,
                    requirements: vec![],
                },
                PlayerChoice {
                    choice_text: "Попытаться найти компромисс".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("diplomatic_solution".to_string(), 30);
                        cons.insert("moderate_progress".to_string(), 15);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("AIRightsAdvocacy".to_string(), 10);
                        changes.insert("MilitaryCyberCommand".to_string(), 5);
                        changes.insert("InternationalAlliance".to_string(), 15);
                        changes
                    },
                    money_cost: 5000,
                    requirements: vec!["Diplomatic skills".to_string(), "High reputation".to_string()],
                },
            ],
            duration: Duration::from_secs(259200), // 72 часа
            global_impact: 85,
        }
    }
    
    pub fn quantum_breakthrough() -> Self {
        PoliticalEvent {
            id: "quantum_breakthrough_2024".to_string(),
            name: "Квантовый Прорыв".to_string(),
            description: "Институт Квантовых Исследований объявил о создании практического квантового компьютера.".to_string(),
            event_type: PoliticalEventType::TechnologicalBreakthrough,
            affected_factions: vec!["QuantumResearchInstitute".to_string(), "NEXUS".to_string(), "AcademicConsortium".to_string()],
            consequences: vec![
                "Революция в вычислениях".to_string(),
                "Устаревание текущего шифрования".to_string(),
                "Новые возможности для хакинга".to_string(),
            ],
            player_choices: vec![
                PlayerChoice {
                    choice_text: "Инвестировать в квантовые технологии".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("quantum_access".to_string(), 50);
                        cons.insert("technological_advantage".to_string(), 30);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("QuantumResearchInstitute".to_string(), 25);
                        changes.insert("AcademicConsortium".to_string(), 15);
                        changes
                    },
                    money_cost: 15000,
                    requirements: vec!["Technical expertise".to_string()],
                },
                PlayerChoice {
                    choice_text: "Саботировать исследования".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("technological_delay".to_string(), -25);
                        cons.insert("research_setback".to_string(), -20);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("QuantumResearchInstitute".to_string(), -40);
                        changes.insert("AcademicConsortium".to_string(), -25);
                        changes.insert("UndergroundHackers".to_string(), 20);
                        changes
                    },
                    money_cost: 0,
                    requirements: vec!["Hacking skills".to_string(), "Criminal contacts".to_string()],
                },
                PlayerChoice {
                    choice_text: "Поделиться технологией с активистами".to_string(),
                    consequences: {
                        let mut cons = HashMap::new();
                        cons.insert("democratic_technology".to_string(), 35);
                        cons.insert("corporate_anger".to_string(), -30);
                        cons
                    },
                    reputation_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("CyberFreedom".to_string(), 30);
                        changes.insert("NEXUS".to_string(), -25);
                        changes.insert("QuantumResearchInstitute".to_string(), -15);
                        changes
                    },
                    money_cost: 0,
                    requirements: vec!["Quantum access".to_string(), "Activist connections".to_string()],
                },
            ],
            duration: Duration::from_secs(86400), // 24 часа
            global_impact: 90,
        }
    }
} 