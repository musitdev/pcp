// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use kernel::*;
use kernel::Trilean::*;
use propagators::PropagatorKind;
use propagators::cmp::x_neq_y::*;
use propagation::events::*;
use propagation::*;
use term::ops::*;
use gcollections::ops::*;
use std::fmt::{Formatter, Debug, Error};

#[derive(Clone)]
pub struct Distinct<V>
{
  props: Vec<XNeqY<V,V>>,
  vars: Vec<V>
}

impl<V> PropagatorKind for Distinct<V> {}

impl<V> Distinct<V> where
  V: Clone
{
  pub fn new(vars: Vec<V>) -> Distinct<V> {
    let mut props = vec![];
    for i in 0..vars.len()-1 {
      for j in i+1..vars.len() {
        let i_neq_j = XNeqY::new(vars[i].clone(), vars[j].clone());
        props.push(i_neq_j);
      }
    }
    Distinct {
      props: props,
      vars: vars
    }
  }
}

impl<V> Debug for Distinct<V> where
  V: Debug
{
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    try!(formatter.write_str("distinct("));
    for v in &self.vars {
      try!(formatter.write_fmt(format_args!("{:?} ", v)));
    }
    formatter.write_str(")")
  }
}

impl<Store, Domain, V> Subsumption<Store> for Distinct<V> where
  V: StoreRead<Store, Value=Domain> + Clone,
  Domain: Bounded + Disjoint
{
  fn is_subsumed(&self, store: &Store) -> Trilean {
    let mut all_entailed = true;
    for p in &self.props {
      match p.is_subsumed(store) {
        False => return False,
        Unknown => all_entailed = false,
        _ => ()
      }
    }
    if all_entailed { True }
    else { Unknown }
  }
}

impl<Store, Domain, V> Propagator<Store> for Distinct<V> where
  V: StoreRead<Store, Value=Domain> + StoreMonotonicUpdate<Store, Domain>,
  Domain: Bounded + Cardinality,
  Domain: Difference<<Domain as Bounded>::Bound, Output=Domain>
{
  fn propagate(&mut self, store: &mut Store) -> bool {
    for p in &mut self.props {
      if !p.propagate(store) {
        return false;
      }
    }
    true
  }
}

impl<V> PropagatorDependencies<FDEvent> for Distinct<V> where
  V: ViewDependencies<FDEvent> + Clone
{
  fn dependencies(&self) -> Vec<(usize, FDEvent)> {
    self.vars.iter().flat_map(|v| v.dependencies(FDEvent::Inner)).collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use kernel::*;
  use kernel::Trilean::*;
  use propagation::events::*;
  use propagation::events::FDEvent::*;
  use interval::interval::*;
  use propagators::test::*;

  #[test]
  fn distinct_test() {
    let zero = (0,0).to_interval();
    let one = (1,1).to_interval();
    let two = (2,2).to_interval();
    let dom0_1 = (0,1).to_interval();
    let dom0_2 = (0,2).to_interval();
    let dom0_3 = (0,3).to_interval();

    distinct_test_one(1, vec![zero,one,two], True, True, vec![], true);
    distinct_test_one(2, vec![zero,zero,two], False, False, vec![], false);
    distinct_test_one(3, vec![zero,one,dom0_3], Unknown, True, vec![(2, Bound)], true);
    distinct_test_one(4, vec![zero,one,dom0_2], Unknown, True, vec![(2, Assignment)], true);
    distinct_test_one(5, vec![zero,one,dom0_1], Unknown, False, vec![], false);
    distinct_test_one(6, vec![zero,dom0_3,dom0_3], Unknown, Unknown, vec![(1, Bound),(2, Bound)], true);
    distinct_test_one(7, vec![dom0_3], True, True, vec![], true);
  }

  fn distinct_test_one(test_num: u32, doms: Vec<Interval<i32>>,
    before: Trilean, after: Trilean,
    delta_expected: Vec<(usize, FDEvent)>, propagate_success: bool)
  {
    nary_propagator_test(test_num, Distinct::new, doms, before, after, delta_expected, propagate_success);
  }
}
