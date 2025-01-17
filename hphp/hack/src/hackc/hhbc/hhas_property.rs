// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use bitflags::bitflags;
use ffi::{Maybe, Slice, Str};
use hhas_attribute::HhasAttribute;
use hhas_type::HhasTypeInfo;
use hhbc_ast::Visibility;
use instruction_sequence::InstrSeq;

bitflags! {
    #[repr(C)]
    pub struct HhasPropertyFlags: u16 {
        const IS_ABSTRACT = 1 << 0;
        const IS_STATIC = 1 << 1;
        const IS_DEEP_INIT = 1 << 2;
        const IS_CONST = 1 << 3;
        const IS_LSB = 1 << 4;
        const IS_NO_BAD_REDECLARE = 1 << 5;
        const HAS_SYSTEM_INITIAL = 1 << 6;
        const NO_IMPLICIT_NULL = 1 << 7;
        const INITIAL_SATISFIES_TC = 1 << 8;
        const IS_LATE_INIT = 1 << 9;
        const IS_READONLY = 1 << 10;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct HhasProperty<'arena> {
    pub name: hhbc_id::prop::PropType<'arena>,
    pub flags: HhasPropertyFlags,
    pub attributes: Slice<'arena, HhasAttribute<'arena>>,
    pub visibility: Visibility,
    pub initial_value: Maybe<runtime::TypedValue<'arena>>,
    pub initializer_instrs: Maybe<InstrSeq<'arena>>,
    pub type_info: HhasTypeInfo<'arena>,
    pub doc_comment: Maybe<Str<'arena>>,
}
impl<'arena> HhasProperty<'arena> {
    pub fn is_private(&self) -> bool {
        self.visibility == Visibility::Private
    }
    pub fn is_protected(&self) -> bool {
        self.visibility == Visibility::Protected
    }
    pub fn is_public(&self) -> bool {
        self.visibility == Visibility::Public
    }
    pub fn is_late_init(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_LATE_INIT)
    }
    pub fn is_no_bad_redeclare(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_NO_BAD_REDECLARE)
    }
    pub fn initial_satisfies_tc(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::INITIAL_SATISFIES_TC)
    }
    pub fn no_implicit_null(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::NO_IMPLICIT_NULL)
    }
    pub fn has_system_initial(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::HAS_SYSTEM_INITIAL)
    }
    pub fn is_const(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_CONST)
    }
    pub fn is_deep_init(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_DEEP_INIT)
    }
    pub fn is_lsb(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_LSB)
    }
    pub fn is_static(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_STATIC)
    }
    pub fn is_readonly(&self) -> bool {
        self.flags.contains(HhasPropertyFlags::IS_READONLY)
    }
}
