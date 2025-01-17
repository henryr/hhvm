// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
use std::rc::Rc;

use oxidized::nast;

use crate::errors::HackError;
use crate::reason::Reason;
use crate::typing_ctx::TypingCtx;
use crate::typing_toplevel::TypingToplevel;

use crate::tast;

pub struct TypingCheckJob;

impl TypingCheckJob {
    pub fn type_fun<R: Reason>(
        ctx: Rc<TypingCtx<R>>,
        ast: &nast::FunDef,
    ) -> (Option<tast::Def<R>>, Vec<HackError<R>>) {
        let mut errors = Vec::new();
        let (def, typing_errors) = TypingToplevel::fun_def(ctx, ast);
        errors.extend(typing_errors.into_iter().map(|e| e.into()));
        let def = Some(oxidized::aast::Def::Fun(Box::new(def)));
        (def, errors)
    }

    pub fn type_class<R: Reason>(
        _ctx: Rc<TypingCtx<R>>,
        _ast: &nast::Class_,
    ) -> (Option<tast::Def<R>>, Vec<HackError<R>>) {
        todo!()
    }

    pub fn type_typedef<R: Reason>(
        _ctx: Rc<TypingCtx<R>>,
        _ast: &nast::Typedef,
    ) -> (Option<tast::Def<R>>, Vec<HackError<R>>) {
        todo!()
    }

    pub fn type_const<R: Reason>(
        _ctx: Rc<TypingCtx<R>>,
        _ast: &nast::Gconst,
    ) -> (Option<tast::Def<R>>, Vec<HackError<R>>) {
        todo!()
    }
}
