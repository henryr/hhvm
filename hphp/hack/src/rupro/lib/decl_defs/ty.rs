// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use hcons::Hc;

use crate::pos::{PosId, Symbol};
use crate::reason::Reason;

pub type Prim = oxidized::aast::Tprim;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunParam<R: Reason, TY> {
    pub pos: R::Pos,
    pub name: Option<Symbol>,
    pub ty: TY,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunType<R: Reason, TY> {
    pub params: Vec<FunParam<R, TY>>,
    pub ret: TY,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeclTy_<R: Reason, TY> {
    /// A primitive type.
    DTprim(Prim),
    /// Either an object type or a type alias, ty list are the arguments.
    DTapply(PosId<R::Pos>, Vec<TY>),
    /// A wrapper around `FunType`, which contains the full type information
    /// for a function, method, lambda, etc.
    DTfun(FunType<R, TY>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclTy<R: Reason>(R, Hc<DeclTy_<R, DeclTy<R>>>);

impl<R: Reason> DeclTy<R> {
    pub fn new(reason: R, ty: Hc<DeclTy_<R, DeclTy<R>>>) -> Self {
        Self(reason, ty)
    }

    pub fn pos(&self) -> &R::Pos {
        self.0.pos()
    }

    pub fn reason(&self) -> &R {
        &self.0
    }

    pub fn node(&self) -> &Hc<DeclTy_<R, DeclTy<R>>> {
        &self.1
    }

    pub fn unwrap_class_type(&self) -> Option<(&R, &PosId<R::Pos>, &[DeclTy<R>])> {
        use DeclTy_::*;
        let r = self.reason();
        match &**self.node() {
            DTapply(pos_id, tyl) => Some((r, pos_id, tyl)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CeVisibility<R: Reason> {
    Public,
    Private(Symbol),
    Protected(Symbol),
    Internal(PosId<R::Pos>),
}
