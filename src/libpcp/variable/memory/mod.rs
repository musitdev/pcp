// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod copy;

pub use variable::memory::copy::*;

use variable::ops::*;
use gcollections::ops::*;
use gcollections::ops::sequence::ordering::*;
use std::ops::Index;
use std::fmt::Display;

pub trait DomainConcept :
  Clone + Display + Bounded + Cardinality + Subset
{}

impl<R> DomainConcept for R where
  R: Clone + Display + Bounded + Cardinality + Subset
{}

pub trait MemoryConcept<Domain> :
   Cardinality<Size=usize>
 + Iterable<Item=Domain>
 + Empty
 + Push<Back, Domain>
 + Update<usize, Domain>
 + Index<usize, Output=Domain>
 + Freeze
 + Display
 + Clone // TO DELETE
where
 Domain: DomainConcept
{}

impl<Domain, R> MemoryConcept<Domain> for R where
 R: Cardinality<Size=usize>
 + Iterable<Item=Domain>
 + Empty
 + Push<Back, Domain>
 + Update<usize, Domain>
 + Index<usize, Output=Domain>
 + Freeze
 + Display
 + Clone, // TO DELETE
 Domain: DomainConcept
{}
