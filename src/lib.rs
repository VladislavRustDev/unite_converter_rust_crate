//! Unit Converter Library
//!
//! A simple library to convert between various units of measurement.
//! It currently supports conversions between:
//! - Kilometers and meters
//! - Kilometers and miles
//! - Hours and minutes
//! - Minutes and seconds
//!
//! ## Example Usage
//!
//! ```
//! use unit_converter::*;
//! 
//! // Convert 1 kilometer to meters
//! let meters = km2m(1.0);
//! assert_eq!(meters, 1000.0);
//! ```

/// Converts kilometers to meters
/// 
/// # Arguments
/// * `km` - Distance in kilometers
/// 
/// # Returns
/// Distance in meters
/// 
/// # Example
/// ```
/// use unit_converter::*;
/// 
/// assert_eq!(km2m(1.0), 1000.0);
/// ```
pub fn km2m(km: f64) -> f64 {
    km * 1000.0
}

/// Converts kilometers to miles
/// 
/// # Arguments
/// * `km` - Distance in kilometers
/// 
/// # Returns
/// Distance in miles
/// 
/// # Example
/// ```
/// use unit_converter::*;
/// 
/// assert!((km2mi(1.0) - 0.621371).abs() < 1e-6);
/// ```
pub fn km2mi(km: f64) -> f64 {
    km * 0.62137119224
}

/// Converts hours to minutes
/// 
/// # Arguments
/// * `hour` - Time in hours
/// 
/// # Returns
/// Time in minutes
/// 
/// # Example
/// ```
/// use unit_converter::*;
/// 
/// assert_eq!(hour2min(1.0), 60.0);
/// ```
pub fn hour2min(hour: f64) -> f64 {
    hour * 60.0
}

/// Converts minutes to seconds
/// 
/// # Arguments
/// * `min` - Time in minutes
/// 
/// # Returns
/// Time in seconds
/// 
/// # Example
/// ```
/// use unit_converter::*;
/// 
/// assert_eq!(min2sec(1.0), 60.0);
/// ```
pub fn min2sec(min: f64) -> f64 {
    min * 60.0
}

/// Converts meters to kilometers
/// 
/// # Arguments
/// * `meter` - Distance in meters
/// 
/// # Returns
/// Distance in kilometers
/// 
/// # Example
/// ```
/// use unit_converter::*;
/// 
/// assert_eq!(meter2km(1000.0), 1.0);
/// ```
pub fn meter2km(meter: f64) -> f64 {
    meter / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_to_kilometers() {
        assert_eq!(meter2km(1000.0), 1.0);
    }

    #[test]
    fn test_km2mi() {
        assert!((km2mi(1.0) - 0.621371).abs() < 1e-6);
    }

    #[test]
    fn test_hour2min() {
        assert_eq!(hour2min(1.0), 60.0);
    }

    #[test]
    fn test_min2sec() {
        assert_eq!(min2sec(1.0), 60.0);
    }
}
