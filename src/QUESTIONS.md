## 2023
### day 02
- what the hell is going on with this code:
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
