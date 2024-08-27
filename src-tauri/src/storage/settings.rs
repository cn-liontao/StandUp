use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum CalendarView {
    Quarter,
    Month
}
impl CalendarView {
    fn init() -> CalendarView {
        CalendarView::Quarter
    }
}

fn default_theme() -> String {
    "light".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "default_theme")]
    pub(crate) theme: String,

    #[serde(default = "CalendarView::init")]
    pub calendar_view: CalendarView,

    #[serde(default)]
    pub hide_on_start: bool,

    #[serde(default)]
    pub start_with_system: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: "light".to_string(),
            calendar_view: CalendarView::Quarter,
            hide_on_start: false,
            start_with_system: false
        }
    }
}