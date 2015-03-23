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

use solver::event::EventIndex;

#[derive(Copy, PartialEq, Eq, Debug)]
pub enum Status
{
  Entailed,
  Disentailed,
  Unknown
}

pub trait Propagator
{
  type Event: EventIndex;
  type SharedVar;

  // If the result is Entailed or Disentailed, it must not
  // change after a propagate call.
  // Also no need to check if the variables are failed, this
  // should be handled at the return of propagate.
  fn status(&self) -> Status;

  // The propagator is stable if no event are added into `events`.
  // Returns `false` if the propagator is failed.
  fn propagate(&mut self, events: &mut Vec<(usize, Self::Event)>) -> bool;

  // Each event on a variable that can change the result of
  // the `status` method should be listed here.
  fn dependencies(&self) -> Vec<(usize, Self::Event)>;

  fn deep_clone(&self, cloned_vars: &Vec<Self::SharedVar>) -> Box<Propagator<Event=Self::Event, SharedVar=Self::SharedVar>>;
}
