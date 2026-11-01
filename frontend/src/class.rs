// Class and enumeration specifiers.

use crate::ast::*;
use crate::decl2::Ctx;
use crate::lexer::TK;
use crate::parser::*;
use crate::types::*;

impl P {
    /// Parse `struct|class|union ...`. Returns the type and whether this
    /// was a definition.
    pub fn parse_class_specifier(&mut self) -> PR<(QTy, bool)> {
        let line = self.line();
        let start_pos = self.pos;
        let key = match self.tok().text.as_str() {
            "struct" => 0u8,
            "class" => 1,
            _ => 2,
        };
        self.pos += 1;
        self.last_align = 0;
        let outer_attrs = std::mem::take(&mut self.pending_attrs);
        self.skip_attributes();
        let own_attrs: Vec<String> = std::mem::replace(&mut self.pending_attrs, outer_attrs).into_iter().map(|a| a.name).collect();
        let class_align = self.last_align;
        // name
        let mut name = String::new();
        let mut qual: Option<Result<usize, ()>> = None;
        let mut spec_args = false;
        // The class of a specialization that a replay makes.
        let mut inst_req: Option<crate::templ::InstReq> = None;
        // The template and arguments after the name of an explicit or
        // partial specialization.
        let mut spec_targs: Option<(usize, Vec<TArg>)> = None;
        let explicit_spec = std::mem::take(&mut self.explicit_spec);
        if self.is_ident() || self.at("::") {
            self.last_qual_templ = None;
            qual = self.parse_nested_name()?;
            name = self.ident()?;
            if let Some(crate::templ::InstReq::Class { name: n, .. }) = &self.inst {
                if *n == name && qual.is_none() {
                    inst_req = self.inst.take();
                }
            }
            if self.at("<") {
                let tid = if inst_req.is_some() {
                    None
                } else {
                    let found = match qual {
                        None => self.lookup(&name),
                        Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
                        Some(Err(())) => None,
                    };
                    found.and_then(|ids| ids.iter().find_map(|&i| self.tid_of(i)))
                };
                if self.inst_depth == 0 && inst_req.is_none() {
                    let found_any = match qual {
                        None => self.lookup(&name).is_some(),
                        Some(Ok(s)) => self.lookup_in_scope(s, &name, true).is_some(),
                        Some(Err(())) => true,
                    };
                    if tid.is_none() && explicit_spec && found_any {
                        // TSP-4: an explicit specialization names a template.
                        self.error(line, "TSP-4", format!("'{}' is not a template", name));
                    } else if tid.is_none() && self.pending_template.is_some() && !found_any && !self.in_template_outer() {
                        // TSP-31: a partial specialization follows its primary
                        // template.
                        self.error(line, "TSP-31", format!("a partial specialization of '{}', which is not declared", name));
                    } else if tid.is_some() && !explicit_spec && self.pending_template.is_none() && !self.in_template() && self.at_class_head_def() {
                        // TSP-4: an explicit specialization starts with
                        // `template <>`.
                        self.error(line, "TSP-4", "an explicit specialization without 'template <>'");
                    }
                }
                match tid {
                    Some(t) if self.templs[t].kind == crate::templ::TKind::Class => {
                        let l = self.line();
                        let raw = self.parse_raw_targs()?;
                        if self.in_template() {
                            if let Some(pats) = self.pattern_targs(t, raw) {
                                spec_targs = Some((t, pats));
                            }
                        } else if let Ok(Some(args)) = self.targs_for(t, raw, l, false) {
                            spec_targs = Some((t, args));
                        }
                    }
                    _ => self.skip_template_args()?,
                }
                spec_args = true;
            }
        }
        if !name.is_empty() && qual.is_none() && (self.at("{") || self.at(":") || self.at("final")) {
            if let Some(oc) = self.current_class() {
                if self.classes[oc].name == name {
                    // CLS-9: no nested class with the name of the class.
                    self.error(line, "CLS-9", format!("nested class with the name of its class '{}'", name));
                }
            }
        }
        let is_final = self.at("final") && (self.at_k(1, "{") || self.at_k(1, ":"));
        if is_final {
            self.pos += 1;
        }
        let is_def = self.at("{") || (self.at(":") && !self.at_k(1, ":"));
        if spec_targs.is_some() && self.friend_spec_pos.is_some_and(|p| p + 1 == start_pos) && self.inst_depth == 0 {
            // TSP-12: no friend partial or explicit specialization.
            self.error(line, "TSP-12", "a friend declaration declares a partial or explicit specialization");
        }
        if let Some((t, args)) = &spec_targs {
            // TSP-32: the class key agrees with the primary template's.
            let pk = match self.decls[self.templs[*t].decl].k {
                DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => Some(self.classes[pc].key),
                _ => None,
            };
            if let Some(pk) = pk {
                if (pk == 2) != (key == 2) {
                    self.error(line, "TSP-32", "the class key of the specialization does not agree with the template's");
                }
            }
            if self.pending_template.is_some() && args.iter().any(|a| a.is_dep()) {
                if self.at(";") || is_def {
                    self.last_templ = Some(crate::templ::Last::Partial(*t, args.clone()));
                }
            } else if explicit_spec && !args.iter().any(|a| a.is_dep()) {
                return self.explicit_class_spec(*t, args.clone(), key, is_def, line);
            }
        }
        if let Some(req) = inst_req {
            return self.inst_class_body(req, key, is_def, line);
        }
        if !is_def {
            if name.is_empty() {
                return Err(self.err("anonymous class without a body"));
            }
            let q = self.elaborated_class(key, &name, qual, spec_args, line)?;
            if let Ty::Class(c) = q.t {
                self.classes[c].align = self.classes[c].align.max(class_align);
            }
            return Ok((q, false));
        }
        // definition
        let mut pending = self.pending_template.take();
        let is_template = pending.is_some() || spec_args || self.in_template();
        let cur = self.decl_scope();
        let mut target_scope = cur;
        let mut cid: Option<usize> = None;
        match qual {
            Some(Ok(s)) => {
                target_scope = s;
                if let Some(ids) = self.scopes[s].names.get(&name).cloned() {
                    for id in ids {
                        if let DK::Class(c) = self.decls[id].k {
                            cid = Some(c);
                        }
                    }
                }
                if cid.is_none() && !spec_args && !self.in_template() {
                    // TYN-7: a nested class defined outside must be declared.
                    self.error(line, "TYN-7", format!("'{}' is not declared in the named scope", name));
                }
            }
            Some(Err(())) => {}
            None => {
                if !spec_args && !name.is_empty() {
                    if let Some(ids) = self.scopes[cur].names.get(&name).cloned() {
                        for id in ids {
                            match &self.decls[id].k {
                                DK::Class(c) => cid = Some(*c),
                                DK::Template(TemplKind::Class(c)) if *c != usize::MAX => cid = Some(*c),
                                DK::Type(_) | DK::Var(_) | DK::Func(_) | DK::Enum(_) | DK::Namespace(_) => {
                                    if !matches!(self.decls[id].k, DK::Var(_) | DK::Func(_)) {
                                        self.error(line, "TYN-2", format!("'{}' already declared as something else", name));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        if spec_args {
            cid = None;
        }
        let c = match cid {
            Some(c) => {
                if self.classes[c].complete {
                    // TYN-7: a class must not be defined two times.
                    self.error(line, "TYN-7", format!("class '{}' defined twice", name));
                }
                if (self.classes[c].key == 2) != (key == 2) {
                    self.error(line, "TYN-10", "class key does not match the earlier declaration");
                }
                c
            }
            None => {
                let parent = if qual.is_some() { target_scope } else { self.cur_scope() };
                let scope = self.new_scope(SK::Class(self.classes.len()), Some(parent), &name);
                self.classes.push(ClassInfo {
                    name: name.clone(),
                    key,
                    scope,
                    complete: false,
                    fields: vec![],
                    bases: vec![],
                    has_conv: false,
                    conv_targets: vec![],
                    ctors: vec![],
                    dtor: None,
                    has_virtual: false,
                    template: is_template,
                    opaque: spec_args,
                    line,
                    align: 0,
                    anon_unions: vec![],
                });
                let c = self.classes.len() - 1;
                if !name.is_empty() && !spec_args && !matches!(qual, Some(Err(()))) {
                    let k = if pending.is_some() { DK::Template(TemplKind::Class(c)) } else { DK::Class(c) };
                    let id = self.add_decl_in(target_scope, &name, k, line);
                    if pending.is_some() {
                        self.pending_template = pending;
                        self.class_templ_early(c, id);
                        pending = self.pending_template.take();
                    }
                }
                c
            }
        };
        if pending.is_some() && name.is_empty() && self.inst_depth == 0 {
            // TPL-4: a class template has a name.
            self.error(line, "TPL-4", "a class without a name after a template head");
        }
        if pending.is_some() && !spec_args && !matches!(qual, Some(Err(()))) && !name.is_empty() {
            self.last_templ = Some(crate::templ::Last::Class(c));
            if pending.as_ref().is_some_and(|p| p.params.contains(&name)) && self.inst_depth == 0 {
                // TPL-18: a class template with the name of its parameter.
                self.error(line, "TPL-18", format!("the class template '{}' has the name of its own parameter", name));
            }
        }
        if matches!(self.last_templ, Some(crate::templ::Last::Partial(..))) && spec_args {
            self.last_partial_class = Some(c);
        }
        if pending.is_some() && matches!(qual, Some(Err(()))) {
            if let Some((t, pats)) = self.last_qual_templ.take() {
                self.last_templ = Some(crate::templ::Last::Member(t, pats));
            }
        }
        self.classes[c].template |= is_template;
        if !own_attrs.is_empty() {
            self.class_attrs.entry(c).or_default().extend(own_attrs.clone());
        }
        self.classes[c].align = self.classes[c].align.max(class_align);
        let class_scope = self.classes[c].scope;
        // bases
        if self.eat(":") {
            loop {
                self.skip_attributes();
                let mut virt = false;
                let mut access = if key == 1 { 2 } else { 0 };
                loop {
                    if self.eat("virtual") {
                        virt = true;
                    } else if self.eat("public") {
                        access = 0;
                    } else if self.eat("protected") {
                        access = 1;
                    } else if self.eat("private") {
                        access = 2;
                    } else {
                        break;
                    }
                }
                let bl = self.line();
                let bq = if self.tok().kind == TK::Ident && crate::decl::simple_type_kw(&self.tok().text) {
                    let t = self.parse_type_id()?;
                    self.error(bl, "DER-1", "base is not a class type");
                    let _ = t;
                    QTy::unknown()
                } else if self.at("decltype") {
                    self.pos += 1;
                    self.parse_decltype()?
                } else if self.qualified_names_type(0) {
                    self.in_base_clause += 1;
                    let r = self.parse_type_name();
                    self.in_base_clause -= 1;
                    r?
                } else {
                    let n = self.tok().text.clone();
                    self.skip_qualified_name()?;
                    if !self.lookup_uncertain() && !is_template && self.lookup(&n).is_none() {
                        self.error(bl, "DER-1", format!("base '{}' is not a class", n));
                    }
                    QTy::unknown()
                };
                self.eat("...");
                self.require_ty(&bq, bl);
                let bc = match bq.t {
                    Ty::Class(b) => {
                        if !self.classes[b].complete {
                            // DRV: a base class must be complete.
                            self.error(bl, "DER-1", "base class is incomplete");
                            None
                        } else {
                            Some(b)
                        }
                    }
                    Ty::Unknown | Ty::Dep(_) => None,
                    _ => {
                        self.error(bl, "DER-1", "base is not a class type");
                        None
                    }
                };
                if let Some(b) = bc {
                    if self.classes[b].has_virtual {
                        self.classes[c].has_virtual = true;
                    }
                    if self.classes[b].key == 2 || key == 2 {
                        self.error(bl, "DER-1", "a union cannot be a base or have a base");
                    }
                }
                self.classes[c].bases.push(BaseInfo { class: bc, virt, access });
                if !self.eat(",") {
                    break;
                }
            }
        }
        self.expect("{")?;
        let saved = self.stack.clone();
        if let Some(Ok(s)) = qual {
            let _ = s;
        }
        self.stack.push(class_scope);
        // injected class name
        if !name.is_empty() && self.scopes[class_scope].names.get(&name).is_none() {
            let id = self.add_decl_in(class_scope, &name, DK::Class(c), line);
            if let Some(&t) = self.class_tid.get(&c) {
                self.injected.insert(id, t);
            }
        }
        self.class_depth += 1;
        self.access_stack.push(if self.classes[c].key == 1 { 2 } else { 0 });
        let saved_templ = self.templ;
        if is_template && self.templ == 0 {
            self.templ = 1;
        }
        while !self.at("}") && self.tok().kind != TK::Eof {
            self.parse_declaration_recover(Ctx::Class(c));
        }
        self.templ = saved_templ;
        self.access_stack.pop();
        self.expect("}")?;
        self.class_depth -= 1;
        self.stack = saved;
        self.classes[c].complete = true;
        self.pending_template = None;
        if self.class_depth == 0 {
            self.run_deferred();
        }
        if pending.is_some() {
            self.templ_class_defined = true;
        }
        if is_template {
            return Ok((QTy::unknown(), true));
        }
        Ok((QTy::new(Ty::Class(c)), true))
    }

    fn elaborated_class(&mut self, key: u8, name: &str, qual: Option<Result<usize, ()>>, spec_args: bool, line: u32) -> PR<QTy> {
        if spec_args {
            return Ok(QTy::unknown());
        }
        let decl_only = self.at(";");
        let found = match qual {
            Some(Ok(s)) => self.lookup_in_scope(s, name, true),
            Some(Err(())) => return Ok(QTy::unknown()),
            None => {
                if decl_only {
                    let cur = self.decl_scope();
                    self.scopes[cur].names.get(name).cloned().filter(|v| !v.is_empty())
                } else {
                    self.lookup_tag(name)
                }
            }
        };
        if let Some(ids) = found {
            for id in ids {
                match &self.decls[id].k {
                    DK::Class(c) => {
                        let c = *c;
                        if (self.classes[c].key == 2) != (key == 2) {
                            // TYN-4: union for a class, or the reverse.
                            self.error(line, "TYN-10", "class key does not match the declaration");
                        }
                        return Ok(QTy::new(Ty::Class(c)));
                    }
                    DK::Template(TemplKind::Class(pc)) => {
                        if self.pending_template.is_some() && *pc != usize::MAX {
                            // TPL-15: another declaration adds defaults.
                            self.last_templ = Some(crate::templ::Last::Class(*pc));
                        }
                        return Ok(QTy::unknown());
                    }
                    DK::Enum(_) => {
                        self.error(line, "TYN-10", "class key used for an enumeration");
                        return Ok(QTy::unknown());
                    }
                    DK::Type(q) => {
                        if let Ty::Class(c) = q.t {
                            // an alias after a class key is not valid
                            self.error(line, "TYN-10", "elaborated type specifier names a typedef");
                            return Ok(QTy::new(Ty::Class(c)));
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Some(Ok(_)) = qual {
            if decl_only {
                // TYN-7: a class key and a qualified name alone.
                self.error(line, "TYN-7", "declaration of a qualified class name without a body");
            } else {
                self.error(line, "NAM-8", format!("no class named '{}' in the named scope", name));
            }
            return Ok(QTy::unknown());
        }
        // TYN-4: declare the class in the enclosing namespace or block.
        let mut scope = self.decl_scope();
        if !decl_only {
            let in_body = !self.fns.is_empty();
            for &s in self.stack.iter().rev() {
                match self.scopes[s].kind {
                    // The outermost block of a function body shares the
                    // parameter scope.
                    SK::Namespace | SK::Block => {
                        scope = s;
                        break;
                    }
                    SK::Params if in_body => {
                        scope = s;
                        break;
                    }
                    _ => {}
                }
            }
        }
        let cscope = self.new_scope(SK::Class(self.classes.len()), Some(scope), name);
        self.classes.push(ClassInfo {
            name: name.to_string(),
            key,
            scope: cscope,
            complete: false,
            fields: vec![],
            bases: vec![],
            has_conv: false,
            conv_targets: vec![],
            ctors: vec![],
            dtor: None,
            has_virtual: false,
            template: self.pending_template.is_some(),
            opaque: false,
            line,
            align: 0,
            anon_unions: vec![],
        });
        let c = self.classes.len() - 1;
        let k = if self.pending_template.is_some() { DK::Template(TemplKind::Class(c)) } else { DK::Class(c) };
        let id = self.add_decl_in(scope, name, k, line);
        if self.pending_template.is_some() {
            self.class_templ_early(c, id);
            self.last_templ = Some(crate::templ::Last::Class(c));
            return Ok(QTy::unknown());
        }
        Ok(QTy::new(Ty::Class(c)))
    }

    /// Whether scope `outer` is `inner` or encloses it.
    pub fn scope_encloses(&self, outer: usize, inner: usize) -> bool {
        let mut s = Some(inner);
        while let Some(x) = s {
            if x == outer {
                return true;
            }
            s = self.scopes[x].parent;
        }
        false
    }

    /// Whether the parser is inside an enclosing template head (a member
    /// template's head inside a class template).
    fn in_template_outer(&self) -> bool {
        self.templ > 1
    }

    /// At the end of a class head before its body or base list, after
    /// the template arguments.
    fn at_class_head_def(&self) -> bool {
        let mut k = 0;
        let mut depth = 0i32;
        loop {
            let t = self.peek(k);
            if t.kind == TK::Eof {
                return false;
            }
            match t.text.as_str() {
                "<" => depth += 1,
                ">" => {
                    depth -= 1;
                    if depth == 0 {
                        return self.at_k(k + 1, "{") || (self.at_k(k + 1, ":") && !self.at_k(k + 2, ":"));
                    }
                }
                ">>" => {
                    depth -= 2;
                    if depth <= 0 {
                        return self.at_k(k + 1, "{") || (self.at_k(k + 1, ":") && !self.at_k(k + 2, ":"));
                    }
                }
                ";" => return false,
                _ => {}
            }
            k += 1;
        }
    }

    /// `template <> class C<args> ...`: an ordinary class for one
    /// specialization (TSP-1, TSP-20).
    fn explicit_class_spec(&mut self, t: usize, args: Vec<TArg>, key: u8, is_def: bool, line: u32) -> PR<(QTy, bool)> {
        if self.inst_depth == 0 && !self.scope_encloses(self.decl_scope(), self.templs[t].scope) {
            // TSP-5: in the template's namespace or one that encloses it.
            self.error(line, "TSP-5", "an explicit specialization outside the namespace of its template");
        }
        if self.templs[t].insts.iter().any(|x| x.0 == args && matches!(x.1, crate::templ::Inst::Class(c) if self.classes[c].complete)) && is_def {
            // TSP-6: declared after a use that made the specialization.
            let name = self.templs[t].name.clone();
            self.error(line, "TSP-6", format!("explicit specialization of '{}' after its first use", name));
        }
        let existing = self.templs[t].explicit.iter().find(|x| x.0 == args).and_then(|x| match x.1 {
            crate::templ::Inst::Class(c) => Some(c),
            _ => None,
        });
        let lazy = self.templs[t].insts.iter().position(|x| x.0 == args && matches!(x.1, crate::templ::Inst::Class(c) if !self.classes[c].complete));
        let c = match existing {
            Some(c) => c,
            None => {
                // A use that named the specialization before is the same
                // class (INST-2).
                let c = match lazy {
                    Some(k) => {
                        let crate::templ::Inst::Class(c) = self.templs[t].insts.remove(k).1 else { unreachable!() };
                        c
                    }
                    None => self.new_inst_class(t, &args, line),
                };
                self.classes[c].key = key;
                self.templs[t].explicit.push((args.clone(), crate::templ::Inst::Class(c)));
                c
            }
        };
        if !is_def {
            return Ok((QTy::new(Ty::Class(c)), false));
        }
        if self.classes[c].complete {
            self.error(line, "TYN-7", "explicit specialization defined twice");
        }
        self.class_body(c, key, line)?;
        Ok((QTy::new(Ty::Class(c)), true))
    }

    /// The definition of the class of a specialization, in a replay.
    fn inst_class_body(&mut self, req: crate::templ::InstReq, key: u8, is_def: bool, line: u32) -> PR<(QTy, bool)> {
        let crate::templ::InstReq::Class { tid, args, existing, .. } = req else { unreachable!() };
        let c = match existing {
            Some(c) => c,
            None => {
                let c = self.new_inst_class(tid, &args, line);
                self.classes[c].key = key;
                c
            }
        };
        if let Some(k) = self.templs[tid].insts.iter().position(|x| x.0 == args) {
            self.templs[tid].insts[k].1 = crate::templ::Inst::Class(c);
        }
        self.inst_out = Some(crate::templ::InstOut::Class(c));
        if !is_def {
            return Ok((QTy::new(Ty::Class(c)), false));
        }
        self.inst_busy.insert(c);
        let r = self.class_body(c, key, line);
        self.inst_busy.remove(&c);
        r?;
        Ok((QTy::new(Ty::Class(c)), true))
    }

    /// The bases and the member list of a class that is not a template:
    /// an explicit specialization or an instance.
    fn class_body(&mut self, c: usize, key: u8, line: u32) -> PR<()> {
        let name = self.classes[c].name.clone();
        let class_scope = self.classes[c].scope;
        if self.at("final") {
            self.pos += 1;
        }
        if self.eat(":") {
            loop {
                self.skip_attributes();
                let mut virt = false;
                let mut access = if key == 1 { 2 } else { 0 };
                loop {
                    if self.eat("virtual") {
                        virt = true;
                    } else if self.eat("public") {
                        access = 0;
                    } else if self.eat("protected") {
                        access = 1;
                    } else if self.eat("private") {
                        access = 2;
                    } else {
                        break;
                    }
                }
                let bl = self.line();
                let hits = self.binding_hits;
                let bq = if self.at("decltype") {
                    self.pos += 1;
                    self.parse_decltype()?
                } else if self.qualified_names_type(0) || self.at("typename") {
                    self.eat("typename");
                    self.in_base_clause += 1;
                    let r = self.parse_type_name();
                    self.in_base_clause -= 1;
                    r?
                } else {
                    self.skip_qualified_name()?;
                    QTy::unknown()
                };
                self.eat("...");
                self.require_ty(&bq, bl);
                let bc = match bq.t {
                    Ty::Class(b) => {
                        if !self.classes[b].complete {
                            self.error(bl, "DER-1", "base class is incomplete");
                            None
                        } else {
                            Some(b)
                        }
                    }
                    Ty::Unknown | Ty::Dep(_) => None,
                    _ => {
                        self.error(bl, "DER-1", "base is not a class type");
                        None
                    }
                };
                if let Some(b) = bc {
                    if self.classes[b].has_virtual {
                        self.classes[c].has_virtual = true;
                    }
                    if self.binding_hits > hits && self.inst_of.contains_key(&c) {
                        // DEP-11: a base whose type depends on a parameter.
                        self.dep_bases.insert((c, b));
                    }
                }
                self.classes[c].bases.push(BaseInfo { class: bc, virt, access });
                if !self.eat(",") {
                    break;
                }
            }
        }
        self.expect("{")?;
        let saved = self.stack.clone();
        self.stack.push(class_scope);
        if !name.is_empty() && self.scopes[class_scope].names.get(&name).is_none() {
            let id = self.add_decl_in(class_scope, &name, DK::Class(c), line);
            if let Some((t, _)) = self.inst_of.get(&c) {
                self.injected.insert(id, *t);
            }
        }
        self.class_depth += 1;
        self.access_stack.push(if key == 1 { 2 } else { 0 });
        let saved_templ = self.templ;
        self.templ = 0;
        while !self.at("}") && self.tok().kind != TK::Eof {
            self.parse_declaration_recover(Ctx::Class(c));
        }
        self.templ = saved_templ;
        self.access_stack.pop();
        self.expect("}")?;
        self.class_depth -= 1;
        self.stack = saved;
        self.classes[c].complete = true;
        if self.class_depth == 0 {
            self.run_deferred();
        }
        Ok(())
    }

    /// Lookup that ignores names other than classes and enumerations
    /// (TYN-4).
    pub fn lookup_tag(&self, name: &str) -> Option<Vec<DeclId>> {
        let noms = self.nominations();
        for &s in self.stack.iter().rev() {
            if let (Some(v), _) = self.lookup_level(s, name, &noms) {
                let tags: Vec<DeclId> = v.into_iter().filter(|&i| matches!(self.decls[i].k, DK::Class(_) | DK::Enum(_) | DK::Template(_) | DK::Type(_))).collect();
                if !tags.is_empty() {
                    return Some(tags);
                }
            }
        }
        None
    }

    pub fn parse_enum_specifier(&mut self, _ds: &crate::decl::DeclSpec) -> PR<(QTy, bool)> {
        let line = self.line();
        self.expect("enum")?;
        let scoped = self.eat("class") || self.eat("struct");
        self.skip_attributes();
        let mut name = String::new();
        let mut qual = None;
        if self.is_ident() || self.at("::") {
            qual = self.parse_nested_name()?;
            name = self.ident()?;
        }
        let mut fixed: Option<Ty> = None;
        if self.at(":") && !self.at_k(1, ":") {
            self.pos += 1;
            let fl = self.line();
            let q = self.parse_type_id()?;
            match &q.t {
                t if t.is_integral() => fixed = Some(t.clone()),
                Ty::Unknown | Ty::Dep(_) => fixed = Some(Ty::Int),
                Ty::Enum(e) => {
                    let _ = e;
                    self.error(fl, "ENUM-17", "underlying type must be an integer type");
                    fixed = Some(Ty::Int);
                }
                _ => {
                    // ENUM-4: a fixed underlying type must be an integer type.
                    self.error(fl, "ENUM-17", "underlying type must be an integer type");
                    fixed = Some(Ty::Int);
                }
            }
        }
        let is_def = self.at("{");
        let cur = self.decl_scope();
        // Find an earlier declaration.
        let prev = match qual {
            Some(Ok(s)) => self.lookup_in_scope(s, &name, false),
            Some(Err(())) => return Ok((QTy::unknown(), false)),
            None => {
                if is_def || self.at(";") {
                    self.scopes[cur].names.get(&name).cloned().filter(|v| !v.is_empty())
                } else {
                    self.lookup_tag(&name)
                }
            }
        };
        let mut eid = None;
        if !name.is_empty() {
            if let Some(ids) = prev {
                for id in ids {
                    match &self.decls[id].k {
                        DK::Enum(e) => eid = Some(*e),
                        DK::Class(_) => {
                            self.error(line, "TYN-10", "enum key used for a class");
                            return Ok((QTy::unknown(), false));
                        }
                        DK::Type(q) => {
                            if let Ty::Enum(e) = q.t {
                                if !is_def && !self.at(";") {
                                    return Ok((QTy::new(Ty::Enum(e)), false));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if !is_def && !self.at(";") && fixed.is_none() {
            // elaborated reference
            if scoped {
                // TYN-4: `enum class` in a reference to an enumeration.
                self.error(line, "TYN-10", "'enum class' in a reference to an enumeration");
            }
            return match eid {
                Some(e) => Ok((QTy::new(Ty::Enum(e)), false)),
                None => {
                    if self.lookup_uncertain() {
                        return Ok((QTy::unknown(), false));
                    }
                    self.error(line, "TYN-7", format!("unknown enumeration '{}'", name));
                    Ok((QTy::unknown(), false))
                }
            };
        }
        let opaque = !is_def;
        if opaque && !scoped && fixed.is_none() {
            // ENUM-7 / TYN-7: an opaque unscoped declaration needs a fixed type.
            self.error(line, "ENUM-18", "opaque declaration of an unscoped enumeration without a fixed underlying type");
        }
        let e = match eid {
            Some(e) => {
                let ei = self.enums[e].clone();
                if ei.scoped != scoped {
                    self.error(line, "ENUM-18", "enumeration redeclared with a different scope kind");
                }
                let old_fixed = if ei.scoped && ei.fixed.is_none() { Some(Ty::Int) } else { ei.fixed.clone() };
                let new_fixed = if scoped && fixed.is_none() { Some(Ty::Int) } else { fixed.clone() };
                if old_fixed != new_fixed {
                    self.error(line, "ENUM-18", "enumeration redeclared with a different underlying type");
                }
                if is_def && ei.complete {
                    self.error(line, "TYN-7", "enumeration defined twice");
                }
                e
            }
            None => {
                let scope = self.new_scope(SK::Enum(self.enums.len()), Some(self.cur_scope()), &name);
                let underlying = fixed.clone().unwrap_or(Ty::Int);
                self.enums.push(EnumInfo { name: name.clone(), scoped, fixed: if scoped && fixed.is_none() { Some(Ty::Int) } else { fixed.clone() }, underlying, complete: false, enumerators: vec![], scope });
                let e = self.enums.len() - 1;
                if !name.is_empty() {
                    if let Some(prev) = self.scopes[cur].names.get(&name).cloned() {
                        for p in prev {
                            if !matches!(self.decls[p].k, DK::Var(_) | DK::Func(_)) {
                                self.error(line, "NAM-3", format!("'{}' already declared", name));
                            }
                        }
                    }
                    self.add_decl_in(cur, &name, DK::Enum(e), line);
                }
                e
            }
        };
        if !is_def {
            return Ok((QTy::new(Ty::Enum(e)), false));
        }
        self.parse_enumerators(e, line)?;
        Ok((QTy::new(Ty::Enum(e)), true))
    }

    fn parse_enumerators(&mut self, e: usize, _line: u32) -> PR<()> {
        self.expect("{")?;
        let escope0 = self.enums[e].scope;
        self.stack.push(escope0);
        let r = self.parse_enumerators_inner(e);
        self.stack.pop();
        r
    }

    fn parse_enumerators_inner(&mut self, e: usize) -> PR<()> {
        let scoped = self.enums[e].scoped;
        let fixed = self.enums[e].fixed.clone();
        let escope = self.enums[e].scope;
        let outer = self.decl_scope();
        let mut next: i128 = 0;
        let mut prev_overflow = false;
        let mut values = Vec::new();
        let mut prev_ty: Option<Ty> = None;
        let mut init_ty: Option<Ty>;
        while !self.at("}") {
            init_ty = None;
            let line = self.line();
            let name = self.ident()?;
            self.pending_attrs.clear();
            self.skip_attributes();
            let en_attrs: Vec<String> = std::mem::take(&mut self.pending_attrs).into_iter().map(|a| a.name).collect();
            let mut v = next;
            if prev_overflow && !self.at("=") {
                // ENUM-1: an implicit value must fit the underlying type.
                self.error(line, "ENUM-14", "implicit enumerator value does not fit");
            }
            if self.eat("=") {
                // ENUM-12: the name is not in scope in its own value.
                let el = self.line();
                let ex = self.parse_conditional()?;
                let et = nonref(&ex.ty);
                init_ty = match &et.t {
                    Ty::Enum(x) if self.enums[*x].complete => Some(Ty::Enum(*x)),
                    t if t.is_integral() => Some(crate::types::Ty::clone(t)),
                    Ty::Enum(_) => None,
                    _ => None,
                };
                if !(et.t.is_integral() || matches!(et.t, Ty::Enum(_) | Ty::Unknown | Ty::Dep(_))) {
                    self.error(el, "CE-25", "enumerator value must have integer or enumeration type");
                }
                match self.const_int(&ex) {
                    Ok(x) => v = x,
                    Err(crate::consteval::CeErr::NotConst(r, m)) => {
                        if !self.in_template() {
                            self.error_because(el, "ENUM-14", r, format!("enumerator value is not a constant expression ({}: {})", r, m));
                        }
                    }
                    Err(crate::consteval::CeErr::Unsupported(m)) => self.unsupported(el, format!("enumerator value: {}", m)),
                }
                if let Some(f) = &fixed {
                    if !fits(v, f) && !ex.ty.is_unknown() {
                        // ENUM-4: each value must fit without narrowing.
                        self.error(el, "ENUM-17", format!("enumerator value {} does not fit the underlying type", v));
                    }
                }
            }
            if let Some(f) = &fixed {
                prev_overflow = v >= f.int_max();
            } else {
                prev_overflow = v >= i64::MAX as i128 && v >= u64::MAX as i128;
            }
            next = v + 1;
            values.push((name.clone(), v));
            // declare
            if self.scopes[escope].names.contains_key(&name) {
                self.error(line, "ENUM-14", format!("enumerator '{}' declared twice", name));
            }
            let id = self.add_decl_in(escope, &name, DK::Enumerator(e, v), line);
            if !en_attrs.is_empty() {
                self.entity_attrs.insert(id, en_attrs);
            }
            // ENUM-10: the type inside the list. A fixed type gives that
            // type. Otherwise the initializer's type, or the type of the
            // enumerator before if it holds the value (else ENUM-13).
            let in_ty = if let Some(f) = &fixed {
                f.clone()
            } else if let Some(t) = init_ty.take() {
                t
            } else {
                match &prev_ty {
                    Some(p) if fits(v, p) => p.clone(),
                    Some(p) => [Ty::Int, Ty::UInt, Ty::Long, Ty::ULong, Ty::LLong, Ty::ULLong].into_iter().find(|t| fits(v, t) && t.int_max() >= p.int_max()).unwrap_or(Ty::LLong),
                    None => Ty::Int,
                }
            };
            prev_ty = Some(in_ty.clone());
            self.enumerator_ty.insert(id, in_ty);
            if !scoped {
                // ENUM-2: unscoped enumerators go in the enclosing scope.
                if let Some(prev) = self.scopes[outer].names.get(&name) {
                    if !prev.is_empty() {
                        self.error(line, "ENUM-15", format!("enumerator '{}' conflicts with an earlier declaration", name));
                    }
                }
                self.scopes[outer].names.entry(name.clone()).or_default().push(id);
            }
            if !self.eat(",") {
                break;
            }
        }
        self.expect("}")?;
        // ENUM-5: choose the underlying type of an unscoped enumeration
        // without a fixed type.
        if fixed.is_none() {
            let min = values.iter().map(|x| x.1).min().unwrap_or(0);
            let max = values.iter().map(|x| x.1).max().unwrap_or(0);
            let cands = [Ty::Int, Ty::UInt, Ty::Long, Ty::ULong, Ty::LLong, Ty::ULLong];
            let mut chosen = None;
            for c in cands {
                if fits(min, &c) && fits(max, &c) {
                    chosen = Some(c);
                    break;
                }
            }
            match chosen {
                Some(c) => self.enums[e].underlying = c,
                None => {
                    self.error(self.line(), "ENUM-5", "no integer type can hold all enumerator values");
                }
            }
        } else if let Some(f) = &fixed {
            if scoped {
                for (_, v) in &values {
                    if !fits(*v, f) {
                        self.error(self.line(), "ENUM-17", "enumerator value does not fit in int");
                        break;
                    }
                }
            }
        }
        self.enums[e].enumerators = values;
        self.enums[e].complete = true;
        Ok(())
    }

    /// ENUM-6: the range of values of an enumeration without a fixed type.
    pub fn enum_range(&self, e: usize) -> (i128, i128) {
        let ei = &self.enums[e];
        if let Some(f) = &ei.fixed {
            return (f.int_min(), f.int_max());
        }
        let min = ei.enumerators.iter().map(|x| x.1).min().unwrap_or(0);
        let max = ei.enumerators.iter().map(|x| x.1).max().unwrap_or(0);
        if ei.enumerators.is_empty() {
            return (0, 1);
        }
        // smallest bit-field that holds all values
        let mut bits = 1u32;
        loop {
            let (lo, hi) = if min < 0 { (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1) } else { (0, (1i128 << bits) - 1) };
            if min >= lo && max <= hi {
                return (lo, hi);
            }
            bits += 1;
            if bits > 64 {
                return (min, max);
            }
        }
    }

    pub fn expr_is_type_unknown(&self, e: &Expr) -> bool {
        e.ty.is_unknown()
    }
}
