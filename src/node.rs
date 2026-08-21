use chrono::{NaiveDate, Utc};
use cosmic_text::Attrs;

pub struct MarkdownNode<'a> {
    pub text: String,
    pub attrs: Attrs<'a>,
}

#[derive(Clone)]
pub struct HabitNode<'a> {
    pub title: String,
    pub state: HabitState,
    pub scheduled: Option<NaiveDate>,
    pub last_repeat: Option<NaiveDate>,
    pub attrs: Attrs<'a>,
}

#[derive(Clone)]
pub enum HabitState {
    TODO,
    TODAY,
    PROGRESS,
    BLOCKED,
    CANC,
    DONE,
    IGNORE,
}

impl<'a> HabitNode<'a> {
    pub fn new(attrs: Attrs<'a>) -> Self {
        Self {
            title: String::from(""),
            scheduled: None,
            state: HabitState::IGNORE,
            last_repeat: None,
            attrs: attrs.clone(),
        }
    }

    pub fn is_scheduled_past(&self) -> bool {
        if let Some(sched) = self.scheduled {
            return sched < Utc::now().date_naive();
        }
        return false;
    }

    pub fn is_scheduled_today(&self) -> bool {
        if let Some(sched) = self.scheduled {
            return sched == Utc::now().date_naive();
        }
        return false;
    }

    pub fn is_scheduled_later(&self) -> bool {
        if let Some(sched) = self.scheduled {
            return sched > Utc::now().date_naive();
        }
        return false;
    }

    pub fn is_done(&self) -> bool {
        // simple todos
        if let HabitState::DONE = self.state {
            return true;
        }
        // habits
        if let Some(last) = self.last_repeat {
            if last == Utc::now().date_naive()
                && (self.is_scheduled_today() || self.is_scheduled_later())
            {
                return true;
            }
        }
        false
    }
}
