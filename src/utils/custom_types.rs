use std::{fmt::Display, str::FromStr};

pub enum PinMode {
    Loud,
    Silent,
    Error,
}

impl FromStr for PinMode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let ret = match s {
            "loud" | "hard" | "violent" => PinMode::Loud,
            "silent" => PinMode::Silent,
            _ => PinMode::Error,
        };
        Ok(ret)
    }
}

pub enum TimeUnit {
    Sec(u64),
    Min(u64),
    Hrs(u64),
    Day(u64),
}

impl FromStr for TimeUnit {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let split: Vec<_> = s.splitn(2, char::is_whitespace).collect();
        let num;
        let unit;

        if split.len() == 1 && split[0].ends_with(&['h', 'm', 's', 'd'][..]) && split[0].len() >= 2 {
            let mut t = split[0].to_owned();
            let u = t.pop().unwrap().to_string();
            t = t.to_string();

            num = match t.parse::<u64>() {
                Ok(n) => n,
                Err(_) => { return Err("Invalid time unit.");}
            };

            unit = u;
        } else if split.len() == 2 {
            num = match split[0].parse::<u64>() {
                Ok(n) => n,
                Err(_) => { return Err("Invalid time unit."); }
            };

            unit = split[1].to_owned();
        } else {
            return Err("Invalid time unit.");
        }

        match &unit as &str {
            "h" | "hours" => Ok(TimeUnit::Hrs(num)),
            "m" | "minutes" => Ok(TimeUnit::Min(num)),
            "s" | "seconds" => Ok(TimeUnit::Sec(num)),
            "d" | "days" => Ok(TimeUnit::Day(num)),
            _ => Err("Invalid time unit."),
        }
    }
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::Sec(t) => write!(f, "{} seconds.", t),
            TimeUnit::Min(t) => write!(f, "{} minutes.", t),
            TimeUnit::Hrs(t) => write!(f, "{} hours.", t),
            TimeUnit::Day(t) => write!(f, "{} days.", t),
        }
    }
}

pub enum LockType {
    Text(String),
    Media(String),
    Poll(String),
    Web(String),
    Other(String),
    Error(String),
}

impl FromStr for LockType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let kind = String::new();
        let ret = match s {
            "all" | "text" => LockType::Text(kind),
            "sticker" | "gif" => LockType::Other(kind),
            "url" | "web" => LockType::Web(kind),
            "media" => LockType::Media(kind),
            "poll" => LockType::Poll(kind),
            _ => LockType::Error(kind),
        };
        Ok(ret)
    }
}

impl Display for LockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LockType::Text(kind) => write!(f, "{}ed <i>all</i> for Non-admins.", kind),
            LockType::Other(kind) => write!(f, "{}ed <i>sticker,gif,game</i> for Non-Admins.", kind),
            LockType::Media(kind) => write!(f, "{}ed <i>Media(photos,animations,documents,stickers/gif,video)</i> for Non-Admins.", kind),
            LockType::Web(kind) => write!(f, "{}ed <i>URL</i> previewing for Non-Admins.", kind),
            LockType::Poll(kind) => write!(f, "{}ed <i>Polls</i> for Non-Admins.", kind),
            LockType::Error(_) => write!(f, "Invalid locktype, please run /locktypes to check available locktypes."),
        }
    }
}

pub enum GbanStats {
    On,
    Off,
    Error,
}

impl FromStr for GbanStats {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "yes" | "on" => Ok(GbanStats::On),
            "no" | "off" => Ok(GbanStats::Off),
            _ => Ok(GbanStats::Error),
        }
    }
}

pub enum WarnMode {
    Soft,
    Hard,
    Error,
}

impl FromStr for WarnMode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "soft" | "smooth" => Ok(WarnMode::Soft),
            "hard" | "strong" => Ok(WarnMode::Hard),
            _ => Ok(WarnMode::Error),
        }
    }
}

pub enum DisableAble {
    Ud,
    Info,
    Start,
    Paste,
    Kickme,
    Adminlist,
    Error,
}

impl FromStr for DisableAble {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "ud" => Ok(DisableAble::Ud),
            "info" => Ok(DisableAble::Info),
            "start" => Ok(DisableAble::Start),
            "paste" => Ok(DisableAble::Paste),
            "kickme" => Ok(DisableAble::Kickme),
            "adminlist" => Ok(DisableAble::Adminlist),
            _ => Ok(DisableAble::Error),
        }
    }
}

pub enum FilterType {
    Animation,
    Audio,
    Sticker,
    Photos,
    Document,
    Text,
    Voice,
    Video,
    Error,
}

impl FromStr for FilterType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "animation" => Ok(FilterType::Animation),
            "audio" => Ok(FilterType::Audio),
            "sticker" => Ok(FilterType::Sticker),
            "photo" => Ok(FilterType::Photos),
            "document" => Ok(FilterType::Document),
            "text" => Ok(FilterType::Text),
            "voice" => Ok(FilterType::Voice),
            "video" => Ok(FilterType::Video),
            _ => Ok(FilterType::Error),
        }
    }
}

pub enum BlacklistMode {
    Delete,
    Warn,
    Ban,
    Kick,
    Error,
}

impl FromStr for BlacklistMode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "delete" => Ok(BlacklistMode::Delete),
            "warn" => Ok(BlacklistMode::Warn),
            "ban" | "hard" => Ok(BlacklistMode::Ban),
            "kick" | "soft" => Ok(BlacklistMode::Kick),
            _ => Ok(BlacklistMode::Error),
        }
    }
}

pub enum ReportStatus {
    On,
    Off,
    Error,
}

impl FromStr for ReportStatus {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "on" | "yes" => Ok(ReportStatus::On),
            "off" | "no" => Ok(ReportStatus::Off),
            _ => Ok(ReportStatus::Error),
        }
    }
}