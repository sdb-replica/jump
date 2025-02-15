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
        
        // Calculate the next bucket to examine using a multiplicative jump
        // Formula: (current_bucket + 1) * jump_distance
        // 
        // Why this works:
        // 1. current_bucket + 1 ensures forward progression through buckets
        // 2. Multiplying by jump_distance (which gets smaller) creates:
        //    - Large initial jumps (for quick bucket space traversal)
        //    - Progressively smaller jumps (for precise final placement)
        //    - Example sequence: 0 → 800 → 900 → 950 → 975 → ...
        // 3. Float conversion (as f64) enables precise fractional jumps
        // 4. Final integer conversion (as i64) gives the next bucket number
        next_jump = ((current_bucket + 1) as f64 * jump_distance) as i64;
    }

    current_bucket as i32
}

