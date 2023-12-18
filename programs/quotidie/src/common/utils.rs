use crate::common::{DAY};

pub fn get_utc_next_day(
    utc_time: i64,
) -> i64 {
    let time_since_mignight = utc_time % DAY;
    let time_till_next_day = DAY - time_since_mignight;
    let next_day = utc_time + time_till_next_day;

    return next_day;
}

pub fn get_next_day_with_offset(
    utc_time: i64,
    utc_next_day: i64,
    offset: i8,
) -> i64 {
    let time_till_next_day_utc = utc_next_day - utc_time;
    let mut time_till_next_day_with_offset = time_till_next_day_utc + i64::from(offset);

    if (time_till_next_day_with_offset > DAY) {
        time_till_next_day_with_offset -= DAY;
    }

    if (time_till_next_day_with_offset < 0) {
        time_till_next_day_with_offset += DAY;
    }

    let next_day_with_offset = utc_time + time_till_next_day_with_offset;
    
    next_day_with_offset
}