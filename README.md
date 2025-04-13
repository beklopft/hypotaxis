Write
```rust
use hypotaxis::ChainLink;

let finalized_data = raw_data
    .to_vec()
    .r#if(NEW_DATA_VALID, |original_data|                  //new
        [original_data, new_raw_data.to_vec()].concat()
    )
    .mutated(|data| data                                   //new
        .sort_by(|a, b| a.1.total_cmp(&b.1))
    )
    .tap(|data| {                                          //new
        assert!( data.iter()
            .all(|(_, probability)| (0.0..=1.0).contains(probability))
        )
    })
    .into_iter()
    .filter(|(_point, probability)| *probability > 0.0)
    .collect::<Vec<_>>()
;
```
instead of
```rust
let mut full_raw_data = if NEW_DATA_VALID {
    [raw_data.to_vec(), new_raw_data.to_vec()].concat()
} else {
    raw_data.to_vec()
};

let mut sorted = full_raw_data;
sorted.sort_by(|a, b| a.1.total_cmp(&b.1));

assert!( sorted.iter()
    .all(|(_, probability)| (0.0..=1.0).contains(probability))
);

let finalized_data = sorted
    .into_iter()
    .filter(|(_point, probability)| *probability > 0.0)
    .collect::<Vec<_>>()
;
```

Can make the builder pattern more concise.