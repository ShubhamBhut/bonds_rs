# bonds_rs

[![Crates.io](https://img.shields.io/crates/v/bonds_rs.svg)](https://crates.io/crates/bonds_rs)
[![Docs.rs](https://docs.rs/bonds_rs/badge.svg)](https://docs.rs/bonds_rs)

The `bonds_rs` crate provides a set of structures and methods for evaluating and dealing various financial instruments, with a focus on corporate bonds. 

## Usage

Add this crate to your `Cargo.toml` file:

```toml
[dependencies]
bonds_rs = "0.1.0"
```

Then, in your Rust code:

```rust
use bonds_rs::{CorporateBond, CompoundingFreq, Bond};

fn main() {
    let bond = CorporateBond::new(
        5.0, //coupon rate
        3.0, //dicount rate
        2, // maturity in years
        1000.0, // face value
        CompoundingFreq::Semiannual,
        Some(1000.0), //buying price
        Some(942.1843778588191), //current selling price
    );

    println!("Coupon Payment: {}", bond.coupon_payment());
    println!("Present Value: {}", bond.present_value());
    println!("Yield to Maturity: {}", bond.yeild_to_maturity());
    println!("Holding Period Return: {}", bond.holding_period_return());
}
```

For more detailed usage instructions and examples, check out the [documentation](https://docs.rs/bonds_rs).

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or a pull request.

## License

This crate is licensed under the [MIT License](LICENSE).
