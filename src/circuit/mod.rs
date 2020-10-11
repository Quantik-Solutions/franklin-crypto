// #[cfg(test)]
pub mod test;

pub mod as_waksman;
pub mod baby_ecc;
pub mod baby_eddsa;
pub mod baby_pedersen_hash;
pub mod blake2s;
pub mod boolean;
pub mod ecc;
pub mod float_point;
pub mod lookup;
pub mod multieq;
pub mod multipack;
pub mod num;
pub mod pedersen_hash;
pub mod polynomial_lookup;
pub mod sha256;
pub mod uint32;
// pub mod linear_combination;
pub mod expression;
// pub mod shark_mimc;
pub mod rescue;

pub mod sapling;
pub mod sprout;

use bellman::SynthesisError;

// TODO: This should probably be removed and we
// should use existing helper methods on `Option`
// for mapping with an error.
/// This basically is just an extension to `Option`
/// which allows for a convenient mapping to an
/// error on `None`.
pub trait Assignment<T> {
  fn get(&self) -> Result<&T, SynthesisError>;
  fn grab(self) -> Result<T, SynthesisError>;
}

impl<T: Clone> Assignment<T> for Option<T> {
  fn get(&self) -> Result<&T, SynthesisError> {
    match *self {
      Some(ref v) => Ok(v),
      None => Err(SynthesisError::AssignmentMissing),
    }
  }

  fn grab(self) -> Result<T, SynthesisError> {
    match self {
      Some(v) => Ok(v),
      None => Err(SynthesisError::AssignmentMissing),
    }
  }
}
