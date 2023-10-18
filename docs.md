# Bonds_rs Crate

This crate provides structures and methods for evaluating and dealing with various corporate bonds.

## Bond Trait

The `Bond` trait defines the behavior of a financial bond.
### Methods

- `fn coupon_payment(&self) -> f64`: Computes the total coupon payments over the bond's lifespan.
- `fn present_value(&self) -> f64`: Calculates the present value of the bond.
- `fn holding_period_return(&self) -> f64`: Computes the holding period return of the bond.
- `fn yeild_to_maturity(&self) -> f64`: Calculates the yield to maturity of the bond.

## CorporateBond Struct

The `CorporateBond` struct represents a corporate bond, implementing the `Bond` trait.

### Fields

- `coupon_rate`: The annual coupon rate of the bond.
- `discount_rate`: The annual discount rate (interest rate) used in bond valuation.
- `maturity`: The maturity period of the bond in years.
- `face_value`: The nominal value of the bond.
- `compounding_freq`: The frequency at which interest is compounded (e.g., Annual, Semiannual, etc.).
- `buying_price`: The buying price of the bond (optional).
- `current_selling_price`: The current selling price of the bond (optional).
- `effective_annual_rate`: The effective annual rate based on the compounding frequency.

### Methods

- `fn new(...)` (constructor): Creates a new `CorporateBond` instance with the specified parameters.
- `fn coupon_payment(&self) -> f64`: Computes the total coupon payments over the bond's lifespan.
- `fn present_value(&self) -> f64`: Calculates the present value of the bond.
- `fn holding_period_return(&self) -> f64`: Computes the holding period return of the bond.
- `fn yeild_to_maturity(&self) -> f64`: Calculates the yield to maturity of the bond.

## CompoundingFreq Enum

The `CompoundingFreq` enum represents the compounding frequency of interest rates for a bond.

### Variants

- `Annual`: Interest is compounded annually.
- `Semiannual`: Interest is compounded semiannually.
- `Quarterly`: Interest is compounded quarterly.
- `Monthly`: Interest is compounded monthly.

## Examples

```rust
use bonds_rs::{CorporateBond, CompoundingFreq, Bond};

fn main() {
    let bond = CorporateBond::new(
        5.0, //coupon rate (in %)
        3.0, //discount rate (in %)
        2,   //maturity (in years)
        1000.0, //face value
        CompoundingFreq::Biannual,
        Some(1000.0), //buying price
        Some(942.1843778588191), //current selling price
    );

    assert_eq!(bond.coupon_payment(), 96.35961618879344);
    assert_eq!(bond.present_value(), 1038.543846475518);
    assert_eq!(bond.yeild_to_maturity(), 3.022491931915283);
    assert_eq!(bond.holding_period_return(), 3.854399404761266);
    }
```
