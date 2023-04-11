use wasm_bindgen::prelude::*;
mod data_frame;

#[wasm_bindgen]
pub fn pearson_correlation_coefficient(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    assert_eq!(n, y.len());
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();

    let square_sum_x: f64 = x.iter().map(|x| x * x).sum();
    let square_sum_y: f64 = y.iter().map(|y| y * y).sum();

    let product_sum: f64 = x.iter().zip(y).map(|(x, y)| x * y).sum();

    let numerator = (n as f64 * product_sum) - (sum_x * sum_y);

    let denominator = ((n as f64 * square_sum_x) - (sum_x * sum_x)).sqrt()
        * ((n as f64 * square_sum_y) - (sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}
/// Calculates the cosine similarity of two n-dimensional vectors.
///
/// The cosine similarity is a measure of similarity between two non-zero vectors of an inner product space.
/// It measures the cosine of the angle between two vectors and ranges between -1 and 1.
///
/// # Arguments
///
/// * `a`: The first vector.
/// * `b`: The second vector.
///
/// # Panics
///
/// This function will panic if `a` and `b` have different dimensions.
#[wasm_bindgen]
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "Vectors must have same dimensions.");

    let dot_product = dot_product(a, b);
    let magnitude_a = magnitude(a);
    let magnitude_b = magnitude(b);

    dot_product / (magnitude_a * magnitude_b)
}

fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn magnitude(a: &[f64]) -> f64 {
    a.iter().map(|x| x * x).sum::<f64>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let similarity = cosine_similarity(&a, &b);
        assert_relative_eq!(similarity, 0.9746318461970762);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert_relative_eq!(similarity, 0.0);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert_relative_eq!(similarity, 1.0);
    }

    #[test]
    #[should_panic(expected = "Vectors must have same dimensions.")]
    fn test_cosine_similarity_panic() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0];
        cosine_similarity(&a, &b);
    }


    #[test]
    fn test_pearson_correlation_coefficient() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        let corr = pearson_correlation_coefficient(&x, &y);
        assert_relative_eq!(corr, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_pearson_correlation_coefficient_panic() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0, 8.0];

        pearson_correlation_coefficient(&x, &y);
    }
}

