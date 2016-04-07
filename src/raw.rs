//! Minimal wrapper around internal Perl API.

use perl_sys::funcs::*;

pub use perl_sys::types::*;
pub use perl_sys::consts::*;

pub type Stack = OuroborosStack;

#[derive(Copy, Clone)]
pub struct Interpreter (PerlThreadContext);

impl Interpreter {
    pub unsafe fn new(pthx: PerlThreadContext) -> Interpreter { Interpreter(pthx) }

    pub unsafe fn sv_iv(&self, sv: *mut SV) -> IV { ouroboros_sv_iv(self.0, sv) }
    pub unsafe fn sv_uv(&self, sv: *mut SV) -> UV { ouroboros_sv_uv(self.0, sv) }
    pub unsafe fn sv_nv(&self, sv: *mut SV) -> NV { ouroboros_sv_nv(self.0, sv) }

    pub unsafe fn sv_refcnt_inc(&self, sv: *mut SV) { ouroboros_sv_refcnt_inc_void_nn(self.0, sv) }
    pub unsafe fn sv_refcnt_dec(&self, sv: *mut SV) { ouroboros_sv_refcnt_dec_nn(self.0, sv) }

    pub unsafe fn av_fetch(&self, av: *mut AV, key: SSize_t) -> *mut *mut SV { Perl_av_fetch(self.0, av, key, 0) }
    pub unsafe fn av_fetch_lvalue(&self, av: *mut AV, key: SSize_t) -> *mut *mut SV { Perl_av_fetch(self.0, av, key, 1) }
    pub unsafe fn av_store(&self, av: *mut AV, key: SSize_t, sv: *mut SV) -> *mut *mut SV { Perl_av_store(self.0, av, key, sv) }

    pub unsafe fn st_init(&self, stack: &mut Stack) { ouroboros_stack_init(self.0, stack) }
    pub unsafe fn st_prepush(&self, stack: &mut Stack) { ouroboros_stack_prepush(self.0, stack) }
    pub unsafe fn st_putback(&self, stack: &mut Stack) { ouroboros_stack_putback(self.0, stack) }
    pub unsafe fn st_extend(&self, stack: &mut Stack, len: Size_t) { ouroboros_stack_extend(self.0, stack, len) }

    pub unsafe fn st_fetch(&self, stack: &mut Stack, idx: SSize_t) -> *mut SV { ouroboros_stack_fetch(self.0, stack, idx) }
    pub unsafe fn st_push(&self, stack: &mut Stack, val: *mut SV) { ouroboros_stack_push_sv(self.0, stack, val) }
    pub unsafe fn st_push_iv(&self, stack: &mut Stack, val: IV) { ouroboros_stack_push_iv(self.0, stack, val) }
    pub unsafe fn st_push_uv(&self, stack: &mut Stack, val: UV) { ouroboros_stack_push_uv(self.0, stack, val) }
    pub unsafe fn st_push_nv(&self, stack: &mut Stack, val: NV) { ouroboros_stack_push_nv(self.0, stack, val) }

    pub unsafe fn call_pv(&self, name: *const i8, flags: I32) -> I32 { Perl_call_pv(self.0, name, flags) }

    pub unsafe fn new_xs(&self, name: *const i8, func: XSUBADDR_t, file: *const i8) -> *mut CV {
        Perl_newXS(self.0, name, func, file)
    }
    pub unsafe fn new_sv(&self, len: STRLEN) -> *mut SV { Perl_newSV(self.0, len) }
    pub unsafe fn new_sv_iv(&self, val: IV) -> *mut SV { Perl_newSViv(self.0, val) }
    pub unsafe fn new_sv_uv(&self, val: UV) -> *mut SV { Perl_newSVuv(self.0, val) }
    pub unsafe fn new_sv_nv(&self, val: NV) -> *mut SV { Perl_newSVnv(self.0, val) }
    pub unsafe fn new_sv_pvn(&self, val: *const i8, len: STRLEN, flags: U32) -> *mut SV { Perl_newSVpvn_flags(self.0, val, len, flags) }

    pub unsafe fn get_av(&self, name: *const i8) -> *mut AV { Perl_get_av(self.0, name, 0) }
}