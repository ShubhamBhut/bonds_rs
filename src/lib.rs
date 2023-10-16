pub trait Bond {
    fn valuation(&self) ->f32;
    fn holding_period_return(&self) ->f32;
    fn yeild_to_maturity(&self) ->f32;
}

pub struct CorporateBond {
    coupon_rate: f32,
    dicount_rate: f32,
    maturity: u32,
    face_value: f64,
    CompoundingFreq: CompoundingFreq,
}

enum CompoundingFreq {
    Annual,
    Semiannual,
    Quarterly,
    Monthly
}

todo!("Implement all 3 methods for Corporate and ZeroCoupon Bonds");
impl Bond for CorporateBond {
    fn valuation(&self) ->f32 {
        0.0
    }

    fn holding_period_return(&self) ->f32 {
        0.0
    }

    fn yeild_to_maturity(&self) ->f32 {
        0.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
