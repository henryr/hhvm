// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
mod folded;
mod shallow;
mod ty;

pub use folded::{FoldedClass, FoldedElement, SubstContext};
pub use shallow::{ShallowClass, ShallowFun, ShallowMethod};
pub use ty::{CeVisibility, DeclTy, DeclTy_, FunParam, FunType, Prim};
