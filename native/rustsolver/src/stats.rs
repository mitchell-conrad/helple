pub fn std_dev(histogram: &[usize]) -> Option<f64> {
    let mean = mean(histogram);
    let mut sum = 0.0;
    let mut count = 0;

    for (i, n) in histogram.iter().enumerate() {
        sum += (i as f64 - mean?).powf(2f64) * *n as f64;
        count += n;
    }

    if count == 0 {
        return None;
    }

    Some((sum as f64 / count as f64).sqrt())
}

pub fn count(histogram: &[usize]) -> usize {
    histogram.iter().sum()
}

pub fn mean(histogram: &[usize]) -> Option<f64> {
    let mut sum = 0;
    let mut count = 0;
    for (i, n) in histogram.iter().enumerate() {
        sum += i * n;
        count += n;
    }
    if count == 0 {
        return None;
    }
    Some(sum as f64 / count as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_dev() {
        assert_eq!(Some(2.0), std_dev(&vec![0, 0, 1, 0, 3, 2, 0, 1, 0, 1]));
        assert_eq!(None, std_dev(&vec![0, 0]))
    }

    #[test]
    fn test_count() {
        assert_eq!(8, count(&vec![0, 0, 1, 0, 3, 2, 0, 1, 0, 1]));
        assert_eq!(20, count(&vec![8, 12]));
        assert_eq!(0, count(&vec![0, 0]));
        assert_eq!(0, count(&vec![]));
    }

    #[test]
    fn test_mean() {
        assert_eq!(Some(1f64), mean(&vec![0, 10, 0]));
        assert_eq!(Some(1.5f64), mean(&vec![0, 10, 10]));
        assert_eq!(Some(1f64), mean(&vec![1, 1, 1]));
        assert_eq!(Some(0f64), mean(&vec![1]));
        assert_eq!(None, mean(&vec![0]));
    }
}
