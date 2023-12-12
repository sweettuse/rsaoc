## 2023
### day 10
- what are the differences between `()`, `[]`, `{}`?
    - especially in like `vec![]` vs `hashset!{}`
### day 09
- in the below code, is there a way to write this using an iterator that somehow 
breaks when there's nothing more to do?
```rust
fn extrapolate<F>(nums: &[i32], extrap_fn: F) -> i32 
where
    F : Fn(Vec<Vec<i32>>) -> i32
{
    let mut all_diffs: Vec<Vec<i32>> = vec![];
    all_diffs.push(Vec::from(nums));
    while let Some(diffs) = _get_diffs(all_diffs.last().unwrap()) {
        all_diffs.push(diffs);
    }
    extrap_fn(all_diffs)
}
```

### day 08
- in `FromStr`, what's a good default for `type Err = ...`
like i cannot figure out what the hell this should be.
in python i would jam `Exception` in there and call it a day.
what's the equivalent in rust?

### day 07
- in sorting i need to keep calculating, e.g., hand_types/card_ranks when sorting
    - is there a rustic way to cache these values on the struct?
    - ~and maybe make the struct immutable?~ no there is not, basically module-level open,
      but keep private to that scope

### day 02
- SOLUTION (thanks collin) - copy the get:
    ```rust
        let w = WTH { u: map.get("jeb").copied().unwrap_or_default() };
    ```
- ~what the hell is going on with this code:~
    ```rust
    struct WTH {
        u: u32,
    }

    fn jesus() {
        let mut map: HashMap<&str, u32> = HashMap::from([
            ("jeb", 42)
        ]);
        let w = WTH { u: *map.get("jeb").unwrap_or_default() };  // ERROR
        let w = WTH { u: *map.get("jeb").unwrap_or(&0) };  // good/what the hell?
    }

    // error on unwrap_or_default:
    // the trait bound `&u32: std::default::Default` is not satisfied
    // the trait `std::default::Default` is implemented for `u32`
    
    ```

- what's going on in `bounding_reveal`
    - couldn't get reduce/fold to work even though it seemed like the right approach...
    ```rust
    fn bounding_reveal(&self) -> Reveal {
        let mut res = Reveal::default();
        for g in self.reveals.iter() {
            res = res.update_max_of(g);
        }
        res

        // TODO why not this?
        // error:
        //  cannot borrow `*acc` as mutable, as it is behind a `&` reference
        //  `acc` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        // let res = Reveal { red: 0, green: 0, blue: 0 };
        // self.reveals
        // .iter_mut()
        // .fold(&res, |acc, cur| acc.update_max_of(cur));  // ERROR
        // return res;

        // TODO why not this?
        // error:
        //  mismatched types
        //  expected `&Reveal`, found `Reveal`
        // self.reveals
        // .iter()
        // .reduce(|acc, cur| acc.max_of(cur))  // ERROR
        // .unwrap()
        // .clone()
     }
     ```

## 2020
### day 04_alt
- how does `AsRef` work? why do i need it?