pub mod friday;
pub mod monday;
pub mod saturday;
pub mod sunday;
pub mod thursday;
pub mod tuesday;
pub mod utils;
pub mod wednesday;

pub use utils::{
    print_friday_schedule, print_monday_schedule, print_saturday_schedule, print_sunday_schedule,
    print_thursday_schedule, print_tuesday_schedule, print_wednesday_schedule,
    write_tomorrow_to_vault,
};
