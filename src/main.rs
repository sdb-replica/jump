mod algo;
use algo::jump_hash;

fn main() {
    // Example usage
    let bucket = jump_hash(123456789, 10);
    println!("Bucket for key 123456789: {}", bucket);
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

    #[test]
    fn test_bucket_expansion() {
        let key = 123456789;
        let bucket_10 = jump_hash(key, 10);
        let bucket_20 = jump_hash(key, 20);
        
        // When increasing buckets, keys should either:
        // 1. Stay in the same bucket
        // 2. Move to a new bucket with a higher number
        assert!(bucket_20 >= bucket_10);
        
        // Verify the new bucket is within valid range
        assert!(bucket_20 < 20);
    }

    #[test]
    fn test_minimal_redistribution() {
        let num_keys = 1_000_000_000;  // 1 billion keys
        let initial_buckets = 1000;
        let new_buckets = 1500;
        let mut moved_keys = 0;

        // Test a sample of 1 billion keys
        for key in 0..num_keys {
            let initial_bucket = jump_hash(key, initial_buckets);
            let new_bucket = jump_hash(key, new_buckets);
            
            if new_bucket != initial_bucket {
                moved_keys += 1;
            }
        }

        // Calculate percentage of keys that moved
        let move_percentage = (moved_keys as f64 / num_keys as f64) * 100.0;
        
        // The theoretical expected movement is around 33% (500/1500)
        // We'll allow some variance but ensure it's close to theoretical
        assert!(move_percentage > 30.0 && move_percentage < 35.0,
                "Expected ~33% movement, got {}%", move_percentage);
        
        println!("Moved {}% of keys when increasing buckets from {} to {}", 
                move_percentage, initial_buckets, new_buckets);
    }
}
