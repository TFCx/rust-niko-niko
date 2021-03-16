use enum_map::EnumMap;
use enum_map::enum_map;
use enum_map::Enum;

use std::collections::HashMap;

extern crate time;
use time::{Date, OffsetDateTime};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    name:String
}

impl User {
    fn new(name : String) -> Self {
        Self { name : name }
    }
}

#[derive(Debug, Enum, Copy, Clone)]
enum Mood {
    Red,
    Orange,
    Yellow,
    Green,
    Blue
}

#[derive(Debug, Clone)]
struct MoodAnalysis {
    hash_map:EnumMap<Mood, i32>
}

impl MoodAnalysis {
    fn new() -> Self {
        let hash_map = enum_map! {
            Mood::Red => 0,
            Mood::Orange => 0,
            Mood::Yellow => 0,
            Mood::Green => 0,
            Mood::Blue => 0,
        };
        Self { hash_map : hash_map }
    }
}


#[derive(Debug, Clone)]
struct DayAnalysis {
    date:Date,
    moods:MoodAnalysis,
}

impl DayAnalysis {

    fn new() -> DayAnalysis {
        DayAnalysis {
            date : OffsetDateTime::try_now_local().unwrap().date(),
            moods : MoodAnalysis::new(),
        }
    }

    fn add_mood(&mut self, date:Date, mood:Mood) {
        assert_eq!(date, self.date);
        self.moods.hash_map[mood] = self.moods.hash_map[mood] + 1;
    }
}

#[derive(Debug, Clone)]
struct InProgressReport {
    hash_map:HashMap<User, Mood>
}

impl InProgressReport {

    fn new() -> InProgressReport {
        InProgressReport {
            hash_map : HashMap::new()
        }
    }

    fn add_or_change_report(&mut self, user:User, mood:Mood) {
        self.hash_map.insert(user, mood);
    }
}

#[derive(Debug, Clone)]
struct TeamReport {
    moods_history:Vec<DayAnalysis>,
    users:Vec<User>
}

impl TeamReport {

    fn new() -> TeamReport {
        TeamReport {
            moods_history : vec!(),
            users : vec!(),
        }
    }

    fn add_mood(&mut self, date:Date, mood:Mood) {
        assert_eq!(date, OffsetDateTime::try_now_local().unwrap().date());
    }
}

fn main() {

    let user_a = User::new("John".to_string());
    let user_b = User::new("Mary".to_string());

    let mut day_d = InProgressReport::new();
    day_d.add_or_change_report(user_a.clone(), Mood::Red);
    day_d.add_or_change_report(user_b.clone(), Mood::Green);
    day_d.add_or_change_report(user_a.clone(), Mood::Yellow);



    let mut team_report = TeamReport::new();
    dbg!(&mut team_report);

    let now = OffsetDateTime::try_now_local().unwrap();
    let date_today = now.date();
    let date_yesterday = date_today.previous_day();
    team_report.add_mood(date_today.clone(), Mood::Red);
    team_report.add_mood(date_today.clone(), Mood::Orange);
    team_report.add_mood(date_today.clone(), Mood::Green);
    team_report.add_mood(date_today.clone(), Mood::Green);
    team_report.add_mood(date_today.clone(), Mood::Blue);
    //team_report.add_mood(date_yesterday.clone(), Mood::Blue);
    //team_report.add_mood(date_yesterday.clone(), Mood::Blue);
    dbg!(&mut team_report);
}

// Vec des pairs (date, vec(humeurs))
