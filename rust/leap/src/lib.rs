// Leap year is defined by
// The year can be evenly divided by 4;
// If the year can be evenly divided by 100, it is NOT a leap year, unless;
// The year is also evenly divisible by 400. Then it is a leap year.

pub fn is_leap_year(year: i32) -> bool {
     year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
