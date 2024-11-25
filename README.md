## Test RUST blockchain implementation 

### Blockchain struct
``` 
- genesis_block  | Block
- chain          | Vec<Block>
- difficulty     | usize
```


### Block struct
``` 
- index          | u64
- timestamp      | u64

- data           | String
- proof_of_work  | u64
- previous_hash  | String
- hash           | String
```

- maybe later i use it for chat app 