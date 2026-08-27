use crate::node::{HabitNode, HabitState};
use chrono::{NaiveDate, ParseResult};
use cosmic_text::{Attrs, Metrics, Style, Weight};
use regex::Regex;

pub fn parse_emacs<'a>(input: &str, default_attributes: &Attrs<'a>) -> Vec<HabitNode<'a>> {
    let mut output: Vec<HabitNode> = Vec::new();
    let mut working_node = HabitNode::new(default_attributes.clone());
    let mut ignore_me = false;
    let rx = Regex::new(r"([\*]*)( TODO | PROGRESS | TODAY | DONE )|(\:(.*)\:)").unwrap();

    for line in input.lines() {
        let tline = line.trim();

        if tline.is_empty() || tline.chars().count() < 3 {
            continue;
        }

        if tline.starts_with("**") {
            // new entry!!
            if !working_node.title.is_empty() && !ignore_me {
                output.push(working_node)
            };
            working_node = HabitNode::new(default_attributes.clone());
            ignore_me = false;
            working_node.state = parse_todo_state(tline);

            match working_node.state {
                HabitState::TODO | HabitState::PROGRESS | HabitState::DONE | HabitState::TODAY => {
                    working_node.title = rx.replace_all(line, "").trim().to_owned() + "\n";
                }
                _ => {
                    ignore_me = true;
                }
            }
            continue;
        }

        // otherwise this is a child line
        if tline.starts_with("SCHEDULED") {
            working_node.scheduled =
                Some(parse_date(tline, "<", ">").expect("parsing error with scheduled date :("));
        }
        if tline.starts_with(":LAST_REPEAT:") || tline.starts_with("CLOSED:") {
            working_node.last_repeat =
                Some(parse_date(tline, "[", "]").expect("parsing error with last repeat date :("));
        }
    }
    output
}

fn parse_date(st: &str, s_delim: &str, e_delim: &str) -> ParseResult<NaiveDate> {
    let sdate = st.rfind(s_delim);
    let edate = st.rfind(e_delim);
    if let (Some(s), Some(e)) = (sdate, edate) {
        let full_datestring = &st[s + 1..e];
        let date_part = &full_datestring[0..10];
        // let repetition_part = &full_datestring[10..];
        NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
    } else {
        NaiveDate::parse_from_str("", "")
    }
}

fn parse_todo_state(st: &str) -> HabitState {
    if st.contains(" TODO ") {
        return HabitState::TODO;
    }
    if st.contains(" TODAY ") {
        return HabitState::TODAY;
    }
    if st.contains(" PROGRESS ") {
        return HabitState::TODO;
    }
    if st.contains(" BLOCKED ") {
        return HabitState::BLOCKED;
    }
    if st.contains(" CANC ") {
        return HabitState::CANC;
    }
    if st.contains(" DONE ") {
        return HabitState::DONE;
    }
    HabitState::IGNORE
}
pub fn add_todo_symbols(habits: &mut Vec<HabitNode>) {
    for habit in habits {
        match habit.is_done() {
            true => habit.title = "     ".to_owned() + &habit.title,
            false => {
                habit.title = if habit.is_scheduled_past() {
                    "     ".to_owned()
                } else {
                    "     ".to_owned()
                } + &habit.title
            }
        }
    }
}

pub fn sort_todos<'a>(habits: &Vec<HabitNode<'a>>) -> Vec<HabitNode<'a>> {
    let mut sorted: Vec<HabitNode> = Vec::new();
    let first = habits.first();
    let mut separator;

    if let Some(habit) = first {
        separator = HabitNode::new(habit.attrs.clone());
    } else {
        return Vec::new();
    }

    // extremely naiive soln TODO please come back to me
    separator.attrs = separator
        .attrs
        .weight(Weight::BOLD)
        .style(Style::Italic)
        .metrics(Metrics::new(28., 34.));

    separator.title = "Overdue:\n".to_owned();
    sorted.push(separator.clone());
    for habit in habits {
        if habit.is_scheduled_past() && !habit.is_done() {
            sorted.push(habit.clone());
        }
    }

    separator.title = "Today:\n".to_owned();
    sorted.push(separator.clone());
    for habit in habits {
        if habit.is_scheduled_today() && !habit.is_done() {
            sorted.push(habit.clone());
        }
    }

    separator.title = "Later:\n".to_owned();
    sorted.push(separator.clone());
    for habit in habits {
        if habit.is_scheduled_later() && !habit.is_done() {
            sorted.push(habit.clone())
        }
    }

    separator.title = "Done! :)\n".to_owned();
    sorted.push(separator.clone());
    for habit in habits {
        if habit.is_done() {
            sorted.push(habit.clone())
        }
    }

    sorted
}
