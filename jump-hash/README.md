# Jump Hash Algorithm

Jump Hash (or Jump Consistent Hash) is a fast, minimal memory, consistent hashing algorithm developed by Google. It was introduced in 2014 to solve the problem of distributing keys across a changing number of buckets.

## What is Jump Hash?

Jump Hash maps keys to buckets in a way that:
- Is uniformly distributed
- Is consistent (minimal remapping when buckets are added/removed)
- Uses minimal memory (no tables/arrays needed)
- Is extremely fast

## How it Works

The algorithm works by "jumping" forward through potential bucket numbers based on a pseudo-random number generated from the input key. The process continues until we run out of buckets to jump to, landing on our final bucket.

Basic principle:
1. Start with bucket 0
2. Generate a random number based on current state
3. Jump forward to a new bucket based on that number
4. If we would jump past our last bucket, we're done
5. Otherwise, repeat from step 2

## Key Features

- **Consistent**: When buckets are added/removed, most keys don't change location
- **Minimal Movement**: When going from n to n+1 buckets, only k/n keys need to move (where k is total keys)
- **Memory Efficient**: Uses only a few variables, regardless of bucket count
- **Fast**: O(ln n) time complexity, where n is the number of buckets
- **Deterministic**: Same key always maps to same bucket for given bucket count

## Use Cases

- Load balancing
- Distributed caching
- Data partitioning
- Sharding
- Service discovery

## References

- [A Fast, Minimal Memory, Consistent Hash Algorithm](https://arxiv.org/abs/1406.2294) - Original paper by John Lamping and Eric Veach
- [Google Research Blog Post](https://ai.googleblog.com/2017/04/consistent-hashing-with-bounded-loads.html)

## License

This implementation is available under the MIT License.
