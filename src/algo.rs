/// Implements the Jump Consistent Hash algorithm.
/// 
/// # Arguments
/// 
/// * `key` - The key to hash
/// * `num_buckets` - The number of buckets to distribute across (must be positive)
/// 
/// # Returns
/// 
/// Returns a bucket number in the range [0, num_buckets)
/// 
/// # Examples
/// 
/// ```
/// use jump_hash::jump_hash;
/// 
/// let bucket = jump_hash(123456789, 10);
/// assert!(bucket >= 0 && bucket < 10);
/// ```
pub fn jump_hash(mut key: u64, num_buckets: i32) -> i32 {
    // Handle edge cases
    if num_buckets <= 0 {
        return 0;
    }

    let mut current_bucket: i64 = -1;
    let mut next_jump: i64 = 0;

    while next_jump < num_buckets as i64 {
        current_bucket = next_jump;
        
        // Transform the key using a Linear Congruential Generator (LCG)
        // Formula: next = (a * current + c) mod m
        // where:
        //   a = 2862933555777941757 (carefully chosen prime multiplier)
        //   c = 1 (increment)
        //   m = 2^64 (implicit modulo from u64)
        // Uses wrapping operations to handle overflow correctly
        key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
        
        // Extract the higher-order bits by right-shifting 33 positions
        // This gives us a more uniform distribution for the jump distance
        // by taking the most significant bits after the LCG transformation
        let random_bits = key >> 33;
        
        // Calculate jump distance using the extracted random bits
        // The denominator is incremented by 1 to avoid division by zero
        let jump_distance = (1i64 << 31) as f64 / (random_bits + 1) as f64;
        next_jump = ((current_bucket + 1) as f64 * jump_distance) as i64;
    }

    current_bucket as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_distribution() {
        let bucket = jump_hash(123456789, 10);
        assert!(bucket >= 0 && bucket < 10);
    }

    #[test]
    fn test_consistency() {
        let key = 987654321;
        let bucket1 = jump_hash(key, 10);
        let bucket2 = jump_hash(key, 10);
        assert_eq!(bucket1, bucket2);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(jump_hash(123, 0), 0);
        assert_eq!(jump_hash(123, 1), 0);
        assert_eq!(jump_hash(0, 5), jump_hash(0, 5));
    }
}
