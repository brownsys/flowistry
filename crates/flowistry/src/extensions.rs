//! Extra features for evaluating / ablating the precision of Flowistry's algorithm.
use std::{cell::RefCell, str::FromStr};

use fluid_let::fluid_let;
pub use fluid_let::fluid_set;
use rustc_middle::{mir::TerminatorKind, ty::TyCtxt};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Hash)]
pub enum MutabilityMode {
  DistinguishMut,
  IgnoreMut,
}

impl FromStr for MutabilityMode {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DistinguishMut" => Ok(Self::DistinguishMut),
      "IgnoreMut" => Ok(Self::IgnoreMut),
      _ => Err(format!("Could not parse: {s}")),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Hash)]
pub enum ContextMode {
  SigOnly,
  Recurse,
}

impl FromStr for ContextMode {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "SigOnly" => Ok(Self::SigOnly),
      "Recurse" => Ok(Self::Recurse),
      _ => Err(format!("Could not parse: {s}")),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Hash)]
pub enum PointerMode {
  Precise,
  Conservative,
}

impl FromStr for PointerMode {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Precise" => Ok(Self::Precise),
      "Conservative" => Ok(Self::Conservative),
      _ => Err(format!("Could not parse: {s}")),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Hash)]
pub struct EvalMode {
  pub mutability_mode: MutabilityMode,
  pub context_mode: ContextMode,
  pub pointer_mode: PointerMode,
}

impl Default for EvalMode {
  fn default() -> Self {
    EvalMode {
      mutability_mode: MutabilityMode::DistinguishMut,
      context_mode: ContextMode::SigOnly,
      pointer_mode: PointerMode::Precise,
    }
  }
}

pub trait RecurseSelector {
  fn is_selected<'tcx>(&self, tcx: TyCtxt<'tcx>, tk: &TerminatorKind<'tcx>) -> bool;
}

fluid_let!(pub static EVAL_MODE: EvalMode);
fluid_let!(pub static REACHED_LIBRARY: RefCell<bool>);
fluid_let!(pub static RECURSE_SELECTOR: Box<dyn RecurseSelector>);

pub fn is_extension_active(f: impl Fn(EvalMode) -> bool) -> bool {
  EVAL_MODE.copied().map(f).unwrap_or(false)
}
