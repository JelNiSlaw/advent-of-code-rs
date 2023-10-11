pub fn bit_counts(numbers: &[u32], bit_count: usize) -> Vec<u32> {
    let mut bit_counts = vec![0u32; bit_count];

    for number in numbers {
        for i in 0..bit_count {
            if number & 1 << i != 0 {
                bit_counts[bit_count - i - 1] += 1;
            }
        }
    }

    bit_counts
}
