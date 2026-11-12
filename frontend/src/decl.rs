// Declarations: decl-specifiers, declarators, and type-ids.
//
// The Tidal grammar for declarators (specification/declarations/declarators.md)
// gives `declarator`, `ptr_part`, `direct`, and `params`, but no
// production for `decl_specifiers`, `type`, `type_id`, or
// `simple_declaration`. Those follow general knowledge and are logged in
// the report as grammar gaps.

use crate::ast::*;
use crate::lexer::TK;
use crate::parser::*;
use crate::types::*;

#[derive(Clone, Debug, Default)]
pub struct DeclSpec {
    pub storage: Vec<Storage>,
    pub typedef: bool,
    pub inline: bool,
    pub constexpr: bool,
    pub consteval: bool,
    pub constinit: bool,
    pub virtual_: bool,
    pub explicit: bool,
    pub friend: bool,
    pub mutable: bool,
    pub register: bool,
    pub c: u32,
    pub v: u32,
    pub ty: Option<QTy>,
    /// Some(false) for `auto`, Some(true) for `decltype(auto)`.
    pub auto: Option<bool>,
    pub kw: Vec<String>,
    pub defined_class: Option<usize>,
    pub defined_enum: Option<usize>,
    /// The specifiers were only an elaborated type specifier or a class or
    /// enum head, so `struct S;` declares S.
    pub tag_only: bool,
    pub line: u32,
    pub named_type: bool,
    pub concept_auto: bool,
    /// The type constraint before `auto`: the concept, its other
    /// arguments, and the span of the name and arguments (CPT-20,
    /// CPT-22).
    pub cons_auto: Option<(usize, Vec<crate::templ::RawArg>, (usize, usize, Option<usize>, usize))>,
    /// A class template named without arguments: its arguments are
    /// deduced from the initializer (CTAD-1).
    pub ctad: Option<usize>,
    pub align: u64,
    pub has_alignas: bool,
}

#[derive(Clone, Debug)]
pub struct FnDecl {
    pub params: Vec<DeclId>,
    pub ptys: Vec<QTy>,
    pub variadic: bool,
    pub cv_c: bool,
    pub cv_v: bool,
    pub refq: u8,
    pub noexcept: bool,
    /// `noexcept(false)` was written.
    pub noexcept_false: bool,
    pub trailing: Option<QTy>,
    pub scope: usize,
    pub defaults: usize,
    pub has_explicit_this: bool,
    pub trailing_auto: Option<(DeclSpec, Declarator)>,
    /// In a template, the operand of `noexcept` depends on a parameter
    /// (DEP-9): a function type made from this declarator is dependent.
    pub noexcept_dep: bool,
    /// DED-11: the operand of `noexcept` is a constant template
    /// parameter.
    pub noexcept_param: Option<(u32, u32)>,
}

#[derive(Clone, Debug)]
pub enum DOp {
    Ptr(bool, bool),
    LRef,
    RRef,
    MemPtr(Option<usize>, bool, bool),
    /// A pointer to member of a class that is a template parameter.
    MemPtrDep(QTy, bool, bool),
    Array(Option<u64>),
    Func(Box<FnDecl>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NameKind {
    Plain,
    Ctor,
    Dtor,
    Operator(String),
    Conversion,
    Literal,
}

#[derive(Clone, Debug)]
pub struct Declarator {
    pub name: Option<String>,
    pub kind: NameKind,
    /// Scope named by a qualifier, as in `N::f` or `S::g`.
    pub qual: Option<usize>,
    pub qual_unknown: bool,
    /// For a member of a class template defined outside its class: the
    /// scope of the pattern class (NAM-21).
    pub pattern_scope: Option<usize>,
    pub ops: Vec<DOp>,
    pub line: u32,
    pub conv_ty: Option<QTy>,
    pub template_args: bool,
    pub pack: bool,
}

impl Declarator {
    pub fn func(&self) -> Option<&FnDecl> {
        // The declarator declares a function if its last op is a Func.
        match self.ops.last() {
            Some(DOp::Func(f)) => Some(f),
            _ => None,
        }
    }
}

pub fn simple_type_kw(s: &str) -> bool {
    matches!(
        s,
        "void" | "bool" | "char" | "char8_t" | "char16_t" | "char32_t" | "wchar_t" | "short" | "int" | "long" | "signed" | "unsigned" | "float" | "double"
    )
}

impl P {
    /// Does the current token start a type (for casts, sizeof, and
    /// template arguments)?
    pub fn starts_type(&self) -> bool {
        let t = self.tok();
        if t.kind != TK::Ident {
            return false;
        }
        let s = t.text.as_str();
        if simple_type_kw(s) || matches!(s, "const" | "volatile" | "struct" | "class" | "union" | "enum" | "typename" | "decltype" | "auto") {
            return true;
        }
        if is_keyword(s) {
            return false;
        }
        self.qualified_names_type(0)
    }

    /// Does a (possibly qualified) name starting at offset k name a type?
    pub fn qualified_names_type(&self, mut k: usize) -> bool {
        if self.has_fact(self.pos + k, crate::templ::FACT_VALUE_QUAL) {
            // DEP-19: the template read the name as a value.
            return false;
        }
        let mut scope: Option<usize> = None;
        if self.at_k(k, "::") {
            scope = Some(self.global);
            k += 1;
        }
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident || is_keyword(&t.text) {
                return false;
            }
            let found = match scope {
                None if self.at_k(k + 1, "::") => self.lookup_scope_name(&t.text),
                None => self.lookup(&t.text),
                Some(s) => self.lookup_in_scope(s, &t.text, true),
            };
            let Some(ids) = found else {
                // A member of a specialization whose members are not made
                // yet: the tokens after the name decide.
                if let Some(sc) = scope {
                    if let SK::Class(c) = self.scopes[sc].kind {
                        if self.inst_of.contains_key(&c) && !self.classes[c].complete && !self.in_template() {
                            return self.declarator_after_qualified(k);
                        }
                    }
                }
                return false;
            };
            let id = ids[0];
            // template-id
            let mut next = k + 1;
            if self.at_k(next, "<") && self.names_template(id) {
                next = self.skip_angle_at(next);
            }
            if self.at_k(next, "::") && next > k + 1 {
                // A template-id before `::`: the member is looked up in the
                // pattern and in the specializations.
                if let Some(t) = self.tid_of(id) {
                    let member = self.peek(next + 1).text.clone();
                    if self.in_template() && self.inst_depth == 0 && self.templs[t].kind == crate::templ::TKind::Class && !self.names_current_inst(t, k + 2, next - 1) {
                        // DEP-15: a dependent template-id that does not name
                        // the current instantiation; its member is a value
                        // without `typename` (DEP-19).
                        if self.peek(next + 2).kind == TK::Ident {
                            self.dep15_hit.set(self.peek(next + 2).line);
                        }
                        return false;
                    }
                    match self.templ_member_scope(t, &member) {
                        Some(s) => {
                            scope = Some(s);
                            k = next + 1;
                            continue;
                        }
                        None => return false,
                    }
                }
            }
            if self.at_k(next, "::") {
                match &self.decls[id].k {
                    DK::Namespace(s) => scope = Some(*s),
                    DK::Class(c) => scope = Some(self.classes[*c].scope),
                    DK::Enum(e) => scope = Some(self.enums[*e].scope),
                    DK::TypeParam | DK::Template(_) => {
                        // `T::x` is a value unless `typename` says otherwise.
                        return false;
                    }
                    DK::Type(q) => match &q.t {
                        Ty::Class(c) => scope = Some(self.classes[*c].scope),
                        Ty::Enum(e) => scope = Some(self.enums[*e].scope),
                        // A member alias of a pattern, looked up through a
                        // template-id: its type is made with the
                        // specialization.
                        Ty::Dep(_) if !self.in_template() => return self.declarator_after_qualified(next + 1),
                        _ => return false,
                    },
                    _ => return false,
                }
                k = next + 1;
                continue;
            }
            return match &self.decls[id].k {
                DK::Type(_) | DK::Class(_) | DK::Enum(_) | DK::TypeParam => true,
                DK::Template(TemplKind::Class(_)) | DK::Template(TemplKind::Alias) => true,
                DK::Template(TemplKind::Concept) => self.at_k(next, "auto") || self.at_k(next, "decltype"),
                _ => false,
            };
        }
    }

    /// DEP-20: a qualified name whose first name is a template parameter
    /// or a template-id, which is dependent in a template.
    pub fn dependent_qualifier_at(&self) -> bool {
        self.dependent_qualifier_at_k(0)
    }
    pub fn dependent_qualifier_at_k(&self, k: usize) -> bool {
        let t = self.peek(k);
        if t.kind != TK::Ident || is_keyword(&t.text) {
            return false;
        }
        let Some(ids) = self.lookup_scope_name(&t.text) else { return false };
        let id = ids[0];
        let mut next = k + 1;
        if self.at_k(next, "<") && self.names_template(id) {
            next = self.skip_angle_at(next);
        }
        if !self.at_k(next, "::") || self.peek(next + 1).kind != TK::Ident {
            return false;
        }
        matches!(self.decls[id].k, DK::TypeParam) || self.tparam.contains_key(&id) || next > k + 1
    }

    /// From offset `k` at a qualified name: whether a `<` follows a name
    /// after `::` with no `template` before it.
    pub fn less_after_qualified(&self, mut k: usize) -> bool {
        let mut after_colons = false;
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident || is_keyword(&t.text) {
                return false;
            }
            k += 1;
            if self.at_k(k, "<") {
                if after_colons {
                    return true;
                }
                k = self.skip_angle_at(k);
            }
            if self.at_k(k, "::") {
                k += 1;
                after_colons = true;
                continue;
            }
            return false;
        }
    }

    /// From offset `k` at a name in a qualified name: whether a declarator
    /// follows the last name, so that the qualified name names a type.
    pub fn declarator_after_qualified(&self, mut k: usize) -> bool {
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident || is_keyword(&t.text) {
                return false;
            }
            k += 1;
            if self.at_k(k, "<") {
                k = self.skip_angle_at(k);
            }
            if self.at_k(k, "::") {
                k += 1;
                continue;
            }
            break;
        }
        let n = self.peek(k);
        if n.kind == TK::Ident && !is_keyword(&n.text) {
            return true;
        }
        (self.at_k(k, "*") || self.at_k(k, "&") || self.at_k(k, "&&")) && self.peek(k + 1).kind == TK::Ident
            || self.at_k(k, ">") || self.at_k(k, ",") || self.at_k(k, ">>")
    }

    /// Given offset k at `<`, return the offset after the matching `>`.
    /// DEP-15: whether the arguments at offsets `a..b` of the class
    /// template `t` name the current instantiation, or depend on no
    /// template parameter. The current instantiation is the template with
    /// its own parameters in order, or, inside a partial specialization,
    /// with that partial specialization's arguments.
    pub fn names_current_inst(&self, t: usize, a: usize, b: usize) -> bool {
        let toks: Vec<&crate::lexer::Token> = (a..b).map(|k| self.peek(k)).collect();
        let dependent_tok = |tk: &crate::lexer::Token| -> bool {
            if tk.kind != TK::Ident {
                return false;
            }
            let Some(ids) = self.lookup(&tk.text) else { return false };
            let id = ids[0];
            if self.tparam.contains_key(&id) || matches!(self.decls[id].k, DK::TypeParam) {
                return true;
            }
            match &self.decls[id].k {
                DK::Type(q) => q.t.contains_unknown() || matches!(q.t, Ty::Dep(_)),
                // A member of a class template, such as a constant that a
                // parameter gives.
                DK::Var(_) | DK::Field(_) => matches!(self.scopes[self.decls[id].scope].kind, SK::Class(c) if self.classes[c].template),
                _ => false,
            }
        };
        if !toks.iter().any(|tk| dependent_tok(tk)) {
            return true;
        }
        // The template's own parameters in order.
        let depth = self.templs[t].depth;
        let mut i = 0u32;
        let mut own = true;
        let mut k = 0;
        while k < toks.len() {
            let tk = toks[k];
            let param = self.lookup(&tk.text).and_then(|ids| {
                let id = ids[0];
                match self.tparam.get(&id) {
                    Some(&(d, j)) => Some((d, j)),
                    None => match &self.decls[id].k {
                        DK::Type(q) => match &q.t {
                            Ty::Dep(dd) => match &**dd {
                                crate::types::Dep::Param(d, j) => Some((*d, *j)),
                                _ => None,
                            },
                            _ => None,
                        },
                        _ => None,
                    },
                }
            });
            if param != Some((depth, i)) {
                own = false;
                break;
            }
            k += 1;
            if k < toks.len() && toks[k].text == "..." {
                k += 1;
            }
            if k < toks.len() {
                if toks[k].text != "," {
                    own = false;
                    break;
                }
                k += 1;
            }
            i += 1;
        }
        // Inside a partial specialization: its own argument list, and
        // not the primary's.
        let mine: Vec<&str> = toks.iter().map(|x| x.text.as_str()).collect();
        let mut in_partial = false;
        for &sc in &self.stack {
            if let SK::Class(c) = self.scopes[sc].kind {
                if let Some(theirs) = self.partial_arg_toks.get(&c) {
                    if self.classes[c].name == self.templs[t].name {
                        in_partial = true;
                        if theirs.iter().map(|x| x.as_str()).collect::<Vec<_>>() == mine {
                            return true;
                        }
                        // G116: an argument that names a type alias for the
                        // partial specialization's argument is equivalent
                        // (TPL-20).
                        let split = |v: &[&str]| -> Vec<Vec<String>> {
                            let mut out = vec![Vec::new()];
                            let mut d = 0i32;
                            for &x in v {
                                match x {
                                    "<" | "(" | "[" => d += 1,
                                    ">" | ")" | "]" => d -= 1,
                                    ">>" => d -= 2,
                                    _ => {}
                                }
                                if x == "," && d == 0 {
                                    out.push(Vec::new());
                                } else {
                                    out.last_mut().unwrap().push(x.to_string());
                                }
                            }
                            out
                        };
                        let simple_ty = |v: &[String]| -> Option<QTy> {
                            let first = v.first()?;
                            let ids = self.lookup(first)?;
                            let id = ids[0];
                            let mut q = match self.tparam.get(&id) {
                                Some(&(d, j)) => crate::templ::dep(crate::types::Dep::Param(d, j)),
                                None => match &self.decls[id].k {
                                    DK::Type(q) => q.clone(),
                                    DK::Class(c) => QTy::new(Ty::Class(*c)),
                                    _ => return None,
                                },
                            };
                            for x in &v[1..] {
                                q = match x.as_str() {
                                    "*" => QTy::new(Ty::Ptr(Box::new(q))),
                                    "&" => QTy::new(Ty::LRef(Box::new(q))),
                                    "&&" => QTy::new(Ty::RRef(Box::new(q))),
                                    "const" => QTy { c: true, ..q },
                                    _ => return None,
                                };
                            }
                            Some(q)
                        };
                        let theirs_v: Vec<&str> = theirs.iter().map(|x| x.as_str()).collect();
                        let (a, b) = (split(&mine), split(&theirs_v));
                        if a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y || matches!((simple_ty(x), simple_ty(y)), (Some(p), Some(q)) if p == q)) {
                            return true;
                        }
                    }
                }
            }
        }
        if own && !in_partial && i as usize == self.templs[t].params.len() {
            return true;
        }
        false
    }

    pub fn skip_angle_at(&self, mut k: usize) -> usize {
        let mut depth = 0i32;
        loop {
            let t = self.peek(k);
            if t.kind == TK::Eof {
                return k;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "<" => depth += 1,
                    ">" => {
                        depth -= 1;
                        if depth == 0 {
                            return k + 1;
                        }
                    }
                    ">>" => {
                        if depth == 1 {
                            // The list ends at the first half of `>>`: the
                            // token after it is the second `>`.
                            return k;
                        }
                        depth -= 2;
                        if depth <= 0 {
                            return k + 1;
                        }
                    }
                    "(" | "[" | "{" => {
                        let mut dd = 0;
                        loop {
                            let tt = self.peek(k);
                            if tt.kind == TK::Eof {
                                return k;
                            }
                            if tt.kind == TK::Punct && matches!(tt.text.as_str(), "(" | "[" | "{") {
                                dd += 1;
                            }
                            if tt.kind == TK::Punct && matches!(tt.text.as_str(), ")" | "]" | "}") {
                                dd -= 1;
                                if dd == 0 {
                                    break;
                                }
                            }
                            k += 1;
                        }
                    }
                    ";" | "}" => return k,
                    _ => {}
                }
            }
            k += 1;
        }
    }

    pub fn skip_attributes(&mut self) {
        loop {
            if self.at("[") && self.at_k(1, "[") {
                self.check_attributes();
                continue;
            }
            if self.at("alignas") {
                // ALN-1: alignas(constant) or alignas(type).
                let line = self.line();
                self.pos += 1;
                if self.at("(") {
                    let save = self.pos;
                    self.pos += 1;
                    // PACK-16: `alignas(T...)` takes the largest of the
                    // alignments; an empty expansion has no effect.
                    let val: Option<u64> = match self.parse_expansion(|p| p.alignas_operand(line)) {
                        Ok((vs, true)) => vs.into_iter().try_fold(0u64, |m, v| v.map(|v| m.max(v))),
                        Ok((mut vs, false)) => vs.pop().flatten(),
                        Err(_) => None,
                    };
                    if val.is_none() || !self.at(")") {
                        self.pos = save;
                        self.skip_balanced();
                        self.align_unknown = true;
                    } else {
                        self.pos += 1;
                    }
                    let v = val.unwrap_or(0);
                    self.last_align = self.last_align.max(v);
                    self.saw_alignas = true;
                }
                continue;
            }
            break;
        }
    }

    /// BF-1, ALN-8: a class operand that must convert to an integral or
    /// unscoped enumeration type. The class is a literal type, and exactly
    /// one of its conversion functions gives such a type, a `constexpr`
    /// one. Returns the reason when the class does not meet this.
    pub fn int_class_operand(&mut self, e: &Expr) -> Option<String> {
        self.int_class_convert(e).err()
    }

    /// BF-1, ALN-8: as `int_class_operand`, with the operand converted to
    /// the one target type when the front end can form the conversion.
    pub fn int_class_convert(&mut self, e: &Expr) -> Result<Option<Expr>, String> {
        let r = self.int_class_convert0(e);
        match r {
            Some(Err(w)) => Err(w),
            Some(Ok(x)) => Ok(x),
            None => Ok(None),
        }
    }

    fn int_class_convert0(&mut self, e: &Expr) -> Option<Result<Option<Expr>, String>> {
        let Ty::Class(c) = nonref(&e.ty).t else { return None };
        if self.in_template() || !self.class_ok(c) {
            return None;
        }
        if self.literal_type(&QTy::new(Ty::Class(c))) == Some(false) {
            return Some(Err(format!("the class '{}' is not a literal type", self.classes[c].name)));
        }
        let mut fs = Vec::new();
        self.conv_fns(c, 0, &mut fs);
        let mut ints = Vec::new();
        for d in fs {
            if self.is_func_templ(d) {
                return None;
            }
            let Some(f) = self.func(d) else { return None };
            if f.explicit {
                continue;
            }
            let r = nonref(&f.fty.ret);
            if r.t.is_integral() || matches!(r.t, Ty::Enum(en) if !self.enums[en].scoped) {
                ints.push(d);
            }
        }
        // BF-1, ALN-8: several functions can give the one target type.
        let mut targets: Vec<QTy> = Vec::new();
        for &d in &ints {
            let t = nonref(&self.func(d).unwrap().fty.ret).unqual();
            if !targets.contains(&t) {
                targets.push(t);
            }
        }
        match targets.len() {
            0 => Some(Err(format!("the class '{}' has no conversion function to an integral type", self.classes[c].name))),
            1 => {
                // The function that the conversion selects is constexpr.
                let mark = self.d.list.len();
                let x = self.user_conv(e, &targets[0], e.line);
                if self.d.list.len() != mark {
                    self.d.list.truncate(mark);
                    return None;
                }
                let x = x?;
                let mut sel = None;
                crate::ast::walk_expr(&x, &mut |y| {
                    if let EK::Call(_, _, Some(f)) = &y.k {
                        if ints.contains(f) && sel.is_none() {
                            sel = Some(*f);
                        }
                    }
                });
                let sel = match sel {
                    Some(f) => f,
                    None if ints.len() == 1 => ints[0],
                    None => return None,
                };
                let f = self.func(sel).unwrap();
                if f.constexpr || f.consteval {
                    Some(Ok(Some(x)))
                } else {
                    Some(Err(format!("the conversion function of '{}' is not constexpr", self.classes[c].name)))
                }
            }
            _ => Some(Err(format!("the class '{}' has conversion functions to more than one integral type", self.classes[c].name))),
        }
    }

    /// The operand of `alignas`: a type or a constant (ALN-1). None when
    /// the front end cannot tell. Leaves the position after the operand.
    fn alignas_operand(&mut self, line: u32) -> PR<Option<u64>> {
        let mut val: Option<u64> = None;
                    if self.starts_type() {
                        if let Ok(t) = self.parse_type_id() {
                            // The declarator took the `...` of a pack
                            // expansion.
                            if self.at(")") && self.pos > 0 && self.toks[self.pos - 1].kind == TK::Punct && self.toks[self.pos - 1].text == "..." {
                                self.pos -= 1;
                            }
                            if self.at(")") || self.at("...") {
                                val = self.alignof_ty(&nonref(&t).t).or(Some(0));
                            }
                        }
                    } else if let Ok(e) = self.parse_assignment() {
                        // ALN-8: a class operand converts to an integer by
                        // its conversion function.
                        let e = match self.int_class_convert(&e) {
                            Err(why) => {
                                self.error(line, "ALN-8", format!("the operand of alignas does not convert to an integral type: {}", why));
                                return Ok(Some(0));
                            }
                            Ok(Some(x)) => x,
                            Ok(None) => e,
                        };
                        let e = if matches!(nonref(&e.ty).t, Ty::Class(_)) && !self.in_template() {
                            let mark = self.d.list.len();
                            match self.user_conv(&e, &QTy::new(Ty::size_t()), line) {
                                Some(x) if self.d.list.len() == mark => x,
                                _ => {
                                    self.d.list.truncate(mark);
                                    e
                                }
                            }
                        } else {
                            e
                        };
                        if matches!(nonref(&e.ty).t, Ty::Enum(en) if self.enums[en].scoped) && !self.in_template() {
                            // ALN-8 (Q-0313): a scoped enumeration does not
                            // convert to an integer.
                            self.error(line, "ALN-8", "the operand of alignas has a scoped enumeration type");
                            return Ok(Some(0));
                        }
                        if self.at(")") || self.at("...") {
                            match self.const_int(&e) {
                                Ok(v) => {
                                    if v < 0 || (v != 0 && (v & (v - 1)) != 0) {
                                        // ALN-2: zero or a power of two.
                                        self.error(line, "ALN-2", "alignment is not a power of two");
                                    } else if v > 1 << 28 {
                                        // ALN-5: above the largest extended
                                        // alignment of the matrix targets.
                                        self.error(line, "ALN-5", "alignment larger than the largest extended alignment, 2^28");
                                    }
                                    val = Some(v.max(0) as u64);
                                }
                                Err(crate::consteval::CeErr::NotConst(..)) => {
                                    if !self.in_template() {
                                        self.error(line, "ALN-2", "alignment is not a constant expression");
                                    }
                                    val = Some(0);
                                }
                                Err(_) => val = Some(0),
                            }
                        }
                    }
        Ok(val)
    }

    /// Attributes (specification/declarations/attributes.md). The front end parses
    /// each attribute, checks its arguments (ATTR-2, ATTR-13, ATTR-17),
    /// and records it in `pending_attrs` for the statement or declaration.
    fn check_attributes(&mut self) {
        let open = self.pos;
        self.pos += 2; // [[
        let using = if self.eat("using") {
            if self.is_ident() {
                self.pos += 1;
            }
            self.eat(":");
            true
        } else {
            false
        };
        loop {
            if self.at("]") || self.tok().kind == TK::Eof {
                break;
            }
            if self.eat(",") {
                continue;
            }
            let line = self.line();
            if !self.is_ident() {
                break;
            }
            let mut name = self.tok().text.clone();
            self.pos += 1;
            let mut ns = false;
            if self.at("::") {
                self.pos += 1;
                ns = true;
                if self.is_ident() {
                    name = self.tok().text.clone();
                    self.pos += 1;
                }
            }
            if using && ns {
                // ATTR-17: with a `using` prefix, no attribute namespace.
                self.error(line, "ATTR-17", "attribute with its own namespace in a specifier with a 'using' prefix");
            }
            let std = !ns && !using;
            let mut assume = None;
            let mut nargs: Option<(usize, bool)> = None; // (count, all string literals)
            if self.at("(") {
                let start = self.pos;
                if std && name == "assume" {
                    self.pos += 1;
                    match self.parse_assignment() {
                        Ok(e) => {
                            if self.at(",") {
                                // ATTR-13: no comma operator at the top level.
                                self.error(line, "ATTR-13", "assumption with a comma operator at its top level");
                            } else if self.at(")") {
                                let b = self.contextual_bool(e, "ATTR-13");
                                assume = Some(b);
                            }
                        }
                        Err(_) => {}
                    }
                    self.pos = start;
                    self.skip_balanced();
                } else {
                    self.skip_balanced();
                    // count the top-level arguments
                    let mut count = 0;
                    let mut all_str = true;
                    let mut depth = 0;
                    let mut cur = 0;
                    for k in start + 1..self.pos - 1 {
                        let t = &self.toks[k];
                        match t.text.as_str() {
                            "(" | "[" | "{" => depth += 1,
                            ")" | "]" | "}" => depth -= 1,
                            "," if depth == 0 => {
                                count += 1;
                                if cur == 0 {
                                    all_str = false;
                                }
                                cur = 0;
                                continue;
                            }
                            _ => {}
                        }
                        cur += 1;
                        if t.kind != TK::Str {
                            all_str = false;
                        }
                    }
                    if self.pos - 1 > start + 1 {
                        count += 1;
                    }
                    nargs = Some((count, all_str));
                }
            }
            self.eat("...");
            if std {
                match (name.as_str(), nargs) {
                    ("deprecated" | "nodiscard", Some((n, all_str))) if n > 1 || (n == 1 && !all_str) => {
                        // ATTR-2: no argument or one string literal.
                        self.error(line, "ATTR-2", format!("'{}' takes no argument or one string literal", name));
                    }
                    ("noreturn" | "fallthrough" | "likely" | "unlikely" | "maybe_unused" | "no_unique_address", Some(_)) => {
                        // ATTR-2: these attributes have no argument.
                        self.error(line, "ATTR-2", format!("'{}' takes no argument", name));
                    }
                    _ => {}
                }
                if name == "no_unique_address" {
                    self.saw_nua = true;
                }
                self.pending_attrs.push(Attr { name: name.clone(), ns, assume, line });
            }
        }
        // close the specifier
        if !(self.eat("]") && self.eat("]")) {
            self.pos = open + 1;
            self.skip_balanced();
            self.eat("]");
        }
    }

    pub fn parse_decl_specs(&mut self, in_class: Option<usize>) -> PR<DeclSpec> {
        let itn = std::mem::take(&mut self.implicit_tn) && (self.in_template() || self.stack.iter().any(|&s| self.scopes[s].kind == SK::Template));
        let mut ds = DeclSpec { line: self.line(), ..Default::default() };
        loop {
            // ATTR-19, ATTR-2: an attribute after a type specifier applies
            // to the type, and `maybe_unused` does not apply to a type.
            let typed = ds.ty.is_some() || ds.kw.iter().any(|k| simple_type_kw(k));
            let attrs0 = self.pending_attrs.len();
            self.skip_attributes();
            if typed && self.inst_depth == 0 {
                let bad: Vec<u32> = self.pending_attrs.get(attrs0..).unwrap_or(&[]).iter().filter(|a| a.name == "maybe_unused" && !a.ns).map(|a| a.line).collect();
                for l in bad {
                    self.error(l, "ATTR-2", "'maybe_unused' after the type specifiers applies to the type, and it does not apply to a type");
                }
            }
            let t = self.tok().clone();
            if t.kind == TK::Punct && t.text == "::" && ds.ty.is_none() && ds.kw.is_empty() && ds.auto.is_none() && !self.is_ctor_start(in_class) && self.qualified_names_type(0) {
                self.ctad_tid = None;
                let q = self.parse_type_name()?;
                ds.ctad = self.ctad_tid.take();
                ds.ty = Some(q);
                ds.named_type = true;
                continue;
            }
            if t.kind != TK::Ident {
                break;
            }
            let s = t.text.as_str();
            match s {
                "static" => ds.storage.push(Storage::Static),
                "extern" => {
                    if self.peek(1).kind == TK::Str {
                        break;
                    }
                    ds.storage.push(Storage::Extern)
                }
                "thread_local" => ds.storage.push(Storage::ThreadLocal),
                "register" => ds.register = true,
                "mutable" => ds.mutable = true,
                "typedef" => ds.typedef = true,
                "inline" => ds.inline = true,
                "constexpr" => ds.constexpr = true,
                "consteval" => ds.consteval = true,
                "constinit" => ds.constinit = true,
                "virtual" => ds.virtual_ = true,
                "friend" => {
                    ds.friend = true;
                    self.friend_spec_pos = Some(self.pos);
                }
                "explicit" => {
                    ds.explicit = true;
                    if self.at_k(1, "(") {
                        // CTOR-9: explicit(c) needs a constant c.
                        self.pos += 2;
                        let start = self.pos - 1;
                        let l = self.line();
                        let cstart = self.pos;
                        match self.parse_assignment() {
                            Ok(e) if self.at(")") => {
                                if self.in_template() {
                                    // The condition depends on the arguments
                                    // of a specialization (CTAD-5).
                                    let (vid, base) = self.origin_of(cstart);
                                    self.pending_explicit_cond = Some(crate::templ::Source { toks: self.copy_toks(cstart, self.pos), stack: self.stack.clone(), ctx: crate::decl2::Ctx::Namespace, access: 0, limit: self.decls.len(), vid, base });
                                }
                                self.pos += 1;
                                if !self.in_template() && !e.ty.is_unknown() && nonref(&e.ty).t != Ty::Bool && self.is_narrowing(&e, &Ty::Bool) {
                                    // CE-19 (Q-0078): the conversion of the
                                    // condition to bool must not narrow.
                                    self.error(l, "CE-19", "the condition of explicit narrows in its conversion to bool");
                                }
                                if let Some(v) = self.eval_bool_const(&e, l, "CTOR-9") {
                                    ds.explicit = v;
                                }
                            }
                            _ => {
                                self.pos = start;
                                self.skip_balanced();
                            }
                        }
                        continue;
                    }
                }
                "const" => ds.c += 1,
                "volatile" => ds.v += 1,
                _ if simple_type_kw(s) => {
                    if ds.ty.is_some() || ds.auto.is_some() {
                        // TYN-2 / AUTO-8: a type specifier combined with another type.
                        self.error(t.line, "AUTO-8", format!("'{}' combined with another type specifier", s));
                    }
                    ds.kw.push(s.to_string());
                }
                "auto" => {
                    if ds.ty.is_some() || !ds.kw.is_empty() {
                        self.error(t.line, "AUTO-8", "'auto' combined with another type specifier");
                    }
                    ds.auto = Some(false);
                }
                "decltype" => {
                    if ds.ty.is_some() || !ds.kw.is_empty() || ds.auto.is_some() {
                        break;
                    }
                    if self.at_k(1, "(") && self.at_k(2, "auto") && self.at_k(3, ")") {
                        self.pos += 4;
                        ds.auto = Some(true);
                        continue;
                    }
                    self.pos += 1;
                    let mut q = self.parse_decltype()?;
                    if self.at("::") {
                        // DTY: decltype as a nested name qualifier
                        if let Ty::Class(c) = nonref(&q).t {
                            // INST-1: a qualified name that names a member
                            // makes the class.
                            let l = self.line();
                            self.require_class(c, l);
                            if self.peek(1).kind == TK::Ident {
                                let n = self.peek(1).text.clone();
                                if let Some(ids) = self.lookup_member(c, &n) {
                                    if let Some(t) = ids.iter().find_map(|&i| match &self.decls[i].k {
                                        DK::Type(t) => Some(t.clone()),
                                        DK::Class(cc) => Some(QTy::new(Ty::Class(*cc))),
                                        DK::Enum(e) => Some(QTy::new(Ty::Enum(*e))),
                                        _ => None,
                                    }) {
                                        self.pos += 2;
                                        q = t;
                                    } else {
                                        // a value: leave `::` for the expression parser
                                        ds.ty = Some(q);
                                        ds.named_type = true;
                                        break;
                                    }
                                }
                            }
                        } else if q.is_unknown() {
                            self.pos += 1;
                            self.skip_qualified_name()?;
                        }
                    }
                    ds.ty = Some(q);
                    ds.named_type = true;
                    continue;
                }
                "struct" | "class" | "union" => {
                    if ds.ty.is_some() || !ds.kw.is_empty() {
                        break;
                    }
                    let start = self.pos;
                    let (q, defined) = self.parse_class_specifier()?;
                    if !defined && self.at(";") && !ds.friend && ds.kw.is_empty() && !self.in_template() && self.pending_template.is_none() && start > 0 && self.toks[start - 1].text != "template" {
                        // TYN-7: a class key and a qualified name alone is
                        // not a declaration.
                        let named = &self.toks[start + 1..self.pos];
                        if named.iter().any(|t| t.text == "::") && !named.iter().any(|t| t.text == "<") {
                            let l = self.line();
                            self.error(l, "TYN-7", "a declaration of only a class key and a qualified name");
                        }
                    }
                    if let Ty::Class(c) = q.t {
                        if defined {
                            ds.defined_class = Some(c);
                        }
                    }
                    ds.tag_only = true;
                    ds.ty = Some(q);
                    continue;
                }
                "enum" => {
                    if ds.ty.is_some() || !ds.kw.is_empty() {
                        break;
                    }
                    let (q, defined) = self.parse_enum_specifier(&ds)?;
                    if let Ty::Enum(e) = q.t {
                        if defined {
                            ds.defined_enum = Some(e);
                        }
                    }
                    ds.tag_only = true;
                    ds.ty = Some(q);
                    continue;
                }
                "typename" => {
                    if ds.ty.is_some() || !ds.kw.is_empty() {
                        break;
                    }
                    self.pos += 1;
                    if self.at("decltype") {
                        // DEP-22: a nested name can start with a decltype
                        // specifier, which the decltype branch reads.
                        continue;
                    }
                    if self.tok().kind == TK::Ident && !is_keyword(&self.tok().text) && !self.at_k(1, "::") && !(self.at_k(1, "<") && self.at_k(self.skip_angle_at(1), "::")) {
                        // DEP-21, DEP-29: the name after `typename` is a
                        // qualified name.
                        self.error(t.line, "DEP-21", "the name after 'typename' is not a qualified name");
                    }
                    self.ctad_tid = None;
                    let q = self.parse_typename_type()?;
                    ds.ctad = self.ctad_tid.take();
                    ds.ty = Some(q);
                    ds.named_type = true;
                    continue;
                }
                _ => {
                    if is_keyword(s) {
                        break;
                    }
                    if ds.ty.is_some() || !ds.kw.is_empty() || ds.auto.is_some() {
                        break;
                    }
                    // A constructor declarator, not a type.
                    if self.is_ctor_start(in_class) {
                        break;
                    }
                    if (itn && self.dependent_qualifier_at() && self.less_after_qualified(0)) || self.has_fact(self.pos, crate::templ::FACT_LESS) {
                        // DEP-20, DEP-24: a type-only position whose last
                        // name is followed by `<`.
                        let q = self.parse_typename_type()?;
                        ds.ty = Some(q);
                        ds.named_type = true;
                        continue;
                    }
                    if !self.qualified_names_type(0) {
                        if itn && self.dependent_qualifier_at() {
                            // DEP-20: a type-only position.
                            let q = self.parse_typename_type()?;
                            ds.ty = Some(q);
                            ds.named_type = true;
                            continue;
                        }
                        break;
                    }
                    // A concept before auto (AUTO-13).
                    if let Some((ctid, n)) = self.concept_at() {
                        let ns = self.pos;
                        self.pos += n;
                        let mut raw = Vec::new();
                        let mut lt = None;
                        if self.at("<") {
                            lt = Some(self.pos);
                            raw = self.parse_raw_targs()?;
                        }
                        let end = self.pos;
                        if self.at("auto") || (self.at("decltype") && self.at_k(1, "(") && self.at_k(2, "auto")) {
                            ds.concept_auto = true;
                            if !self.templs[ctid].params.first().is_some_and(|p| matches!(p.kind, crate::templ::TPK::Type)) {
                                // CPT-18: a type concept.
                                self.error(t.line, "CPT-18", format!("the concept '{}' is not a type concept", self.templs[ctid].name));
                            } else {
                                ds.cons_auto = Some((ctid, raw, (ns, ns + n, lt, end)));
                            }
                            continue;
                        }
                        // CPT-18: a concept name before a declarator needs
                        // `auto`.
                        self.error(t.line, "CPT-18", "a concept name is used as a type without 'auto'");
                        ds.ty = Some(QTy::unknown());
                        continue;
                    }
                    self.ctad_tid = None;
                    let q = self.parse_type_name()?;
                    ds.ctad = self.ctad_tid.take();
                    if self.at("auto") || self.at("decltype") {
                        ds.concept_auto = true;
                        continue;
                    }
                    ds.ty = Some(q);
                    ds.named_type = true;
                    continue;
                }
            }
            self.pos += 1;
        }
        ds.align = self.last_align;
        ds.has_alignas = self.saw_alignas;
        if !ds.kw.is_empty() {
            let base = self.resolve_kw(&ds.kw, ds.line)?;
            ds.ty = Some(QTy::new(base));
        }
        if ds.c > 1 || ds.v > 1 {
            // CV-2: the same cv-qualifier twice in one type.
            self.error(ds.line, "CV-2", "duplicate cv-qualifier");
        }
        if let Some(t) = &mut ds.ty {
            *t = add_cv(t, ds.c > 0, ds.v > 0);
        }
        if !ds.kw.is_empty() || ds.named_type {
            ds.tag_only = false;
        }
        Ok(ds)
    }

    fn resolve_kw(&mut self, kw: &[String], line: u32) -> PR<Ty> {
        let count = |s: &str| kw.iter().filter(|k| k.as_str() == s).count();
        let signed = count("signed");
        let unsigned = count("unsigned");
        let short = count("short");
        let long = count("long");
        let int = count("int");
        let others: Vec<&String> = kw.iter().filter(|k| !matches!(k.as_str(), "signed" | "unsigned" | "short" | "long" | "int")).collect();
        // TYP-7: the combinations of type specifiers that a program must not write.
        let bad = |p: &mut P| {
            p.error(line, "TYP-7", format!("invalid combination of type specifiers: {}", kw.join(" ")));
        };
        if signed + unsigned > 1 || (short > 0 && long > 0) || long > 2 || short > 1 || int > 1 || others.len() > 1 {
            bad(self);
            return Ok(Ty::Int);
        }
        if let Some(o) = others.first() {
            let o = o.as_str();
            let t = match o {
                "char" => {
                    if short + long + int > 0 {
                        bad(self);
                    }
                    if signed > 0 {
                        Ty::SChar
                    } else if unsigned > 0 {
                        Ty::UChar
                    } else {
                        Ty::Char
                    }
                }
                "double" if long == 1 && signed + unsigned + short + int == 0 => Ty::LDouble,
                _ => {
                    if signed + unsigned + short + long + int > 0 {
                        bad(self);
                    }
                    match o {
                        "void" => Ty::Void,
                        "bool" => Ty::Bool,
                        "char8_t" => Ty::Char8,
                        "char16_t" => Ty::Char16,
                        "char32_t" => Ty::Char32,
                        "wchar_t" => Ty::WChar,
                        "float" => Ty::Float,
                        "double" => Ty::Double,
                        _ => Ty::Int,
                    }
                }
            };
            return Ok(t);
        }
        Ok(match (unsigned > 0, short, long) {
            (false, 1, _) => Ty::Short,
            (true, 1, _) => Ty::UShort,
            (false, 0, 1) => Ty::Long,
            (true, 0, 1) => Ty::ULong,
            (false, 0, 2) => Ty::LLong,
            (true, 0, 2) => Ty::ULLong,
            (true, _, _) => Ty::UInt,
            _ => Ty::Int,
        })
    }

    pub fn is_ctor_start(&self, in_class: Option<usize>) -> bool {
        // `C(` inside class C, or `C::C(` / `C::~C(` outside.
        if let Some(c) = in_class {
            let name = &self.classes[c].name;
            if self.tok().text == *name && self.at_k(1, "(") {
                return true;
            }
        }
        let mut k = 0;
        if self.at_k(0, "::") {
            k = 1;
        }
        let mut prev: Option<String> = None;
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident {
                return false;
            }
            let name = t.text.clone();
            let mut next = k + 1;
            if self.at_k(next, "<") {
                if let Some(ids) = self.lookup(&name) {
                    if matches!(self.decls[ids[0]].k, DK::Template(_)) {
                        next = self.skip_angle_at(next);
                    }
                }
            }
            if self.at_k(next, "::") {
                if self.at_k(next + 1, "~") {
                    return true;
                }
                prev = Some(name);
                k = next + 1;
                continue;
            }
            return prev.as_deref() == Some(name.as_str()) && self.at_k(next, "(");
        }
    }

    /// `C::operator` or `N::C<A>::operator`: the qualified name of an
    /// operator or conversion function defined outside its class.
    pub fn at_qualified_operator(&self) -> bool {
        let mut k = 0;
        if self.at_k(0, "::") {
            k = 1;
        }
        let mut seen = false;
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident {
                return false;
            }
            if t.text == "operator" {
                return seen;
            }
            if is_keyword(&t.text) {
                return false;
            }
            let mut next = k + 1;
            if self.at_k(next, "<") {
                next = self.skip_angle_at(next);
            }
            if !self.at_k(next, "::") {
                return false;
            }
            seen = true;
            k = next + 1;
        }
    }

    pub fn skip_qualified_name(&mut self) -> PR<()> {
        self.eat("::");
        loop {
            if self.at("template") {
                self.pos += 1;
            }
            self.ident()?;
            if self.at("<") {
                self.skip_balanced();
            }
            if !self.eat("::") {
                return Ok(());
            }
        }
    }

    /// Parse a qualifier `A::B::` and return the scope it names, if any.
    /// Leaves the position at the final name. Returns Err(()) if the
    /// qualifier names something unknown.
    pub fn parse_nested_name(&mut self) -> PR<Option<Result<usize, ()>>> {
        let mut scope: Option<Result<usize, ()>> = None;
        if self.at("::") && self.peek(1).kind == TK::Ident {
            self.pos += 1;
            scope = Some(Ok(self.global));
        }
        loop {
            if self.at("template") && self.peek(1).kind == TK::Ident {
                self.pos += 1;
            }
            if !(self.tok().kind == TK::Ident && !is_keyword(&self.tok().text)) {
                return Ok(scope);
            }
            // Is this name followed by `::` (possibly after template args)?
            let name = self.tok().text.clone();
            // NAM-15: the name before `::` finds only namespaces and types.
            let found = match scope {
                None => self.lookup_scope_name(&name),
                Some(Ok(s)) => self.lookup_in_scope(s, &name, true).map(|v| self.scope_names_only(v)).filter(|v| !v.is_empty()),
                Some(Err(())) => None,
            };
            let mut next = 1;
            let is_templ = found.as_ref().is_some_and(|ids| self.names_template(ids[0]) && !self.func(ids[0]).is_some());
            if self.at_k(1, "<") && (is_templ || matches!(scope, Some(Err(())))) {
                next = self.skip_angle_at(1);
            }
            if !self.at_k(next, "::") || self.peek(next + 1).kind != TK::Ident && !self.at_k(next + 1, "~") && !self.at_k(next + 1, "template") && !self.at_k(next + 1, "operator") {
                return Ok(scope);
            }
            if is_templ && next > 1 && self.tname_of(found.as_ref().unwrap()[0]).is_some() {
                // A template-id before `::` names a class (TSP-13).
                let id = found.as_ref().unwrap()[0];
                let line = self.line();
                self.pos += 1;
                self.last_qual_templ = None;
                let q = self.parse_template_id_type(id, line)?;
                self.expect("::")?;
                let s = match &q.t {
                    Ty::Class(c) => {
                        let c = *c;
                        self.require_class(c, line);
                        Ok(self.classes[c].scope)
                    }
                    Ty::Dep(d) => {
                        if let Dep::Id(TName::Templ(t), pats) = &**d {
                            self.last_qual_templ = Some((*t, pats.clone()));
                        }
                        Err(())
                    }
                    _ => Err(()),
                };
                scope = Some(s);
                continue;
            }
            if let Some(ids) = &found {
                let c = match &self.decls[ids[0]].k {
                    DK::Class(c) => Some(*c),
                    DK::Type(q) => match q.t {
                        Ty::Class(c) => Some(c),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(c) = c {
                    let l = self.line();
                    self.require_class(c, l);
                }
            }
            // PACK-21: a pack of types before `::` gives the element of
            // the current expansion.
            let found = match found {
                Some(ids) if matches!(self.decls[ids[0]].k, DK::Type(_)) && self.pack_binds.contains_key(&ids[0]) => {
                    let v = self.pack_binds[&ids[0]].clone();
                    self.last_pack_ref = Some(ids[0]);
                    self.pack_refs.push(v.len());
                    match v.get(self.pack_index.unwrap_or(0)) {
                        Some(TArg::Type(QTy { t: Ty::Class(c), .. })) => {
                            let c = *c;
                            let l = self.line();
                            self.require_class(c, l);
                            scope = Some(Ok(self.classes[c].scope));
                            self.pos += next + 1;
                            continue;
                        }
                        _ => Some(ids),
                    }
                }
                f => f,
            };
            let s = match found {
                Some(ids) => match &self.decls[ids[0]].k {
                    DK::Namespace(s) => Ok(*s),
                    DK::Class(c) => {
                        if self.classes[*c].template && next > 1 {
                            Err(())
                        } else {
                            Ok(self.classes[*c].scope)
                        }
                    }
                    DK::Enum(e) => Ok(self.enums[*e].scope),
                    DK::Type(q) => match &q.t {
                        Ty::Class(c) => Ok(self.classes[*c].scope),
                        Ty::Enum(e) => Ok(self.enums[*e].scope),
                        Ty::Unknown | Ty::Dep(_) => Err(()),
                        _ if self.at_k(next + 1, "~") && self.at_k(next + 2, &name) => {
                            // NAM-16: in a destructor name the qualifier can
                            // name a scalar type.
                            return Ok(scope);
                        }
                        _ => {
                            if !self.in_template() {
                                // NAM-16: a fundamental type is not a qualifier.
                                self.error(self.line(), "NAM-16", format!("'{}' names a type that is not a class, enumeration, or namespace, and cannot come before '::'", name));
                            }
                            Err(())
                        }
                    },
                    _ => Err(()),
                },
                None => {
                    let certain = match scope {
                        None => !self.lookup_uncertain(),
                        Some(Ok(s)) => !self.scopes[s].uncertain && !matches!(self.scopes[s].kind, SK::Class(c) if self.class_uncertain(c)) && !self.in_template(),
                        Some(Err(())) => false,
                    };
                    let hidden_by_value = scope.is_none() && self.lookup(&name).is_some();
                    if certain && hidden_by_value {
                        // NAM-16: only a variable or function has the name.
                        self.error(self.line(), "NAM-16", format!("'{}' before '::' is not a namespace, class, or enumeration", name));
                    }
                    Err(())
                }
            };
            self.pos += next + 1;
            scope = Some(s);
        }
    }

    /// NAM-15: unqualified lookup of a name before `::`, which skips
    /// declarations other than namespaces and types.
    pub fn lookup_scope_name(&self, name: &str) -> Option<Vec<DeclId>> {
        let noms = self.nominations();
        for &s in self.stack.iter().rev() {
            // NAM-40: a name before `::` finds a class that a variable
            // hides.
            if let (Some(v), _) = self.lookup_level_k(s, name, &noms, false) {
                let v = self.scope_names_only(v);
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }
        None
    }
    pub fn scope_names_only(&self, v: Vec<DeclId>) -> Vec<DeclId> {
        v.into_iter().filter(|&i| matches!(self.decls[i].k, DK::Namespace(_) | DK::Class(_) | DK::Enum(_) | DK::Type(_) | DK::Template(_) | DK::TypeParam)).collect()
    }

    /// Parse a type name (possibly qualified, possibly a template-id)
    /// that is known to name a type.
    pub fn parse_type_name(&mut self) -> PR<QTy> {
        let line = self.line();
        let scope = self.parse_nested_name()?;
        let name = self.ident()?;
        let (found, amb) = match scope {
            None => self.lookup_full(&name),
            Some(Ok(s)) => self.lookup_qualified(s, &name),
            Some(Err(())) => {
                if self.at("<") {
                    self.skip_balanced();
                }
                return Ok(QTy::unknown());
            }
        };
        let amb = amb && !(self.at("<") && found.as_ref().is_some_and(|ids| {
            // TARG-11: with an argument list, the injected class names of
            // one template's specializations name the template.
            // A constructor shares the class's name and is no type.
            let ts: Vec<Option<usize>> = ids.iter().filter(|&&i| !matches!(self.decls[i].k, DK::Func(_))).map(|&i| self.tid_of(i)).collect();
            !ts.is_empty() && ts.iter().all(|t| t.is_some() && *t == ts[0])
        }));
        if !self.in_template() {
            if amb {
                if scope.is_none() && self.stack.iter().any(|&s| matches!(self.scopes[s].kind, SK::Class(_))) {
                    // NAM-36, DER-4: an inherited name from two bases.
                    self.error_because(line, "NAM-36", "DER-4", format!("'{}' names different classes in two bases", name));
                } else {
                    self.report_ambiguous(scope.and_then(|r| r.ok()), &name, line);
                }
                return Ok(QTy::unknown());
            }
            if let (Some(Ok(_)), Some(ids), 0) = (scope, &found, self.in_base_clause) {
                if !self.at("::") && self.hides_class(ids) {
                    // NAM-41: the qualified name finds the variable, data
                    // member, function, or enumerator, which hides the class.
                    self.error(line, "NAM-41", format!("'{}' names a member that is not a type, which hides the class", name));
                    return Ok(QTy::unknown());
                }
            }
            if let Some(Ok(s)) = scope {
                if let SK::Class(c) = self.scopes[s].kind {
                    if self.classes[c].name == name && !self.classes[c].template && self.in_base_clause == 0 {
                        // NAM-34: after `C::`, `C` names the constructors.
                        self.error(line, "NAM-34", format!("'{0}::{0}' names the constructors of '{0}', not a type", name));
                        return Ok(QTy::unknown());
                    }
                }
            }
            if let (Some(Ok(s)), Some(ids)) = (scope, &found) {
                if let (SK::Class(_), Some(&id)) = (&self.scopes[s].kind, ids.iter().find(|&&i| matches!(self.decls[i].k, DK::Type(_) | DK::Class(_) | DK::Enum(_)))) {
                    // ACC-2, ACC-4: a member type named outside its class.
                    // The declaration of a member defined after its class
                    // (a qualified declarator next) checks access in the
                    // class (ACC-11).
                    let mut k = 0;
                    while matches!(self.peek(k).text.as_str(), "*" | "&" | "&&" | "const" | "volatile") {
                        k += 1;
                    }
                    let member_def = self.peek(k).kind == TK::Ident && self.peek(k + 1).text == "::";
                    let c = match self.scopes[s].kind {
                        SK::Class(c) => c,
                        _ => unreachable!(),
                    };
                    if !member_def && self.in_base_clause == 0 && self.access_free == 0 && !self.named_by_using(c, &name, id, 0) && !self.accessible(id, false) {
                        let rule = if self.member_access.get(&id) == Some(&1) { "ACC-4" } else { "ACC-2" };
                        self.error(line, rule, format!("'{}' is not accessible here", name));
                    }
                }
            }
            if scope.is_none() && self.private_base_name(&name) {
                // NAM-36, ACC-14: the name is a member of a private base
                // of another class.
                self.error_because(line, "NAM-36", "ACC-14", format!("'{}' is inherited through a private base and is not accessible here", name));
            }
        }
        let Some(ids) = found else {
            return Err(PErr { line, msg: format!("unknown type name '{}'", name) });
        };
        let id = *ids.iter().find(|&&i| matches!(self.decls[i].k, DK::Type(_) | DK::Class(_) | DK::Enum(_) | DK::TypeParam | DK::Template(_))).unwrap_or(&ids[0]);
        if self.at("<") && self.names_template(id) && !matches!(self.decls[id].k, DK::Template(_)) {
            return self.parse_template_id_type(id, line);
        }
        if self.at("<") && matches!(self.decls[id].k, DK::Template(_)) {
            if name == "initializer_list" && self.scopes[self.decls[id].scope].name == "std" && !self.in_template() {
                // LSUP-1: std::initializer_list<E> becomes a class that
                // the front end knows by its element type (ICS-16).
                let save = self.pos;
                self.pos += 1;
                let el = self.parse_type_id();
                if let Ok(el) = el {
                    if self.eat(">") && !el.is_unknown() {
                        return Ok(QTy::new(Ty::Class(self.init_list_class(el, line))));
                    }
                }
                self.pos = save;
            }
            return self.parse_template_id_type(id, line);
        }
        if let DK::Template(TemplKind::Class(_) | TemplKind::Alias) = &self.decls[id].k {
            // TARG-10: a template name needs an argument list. A
            // variable declared with the name and an initializer deduces
            // the arguments, which the front end does not model.
            let ctad = self.peek(0).kind == TK::Ident && !is_keyword(&self.peek(0).text) && (self.at_k(1, "{") || self.at_k(1, "(") || self.at_k(1, "=") || self.at_k(1, ";") || self.at_k(1, ":") || self.at_k(1, ","));
            // TPL-11: the type of a constant template parameter.
            let ctad = ctad || self.at("{") || self.at("(") || self.in_new_type || self.in_tparam_decl;
            if !ctad && !self.in_tparam_decl && (!self.in_template() || scope.is_some()) && !self.tparam.contains_key(&id) && self.tid_of(id).is_some() {
                self.error(line, "TARG-10", format!("the template '{}' is used without an argument list", name));
            }
            if ctad && matches!(self.decls[id].k, DK::Template(TemplKind::Class(_) | TemplKind::Alias)) && !self.tparam.contains_key(&id) {
                self.ctad_tid = self.tid_of(id);
            }
            return Ok(QTy::unknown());
        }
        if self.binding_scopes.contains(&self.decls[id].scope) {
            self.binding_hits += 1;
        }
        Ok(match &self.decls[id].k {
            DK::Type(q) => {
                if let Some(v) = self.pack_binds.get(&id) {
                    self.last_pack_ref = Some(id);
                    self.pack_refs.push(v.len());
                    let k = self.pack_index.unwrap_or(0);
                    match v.get(k) {
                        Some(TArg::Type(t)) => return Ok(t.clone()),
                        _ => return Ok(QTy::unknown()),
                    }
                }
                q.clone()
            }
            DK::Class(c) => {
                if self.classes[*c].template {
                    QTy::unknown()
                } else {
                    QTy::new(Ty::Class(*c))
                }
            }
            DK::Enum(e) => QTy::new(Ty::Enum(*e)),
            DK::TypeParam => {
                if self.in_template() {
                    self.last_pack_ref = Some(id);
                    let p = self.pos.saturating_sub(1);
                    self.note_pack_use(id, p, line);
                }
                self.tparam_type(id)
            }
            _ => QTy::unknown(),
        })
    }

    /// A template-id that names a type, at `<` after the name `id`
    /// (TARG-1, TSP-13).
    pub fn parse_template_id_type(&mut self, id: DeclId, line: u32) -> PR<QTy> {
        use crate::templ::RawArg;
        let tn = self.tname_of(id);
        let mut raw = self.parse_raw_targs()?;
        if let Some(inner) = self.ttp_inner.get(&id).cloned() {
            // TPL-40: the defaults of the template template parameter's
            // own head fill the list.
            for p in inner.iter().skip(raw.len()) {
                let Some(toks) = p.default.clone() else { break };
                let src = crate::templ::Source { toks, stack: self.stack.clone(), ctx: crate::decl2::Ctx::Namespace, access: 0, limit: self.lookup_limit, vid: usize::MAX, base: 0 };
                let r = self.replay(&src, &[], &[], None, |q| q.parse_raw_targ().ok());
                match r {
                    Some((Some(a), _)) => raw.push(a),
                    _ => break,
                }
            }
        }
        let dependent = raw.iter().any(|r| match r {
            RawArg::Type(q) => q.is_unknown(),
            RawArg::Expr(e) => (e.ty.is_unknown() && !matches!(e.k, EK::InitList(_))) || crate::templ::has_unknown(e),
            RawArg::Templ(TName::Param(..)) => true,
            RawArg::Templ(_) => false,
            RawArg::Expand(_) => true,
        });
        match tn {
            Some(TName::Templ(tid)) => {
                if !matches!(self.templs[tid].kind, crate::templ::TKind::Class | crate::templ::TKind::Alias) {
                    return Ok(QTy::unknown());
                }
                if self.templs[tid].kind == crate::templ::TKind::Alias {
                    self.check_expansion_to_nonpack(tid, &raw, line);
                }
                if self.in_template() || dependent {
                    match self.pattern_targs(tid, raw) {
                        Some(args) => Ok(self.class_ref(tid, args, line)),
                        None => Ok(QTy::unknown()),
                    }
                } else {
                    match self.targs_for(tid, raw, line, false) {
                        Ok(Some(args)) => Ok(self.class_ref(tid, args, line)),
                        _ => Ok(QTy::unknown()),
                    }
                }
            }
            Some(TName::Param(d, i)) => {
                if let Some(inner) = self.ttp_inner.get(&id) {
                    let need = inner.iter().filter(|p| p.default.is_none() && !p.pack).count();
                    let has_pack = inner.iter().any(|p| p.pack);
                    if raw.len() < need || (raw.len() > inner.len() && !has_pack) {
                        // TARG-6: the arguments fit the parameter's own head
                        // (TPL-40).
                        self.error(line, "TARG-6", "the arguments do not fit the head of the template template parameter");
                    }
                }
                let mut args = Vec::new();
                for r in raw {
                    match r {
                        RawArg::Type(q) => args.push(TArg::Type(q)),
                        RawArg::Templ(n) => args.push(TArg::Templ(n)),
                        _ => args.push(TArg::Val(TVal::Opaque)),
                    }
                }
                Ok(crate::templ::dep(Dep::Id(TName::Param(d, i), args)))
            }
            None => Ok(QTy::unknown()),
        }
    }

    /// `typename` followed by a qualified name (a dependent type in a
    /// template, or a member type in a specialization).
    pub fn parse_typename_type(&mut self) -> PR<QTy> {
        let line = self.line();
        let save = self.pos;
        let mark = self.d.list.len();
        let scope = self.parse_nested_name();
        if let Ok(Some(Ok(s))) = scope {
            let kw = self.eat("template");
            let name = self.ident()?;
            let mut found = self.lookup_qualified(s, &name).0;
            if !self.in_template() && !self.at("::") && found.as_ref().is_some_and(|ids| self.hides_class(ids)) {
                // NAM-41, DEP-22: the qualified name finds the member that
                // hides the class, which is not a type.
                found = None;
            }
            if let Some(ids) = found {
                if let Some(&id) = ids.iter().find(|&&i| matches!(self.decls[i].k, DK::Type(_) | DK::Class(_) | DK::Enum(_) | DK::Template(_) | DK::TypeParam)) {
                    if !self.in_template() && !self.accessible(id, false) {
                        // DED-26: access is checked in substitution too.
                        self.error(line, "ACC-2", format!("'{}' is not accessible here", name));
                    }
                    if !kw && self.at("<") && self.names_template(id) && self.has_fact(save, crate::templ::FACT_LESS) {
                        // DEP-24: the pattern read `<` after the name as the
                        // less-than operator.
                        self.error(line, "DEP-24", format!("the template reads '<' after '{}' as the less-than operator; the member template needs 'template'", name));
                        self.skip_template_args()?;
                        return Ok(QTy::unknown());
                    }
                    if self.at("<") && self.names_template(id) {
                        return self.parse_template_id_type(id, line);
                    }
                    if matches!(self.decls[id].k, DK::Template(TemplKind::Class(_) | TemplKind::Alias)) {
                        // CTAD-1: a member class template named without
                        // arguments deduces them.
                        self.ctad_tid = self.tid_of(id);
                        return Ok(QTy::unknown());
                    }
                    return Ok(match &self.decls[id].k {
                        DK::Type(q) => q.clone(),
                        DK::Class(c) if !self.classes[*c].template => QTy::new(Ty::Class(*c)),
                        DK::Enum(e) => QTy::new(Ty::Enum(*e)),
                        _ => QTy::unknown(),
                    });
                }
            }
            if self.at("<") {
                self.skip_template_args()?;
            }
            let certain = match self.scopes[s].kind {
                // DED-24: an incomplete class has no members to find,
                // unless its own definition is being read.
                SK::Class(c) => !self.class_uncertain(c) && (self.classes[c].complete || (!self.stack.contains(&s) && !self.inst_busy.contains(&c) && !self.inst_of.contains_key(&c))),
                _ => !self.scopes[s].uncertain,
            };
            if certain && !self.in_template() {
                self.error(line, "NAM-9", format!("no type named '{}' in the named scope", name));
            }
            return Ok(QTy::unknown());
        }
        self.pos = save;
        if self.in_template() {
            self.d.list.truncate(mark);
        }
        self.skip_qualified_name()?;
        if self.in_template() {
            // DEP-24: a `<` after a name that follows `::` without
            // `template`.
            let toks = &self.toks[save..self.pos];
            let less = (2..toks.len()).any(|i| toks[i].text == "<" && toks[i - 1].kind == TK::Ident && toks[i - 2].text == "::");
            if less {
                self.note_fact(save, crate::templ::FACT_LESS);
            }
        }
        Ok(if self.in_template() { crate::templ::dep(Dep::Opaque) } else { QTy::unknown() })
    }

    /// The class for `std::initializer_list<E>`. The front end does not
    /// model its members, so the class is opaque.
    pub fn init_list_class(&mut self, el: QTy, line: u32) -> usize {
        if let Some(x) = self.init_list_classes.iter().find(|x| x.0 == el) {
            return x.1;
        }
        // LSUP-1: a program that declares std::initializer_list with the
        // shape the language needs gets a specialization of its template.
        if self.init_list_shape_ok() && self.inst_depth < 64 {
            let tid = self.lookup_in_scope(self.global, "std", false).and_then(|ns| match self.decls[ns[0]].k {
                DK::Namespace(s) => self.lookup_in_scope(s, "initializer_list", false),
                _ => None,
            }).and_then(|ids| ids.iter().find_map(|&i| self.tid_of(i)));
            if let Some(tid) = tid {
                let q = self.class_inst(tid, vec![TArg::Type(el.clone())], line);
                if let Ty::Class(c) = q.t {
                    self.init_list_classes.push((el, c));
                    self.require_class(c, line);
                    return c;
                }
            }
        }
        let parent = self.global;
        let scope = self.new_scope(SK::Class(self.classes.len()), Some(parent), "initializer_list");
        self.classes.push(ClassInfo {
            name: "initializer_list".into(),
            key: 1,
            scope,
            complete: true,
            fields: vec![],
            bases: vec![],
            has_conv: false,
            conv_targets: vec![],
            ctors: vec![],
            dtor: None,
            has_virtual: false,
            template: false,
            opaque: true,
            line,
            align: 8,
            anon_unions: vec![],
        });
        let c = self.classes.len() - 1;
        self.init_list_classes.push((el, c));
        c
    }

    pub fn skip_template_args(&mut self) -> PR<()> {
        // Parse `<` args `>` loosely: each argument is a type-id or an
        // expression. Only the brackets matter here.
        self.expect("<")?;
        let mut depth = 1;
        loop {
            let t = self.tok().clone();
            if t.kind == TK::Eof {
                return Err(self.err("unterminated template argument list"));
            }
            if t.kind == TK::Ident && self.at_k(1, "::") && self.peek(2).text == t.text && !self.at_k(3, "::") && !self.at_k(3, "(") && !self.in_template() {
                let is_class = self.lookup_scope_name(&t.text).is_some_and(|ids| ids.iter().any(|&i| matches!(self.decls[i].k, DK::Class(c) if !self.classes[c].template && self.classes[c].name == t.text)));
                if is_class && !(self.pos > 0 && self.toks[self.pos - 1].text == "::") {
                    // NAM-34: `C::C` names the constructors, not a type.
                    self.error(t.line, "NAM-34", format!("'{0}::{0}' names the constructors of '{0}', not a type", t.text));
                }
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "<" => depth += 1,
                    ">" => {
                        depth -= 1;
                        if depth == 0 {
                            self.pos += 1;
                            return Ok(());
                        }
                    }
                    ">>" => {
                        if depth == 1 {
                            self.expect(">")?;
                            return Ok(());
                        }
                        depth -= 2;
                        if depth == 0 {
                            self.pos += 1;
                            return Ok(());
                        }
                    }
                    "(" | "[" | "{" => {
                        self.skip_balanced();
                        continue;
                    }
                    ";" | "}" => return Err(self.err("unterminated template argument list")),
                    _ => {}
                }
            }
            self.pos += 1;
        }
    }

    /// `decltype(expr)` after the keyword (specification/expressions/decltype.md).
    pub fn parse_decltype(&mut self) -> PR<QTy> {
        self.expect("(")?;
        // An unparenthesized id-expression or member access gives the
        // declared type; anything else gives a type from the value
        // category (DTY rules).
        let parenthesized = self.at("(");
        self.in_unevaluated += 1;
        // A `>` inside the parentheses does not close a template argument
        // list.
        let save = self.no_gt;
        self.no_gt = 0;
        let e = self.parse_expression();
        self.no_gt = save;
        self.in_unevaluated -= 1;
        let e = e?;
        self.expect(")")?;
        // DED-2: a template-id that names one specialization.
        let e = if matches!(e.k, EK::TemplId(None, ..)) { { let l = e.line; self.resolve_templ_id(e, l) } } else { e };
        self.check_unevaluated_dtor(&e, true);
        self.warn_unevaluated_effect(&e);
        if let Some((fs, addr)) = Self::overload_set_of(&e) {
            if !self.in_template() && fs.iter().all(|&f| self.func(f).is_some_and(|f| !f.constrained && f.class.is_none_or(|c| !self.class_uncertain(c)))) {
                if addr {
                    // OVL-20: decltype gives no target type.
                    self.error(e.line, "OVL-20", format!("the address of the overload set '{}' has no target type", self.decls[fs[0]].name));
                } else {
                    // DTY-4: the name of an overloaded function.
                    self.error(e.line, "DTY-4", format!("decltype of the overloaded function name '{}'", self.decls[fs[0]].name));
                }
            }
        }
        let t = self.decltype_of(&e, parenthesized);
        if t.is_unknown() {
            let mut x = &e;
            while let EK::Conv(_, y) = &x.k {
                x = y;
            }
            let cause = match &x.k {
                EK::Var(id) => match self.var(*id).and_then(|v| v.init.clone()) {
                    Some(init) => crate::templ::first_unknown(&init).unwrap_or_else(|| format!("the type of '{}', which its initializer does not give", self.decls[*id].name)),
                    None => format!("the type of '{}', which the front end does not know", self.decls[*id].name),
                },
                EK::TemplId(..) => "a template-id that names a function specialization".into(),
                _ => crate::templ::first_unknown(&e).unwrap_or_else(|| "decltype of an expression of unknown type".into()),
            };
            self.unknown_cause = Some(cause);
        }
        Ok(t)
    }

    pub fn decltype_of(&self, e: &Expr, parenthesized: bool) -> QTy {
        if !parenthesized {
            match &e.k {
                EK::Var(id) => {
                    if let DK::Var(v) = &self.decls[*id].k {
                        return v.ty.clone();
                    }
                }
                EK::Member(_, id) => {
                    if let DK::Field(f) = &self.decls[*id].k {
                        return f.ty.clone();
                    }
                    if let DK::Var(v) = &self.decls[*id].k {
                        return v.ty.clone();
                    }
                }
                EK::Func(id) => {
                    if let DK::Func(f) = &self.decls[*id].k {
                        return QTy::new(Ty::Func(Box::new(f.fty.clone())));
                    }
                }
                _ => {}
            }
        }
        if e.ty.is_unknown() {
            return QTy::unknown();
        }
        match e.cat {
            Cat::L => QTy::new(Ty::LRef(Box::new(e.ty.clone()))),
            Cat::X => QTy::new(Ty::RRef(Box::new(e.ty.clone()))),
            Cat::PR => e.ty.clone(),
        }
    }

    /// type-id: specifiers and an abstract declarator.
    pub fn parse_type_id(&mut self) -> PR<QTy> {
        let ds = self.parse_decl_specs(None)?;
        let base = self.ds_base(&ds)?;
        let d = self.parse_declarator(true, false)?;
        if d.name.is_some() {
            return Err(self.err("unexpected name in type-id"));
        }
        self.check_no_defaults_in(&d, d.line);
        Ok(self.apply_ops(base, &d.ops, d.line))
    }

    pub fn ds_base(&mut self, ds: &DeclSpec) -> PR<QTy> {
        if let Some(t) = &ds.ty {
            return Ok(t.clone());
        }
        if ds.auto.is_some() {
            let mut q = QTy::unknown();
            q.c = ds.c > 0;
            q.v = ds.v > 0;
            return Ok(q);
        }
        Err(PErr { line: ds.line, msg: "declaration has no type specifier".into() })
    }

    /// Apply declarator operations to a base type (DCL-1).
    pub fn apply_ops(&mut self, base: QTy, ops: &[DOp], line: u32) -> QTy {
        let mut t = base;
        for (oi, op) in ops.iter().enumerate() {
            if let DOp::Func(f) = op {
                if f.noexcept_dep && f.noexcept_param.is_none() && oi + 1 < ops.len() {
                    // DEP-9: a function type with a dependent noexcept
                    // inside another type, as in a pointer to it.
                    t = crate::templ::dep(Dep::Opaque);
                    continue;
                }
            }
            t = match op {
                DOp::Ptr(c, v) => {
                    if t.t.is_ref() {
                        // DCL-2: no pointer to a reference.
                        self.error(line, "DCL-17", "pointer to a reference");
                    }
                    if let Ty::Func(f) = &t.t {
                        if f.cv_c || f.cv_v || f.refq != 0 {
                            self.error(line, "DCL-16", "pointer to a function type with qualifiers");
                        }
                    }
                    QTy { t: Ty::Ptr(Box::new(t)), c: *c, v: *v }
                }
                DOp::LRef | DOp::RRef => {
                    let is_l = matches!(op, DOp::LRef);
                    if oi > 0 && matches!(ops[oi - 1], DOp::LRef | DOp::RRef) {
                        // DCL-18: no reference to a reference written in
                        // the declarator.
                        self.error(line, "DCL-18", "reference to a reference");
                    }
                    if t.t == Ty::Void {
                        // DCL-3: no reference to void.
                        self.error(line, "DCL-18", "reference to void");
                    }
                    if let Ty::Func(f) = &t.t {
                        if f.cv_c || f.cv_v || f.refq != 0 {
                            self.error(line, "DCL-16", "reference to a function type with qualifiers");
                        }
                    }
                    match &t.t {
                        // DCL-4: reference collapsing through an alias.
                        Ty::LRef(_) => t,
                        Ty::RRef(i) => {
                            if is_l {
                                QTy::new(Ty::LRef(i.clone()))
                            } else {
                                t
                            }
                        }
                        _ => QTy::new(if is_l { Ty::LRef(Box::new(t)) } else { Ty::RRef(Box::new(t)) }),
                    }
                }
                DOp::MemPtrDep(cls, cc, vv) => {
                    let mut q = crate::templ::dep(Dep::MemPtr(cls.clone(), t));
                    q.c = *cc;
                    q.v = *vv;
                    q
                }
                DOp::MemPtr(c, cc, vv) => {
                    if t.t.is_ref() {
                        self.error(line, "DCL-23", "pointer to member of reference type");
                    }
                    if t.t == Ty::Void {
                        // MPTR-8: no pointer to a member of type void.
                        self.error(line, "MPTR-8", "pointer to member of type void");
                    }
                    match c {
                        Some(c) => QTy { t: Ty::MemPtr(*c, Box::new(t)), c: *cc, v: *vv },
                        None => QTy::unknown(),
                    }
                }
                DOp::Array(n) => {
                    match &t.t {
                        Ty::Void => self.error(line, "DCL-20", "array of void"),
                        Ty::Func(_) => self.error(line, "DCL-20", "array of functions"),
                        Ty::LRef(_) | Ty::RRef(_) => self.error(line, "DCL-18", "array of references"),
                        Ty::Array(_, None) => self.error(line, "DCL-20", "only the first array bound can be omitted"),
                        Ty::Class(c) if !self.classes[*c].complete && !self.in_template() => {
                            // INST-1: an array element needs the class
                            // complete, which instantiates a specialization.
                            let c = *c;
                            self.require_class(c, line);
                            if !self.classes[c].complete && !self.class_uncertain(c) {
                                self.error(line, "DCL-20", "array of incomplete class type")
                            }
                        }
                        _ => {}
                    }
                    if let Some((d, i)) = n.and_then(arr_param_decode) {
                        let (c, v) = (t.c, t.v);
                        let mut q = crate::templ::dep(Dep::Array(t, TVal::Param(d, i)));
                        q.c = c;
                        q.v = v;
                        q
                    } else if *n == Some(u64::MAX) {
                        QTy::unknown()
                    } else {
                        // CV-6: cv on an array applies to the elements.
                        let (c, v) = (t.c, t.v);
                        let at = QTy { t: Ty::Array(Box::new(t), *n), c, v };
                        let max: u128 = if crate::types::ilp32() { u32::MAX as u128 } else { u64::MAX as u128 };
                        if n.is_some() && !self.in_template() && self.bytes_at_least(&at.t, 0).is_some_and(|b| b > max) {
                            // STG-1, DCL-19: the size of the array in bytes
                            // fits in std::size_t.
                            self.error(line, "DCL-19", "the size of the array does not fit in std::size_t");
                        }
                        at
                    }
                }
                DOp::Func(f) => {
                    match &t.t {
                        // DCL-7: no function returning an array or a function.
                        Ty::Array(..) => self.error(line, "DCL-21", "function returning an array"),
                        Ty::Func(_) => self.error(line, "DCL-21", "function returning a function"),
                        _ => {}
                    }
                    if t.v && f.trailing.is_none() && self.inst_depth == 0 && !self.in_template() {
                        // CV-9: a volatile return type is deprecated.
                        self.warn(line, "CV-9", "a return type with a top-level 'volatile' is deprecated");
                    }
                    let ret = if let Some(tr) = &f.trailing {
                        if !(t.t == Ty::Unknown && !t.c) {
                            // DCL-7: a trailing return type needs `auto`.
                        }
                        tr.clone()
                    } else {
                        t
                    };
                    QTy::new(Ty::Func(Box::new(FnTy {
                        ret,
                        params: f.ptys.clone(),
                        variadic: f.variadic,
                        noexcept: f.noexcept,
                        cv_c: f.cv_c,
                        cv_v: f.cv_v,
                        refq: f.refq,
                        noexcept_param: f.noexcept_param,
                    })))
                }
            };
        }
        // An array type through an alias: cv applies to elements.
        t
    }

    /// Parse a declarator (DCL-1). `abstract_ok`: the name can be absent.
    /// `param_ctx`: inside a parameter list.
    pub fn parse_declarator(&mut self, abstract_ok: bool, param_ctx: bool) -> PR<Declarator> {
        let line = self.line();
        let mut ptr_ops = Vec::new();
        loop {
            self.skip_attributes();
            if self.eat("*") {
                let (c, v) = self.parse_cv();
                ptr_ops.push(DOp::Ptr(c, v));
            } else if self.eat("&") {
                ptr_ops.push(DOp::LRef);
            } else if self.eat("&&") {
                ptr_ops.push(DOp::RRef);
            } else if self.is_member_ptr_start() {
                // `C::*` (MPTR grammar).
                let save = self.pos;
                let scope = self.parse_nested_name()?;
                let cname = self.ident();
                let mut dep_class: Option<QTy> = None;
                let class = match (scope, cname) {
                    (None, Ok(n)) => self.lookup(&n).and_then(|ids| {
                        if self.tparam.contains_key(&ids[0]) && matches!(self.decls[ids[0]].k, DK::TypeParam) {
                            dep_class = Some(self.tparam_type(ids[0]));
                        }
                        self.class_of_decl(ids[0])
                    }),
                    (Some(Ok(s)), Ok(n)) => self.lookup_in_scope(s, &n, true).and_then(|ids| self.class_of_decl(ids[0])),
                    _ => None,
                };
                if !self.eat("::") || !self.eat("*") {
                    self.pos = save;
                    return Err(self.err("bad pointer-to-member declarator"));
                }
                let (c, v) = self.parse_cv();
                // Inside a member template of an instance (INST-6), a name
                // that the instance binds to a type that is not a class is
                // known too.
                if class.is_none() && dep_class.is_none() && (!self.in_template() || self.inst_depth > 0) {
                    // MPTR-8: the class of a pointer to member is a class.
                    let named = self.toks[save..self.pos].iter().find(|t| t.kind == TK::Ident).map(|t| t.text.clone());
                    if let Some(n) = named {
                        if let Some(ids) = self.lookup(&n) {
                            if let DK::Type(q) = &self.decls[ids[0]].k {
                                if !q.is_unknown() && !matches!(q.t, Ty::Class(_) | Ty::Dep(_)) && !q.t.contains_unknown() {
                                    self.error(line, "MPTR-8", format!("'{}' is not a class, so it has no pointer to member", n));
                                }
                            }
                        }
                    }
                }
                match dep_class {
                    Some(q) => ptr_ops.push(DOp::MemPtrDep(q, c, v)),
                    None => ptr_ops.push(DOp::MemPtr(class, c, v)),
                }
            } else {
                break;
            }
        }
        let mut d = Declarator {
            name: None,
            kind: NameKind::Plain,
            qual: None,
            qual_unknown: false,
            pattern_scope: None,
            ops: Vec::new(),
            line,
            conv_ty: None,
            template_args: false,
            pack: false,
        };
        let mut inner_ops: Vec<DOp> = Vec::new();
        if self.eat("...") {
            d.pack = true;
        }
        // direct declarator
        if self.at("(") && self.paren_is_grouping(abstract_ok, param_ctx) {
            self.pos += 1;
            let inner = self.parse_declarator(abstract_ok, param_ctx)?;
            self.expect(")")?;
            d.name = inner.name;
            d.kind = inner.kind;
            d.qual = inner.qual;
            d.qual_unknown = inner.qual_unknown;
            d.conv_ty = inner.conv_ty;
            d.line = inner.line;
            // PACK-1: `...` can come inside the parentheses, as in
            // `R (*... fp)()`. PACK-2: not for a pack with no name.
            if inner.pack && d.name.is_none() && self.inst_depth == 0 {
                self.error(inner.line, "PACK-2", "a pack with no name has its '...' inside parentheses");
            }
            d.pack |= inner.pack;
            inner_ops = inner.ops;
        } else if self.is_ident() || self.at("::") || self.at("~") || self.at("operator") {
            self.parse_declarator_name(&mut d)?;
        } else if !abstract_ok {
            return Err(self.err(format!("expected a declarator, found '{}'", self.tok_desc())));
        }
        self.skip_attributes();
        // NAM-21: after a qualified name, the parameters and a trailing
        // return type see the members of the named scope.
        let depth = self.stack.len();
        if let Some(q) = d.qual {
            if !self.stack.contains(&q) {
                self.push_enclosing_for(q);
            }
        } else if d.qual_unknown && self.in_template() {
            // NAM-21: a member of a class template defined outside the
            // class sees the members of the pattern class.
            if let Some(crate::templ::Last::Member(t, _)) = &self.last_templ {
                if let DK::Template(TemplKind::Class(pc)) = self.decls[self.templs[*t].decl].k {
                    if pc != usize::MAX {
                        let cs = self.classes[pc].scope;
                        d.pattern_scope = Some(cs);
                        if !self.stack.contains(&cs) {
                            self.stack.push(cs);
                        }
                    }
                }
            }
        }
        let r = self.declarator_suffixes(&d, &inner_ops, abstract_ok, param_ctx);
        self.stack.truncate(depth);
        let suffixes = r?;
        // ops: pointers apply first, then suffixes from the innermost
        // (rightmost) outward, then the parenthesized inner declarator.
        let mut ops = ptr_ops;
        for s in suffixes.into_iter().rev() {
            ops.push(s);
        }
        ops.extend(inner_ops);
        d.ops = ops;
        Ok(d)
    }

    fn declarator_suffixes(&mut self, d: &Declarator, inner_ops: &[DOp], abstract_ok: bool, param_ctx: bool) -> PR<Vec<DOp>> {
        let mut suffixes = Vec::new();
        loop {
            if self.at("[") {
                if self.at_k(1, "[") {
                    self.skip_attributes();
                    continue;
                }
                self.pos += 1;
                if self.eat("]") {
                    suffixes.push(DOp::Array(None));
                } else {
                    let bl = self.line();
                    let e = self.parse_assignment()?;
                    self.expect("]")?;
                    // A bound that is a constant parameter (DED-P3).
                    let pb = match &e.k {
                        EK::Unknown(w) if w.starts_with("template parameter#") => w["template parameter#".len()..].parse::<usize>().ok().and_then(|id| self.tparam.get(&id).copied()),
                        _ => None,
                    };
                    let n = match pb {
                        Some((d, i)) => Some(arr_param_encode(d, i)),
                        None => self.array_bound(&e, bl),
                    };
                    suffixes.push(DOp::Array(n));
                }
            } else if self.at("(") && (d.name.is_some() || inner_ops.iter().any(|o| !matches!(o, DOp::Func(_))) || !inner_ops.is_empty() || abstract_ok) {
                // A parenthesized initializer looks like this too; the
                // caller decides with `looks_like_params` before calling.
                let tn = d.qual.is_some() || d.qual_unknown || matches!(self.scopes[self.cur_scope()].kind, SK::Class(_));
                // DEP-19, DEP-29: `f(T::name) {` at namespace scope is a
                // variable template with an initializer followed by a body,
                // a syntax error at the definition. The front end reports
                // it and goes on as if the declaration were a function.
                let value_param = d.name.is_some() && !param_ctx && !tn && self.fns.is_empty() && self.class_depth == 0 && self.in_template() && self.dependent_qualifier_at_k(1) && self.at_k(self.matching_close(0) + 1, "{");
                if value_param && self.inst_depth == 0 && !self.looks_like_params_tn(tn) {
                    // DEP-29: the parentheses hold the initializer of a
                    // variable template, and a body cannot follow them.
                    let l = self.line();
                    self.error_because(l, "DEP-29", "DEP-19", "a function body after the initializer of a variable template ('T::name' without 'typename' is a value here)");
                }
                if d.name.is_some() && !param_ctx && !self.looks_like_params_tn(tn) && !value_param && !self.has_fact(self.pos + 1, crate::templ::FACT_VALUE_PARAM) {
                    break;
                }
                // DEP-20: the parameters of a member, of a friend in a
                // class, and of a function declared with a qualified name.
                if d.qual.is_some() || d.qual_unknown || matches!(self.scopes[self.cur_scope()].kind, SK::Class(_)) {
                    self.params_tn = true;
                } else if d.name.is_some() && !param_ctx && self.fns.is_empty() && self.class_depth == 0 {
                    self.params_value = true;
                }
                let f = self.parse_params()?;
                suffixes.push(DOp::Func(Box::new(f)));
            } else {
                break;
            }
        }
        Ok(suffixes)
    }

    pub fn class_of_decl(&self, id: DeclId) -> Option<usize> {
        match &self.decls[id].k {
            DK::Class(c) => Some(*c),
            DK::Type(q) => match q.t {
                Ty::Class(c) => Some(c),
                _ => None,
            },
            _ => None,
        }
    }

    fn is_member_ptr_start(&self) -> bool {
        // id (:: id)* :: *
        let mut k = 0;
        if self.at_k(0, "::") {
            k = 1;
        }
        loop {
            if self.peek(k).kind != TK::Ident || is_keyword(&self.peek(k).text) {
                return false;
            }
            k += 1;
            if self.at_k(k, "<") {
                k = self.skip_angle_at(k);
            }
            if !self.at_k(k, "::") {
                return false;
            }
            k += 1;
            if self.at_k(k, "*") {
                return true;
            }
        }
    }

    /// At `(` in a declarator: is it a grouping parenthesis or a
    /// parameter list?
    fn paren_is_grouping(&self, abstract_ok: bool, param_ctx: bool) -> bool {
        let n = self.peek(1);
        if n.kind == TK::Punct {
            return matches!(n.text.as_str(), "*" | "&" | "&&" | "(" | "::" | "...") || (n.text == "[" && self.at_k(2, "["));
        }
        if n.kind == TK::Ident {
            if is_keyword(&n.text) {
                return n.text == "operator";
            }
            if self.is_member_ptr_start_at(1) {
                return true;
            }
            // `(name)` groups unless the name is a type in an abstract
            // context, where `(T)` is a parameter list.
            if abstract_ok || param_ctx {
                let names_type = self.qualified_names_type(1);
                return !names_type;
            }
            return true;
        }
        false
    }

    fn is_member_ptr_start_at(&self, off: usize) -> bool {
        let mut k = off;
        loop {
            if self.peek(k).kind != TK::Ident || is_keyword(&self.peek(k).text) {
                return false;
            }
            k += 1;
            if !self.at_k(k, "::") {
                return false;
            }
            k += 1;
            if self.at_k(k, "*") {
                return true;
            }
        }
    }

    pub fn parse_declarator_name(&mut self, d: &mut Declarator) -> PR<()> {
        d.line = self.line();
        self.last_qual_templ = None;
        let scope = self.parse_nested_name()?;
        if scope.is_some() && self.pos > 0 && self.toks[self.pos - 1].text == "template" && self.toks[self.pos - 1].kind == TK::Ident && self.inst_depth == 0 {
            // DEP-27 (Q-0227): no `template` after the qualifier of the
            // name that the declaration declares.
            self.error(d.line, "DEP-27", "the keyword 'template' comes after the qualifier of the declared name");
        }
        match scope {
            Some(Ok(s)) => d.qual = Some(s),
            Some(Err(())) => {
                d.qual_unknown = true;
                // TPL-23: a member of a class template defined outside.
                if self.pending_template.is_some() {
                    if let Some((t, pats)) = self.last_qual_templ.take() {
                        self.last_templ = Some(crate::templ::Last::Member(t, pats));
                    }
                }
            }
            None => {}
        }
        if self.eat("~") {
            let n = self.ident()?;
            d.name = Some(format!("~{}", n));
            d.kind = NameKind::Dtor;
            if self.at("<") {
                self.skip_balanced();
            }
            return Ok(());
        }
        if self.eat("operator") {
            let op = self.parse_operator_name(d)?;
            d.name = Some(format!("operator{}", op));
            if self.at("<") && d.kind != NameKind::Conversion && !self.member_op_name {
                self.skip_template_args()?;
                d.template_args = true;
            }
            return Ok(());
        }
        let n = self.ident()?;
        // ctor: the name is the class name in its own scope
        if let Some(s) = d.qual {
            if let SK::Class(c) = self.scopes[s].kind {
                if self.classes[c].name == n {
                    d.kind = NameKind::Ctor;
                }
            }
        } else if let Some(c) = self.current_class() {
            if self.classes[c].name == n && self.at("(") {
                d.kind = NameKind::Ctor;
            }
        }
        if self.at("<") {
            let found = match d.qual {
                Some(s) => self.lookup_in_scope(s, &n, true),
                None => self.lookup(&n),
            };
            let fn_templ = found.as_ref().is_some_and(|ids| ids.iter().any(|&i| self.func(i).is_some_and(|f| f.template)));
            if fn_templ {
                // The template arguments of a declaration of a function
                // template specialization (DED-32, TSP-7).
                let save = self.pos;
                let mark = self.d.list.len();
                self.d.muted += 1;
                let raw = self.parse_raw_targs();
                self.d.muted -= 1;
                self.d.list.truncate(mark);
                match raw {
                    Ok(raw) if self.at("(") => {
                        self.decl_fn_targs = Some(std::rc::Rc::new(raw));
                        d.template_args = true;
                        d.name = Some(n);
                        return Ok(());
                    }
                    _ => self.pos = save,
                }
            }
            let is_templ = found.is_some_and(|ids| ids.iter().any(|&i| matches!(self.decls[i].k, DK::Template(_)) || self.func(i).is_some_and(|f| f.template)));
            if is_templ || self.in_template() {
                d.template_args = true;
                // keep the argument tokens for variable template partial
                // specializations: the caller re-parses them.
                d.name = Some(n);
                return Ok(());
            }
        }
        d.name = Some(n);
        Ok(())
    }

    fn parse_operator_name(&mut self, d: &mut Declarator) -> PR<String> {
        let t = self.tok().clone();
        if t.kind == TK::Punct {
            let s = t.text.clone();
            self.pos += 1;
            let op = match s.as_str() {
                "(" => {
                    self.expect(")")?;
                    "()".to_string()
                }
                "[" => {
                    self.expect("]")?;
                    "[]".to_string()
                }
                "." | ".*" | "::" | "?" | ":" => {
                    // OPR-2: these operators cannot be overloaded.
                    self.error(t.line, "OPR-2", format!("'operator{}' cannot be overloaded", s));
                    s
                }
                _ => s,
            };
            d.kind = NameKind::Operator(op.clone());
            return Ok(op);
        }
        if t.kind == TK::Str {
            // literal operator: operator "" _x (the lexer may have joined
            // the suffix to the string token)
            self.pos += 1;
            let n = if matches!(t.lit, Some(crate::lexer::Lit::UserDefined { .. })) { t.text.clone() } else { self.ident()? };
            d.kind = NameKind::Literal;
            return Ok(format!("\"\"{}", n));
        }
        if t.text == "new" || t.text == "delete" {
            self.pos += 1;
            let mut s = t.text.clone();
            if self.at("[") && self.at_k(1, "]") {
                self.pos += 2;
                s.push_str("[]");
            }
            d.kind = NameKind::Operator(s.clone());
            return Ok(s);
        }
        if t.text == "co_await" {
            self.pos += 1;
            d.kind = NameKind::Operator("co_await".into());
            return Ok("co_await".into());
        }
        // conversion function: operator type-specifiers ptr-ops
        let ds = self.parse_decl_specs(None)?;
        self.last_conv_auto = ds.auto.is_some();
        let mut q = self.ds_base(&ds)?;
        loop {
            if self.eat("*") {
                let (c, v) = self.parse_cv();
                q = QTy { t: Ty::Ptr(Box::new(q)), c, v };
            } else if self.eat("&") {
                q = QTy::new(Ty::LRef(Box::new(q)));
            } else if self.eat("&&") {
                q = QTy::new(Ty::RRef(Box::new(q)));
            } else if self.is_member_ptr_start() {
                // `operator int C::*`
                let save = self.pos;
                let scope = self.parse_nested_name()?;
                let cname = self.ident();
                let class = match (scope, cname) {
                    (None, Ok(n)) => self.lookup(&n).and_then(|ids| self.class_of_decl(ids[0])),
                    (Some(Ok(s)), Ok(n)) => self.lookup_in_scope(s, &n, true).and_then(|ids| self.class_of_decl(ids[0])),
                    _ => None,
                };
                if !self.eat("::") || !self.eat("*") {
                    self.pos = save;
                    break;
                }
                let (c, v) = self.parse_cv();
                q = match class {
                    Some(cl) => QTy { t: Ty::MemPtr(cl, Box::new(q)), c, v },
                    None => QTy::unknown(),
                };
            } else {
                break;
            }
        }
        d.kind = NameKind::Conversion;
        let name = format!(" {}", show(&q));
        d.conv_ty = Some(q);
        Ok(name)
    }

    pub fn parse_cv(&mut self) -> (bool, bool) {
        let mut c = false;
        let mut v = false;
        loop {
            if self.at("const") {
                if c {
                    self.error(self.line(), "CV-2", "duplicate 'const'");
                }
                c = true;
                self.pos += 1;
            } else if self.at("volatile") {
                if v {
                    self.error(self.line(), "CV-2", "duplicate 'volatile'");
                }
                v = true;
                self.pos += 1;
            } else {
                return (c, v);
            }
        }
    }

    /// After a declarator name, decide if `(` starts a parameter list
    /// (DCL-13: a declaration that can be a function declaration is one).
    pub fn looks_like_params(&self) -> bool {
        self.looks_like_params_tn(false)
    }
    /// `tn`: the parameters are in type-only positions (DEP-20).
    pub fn looks_like_params_tn(&self, tn: bool) -> bool {
        // at "("
        let n = self.peek(1);
        if n.kind == TK::Punct {
            return n.text == ")" || n.text == "..." || (n.text == "[" && self.at_k(2, "[")) || (n.text == "::" && self.qualified_names_type(1));
        }
        if n.kind == TK::Ident {
            let s = n.text.as_str();
            if simple_type_kw(s)
                || matches!(
                    s,
                    "const" | "volatile" | "struct" | "class" | "union" | "enum" | "typename" | "auto" | "register" | "decltype" | "this" | "static" | "extern"
                        | "thread_local" | "mutable" | "inline" | "constexpr" | "alignas" | "constinit" | "consteval"
                )
            {
                // `decltype(x)` could start an expression, but rarely does.
                return true;
            }
            if is_keyword(s) {
                return false;
            }
            if !self.qualified_names_type(1) && !(tn && self.in_template() && self.dependent_qualifier_at_k(1)) {
                return false;
            }
            // `T{...}` or `T(...)` after the type is an expression
            let mut k = 1;
            loop {
                if self.peek(k).kind == TK::Ident {
                    k += 1;
                    if self.at_k(k, "<") {
                        k = self.skip_angle_at(k);
                    }
                    if self.at_k(k, "::") {
                        k += 1;
                        continue;
                    }
                }
                break;
            }
            if self.at_k(k, "(") {
                // `T(5)`: a declarator in parentheses starts with a name,
                // `*`, `&`, `&&`, `(`, `[`, `::`, or `...`, so a literal
                // or another operator makes an expression (DCL-1).
                let t = self.peek(k + 1);
                match t.kind {
                    TK::Ident => {}
                    TK::Punct => {
                        if !matches!(t.text.as_str(), "*" | "&" | "&&" | "(" | ")" | "[" | "::" | "...") {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            return !self.at_k(k, "{");
        }
        false
    }

    /// Parse `( params ) cv ref noexcept -> trailing`.
    pub fn parse_params(&mut self) -> PR<FnDecl> {
        self.param_nest += 1;
        let top = self.param_nest == 1;
        let saved_k = self.auto_param_k;
        if top {
            self.auto_param_k = 0;
        }
        let r = self.parse_params_inner();
        if top {
            if let Some(a) = self.abbrev.as_mut() {
                a.closed = true;
            }
        } else {
            self.auto_param_k = saved_k;
        }
        self.param_nest -= 1;
        r
    }

    fn parse_params_inner(&mut self) -> PR<FnDecl> {
        let ptn = std::mem::take(&mut self.params_tn);
        let saved = std::mem::replace(&mut self.in_params_tn, ptn);
        let pv = std::mem::take(&mut self.params_value);
        let saved_v = std::mem::replace(&mut self.in_params_value, pv);
        let r = self.parse_params_inner2();
        self.in_params_tn = saved;
        self.in_params_value = saved_v;
        r
    }

    fn parse_params_inner2(&mut self) -> PR<FnDecl> {
        self.expect("(")?;
        let scope = self.push_scope(SK::Params);
        let mut f = FnDecl {
            params: Vec::new(),
            ptys: Vec::new(),
            variadic: false,
            cv_c: false,
            cv_v: false,
            refq: 0,
            noexcept: false,
            noexcept_false: false,
            trailing: None,
            scope,
            defaults: 0,
            has_explicit_this: false,
            trailing_auto: None,
            noexcept_dep: false,
            noexcept_param: None,
        };
        let r = self.parse_param_list(&mut f);
        self.pop_scope();
        r?;
        let (c, v) = self.parse_cv();
        f.cv_c = c;
        f.cv_v = v;
        if self.eat("&") {
            f.refq = 1;
        } else if self.eat("&&") {
            f.refq = 2;
        }
        if self.at("noexcept") {
            self.pos += 1;
            f.noexcept = true;
            let member = matches!(self.stack.last().map(|&s| &self.scopes[s].kind), Some(SK::Class(_))) && !self.in_template() && self.inst_depth == 0 && self.pending_template.is_none();
            if self.at("(") && member {
                // CLS-2, EXS-1: the operand of a member's noexcept is in a
                // complete-class context. When it needs later members, it
                // is parsed again at the end of the class.
                let open = self.pos;
                // A trial parse of an operand that names the class makes
                // what it names, such as a specialization over the class,
                // while the class is incomplete (INST-30). Such an operand
                // waits for the end of the class without a trial.
                let cname = match self.stack.last().map(|&s| &self.scopes[s].kind) {
                    Some(SK::Class(c)) => self.classes[*c].name.clone(),
                    _ => String::new(),
                };
                self.skip_balanced();
                let names_class = !cname.is_empty() && (open + 1..self.pos).any(|k| self.toks[k].kind == TK::Ident && self.toks[k].text == cname);
                self.pos = open + 1;
                self.stack.push(scope);
                let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
                self.d.muted += 1;
                let e = if names_class { Err(self.err("deferred")) } else { self.parse_expression() };
                let v = match &e {
                    Ok(e) if self.at(")") => self.eval_bool_const(e, 0, "EXS-7"),
                    _ => None,
                };
                self.d.muted -= 1;
                let failed = names_class || self.d.muted_errors != me || self.d.muted_unsup != mu || v.is_none();
                self.d.muted_errors = me;
                self.d.muted_unsup = mu;
                self.stack.pop();
                self.pos = open;
                if failed {
                    let stack = { let mut st = self.stack.clone(); st.push(scope); st };
                    self.skip_balanced();
                    self.pending_noexcept_defer = Some((open + 1, self.pos - 1, stack));
                    f.noexcept = false;
                }
            }
            if self.at("(") {
                self.pos += 1;
                let l = self.line();
                // the parameters are in scope in the noexcept operand
                self.stack.push(scope);
                // EXS-9, DED-27: in a specialization the operand is
                // substituted only when a use needs it.
                let in_inst = self.inst_depth > 0;
                let mark = self.d.list.len();
                let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
                let saved_muted = if in_inst { std::mem::take(&mut self.d.muted) } else { self.d.muted };
                let e = self.parse_expression();
                if in_inst {
                    self.d.muted = saved_muted;
                    self.d.muted_errors = me;
                    self.d.muted_unsup = mu;
                }
                self.stack.pop();
                let e = e?;
                self.expect(")")?;
                if in_inst && (self.d.list.len() != mark || e.ty.is_unknown()) {
                    let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                    if !kept.is_empty() {
                        self.pending_noexcept_errors = Some(kept);
                    }
                    f.noexcept = false;
                    self.skip_attributes();
                    if self.eat("->") {
                        let save = self.stack.clone();
                        self.stack.push(scope);
                        let r = (|| -> PR<QTy> {
                            self.implicit_tn = true;
                            let ds = self.parse_decl_specs(None)?;
                            let d = self.parse_declarator(true, false)?;
                            if ds.auto.is_some() {
                                f.trailing_auto = Some((ds, d));
                                return Ok(QTy::unknown());
                            }
                            let base = self.ds_base(&ds)?;
                            Ok(self.apply_ops(base, &d.ops, d.line))
                        })();
                        self.stack = save;
                        f.trailing = Some(r?);
                    }
                    return Ok(f);
                }
                if self.in_template() && self.inst_depth == 0 && (e.ty.is_unknown() || matches!(e.k, EK::Unknown(_))) {
                    f.noexcept_dep = true;
                    if let EK::Unknown(w) = &e.k {
                        if let Some(id) = w.strip_prefix("template parameter#").and_then(|x| x.parse::<usize>().ok()) {
                            f.noexcept_param = self.tparam.get(&id).copied();
                        }
                    }
                }
                // EXS-7 (Q-0078): the conversion to bool must not narrow.
                if !self.in_template() && !e.ty.is_unknown() && nonref(&e.ty).t != Ty::Bool && self.is_narrowing(&e, &Ty::Bool) {
                    self.error(l, "EXS-7", "the operand of noexcept narrows in its conversion to bool");
                }
                // EXS-1 / CE-11: the operand is a constant of type bool.
                if let Some(v) = self.eval_bool_const(&e, l, "EXS-7") {
                    f.noexcept = v;
                    f.noexcept_false = !v;
                }
            }
        } else if self.at("throw") && self.at_k(1, "(") {
            // dynamic exception specification: removed in C++17 except throw()
            let tl = self.line();
            self.pos += 1;
            if !self.at_k(1, ")") {
                // EXS-7: `throw` with a list of types is not valid.
                self.error(tl, "EXS-7", "a dynamic exception specification with types");
            }
            self.skip_balanced();
            f.noexcept = true;
        }
        self.skip_attributes();
        if self.eat("->") {
            let save = self.stack.clone();
            self.stack.push(scope);
            let r = (|| -> PR<QTy> {
                self.implicit_tn = true;
                let ds = self.parse_decl_specs(None)?;
                let d = self.parse_declarator(true, false)?;
                if ds.auto.is_some() {
                    // a placeholder in the trailing return type
                    f.trailing_auto = Some((ds, d));
                    return Ok(QTy::unknown());
                }
                let base = self.ds_base(&ds)?;
                Ok(self.apply_ops(base, &d.ops, d.line))
            })();
            self.stack = save;
            f.trailing = Some(r?);
        }
        Ok(f)
    }

    fn parse_param_list(&mut self, f: &mut FnDecl) -> PR<()> {
        if self.eat(")") {
            return Ok(());
        }
        if self.at("void") && self.at_k(1, ")") {
            self.pos += 2;
            return Ok(());
        }
        loop {
            if self.eat("...") {
                f.variadic = true;
                self.expect(")")?;
                return Ok(());
            }
            self.skip_attributes();
            let line = self.line();
            let explicit_this = self.eat("this");
            if explicit_this {
                if !f.params.is_empty() || !f.ptys.is_empty() {
                    // CLS-14: the explicit object parameter comes first.
                    self.error(line, "CLS-14", "an explicit object parameter that is not the first parameter");
                }
                f.has_explicit_this = true;
            }
            let pstart = self.pos;
            let pmark = self.pack_refs.len();
            let mut value_qual = false;
            if self.in_params_value && !self.in_params_tn {
                // DEP-19: outside the positions of DEP-20, a parameter type
                // `T::name` without `typename` reads as a value.
                if self.in_template() && self.dependent_qualifier_at() {
                    self.note_fact(pstart, crate::templ::FACT_VALUE_PARAM);
                    value_qual = true;
                } else if self.has_fact(pstart, crate::templ::FACT_VALUE_PARAM) {
                    self.error(line, "DEP-19", "a parameter type names a member of a dependent class without 'typename'");
                }
            }
            self.implicit_tn = self.in_params_tn || value_qual;
            let ds = self.parse_decl_specs(None)?;
            // DED-34: a placeholder parameter of a function declaration.
            let auto_k = if ds.auto.is_some() && self.param_nest == 1 && !self.at("[") {
                let k = self.auto_param_k;
                self.auto_param_k += 1;
                Some(k)
            } else {
                None
            };
            if !ds.storage.is_empty() || ds.register {
                // STO-6: no storage class on a parameter.
                self.error(line, "STO-6", "storage class on a function parameter");
            }
            if ds.constexpr || ds.consteval || ds.constinit || ds.inline {
                // CX-5, INL-2: not on a parameter.
                self.error(line, if ds.inline { "INL-2" } else { "CX-5" }, "specifier not allowed on a parameter");
            }
            if ds.has_alignas {
                self.error(line, "ALN-2", "alignas on a parameter");
            }
            let base = match auto_k {
                Some(k) => match self.auto_param_base(k, &ds, line)? {
                    Some(q) => add_cv(&q, ds.c > 0, ds.v > 0),
                    None => self.ds_base(&ds)?,
                },
                None => self.ds_base(&ds)?,
            };
            let puses = self.pack_uses.len();
            let d = self.parse_declarator(true, true)?;
            if d.pack && self.inst_depth == 0 {
                // PACK-2: the type of a function parameter pack names a
                // pack or holds `auto`.
                let names_pack = self.pack_uses.len() > puses || self.pack_uses.len() > 0 && self.pack_uses.iter().any(|&(p, _)| p >= pstart) || auto_k.is_some();
                if !names_pack && !self.in_template() && d.name.is_some() {
                    self.error(line, "PACK-2", "the type of a function parameter pack names no pack");
                }
                if self.at("=") {
                    // PACK-4: no default argument for a parameter pack.
                    self.error(line, "PACK-4", "a function parameter pack has a default argument");
                }
            } else if !d.pack && d.name.is_some() && self.at("...") && self.in_template() && self.inst_depth == 0 && self.pack_uses.iter().any(|&(p, _)| p >= pstart) {
                // PACK-3, PACK-8: with a name, the `...` is the variable
                // argument part, and the pack in the type stays unexpanded.
                self.error(line, "PACK-8", "a pack in the type of a named parameter is not expanded");
            }
            if self.at("requires") {
                // CPT-13: no trailing clause on a parameter.
                self.error(self.line(), "CPT-13", "a trailing requires clause on a parameter");
                self.pos += 1;
                let _ = self.cons_clause(false)?;
            }
            if d.pack && (!self.in_template() || self.inst_depth > 0) && self.pack_refs.len() > pmark {
                // A function parameter pack in a specialization: one
                // parameter for each element. INST-5, PACK-1: in a member
                // template of a class specialization, a pack of the class
                // is known, and the pack expands in the declaration.
                let n = self.pack_refs[pmark];
                self.pack_refs.truncate(pmark);
                let end = self.pos;
                let saved = self.pack_index;
                let name = d.name.clone().unwrap_or_default();
                let mut ids = Vec::new();
                for k in 0..n {
                    self.pos = pstart;
                    self.pack_index = Some(k);
                    let r = (|| -> PR<QTy> {
                        let ds = self.parse_decl_specs(None)?;
                        let base = match auto_k {
                            Some(ak) => match self.auto_param_base(ak, &ds, line)? {
                                Some(q) => add_cv(&q, ds.c > 0, ds.v > 0),
                                None => self.ds_base(&ds)?,
                            },
                            None => self.ds_base(&ds)?,
                        };
                        let d = self.parse_declarator(true, true)?;
                        Ok(self.apply_ops(base, &d.ops, d.line))
                    })();
                    self.pack_index = saved;
                    let ty = r?;
                    if ty.t == Ty::Void {
                        // PACK-28: no parameter of type void.
                        self.error(line, "PACK-28", "an expansion gives a parameter of type void");
                    }
                    let ty = match ty.t {
                        Ty::Array(e, _) => QTy::new(Ty::Ptr(e)),
                        Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(ty))),
                        _ => ty,
                    };
                    let vi = VarInfo { ty: ty.clone(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: true, dependent: ty.is_unknown(), defined: true, class: None };
                    let id = self.add_decl(if k == 0 { &name } else { "" }, DK::Var(vi), line);
                    if k > 0 {
                        self.decls[id].name = name.clone();
                    }
                    f.params.push(id);
                    f.ptys.push(QTy { t: ty.t.clone(), c: false, v: false });
                    ids.push(id);
                }
                self.pos = end;
                if ids.is_empty() {
                    let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: true, dependent: true, defined: true, class: None };
                    let id = self.add_decl(&name, DK::Var(vi), line);
                    self.fn_param_packs.insert(id, vec![]);
                }
                for &id in &ids {
                    self.fn_param_packs.insert(id, ids.clone());
                }
                if self.eat(",") {
                    continue;
                }
                if self.eat("...") {
                    f.variadic = true;
                }
                self.expect(")")?;
                return Ok(());
            }
            let mut ty = self.apply_ops(base, &d.ops, d.line);
            if ty.t == Ty::Void && d.ops.is_empty() {
                // DCL-9: `void` only as the whole list.
                self.error(line, "DCL-22", "'void' in a parameter list with other parameters or a name");
            }
            if let Ty::Func(ft) = &ty.t {
                if ft.cv_c || ft.cv_v || ft.refq != 0 {
                    self.error(line, "DCL-16", "parameter of function type with qualifiers");
                }
            }
            // DCL-8: parameter type adjustments.
            let was_array = matches!(ty.t, Ty::Array(..));
            ty = match ty.t {
                Ty::Array(e, _) => QTy::new(Ty::Ptr(e)),
                Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(ty))),
                _ => ty,
            };
            let pty = QTy { t: ty.t.clone(), c: false, v: false };
            let mut default = None;
            let mut defer_default = None;
            if self.eat("=") {
                f.defaults += 1;
                if explicit_this && !self.in_template() {
                    // CLS-14 (Q-0317): no default argument on the explicit
                    // object parameter.
                    self.error(line, "CLS-14", "a default argument on an explicit object parameter");
                }
                if self.class_depth > 0 && self.fns.is_empty() && self.current_class_any().is_some() {
                    // CLS-2: a default argument sees the whole class; parse it later.
                    let start = self.pos;
                    let mut depth = 0i32;
                    while self.tok().kind != TK::Eof {
                        if depth == 0 && (self.at(",") || self.at(")")) {
                            break;
                        }
                        if self.at("(") || self.at("[") || self.at("{") {
                            depth += 1;
                        } else if self.at(")") || self.at("]") || self.at("}") {
                            depth -= 1;
                        } else if self.at("<") && depth == 0 {
                            // template arguments in a default argument are rare; keep going
                        }
                        self.pos += 1;
                    }
                    defer_default = Some((start, self.pos));
                } else {
                    let dl = self.line();
                    // INST-9: in a specialization, a default argument is
                    // made when a call uses it; its errors wait.
                    let keep = self.inst_depth > 0 && !self.in_template();
                    let mark = self.d.list.len();
                    let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
                    let saved_muted = if keep { std::mem::take(&mut self.d.muted) } else { self.d.muted };
                    // DEFARG-10 (Q-0379): an immediate call in a default
                    // argument is not checked at the declaration.
                    self.in_default_arg += 1;
                    let r = (|| -> PR<Expr> {
                        let e = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
                        // DEFARG-5: the default argument is valid for the
                        // parameter type at the declaration.
                        let form = if matches!(e.k, EK::InitList(_)) { crate::decl2::InitForm::CopyList } else { crate::decl2::InitForm::Copy };
                        // DEFARG-10, DEP-30: in a template, a default made
                        // only of literals is checked at the declaration.
                        let literal = {
                            let mut ok = !e.ty.is_unknown();
                            crate::ast::walk_expr(&e, &mut |x| if !matches!(x.k, EK::Int(_) | EK::Float(_) | EK::LFloat(_) | EK::NullPtr | EK::Str(_) | EK::Unary(..) | EK::Binary(..) | EK::Conv(..)) || x.ty.is_unknown() { ok = false });
                            ok
                        };
                        let e = if ty.is_unknown() || (self.in_template() && !(literal && self.inst_depth == 0)) { e } else { self.convert_init(e, &ty, form, dl) };
                        self.check_default_arg(&e, dl);
                        Ok(e)
                    })();
                    self.in_default_arg -= 1;
                    if keep {
                        self.d.muted = saved_muted;
                        let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                        self.d.muted_errors = me;
                        self.d.muted_unsup = mu;
                        if !kept.is_empty() {
                            self.pending_default_errors = Some(kept);
                        }
                    }
                    default = Some(r?);
                }
            }
            let name = d.name.clone().unwrap_or_default();
            if !name.is_empty() {
                if let Some(ids) = self.scopes[self.cur_scope()].names.get(&name) {
                    if !ids.is_empty() {
                        self.error(line, "NAM-3", format!("parameter '{}' declared twice", name));
                    }
                }
            }
            let vi = VarInfo {
                ty: ty.clone(),
                storage: Storage::Auto,
                constexpr: false,
                constinit: false,
                init: default.map(std::rc::Rc::new),
                param: true,
                local: true,
                dependent: ty.is_unknown(),
                defined: true,
                class: None,
            };
            let id = self.add_decl(&name, DK::Var(vi), line);
            if was_array {
                self.array_params.insert(id);
            }
            if let Some(kept) = self.pending_default_errors.take() {
                self.default_errors.insert(id, kept);
            }
            if let Some((start, end)) = defer_default {
                let c = self.current_class_any().unwrap();
                self.deferred.push(Deferred { func: None, field: None, param: Some(id), start, end, stack: self.stack.clone(), templ: self.templ, class: c, noexcept_of: None });
            }
            if ty.v && !ty.t.is_ref() && self.inst_depth == 0 && !self.in_template() {
                // CV-9: a volatile parameter type is deprecated.
                self.warn(line, "CV-9", "a parameter type with a top-level 'volatile' is deprecated");
            }
            f.params.push(id);
            if !explicit_this {
                f.ptys.push(pty);
            }
            if d.pack {
                f.variadic = true;
                self.pack_params.insert(id);
                if self.in_template() {
                    self.pack_decls.insert(id);
                }
            }
            if self.eat(",") {
                continue;
            }
            if self.eat("...") {
                f.variadic = true;
            }
            self.expect(")")?;
            return Ok(());
        }
    }

    /// DEFARG-4: a default argument must not use `this`, a parameter, or a
    /// local variable of an enclosing function.
    pub fn check_default_arg(&mut self, e: &Expr, line: u32) {
        let mut bad: Option<String> = None;
        walk_expr(e, &mut |x| {
            if bad.is_some() {
                return;
            }
            match &x.k {
                EK::This => bad = Some("'this'".into()),
                EK::Var(id) => {
                    if let DK::Var(v) = &self.decls[*id].k {
                        if v.param || (v.local && v.storage == Storage::Auto) {
                            bad = Some(format!("'{}'", self.decls[*id].name));
                        }
                    }
                }
                _ => {}
            }
        });
        if let Some(what) = bad {
            self.error(line, "DEFARG-4", format!("default argument uses {}", what));
        }
    }

    /// The class whose body is being parsed, through parameter scopes.
    pub fn current_class_any(&self) -> Option<usize> {
        for &s in self.stack.iter().rev() {
            match self.scopes[s].kind {
                SK::Class(c) => return Some(c),
                SK::Template | SK::Params => continue,
                _ => return None,
            }
        }
        None
    }

    pub fn current_class(&self) -> Option<usize> {
        for &s in self.stack.iter().rev() {
            match self.scopes[s].kind {
                SK::Class(c) => return Some(c),
                SK::Template => continue,
                _ => return None,
            }
        }
        None
    }

    /// The class whose member function body is being parsed, if any.
    pub fn enclosing_class(&self) -> Option<usize> {
        for &s in self.stack.iter().rev() {
            if let SK::Class(c) = self.scopes[s].kind {
                return Some(c);
            }
        }
        None
    }

    /// Evaluate an array bound (DCL-5).
    fn array_bound(&mut self, e: &Expr, line: u32) -> Option<u64> {
        if e.ty.is_unknown() {
            return Some(u64::MAX);
        }
        let t = nonref(&e.ty);
        if self.is_scoped_enum(&t.t) || !(t.t.is_integral() || self.is_unscoped_enum(&t.t) || matches!(t.t, Ty::Class(_))) {
            self.error_because(line, "DCL-19", "CE-11", "array bound must have integer or unscoped enumeration type");
            return Some(u64::MAX);
        }
        let converted;
        let e = if matches!(t.t, Ty::Class(_)) && !self.in_template() {
            // DCL-5, CVF-1: a class operand converts to std::size_t by a
            // conversion function.
            let mark = self.d.list.len();
            match self.user_conv(e, &QTy::new(Ty::size_t()), line) {
                Some(x) if self.d.list.len() == mark && !matches!(x.k, EK::Unknown(_)) => {
                    converted = x;
                    &converted
                }
                _ => {
                    self.d.list.truncate(mark);
                    e
                }
            }
        } else {
            e
        };
        match self.const_int(e) {
            Ok(v) => {
                if v <= 0 {
                    self.error(line, "DCL-19", "array bound must be greater than zero");
                    return Some(1);
                }
                let max: u128 = if crate::types::ilp32() { u32::MAX as u128 } else { u64::MAX as u128 };
                if v as u128 > max {
                    // DCL-19: the bound converts to the type of sizeof
                    // without loss of value.
                    self.error(line, "DCL-19", "the array bound does not fit in std::size_t");
                    return Some(1);
                }
                // u64::MAX stands for an unknown bound, and the values
                // from ARR_PARAM for a parameter; a larger bound keeps a
                // known size that does not fit.
                Some((v as u64).min(ARR_PARAM - 1))
            }
            Err(crate::consteval::CeErr::NotConst(rule, msg)) => {
                if !self.in_template() {
                    self.error_because(line, "DCL-19", rule, format!("array bound is not a constant expression ({}: {})", rule, msg));
                }
                Some(u64::MAX)
            }
            Err(crate::consteval::CeErr::Unsupported(msg)) => {
                if !self.in_template() {
                    self.unsupported(line, format!("array bound: {}", msg));
                }
                Some(u64::MAX)
            }
        }
    }
}

/// An array bound that names a constant template parameter, kept in the
/// bound of `DOp::Array`.
const ARR_PARAM: u64 = u64::MAX - (1u64 << 48);

pub fn arr_param_encode(d: u32, i: u32) -> u64 {
    ARR_PARAM + (((d as u64) & 0xffff) << 16) + (i as u64 & 0xffff)
}

pub fn arr_param_decode(n: u64) -> Option<(u32, u32)> {
    if (ARR_PARAM..u64::MAX).contains(&n) {
        let x = n - ARR_PARAM;
        Some((((x >> 16) & 0xffff) as u32, (x & 0xffff) as u32))
    } else {
        None
    }
}
