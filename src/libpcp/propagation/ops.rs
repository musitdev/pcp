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

use kernel::trilean::*;
use kernel::consistency::*;
use propagation::concept::*;

pub trait Subsumption<Store>
{
  fn is_subsumed(&self, store: &Store) -> Trilean;
}

pub trait Propagator<VStore>
{
  /// Returns `false` if it failed to propagate (a variable has an empty domain after propagation).
  fn propagate(&mut self, store: &mut VStore) -> bool;
}

pub trait PropagatorDependencies<Event>
{
  /// Each event on a variable that can change the result of the `is_subsumed` method should be listed here.
  fn dependencies(&self) -> Vec<(usize, Event)>;
}

pub trait BoxedClone<VStore, Event>
{
  fn boxed_clone(&self) -> Box<PropagatorConcept<VStore, Event>>;
}

impl<VStore, Event, R> BoxedClone<VStore, Event> for R where
  R: Clone,
  R: Consistency<VStore>,
  R: PropagatorDependencies<Event>,
  R: 'static
{
  fn boxed_clone(&self) -> Box<PropagatorConcept<VStore, Event>> {
    Box::new(self.clone())
  }
}
