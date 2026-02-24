use crate::error::{ClientError, Result};

/// Validate quality grade parameter
pub fn validate_quality_grade(grade: &str) -> Result<()> {
    match grade {
        "casual" | "needs_id" | "research" => Ok(()),
        _ => Err(ClientError::InvalidResponse(
            format!("Invalid quality_grade: {}. Must be casual, needs_id, or research", grade)
        )),
    }
}

/// Validate rank parameter
pub fn validate_rank(rank: &str) -> Result<()> {
    let valid_ranks = [
        "kingdom", "phylum", "subphylum", "superclass", "class", "subclass",
        "superorder", "order", "suborder", "infraorder", "superfamily",
        "epifamily", "family", "subfamily", "supertribe", "tribe", "subtribe",
        "genus", "genushybrid", "species", "hybrid", "subspecies", "variety", "form"
    ];

    if valid_ranks.contains(&rank) {
        Ok(())
    } else {
        Err(ClientError::InvalidResponse(
            format!("Invalid rank: {}. See API docs for valid ranks", rank)
        ))
    }
}

/// Validate interval parameter for histograms
pub fn validate_interval(interval: &str) -> Result<()> {
    match interval {
        "year" | "month" | "week" | "day" | "hour" | "month_of_year" => Ok(()),
        _ => Err(ClientError::InvalidResponse(
            format!("Invalid interval: {}. Must be year, month, week, day, hour, or month_of_year", interval)
        )),
    }
}

/// Validate date format (YYYY-MM-DD)
pub fn validate_date_format(date: &str) -> Result<()> {
    if date.len() == 10 && date.chars().nth(4) == Some('-') && date.chars().nth(7) == Some('-') {
        Ok(())
    } else {
        Err(ClientError::Parse(
            format!("Invalid date format: {}. Expected YYYY-MM-DD", date)
        ))
    }
}

/// Validate per_page is within limits
pub fn validate_per_page(per_page: u32, max: u32) -> Result<()> {
    if per_page > max {
        Err(ClientError::InvalidResponse(
            format!("per_page ({}) exceeds maximum ({})", per_page, max)
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_grade_validation() {
        assert!(validate_quality_grade("research").is_ok());
        assert!(validate_quality_grade("needs_id").is_ok());
        assert!(validate_quality_grade("casual").is_ok());
        assert!(validate_quality_grade("invalid").is_err());
    }

    #[test]
    fn test_rank_validation() {
        assert!(validate_rank("species").is_ok());
        assert!(validate_rank("genus").is_ok());
        assert!(validate_rank("invalid_rank").is_err());
    }

    #[test]
    fn test_interval_validation() {
        assert!(validate_interval("month_of_year").is_ok());
        assert!(validate_interval("year").is_ok());
        assert!(validate_interval("invalid").is_err());
    }

    #[test]
    fn test_date_format_validation() {
        assert!(validate_date_format("2024-01-15").is_ok());
        assert!(validate_date_format("2024/01/15").is_err());
        assert!(validate_date_format("01-15-2024").is_err());
    }
}