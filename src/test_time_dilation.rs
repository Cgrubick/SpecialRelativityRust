


mod tests {
   
    use crate::time_dilation::time_dilation_calc;
    #[cfg(test)]
    #[test]
    fn test_time_dilation() {
        let distance = 4.37;
        let speed = 0.5;
        let expected_proper_time = 8.00;
        let expected_dilated_time = 10.00;

        let (proper_time_year, dilated_time_year) = time_dilation_calc(distance, speed);

        assert_eq!(proper_time_year, expected_proper_time);
        assert_eq!(dilated_time_year, expected_dilated_time);
    }
}

    
