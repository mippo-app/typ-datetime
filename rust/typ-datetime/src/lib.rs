pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use derive_new::new;

use serde::{Deserialize, Serialize};

pub mod time_between;

#[derive(new, Serialize, Deserialize, Debug, Clone)]
pub enum DayType {
    All,
    WorkingDay,
    Holiday,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
