// Declarations: decl-specifiers, declarators, and type-ids.
//
// The Tidal grammar for declarators (spec/declarations/declarators.md)
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
    pub trailing: Option<QTy>,
    pub scope: usize,
    pub defaults: usize,
    pub has_explicit_this: bool,
    pub trailing_auto: Option<(DeclSpec, Declarator)>,
}

#[derive(Clone, Debug)]
pub enum DOp {
    Ptr(bool, bool),
    LRef,
    RRef,
    MemPtr(Option<usize>, bool, bool),
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
                None => self.lookup(&t.text),
                Some(s) => self.lookup_in_scope(s, &t.text, true),
            };
            let Some(ids) = found else {
                return false;
            };
            let id = ids[0];
            // template-id
            let mut next = k + 1;
            if self.at_k(next, "<") && matches!(self.decls[id].k, DK::Template(_)) {
                next = self.skip_angle_at(next);
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

    /// Given offset k at `<`, return the offset after the matching `>`.
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
                    let mut val: Option<u64> = None;
                    if self.starts_type() {
                        if let Ok(t) = self.parse_type_id() {
                            if self.at(")") {
                                val = self.alignof_ty(&nonref(&t).t).or(Some(0));
                            }
                        }
                    } else if let Ok(e) = self.parse_assignment() {
                        if self.at(")") {
                            match self.const_int(&e) {
                                Ok(v) => {
                                    if v < 0 || (v != 0 && (v & (v - 1)) != 0) {
                                        // ALN-2: zero or a power of two.
                                        self.error(line, "ALN-2", "alignment is not a power of two");
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
                    if val.is_none() || !self.at(")") {
                        self.pos = save;
                        self.skip_balanced();
                        self.align_unknown = true;
                    } else {
                        self.pos += 1;
                    }
                    self.eat("...");
                    let v = val.unwrap_or(0);
                    self.last_align = self.last_align.max(v);
                    self.saw_alignas = true;
                }
                continue;
            }
            break;
        }
    }

    /// Attributes (spec/declarations/attributes.md). The front end parses
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
                    ("noreturn", Some(_)) => {
                        // ATTR-2: noreturn has no argument.
                        self.error(line, "ATTR-2", "'noreturn' takes no argument");
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
        let mut ds = DeclSpec { line: self.line(), ..Default::default() };
        loop {
            self.skip_attributes();
            let t = self.tok().clone();
            if t.kind == TK::Punct && t.text == "::" && ds.ty.is_none() && ds.kw.is_empty() && ds.auto.is_none() && !self.is_ctor_start(in_class) && self.qualified_names_type(0) {
                let q = self.parse_type_name()?;
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
                "friend" => ds.friend = true,
                "explicit" => {
                    ds.explicit = true;
                    if self.at_k(1, "(") {
                        // CTOR-9: explicit(c) needs a constant c.
                        self.pos += 2;
                        let start = self.pos - 1;
                        let l = self.line();
                        match self.parse_assignment() {
                            Ok(e) if self.at(")") => {
                                self.pos += 1;
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
                    let (q, defined) = self.parse_class_specifier()?;
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
                    self.skip_qualified_name()?;
                    ds.ty = Some(QTy::unknown());
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
                    if !self.qualified_names_type(0) {
                        break;
                    }
                    // A concept before auto (AUTO-13).
                    let q = self.parse_type_name()?;
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
        let bad = |p: &mut P| {
            p.error_norule(line, format!("invalid combination of type specifiers: {}", kw.join(" ")));
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
            let found = match scope {
                None => self.lookup(&name),
                Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
                Some(Err(())) => None,
            };
            let mut next = 1;
            let is_templ = found.as_ref().is_some_and(|ids| matches!(self.decls[ids[0]].k, DK::Template(_)));
            if self.at_k(1, "<") && (is_templ || matches!(scope, Some(Err(())))) {
                next = self.skip_angle_at(1);
            }
            if !self.at_k(next, "::") || self.peek(next + 1).kind != TK::Ident && !self.at_k(next + 1, "~") && !self.at_k(next + 1, "template") && !self.at_k(next + 1, "operator") {
                return Ok(scope);
            }
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
                        _ => Err(()),
                    },
                    _ => Err(()),
                },
                None => Err(()),
            };
            self.pos += next + 1;
            scope = Some(s);
        }
    }

    /// Parse a type name (possibly qualified, possibly a template-id)
    /// that is known to name a type.
    pub fn parse_type_name(&mut self) -> PR<QTy> {
        let line = self.line();
        let scope = self.parse_nested_name()?;
        let name = self.ident()?;
        let found = match scope {
            None => self.lookup(&name),
            Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
            Some(Err(())) => {
                if self.at("<") {
                    self.skip_balanced();
                }
                return Ok(QTy::unknown());
            }
        };
        let Some(ids) = found else {
            return Err(PErr { line, msg: format!("unknown type name '{}'", name) });
        };
        let id = *ids.iter().find(|&&i| matches!(self.decls[i].k, DK::Type(_) | DK::Class(_) | DK::Enum(_) | DK::TypeParam | DK::Template(_))).unwrap_or(&ids[0]);
        if self.at("<") && matches!(self.decls[id].k, DK::Template(_)) {
            self.skip_template_args()?;
            return Ok(QTy::unknown());
        }
        Ok(match &self.decls[id].k {
            DK::Type(q) => q.clone(),
            DK::Class(c) => {
                if self.classes[*c].template {
                    QTy::unknown()
                } else {
                    QTy::new(Ty::Class(*c))
                }
            }
            DK::Enum(e) => QTy::new(Ty::Enum(*e)),
            _ => QTy::unknown(),
        })
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

    /// `decltype(expr)` after the keyword (spec/expressions/decltype.md).
    pub fn parse_decltype(&mut self) -> PR<QTy> {
        self.expect("(")?;
        // An unparenthesized id-expression or member access gives the
        // declared type; anything else gives a type from the value
        // category (DTY rules).
        let parenthesized = self.at("(");
        self.in_unevaluated += 1;
        let e = self.parse_expression();
        self.in_unevaluated -= 1;
        let e = e?;
        self.expect(")")?;
        Ok(self.decltype_of(&e, parenthesized))
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
        for op in ops {
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
                DOp::MemPtr(c, cc, vv) => {
                    if t.t.is_ref() {
                        self.error(line, "DCL-23", "pointer to member of reference type");
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
                            self.error(line, "DCL-20", "array of incomplete class type")
                        }
                        _ => {}
                    }
                    if *n == Some(u64::MAX) {
                        QTy::unknown()
                    } else {
                        // CV-6: cv on an array applies to the elements.
                        let (c, v) = (t.c, t.v);
                        QTy { t: Ty::Array(Box::new(t), *n), c, v }
                    }
                }
                DOp::Func(f) => {
                    match &t.t {
                        // DCL-7: no function returning an array or a function.
                        Ty::Array(..) => self.error(line, "DCL-21", "function returning an array"),
                        Ty::Func(_) => self.error(line, "DCL-21", "function returning a function"),
                        _ => {}
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
                let class = match (scope, cname) {
                    (None, Ok(n)) => self.lookup(&n).and_then(|ids| self.class_of_decl(ids[0])),
                    (Some(Ok(s)), Ok(n)) => self.lookup_in_scope(s, &n, true).and_then(|ids| self.class_of_decl(ids[0])),
                    _ => None,
                };
                if !self.eat("::") || !self.eat("*") {
                    self.pos = save;
                    return Err(self.err("bad pointer-to-member declarator"));
                }
                let (c, v) = self.parse_cv();
                ptr_ops.push(DOp::MemPtr(class, c, v));
            } else {
                break;
            }
        }
        let mut d = Declarator {
            name: None,
            kind: NameKind::Plain,
            qual: None,
            qual_unknown: false,
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
            inner_ops = inner.ops;
        } else if self.is_ident() || self.at("::") || self.at("~") || self.at("operator") {
            self.parse_declarator_name(&mut d)?;
        } else if !abstract_ok {
            return Err(self.err(format!("expected a declarator, found '{}'", self.tok_desc())));
        }
        self.skip_attributes();
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
                    let n = self.array_bound(&e, bl);
                    suffixes.push(DOp::Array(n));
                }
            } else if self.at("(") && (d.name.is_some() || inner_ops.iter().any(|o| !matches!(o, DOp::Func(_))) || !inner_ops.is_empty() || abstract_ok) {
                // A parenthesized initializer looks like this too; the
                // caller decides with `looks_like_params` before calling.
                if d.name.is_some() && !param_ctx && !self.looks_like_params() {
                    break;
                }
                let f = self.parse_params()?;
                suffixes.push(DOp::Func(Box::new(f)));
            } else {
                break;
            }
        }
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
        let scope = self.parse_nested_name()?;
        match scope {
            Some(Ok(s)) => d.qual = Some(s),
            Some(Err(())) => d.qual_unknown = true,
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
            if self.at("<") && d.kind != NameKind::Conversion {
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
            let is_templ = match d.qual {
                Some(s) => self.lookup_in_scope(s, &n, true),
                None => self.lookup(&n),
            }
            .is_some_and(|ids| ids.iter().any(|&i| matches!(self.decls[i].k, DK::Template(_)) || self.func(i).is_some_and(|f| f.template)));
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
            let n = if matches!(t.lit, Some(crate::lexer::Lit::UserDefined)) { t.text.clone() } else { self.ident()? };
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
        let mut q = self.ds_base(&ds)?;
        loop {
            if self.eat("*") {
                let (c, v) = self.parse_cv();
                q = QTy { t: Ty::Ptr(Box::new(q)), c, v };
            } else if self.eat("&") {
                q = QTy::new(Ty::LRef(Box::new(q)));
            } else if self.eat("&&") {
                q = QTy::new(Ty::RRef(Box::new(q)));
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
            if !self.qualified_names_type(1) {
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
            return !self.at_k(k, "{");
        }
        false
    }

    /// Parse `( params ) cv ref noexcept -> trailing`.
    pub fn parse_params(&mut self) -> PR<FnDecl> {
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
            trailing: None,
            scope,
            defaults: 0,
            has_explicit_this: false,
            trailing_auto: None,
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
            if self.at("(") {
                self.pos += 1;
                let l = self.line();
                // the parameters are in scope in the noexcept operand
                self.stack.push(scope);
                let e = self.parse_expression();
                self.stack.pop();
                let e = e?;
                self.expect(")")?;
                // EXS-1 / CE-11: the operand is a constant of type bool.
                if let Some(v) = self.eval_bool_const(&e, l, "EXS-7") {
                    f.noexcept = v;
                }
            }
        } else if self.at("throw") && self.at_k(1, "(") {
            // dynamic exception specification: removed in C++17 except throw()
            self.pos += 1;
            self.skip_balanced();
            f.noexcept = true;
        }
        self.skip_attributes();
        if self.eat("->") {
            let save = self.stack.clone();
            self.stack.push(scope);
            let r = (|| -> PR<QTy> {
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
                f.has_explicit_this = true;
            }
            let ds = self.parse_decl_specs(None)?;
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
            let base = self.ds_base(&ds)?;
            let d = self.parse_declarator(true, true)?;
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
                    let e = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
                    // DEFARG-5: the default argument is valid for the
                    // parameter type at the declaration.
                    let form = if matches!(e.k, EK::InitList(_)) { crate::decl2::InitForm::CopyList } else { crate::decl2::InitForm::Copy };
                    let e = if ty.is_unknown() || self.in_template() { e } else { self.convert_init(e, &ty, form, dl) };
                    self.check_default_arg(&e, dl);
                    default = Some(e);
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
            if let Some((start, end)) = defer_default {
                let c = self.current_class_any().unwrap();
                self.deferred.push(Deferred { func: None, field: None, param: Some(id), start, end, stack: self.stack.clone(), templ: self.templ, class: c });
            }
            f.params.push(id);
            if !explicit_this {
                f.ptys.push(pty);
            }
            if d.pack {
                f.variadic = true;
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
            self.error(line, "DCL-19", "array bound must have integer or unscoped enumeration type");
            return Some(u64::MAX);
        }
        match self.const_int(e) {
            Ok(v) => {
                if v <= 0 {
                    self.error(line, "DCL-19", "array bound must be greater than zero");
                    return Some(1);
                }
                Some(v as u64)
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
