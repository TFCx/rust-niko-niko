use enum_map::EnumMap;
use enum_map::enum_map;
use enum_map::Enum;

use std::{collections::{HashMap, HashSet}};

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

    fn new(date:Date) -> DayAnalysis {
        DayAnalysis {
            date : date,
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
    hash_map:HashMap<String, Mood>
}

impl InProgressReport {

    fn new() -> InProgressReport {
        InProgressReport {
            hash_map : HashMap::new()
        }
    }

    fn add_or_change_report(&mut self, user:String, mood:Mood) {
        self.hash_map.insert(user, mood);
    }
}

struct NikoNiko {
    all_users:HashSet<String>,
    all_teams_history:HashMap<String, TeamReport>,
    pending:Vec<InProgressReport>,
}

impl NikoNiko {

    fn new() -> NikoNiko {
        NikoNiko {
            all_users: HashSet::new(),
            all_teams_history : HashMap::new(),
            pending : vec!(),
        }
    }

    fn create_team(&mut self, team_name:String) {
        let new_team = TeamReport::new(team_name.clone());
        self.all_teams_history.insert(team_name, new_team);
    }

    fn create_user(&mut self, user_name:String) {
        self.all_users.insert(user_name);
    }

    fn add_user_to_group(&mut self, user_name:String, team_name:String) {
        if None == self.all_users.get(&user_name) {
            return;
        }
        let report = self.all_teams_history.get_mut(&team_name);
        match report {
            Some(report) => {
                report.add_user(user_name.clone());
            },
            None => return,
        }
    }

    fn remove_user_to_group(&mut self, user_name:String, team_name:String) {
        if None == self.all_users.get(&user_name) {
            return;
        }
        let report = self.all_teams_history.get_mut(&team_name);
        match report {
            Some(report) => {
                report.remove_user(user_name.clone());
            },
            None => return,
        }
    }

    // fn add_user_mood(&mut self, user:User, mood:Mood) {

    //     if (pending.empty()) {
    //         let ipr = InProgressReport::new();
    //     } else {

    //     }


    // }
}

#[derive(Debug, Clone)]
struct TeamReport {
    name:String,
    moods_history:Vec<DayAnalysis>,
    users:Vec<String>
}

impl TeamReport {

    fn new(team_name:String) -> TeamReport {
        TeamReport {
            name : team_name,
            moods_history : vec!(),
            users : vec!(),
        }
    }

    fn add_user(&mut self, user:String) {
        self.users.push(user)
    }

    fn remove_user(&mut self, user:String) {
        if let Some(pos) = self.users.iter().position(|x| *x == user) {
            self.users.remove(pos);
        }
    }

    fn transfer(&mut self, report:&InProgressReport) {
        for (user, mood) in &report.hash_map {

            // Is the user part of the group ?
            if let Some(_) = self.users.iter().position(|x| x == user) {

                // At least one day analysis
                if let Some(da) = self.moods_history.last_mut() {
                    if da.date == OffsetDateTime::try_now_local().unwrap().date() {
                        da.add_mood(OffsetDateTime::try_now_local().unwrap().date(), *mood);
                    }
                } else {
                    let now = if let Ok(odt) = OffsetDateTime::try_now_local() {
                        odt.date()
                    } else { Date::try_from_ymd(1, 1, 1).unwrap() };

                    let mut da = DayAnalysis::new(now);
                    da.add_mood(now, *mood);
                    self.moods_history.push(da);
                }

            }
        }
    }

    fn add_mood(&mut self, date:Date, mood:Mood) {
        assert_eq!(date, OffsetDateTime::try_now_local().unwrap().date());
    }
}

fn main() {

    let mut nikoniko = NikoNiko::new();
    // let nikoniko.configure()
    nikoniko.create_team("team1".to_owned());
    nikoniko.create_team("team2".to_owned());
    nikoniko.create_team("team3".to_owned());
    nikoniko.create_user("user1".to_owned());
    nikoniko.create_user("user2".to_owned());
    nikoniko.create_user("user3".to_owned());
    nikoniko.add_user_to_group("user1".to_owned(), "team1".to_owned());
    nikoniko.add_user_to_group("user2".to_owned(), "team2".to_owned());
    nikoniko.add_user_to_group("user3".to_owned(), "team1".to_owned());
    nikoniko.add_user_to_group("user2".to_owned(), "team".to_owned());
    //nikoniko.user_add_mood("user1", mood);
    //nikoniko.user_add_mood("user2", mood);
    // nikoniko.user_add_mood("user", mood) <= rempl le in progress si encore valable, sinon transfert et en cr??e un nouveau

    // let user_a = User::new("John".to_string());
    // let user_b = User::new("Mary".to_string());
    // let user_c = User::new("Peter".to_string());
    // let user_d = User::new("Alan".to_string());
    // let user_e = User::new("Pixie".to_string());

    // let mut team_xxx = TeamReport::new();
    // let mut team_yyy = TeamReport::new();
    // let mut team_zzz = TeamReport::new();
    // let mut team_all = TeamReport::new();

    // team_xxx.add_user(user_a.clone());
    // team_xxx.add_user(user_b.clone());
    // team_xxx.add_user(user_c.clone());

    // team_yyy.add_user(user_a.clone());
    // team_yyy.add_user(user_d.clone());
    // team_yyy.add_user(user_e.clone());

    // team_zzz.add_user(user_b.clone());
    // team_zzz.add_user(user_c.clone());
    // team_zzz.add_user(user_d.clone());
    // team_zzz.remove_user(user_c.clone());

    // team_all.add_user(user_a.clone());
    // team_all.add_user(user_b.clone());
    // team_all.add_user(user_c.clone());
    // team_all.add_user(user_d.clone());
    // team_all.add_user(user_e.clone());

    // let mut day_d1 = InProgressReport::new();
    // day_d1.add_or_change_report(user_a.clone(), Mood::Red);
    // day_d1.add_or_change_report(user_b.clone(), Mood::Green);
    // day_d1.add_or_change_report(user_c.clone(), Mood::Yellow);
    // day_d1.add_or_change_report(user_d.clone(), Mood::Yellow);
    // day_d1.add_or_change_report(user_e.clone(), Mood::Blue);

    // let mut day_d2 = InProgressReport::new();
    // day_d2.add_or_change_report(user_a.clone(), Mood::Red);
    // day_d2.add_or_change_report(user_b.clone(), Mood::Green);
    // day_d2.add_or_change_report(user_c.clone(), Mood::Yellow);
    // day_d2.add_or_change_report(user_d.clone(), Mood::Yellow);
    // day_d2.add_or_change_report(user_e.clone(), Mood::Blue);

    // let now = OffsetDateTime::try_now_local().unwrap();
    // let date_today = now.date();
    // let date_yesterday = date_today.previous_day();

    // team_xxx.transfer(&day_d1);
    // team_yyy.transfer(&day_d1);
    // team_zzz.transfer(&day_d1);
    // team_all.transfer(&day_d1);

    // team_xxx.transfer(&day_d2);
    // team_yyy.transfer(&day_d2);
    // team_zzz.transfer(&day_d2);
    // team_all.transfer(&day_d2);
    //team_report.add_mood(date_yesterday.clone(), Mood::Blue);
    //team_report.add_mood(date_yesterday.clone(), Mood::Blue);
    // dbg!(&mut team_xxx);
    // dbg!(&mut team_yyy);
    // dbg!(&mut team_zzz);
    // dbg!(&mut team_all);

}

// Vec des pairs (date, vec(humeurs))
