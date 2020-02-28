use super::V2Vector;
use crate::common::{CVSSScore, NumValue, Score};

impl CVSSScore for V2Vector {
    fn impact(&self) -> Score {
        Score::from(
            10.41
                * (1.0
                    - (1.0 - self.confidentiality_impact.num_value())
                        * (1.0 - self.integrity_impact.num_value())
                        * (1.0 - self.availability_impact.num_value())),
        )
    }

    fn expoitability(&self) -> Score {
        Score::from(
            20.0 * self.access_vector.num_value()
                * self.access_complexity.num_value()
                * self.authentication.num_value(),
        )
    }

    fn base_score(&self) -> Score {
        let impact = self.impact().value();
        let f_impact = if impact == 0.0 { 0.0 } else { 1.176 };
        let expoitability = self.expoitability().value();
        Score::from(round_to_1_decimal(
            ((0.6 * impact) + (0.4 * expoitability) - 1.5) * f_impact,
        ))
    }

    fn temporal_score(&self) -> Score {
        Score::from(0.0)
    }

    fn environmental_score(&self) -> Score {
        Score::from(0.0)
    }
}

fn round_to_1_decimal(input: f64) -> f64 {
    (input * 10.0).ceil() / 10.0
}