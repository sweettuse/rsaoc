## 2023
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