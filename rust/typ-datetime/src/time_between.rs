use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveTime, Utc};
use derive_new::new;

use serde::{Deserialize, Serialize};

use crate::DayType;

#[derive(new, Serialize, Deserialize, Debug, Clone)]
pub struct Effective {
    // pub target_day_type: Option<DayType>,
    pub timezone: String,
    pub start_datetime: NaiveTime,
    pub end_datetime: NaiveTime,
}

impl Effective {
    pub fn is_active_datetime(&self, utc_datetime: DateTime<Utc>) -> bool {
        //
        // let dt3: DateTime<FixedOffset> = dt1.with_timezone(&FixedOffset::east(3*3600));
        let mut is_active = false;

        let mut forward = true;
        if self.start_datetime > self.end_datetime {
            forward = false;
        }
        let offset = FixedOffset::from_str(&self.timezone).unwrap();
        let local_datetime = utc_datetime.with_timezone(&offset);

        if forward == true {
            if self.start_datetime <= local_datetime.time()
                && local_datetime.time() < self.end_datetime
            {
                is_active = true
            }
        } else {
            // 20:00:00-03:00:00
            // 20:00:00-00:00:00 || 00:00:00-03:00:00
            // 23:00:00 true
            // 05:00:00 false

            if self.start_datetime <= local_datetime.time()
                && local_datetime.time()
                    <= NaiveTime::from_hms_nano_opt(23, 59, 59, 2_000_000_000 - 1).unwrap()
            {
                is_active = true
            } else if NaiveTime::from_hms_opt(0, 0, 0).unwrap() <= local_datetime.time()
                && local_datetime.time() < self.end_datetime
            {
                is_active = true
            }
        }

        return is_active;
    }

    pub fn is_active_time(&self, utc_datetime: DateTime<Utc>) -> bool {
        //
        let mut is_run = false;

        let mut forward = true;
        if self.start_datetime > self.end_datetime {
            forward = false;
        }

        if forward == true {
            if self.start_datetime <= utc_datetime.time() && utc_datetime.time() < self.end_datetime
            {
                is_run = true
            }
        } else {
            // 20:00:00-03:00:00
            // 20:00:00-00:00:00 || 00:00:00-03:00:00
            // 23:00:00 true
            // 05:00:00 false
            if self.start_datetime <= utc_datetime.time()
                && utc_datetime.time()
                    <= NaiveTime::from_hms_nano_opt(23, 59, 59, 2_000_000_000 - 1).unwrap()
            {
                is_run = true
            } else if NaiveTime::from_hms_opt(0, 0, 0).unwrap() <= utc_datetime.time()
                && utc_datetime.time() < self.end_datetime
            {
                is_run = true
            }
        }

        return is_run;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test --package typ-datetime --lib -- time_between::tests::it_works --exact --nocapture
    #[test]
    fn it_works() {
        let a = Effective::new(
            String::from("+0900"),
            NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(1, 0, 0).unwrap(),
        );
        let utc_datetime: DateTime<Utc> = Utc::now();
        println!("{}", utc_datetime);
        let aa = a.is_active_datetime(utc_datetime);

        println!("{}", aa);
    }
}
