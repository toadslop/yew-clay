enum FirstDayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[derive(Debug, PartialEq)]
pub struct AriaLabels {
    button_choose_date: &'static str,
    button_dot: &'static str,
    button_next_month: &'static str,
    button_previous_month: &'static str,
    input: Option<&'static str>,
}

#[derive(Debug, PartialEq)]
pub struct Years {
    end: u8,
    start: u8,
}
