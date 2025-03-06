## Rust Bloom filter

A minimal implementation of a Bloom filter in Rust, designed to help understand its core concepts while getting familiar with Rust. 
This implementation includes utilities to compute optimal parameters (`BitVec` size and number of hash functions) for efficient performance.
Rationale is, if item appeared all the indexes in bitmap should be set to true, since hashing is deterministic. 
Please note it does not work for substrings, since they produce entirely different hash. It only provides an answer to "Did I insert exact same item?",
so for strings the result also depends on how the items were inserted. For example if "TOFU" was inserted and "TOF" is checked, 
it will likely say that it's not in the set.
## Features
- Add elements (`insert_item`) and test if an item is likely present (`might_contain`)
-	Supports user-defined false positive rates and expected element counts
- Uses multiple hash functions for better accuracy
- Compute the ideal bit array size and number of hash functions

## How it works - explanation
Imagine this: You are going shopping and when you already left the house you wonder if you should be milk. And then you get message from your roommate 
that there definitely no milk in the fridge, so you know you should buy it. Orrrr he says he don't know exactly and it might be there, so you need to go back
and check by yourself. That is more or less what Bloom filter does.
Generally speaking, Bloom filter is a probabilistic data structure that supports fast membership testing with a trade-off - it may yield false positives but never false negatives.

## Key formulas
The optimal bit array size (m) and hash function count (k) are calculated as:
- Bit array size (m): m = -n * (ln p) \ (ln 2)^2}
- Number of hash functions (k): k = (m / n) * ln 2
Where:
- n = Expected number of elements
- p = Desired false positive probability

## License
This project is licensed under the MIT License. See `LICENSE` for details.
