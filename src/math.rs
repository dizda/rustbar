pub trait Math {
    fn multiply(&self, other: &String, decimals: usize) -> String;
    fn sub(&self, other: &String, decimals: usize) -> String;
}

impl Math for String {
    fn multiply(&self, right: &String, decimals: usize) -> String {
        let left: f64 = self.parse().unwrap();
        let right: f64 = right.parse().unwrap();

        // round up to avoid arithmetic precision issue
        format!("{:.*}", decimals, (left * right))
    }

    fn sub(&self, right: &String, decimals: usize) -> String {
        let left: f64 = self.parse().unwrap();
        let right: f64 = right.parse().unwrap();

        // round up to avoid arithmetic precision issue
        format!("{:.*}", decimals, (left - right))
    }
}