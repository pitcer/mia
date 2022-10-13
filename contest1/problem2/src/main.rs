type Result<T, E = Error> = std::result::Result<T, E>;
type Error = Box<dyn std::error::Error + 'static>;

fn main() -> Result<()> {
    Ok(())
}

fn minimum_total_length() -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        // input:
        // 4, 100, 2
        // 20, 30, 75, 80
        let actual = minimum_total_length()?;
        assert_eq!(17, actual);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        // input:
        // 5, 100, 3
        // 1, 2, 4, 60, 87
        let actual = minimum_total_length()?;
        assert_eq!(6, actual);
        Ok(())
    }
}
