use chrono::{DateTime, Utc};
use log::debug;
use rand::{thread_rng, Rng};
use std::env;
use std::io::prelude::*;
use termion::{color, style};

use super::task::{DBSCANTask, NUMBER_OF_MEASURMENTS};

const DEFAULT_N: usize = 10;
const DEFAULT_STRING_LEN: usize = 200000;
const DEFAULT_PATTERN_LEN: usize = 200;
const RANGE: f64 = 8.0;
const FROM: f64 = 2.0;
const RANGE_PTS: usize = 8;
const FROM_PTS: usize = 2;

enum Part
{
    Part1,
    Part2,
    Part3,
}

enum EventType
{
    Start,
    End,
}

struct Event
{
    task_number: usize,
    part: Part,
    event_type: EventType,
    timestamp: DateTime<Utc>,
}

impl Event
{
    fn from_task_time(task_number: usize, time: DateTime<Utc>, pos: usize) -> Self
    {
        Self {
            timestamp: time.clone(),
            task_number,
            part: match pos / 2 {
                0 => Part::Part1,
                1 => Part::Part2,
                2 => Part::Part3,
                _ => unreachable!("only 3 parts in task"),
            },
            event_type: match pos % 2 {
                0 => EventType::Start,
                1 => EventType::End,
                _ => unreachable!("only 2 types of events"),
            },
        }
    }
}

pub fn get_n() -> usize
{
    let n = env::var("N");
    match n {
        Ok(n) => n.trim().parse().expect("Parse string to usize"),
        Err(_) => DEFAULT_N,
    }
}

pub fn generate_data(n: usize) -> Vec<DBSCANTask>
{
    // let n = get_n();
    let mut rng = thread_rng();
    (0..n)
        .map(|_| {
            DBSCANTask::new(
                "./data/tsne_scores.csv".to_string(),
                rng.gen_range(FROM..(FROM + RANGE)),
                rng.gen_range(FROM_PTS..(FROM_PTS + RANGE_PTS)),
                (0, 1),
            )
        })
        .collect()
}

fn print_task_number(number: usize)
{
    print!(
        "{}{}{} Task №{:<3} {}",
        style::Bold,
        color::Bg(color::White),
        color::Fg(color::Black),
        number + 1,
        style::Reset
    );
}

fn print_event_part(part: Part)
{
    print!("{}", style::Bold);
    match part {
        Part::Part1 => print!(
            "{}{} Part 1 ",
            color::Bg(color::Yellow),
            color::Fg(color::Black)
        ),
        Part::Part2 => print!(
            "{}{} Part 2 ",
            color::Bg(color::Cyan),
            color::Fg(color::Black)
        ),
        Part::Part3 => print!(
            "{}{} Part 3 ",
            color::Bg(color::Magenta),
            color::Fg(color::Black)
        ),
    }
    print!("{}", style::Reset);
}

fn print_event_type(event_type: EventType)
{
    print!("{}", style::Bold);
    match event_type {
        EventType::Start => print!(
            "{}{} Start ",
            color::Bg(color::Green),
            color::Fg(color::Black)
        ),
        EventType::End => print!(
            "{}{}  End  ",
            color::Bg(color::Red),
            color::Fg(color::Black)
        ),
    }
    print!("{}", style::Reset);
}

fn print_event_timestamp(timestamp: DateTime<Utc>)
{
    print!(
        "{}{}{} {} {}",
        style::Bold,
        color::Bg(color::Blue),
        color::Fg(color::Black),
        timestamp,
        style::Reset
    );
}

fn print_sorted_events(events: Vec<Event>)
{
    for event in events {
        print_task_number(event.task_number);
        print_event_part(event.part);
        print_event_type(event.event_type);
        print_event_timestamp(event.timestamp);
        println!("");
    }
}

pub fn show_log(times: Vec<[DateTime<Utc>; NUMBER_OF_MEASURMENTS]>)
{
    let mut events = Vec::with_capacity(times.len() * 6);
    for (task_number, parts_times) in times.iter().enumerate() {
        for (pos, time) in parts_times.iter().enumerate() {
            events.push(Event::from_task_time(task_number, time.clone(), pos));
        }
    }
    events.sort_by_key(|event| event.timestamp);
    print_sorted_events(events);
}

#[derive(Debug, Clone, Default)]
pub struct Focus(pub usize, pub usize);
impl From<Focus> for (usize, usize)
{
    fn from(e: Focus) -> (usize, usize)
    {
        let Focus { 0: a, 1: b } = e;
        (a, b)
    }
}

pub fn pairs_compare_by_x(a: (f64, f64), b: (f64, f64)) -> std::cmp::Ordering
{
    if a.0 < b.0 {
        std::cmp::Ordering::Less
    } else if a.0 > b.0 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

pub fn pairs_compare_by_y(a: (f64, f64), b: (f64, f64)) -> std::cmp::Ordering
{
    if a.1 < b.1 {
        std::cmp::Ordering::Less
    } else if a.1 > b.1 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}
