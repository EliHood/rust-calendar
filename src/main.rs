use chrono::{prelude::*, Duration};
use datetimeutils::{day_string, days_in_month, month_from_index, month_string, Month};
use slint::{Model, SharedString, VecModel};
use std::rc::Rc;
slint::include_modules!();



fn generate_month(year: u64, month: Option<Month>) -> u64 {
    days_in_month(year, month.unwrap())
}

fn load_calendar(boxes: Rc<VecModel<NewBox>>, year: u64, current_month: Option<Month>) {
    let generate_days = generate_month(year, current_month);

    for new_box in 0..generate_days {
        boxes.insert(
            new_box as usize,
            NewBox {
                visible: true,
                day: new_box as i32 + 1,
            },
        )
    }
}

fn run_calendar(ui: &AppWindow, boxes: Rc<VecModel<NewBox>>, month: u32) {
    let current_month = month_from_index(month as u64);

    load_calendar(boxes.clone(), 2024, current_month);

    let updated_month = month_string(current_month.unwrap());

    ui.set_month(SharedString::from(updated_month));

    ui.set_year(SharedString::from(" 2024"));

    ui.set_boxes(boxes.clone().into());
}

fn get_week_days() -> Rc<VecModel<Weekdays>> {
    let week_vec = vec![
        Weekdays {
            day: SharedString::from("Sunday"),
        },
        Weekdays {
            day: SharedString::from("Monday"),
        },
        Weekdays {
            day: SharedString::from("Tuesday"),
        },
        Weekdays {
            day: SharedString::from("Wednesday"),
        },
        Weekdays {
            day: SharedString::from("Thursday"),
        },
        Weekdays {
            day: SharedString::from("Friday"),
        },
        Weekdays {
            day: SharedString::from("Saturday"),
        },
    ];

    let weekdays = Rc::new(slint::VecModel::<Weekdays>::from(Vec::from(week_vec)));
    weekdays
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let boxes = Rc::new(slint::VecModel::<NewBox>::from(Vec::new()));

    let new_boxes = boxes.clone();

    let now = Utc::now();  
    

    let mut month = now.month(); // returns the month as u32 (1-12)  
    let mut year = now.year(); 
    let day = now.day();

    let current_time = NaiveDate::from_ymd_opt(year, month, day).unwrap();
  
    println!("{}", current_time.weekday());

    ui.set_weekdays(get_week_days().clone().into());

    run_calendar(&ui, new_boxes, month);

    let ui_handle = ui.as_weak();
    ui.on_next_month(move || {
        let ui = ui_handle.unwrap();
        month += 1;
        year += 1;
        let boxes = Rc::new(slint::VecModel::<NewBox>::from(Vec::new()));
        run_calendar(&ui, boxes, month);
    });
    ui.run()
}
