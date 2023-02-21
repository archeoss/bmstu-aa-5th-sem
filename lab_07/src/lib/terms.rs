use super::constants;
use std::str::FromStr;
// use super::{DictEntry, Map};
use regex::{Error, Regex};
use std::slice::Iter;
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Terms {
    VeryAccurate,
    Accurate,
    NotSoAccurate,
    Inaccurate,
    VeryInaccurate,
}

impl Terms {
    pub fn value(&self) -> (f32, f32) {
        use self::Terms::*;
        match self {
            VeryAccurate => (0.0, 2.875),
            Accurate => (2.875, 6.0),
            NotSoAccurate => (6.0, 7.0),
            Inaccurate => (7.0, 8.375),
            VeryInaccurate => (8.375, f32::INFINITY),
        }
    }
    pub fn iter() -> Iter<'static, Terms> {
        use self::Terms::*;
        static TERMS: [Terms; constants::N_TERMS] = [
            VeryAccurate,
            Accurate,
            NotSoAccurate,
            Inaccurate,
            VeryInaccurate,
        ];
        TERMS.iter()
    }

    pub fn to_string(&self) -> String {
        static STRING_TERMS: [&'static str; constants::N_TERMS] = [
            "VeryAccurate",
            "Accurate",
            "NotSoAccurate",
            "Inaccurate",
            "VeryInaccurate",
        ];
        for (id, t) in Self::iter().enumerate() {
            if *t == *self {
                return STRING_TERMS[id].to_string().clone();
            }
        }
        String::new()
    }
}

impl FromStr for Terms {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Terms::*;
        let s = s.to_lowercase();
        if s.contains(&"неметк") {
            if s.contains(&"очень") {
                Ok(VeryInaccurate)
            } else {
                Ok(Inaccurate)
            }
        } else if s.contains(&"метк") {
            if s.contains(&"очень") {
                if s.contains(&"не") {
                    Ok(NotSoAccurate)
                } else {
                    Ok(VeryAccurate)
                }
            } else {
                Ok(Accurate)
            }
        } else {
            Err(regex::Error::Syntax(
                "Строка не соответствует термам".to_string(),
            ))
        }
    }

    type Err = Error;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Task {
    ExactTerm,
    InBetween,
    AllExcept,
    BetterThan,
    WorseThan,
    LikeSomebody,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub terms: Vec<Terms>,
    pub except: Vec<Terms>,
    pub task: Task,
    pub shooters: Vec<String>,
}

pub fn generate_job(mut query: &str) -> Option<Job> {
    // Проверяем, что запрос содержит слово "стрелков"
    let re_shooter = Regex::new(r"\bстрел\w+\b").unwrap();
    if !re_shooter.is_match(query) {
        return None;
    }

    let mut terms: Vec<Terms> = Vec::new();
    let mut except: Vec<Terms> = Vec::new();
    let mut task = Task::ExactTerm;
    let mut shooters: Vec<String> = Vec::new();
    // let exact_regex= Regex::new(r"(?i)очень\s+метк[ийхе]").unwrap();
    if query.contains(&"между") {
        task = Task::InBetween;
    } else if query.contains(&"всех стрелков кроме") {
        task = Task::AllExcept;
    } else if Regex::new(r"(?i)стрелков\s(\w*\s?)лучше")
        .unwrap()
        .is_match(query)
    {
        task = Task::BetterThan;
    } else if Regex::new(r"(?i)стрелков\s(\w*\s?)хуже")
        .unwrap()
        .is_match(query)
    {
        task = Task::WorseThan;
    } else if query.contains(&" как ") {
        task = Task::LikeSomebody;
    }
    let mut regexes = Vec::new();
    regexes.push(Regex::new(r"(?i)(?:не\s)?(?:очень\s)?(?:не)?метк[иыойхе][миыойхе]").unwrap());
    if query.contains(&"кроме") {
        let ind = query.find(&"кроме").unwrap();
        let except_terms = query.split(&"кроме").collect::<Vec<&str>>();
        query = except_terms[0];
        let except_terms = except_terms[1];
        for reg in regexes.iter() {
            for captures in reg.captures_iter(except_terms) {
                for cap in captures.iter().flatten() {
                    except.push(Terms::from_str(cap.as_str()).unwrap());
                }
            }
        }
    }
    if task == Task::LikeSomebody {
        // let ind = query.find(&" как ").unwrap();
        let shooters_ = query.split(&" как ").collect::<Vec<&str>>();
        query = shooters_[0];
        let shooters_ = shooters_[1].trim();
        let mut shooters_ = shooters_.split(&",").collect::<Vec<&str>>();
        shooters_
            .iter_mut()
            .map(|s| s.split(" и ").collect::<Vec<&str>>());

        for item in shooters_ {
            shooters.push(item.to_string());
        }
    }
    for reg in regexes.iter() {
        for captures in reg.captures_iter(query) {
            for cap in captures.iter().flatten() {
                terms.push(Terms::from_str(cap.as_str()).unwrap());
            }
        }
    }

    if terms.is_empty() {
        if task == Task::AllExcept {
            for term in Terms::iter() {
                if !except.contains(term) {
                    terms.push(term.clone());
                }
            }
        }
    }

    if terms.is_empty() && task != Task::LikeSomebody
        || task == Task::LikeSomebody && shooters.is_empty()
    {
        None
    } else {
        Some(Job {
            terms,
            except,
            task,
            shooters,
        })
    }
}
