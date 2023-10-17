pub trait Bond {
    fn present_value(&self) -> f64;
    fn holding_period_return(&self) -> f64;
    fn yeild_to_maturity(&self) -> f64;
}

pub struct CorporateBond {
    coupon_rate: f64,
    discount_rate: f64,
    maturity: u32,
    face_value: f64,
    compounding_freq: CompoundingFreq,
    buying_price: Option<f64>,
    current_selling_price: Option<f64>,
    effective_annual_rate: f64,
}

pub enum CompoundingFreq {
    Annual,
    Semiannual,
    Quarterly,
    Monthly,
}

impl CorporateBond {
    pub fn new(
        coupon_rate: f64,
        discount_rate: f64,
        maturity: u32,
        face_value: f64,
        compounding_freq: CompoundingFreq,
        buying_price: Option<f64>,
        current_selling_price: Option<f64>,
    ) -> Self {
        let effective_annual_rate = match compounding_freq {
            CompoundingFreq::Annual => 0.01 * discount_rate,
            CompoundingFreq::Semiannual => (1.0 + 0.01 * discount_rate / 2.0).powi(2) - 1.0,
            CompoundingFreq::Quarterly => (1.0 + 0.01 * discount_rate / 4.0).powi(4) - 1.0,
            CompoundingFreq::Monthly => (1.0 + 0.01 * discount_rate / 12.0).powi(12) - 1.0,
        };
        CorporateBond {
            coupon_rate,
            discount_rate,
            maturity,
            face_value,
            compounding_freq,
            buying_price,
            current_selling_price,
            effective_annual_rate,
        }
    }
}

impl Bond for CorporateBond {
    fn present_value(&self) -> f64 {
        self.face_value
            / f64::powf(
                (1.0 + (self.effective_annual_rate)).into(),
                self.maturity.into(),
            )
    }

    fn holding_period_return(&self) -> f64 {
        // 100.0
        //     * (f64::powf(
        //         self.current_selling_price.unwrap() / self.buying_price.unwrap(),
        //         1.0 / f64::from(self.maturity),
        //     ) - 1.0)
            100.0
            * (f64::powf(
                self.current_selling_price.unwrap() / self.buying_price.unwrap(),
                1.0,
            ) - 1.0)

    }

    fn yeild_to_maturity(&self) -> f64 {
        100.0
            * (f64::powf(
                self.face_value / self.current_selling_price.unwrap(),
                1.0 / f64::from(self.maturity),
            ) - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_present_value_annual() {
        let bond = CorporateBond::new(5.0, 3.0, 2, 1000.0, CompoundingFreq::Annual, None, None);
        assert_eq!(
            bond.present_value(),
            942.5959614985824,
            "Present value calculation is incorrect"
        );
    }

    #[test]
    fn test_net_present_value_semiannual() {
        let bond = CorporateBond::new(5.0, 3.0, 2, 1000.0, CompoundingFreq::Semiannual, None, None);
        assert_eq!(
            bond.present_value(),
            942.1843778588191,
            "Present value calculation is incorrect"
        );
    }

    #[test]
    fn test_yield_to_maturity() {
        let bond = CorporateBond::new(
            5.0,
            3.0,
            2,
            1000.0,
            CompoundingFreq::Semiannual,
            Some(1000.0),
            Some(942.1843778588191),
        );
        assert_eq!(
            bond.yeild_to_maturity(),
            3.022491931915283,
            "Incorrect yield to maturity"
        );
    }

    #[test]
    fn test_holding_period_return() {
        let bond = CorporateBond::new(
            5.0,
            3.0,
            2,
            1000.0,
            CompoundingFreq::Semiannual,
            Some(1000.0),
            Some(1100.0),
        );
        assert_eq!(
            bond.holding_period_return(),
            10.0,
            "Incorrect Holding period return"
        );
    }
}
