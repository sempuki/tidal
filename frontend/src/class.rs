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
        let attr_seen = self.pos != start_pos + 1;
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
                // A partial specialization of a member template defined
                // outside its class has a qualified name (TSP-9).
                if *n == name {
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
                        let at = self.pos;
                        let raw = self.parse_raw_targs()?;
                        // DEP-15: the argument tokens of a partial
                        // specialization, for the names of its current
                        // instantiation.
                        self.spec_arg_toks = self.toks[at + 1..self.pos.saturating_sub(1).max(at + 1)].iter().map(|t| t.text.clone()).collect();
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
        // ACC-20: a friend without a template head names one
        // specialization, which is valid.
        let headed = explicit_spec || self.pending_template.is_some() || is_def;
        if spec_targs.is_some() && headed && self.friend_spec_pos.is_some_and(|p| p + 1 == start_pos) && self.inst_depth == 0 {
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
                self.head_align = if is_def { class_align } else { 0 };
                return self.explicit_class_spec(*t, args.clone(), key, is_def, line);
            }
        }
        if let Some(req) = inst_req {
            self.head_align = if is_def { class_align } else { 0 };
            return self.inst_class_body(req, key, is_def, line);
        }
        if let Some((t, args)) = &spec_targs {
            // ACC-20: a friend declaration without a template head names
            // one specialization.
            if !headed && !is_def && self.friend_spec_pos.is_some_and(|p| p + 1 == start_pos) && !self.in_template() && self.inst_depth == 0 && !args.iter().any(|a| a.is_dep()) {
                return Ok((self.class_inst(*t, args.clone(), line), false));
            }
        }
        if !is_def {
            if name.is_empty() {
                return Err(self.err("anonymous class without a body"));
            }
            if attr_seen && !self.at(";") && self.inst_depth == 0 {
                // TYN-10 (Q-NEW-elaborated-type-attribute): an attribute
                // after the class key belongs to a declaration of the
                // class alone.
                self.error(line, "TYN-10", "an attribute or alignment specifier after the class key of an elaborated type specifier that is not a declaration of the class alone");
            }
            let friend = self.friend_spec_pos.is_some_and(|p| p + 1 == start_pos);
            let q = self.elaborated_class(key, &name, qual, spec_args, friend, line)?;
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
        if let Some(Ok(s)) = qual {
            // TSP-21: `template <> template <...> struct C<A>::M` gives the
            // member template M of the instance C<A> again. The template
            // that the instance made, with its partial specializations, is
            // no longer used.
            let of_instance = matches!(self.scopes[s].kind, SK::Class(o) if self.inst_of.contains_key(&o));
            if explicit_spec && pending.is_some() && !spec_args && of_instance && self.inst_depth == 0 {
                if let Some(v) = self.scopes[s].names.get_mut(&name) {
                    let decls = &self.decls;
                    v.retain(|&id| !matches!(decls[id].k, DK::Template(TemplKind::Class(_))));
                }
            }
        }
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
                                    } else if pending.is_some() && self.inst_depth == 0 {
                                        // NAM-42: a class template does not
                                        // share its name with a variable or
                                        // a function.
                                        self.error(line, "NAM-42", format!("the class template '{}' has the name of another entity in its scope", name));
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
        if let (Some(c), Some(Ok(s))) = (cid, qual) {
            // TSP-22: an explicit specialization of a member class of an
            // instance replaces the class that the instance made.
            let of_instance = matches!(self.scopes[s].kind, SK::Class(o) if self.inst_of.contains_key(&o));
            if explicit_spec && of_instance && self.classes[c].complete && self.inst_depth == 0 && self.explicit_member_classes.insert(c) {
                let sc = self.classes[c].scope;
                self.scopes[sc].names.clear();
                let ci = &mut self.classes[c];
                ci.complete = false;
                ci.fields.clear();
                ci.bases.clear();
                ci.ctors.clear();
                ci.dtor = None;
                ci.has_virtual = false;
                ci.has_conv = false;
                ci.conv_targets.clear();
                ci.anon_unions.clear();
                ci.align = 0;
            }
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
            let toks = std::mem::take(&mut self.spec_arg_toks);
            self.partial_arg_toks.insert(c, toks);
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
        if !is_template && !self.in_template() && self.classes[c].align > 0 && class_align != self.classes[c].align {
            // ALN-3: the definition has the alignas of an earlier
            // declaration.
            self.error(line, "ALN-3", if class_align == 0 { "a class definition without the alignas of an earlier declaration" } else { "a class definition with an alignas different from an earlier declaration" });
        }
        self.classes[c].align = self.classes[c].align.max(class_align);
        let class_scope = self.classes[c].scope;
        // bases
        if is_final {
            self.final_classes.insert(c);
        }
        // NAM-21: the base clause of a class defined with a qualified name
        // sees the members of the qualifier's scope.
        let base_stack = self.stack.clone();
        if let Some(Ok(s)) = qual {
            if self.at(":") && self.stack.last() != Some(&s) {
                self.stack.push(s);
            }
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
                let bq = if self.tok().kind == TK::Ident && crate::decl::simple_type_kw(&self.tok().text) {
                    let t = self.parse_type_id()?;
                    self.error(bl, "DER-1", "base is not a class type");
                    let _ = t;
                    QTy::unknown()
                } else if self.at("decltype") {
                    self.pos += 1;
                    self.parse_decltype()?
                } else if (self.in_template() && self.dependent_qualifier_at() && self.less_after_qualified(0)) || self.has_fact(self.pos, crate::templ::FACT_LESS) {
                    // DEP-24: a base named `T::box<...>` without
                    // `template`.
                    self.in_base_clause += 1;
                    let r = self.parse_typename_type();
                    self.in_base_clause -= 1;
                    r?
                } else if {
                    // NAM-40: a base name finds a class that a variable hides.
                    self.in_base_clause += 1;
                    let t = self.qualified_names_type(0);
                    self.in_base_clause -= 1;
                    t
                } {
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
                let expand = self.eat("...");
                self.class_base_types.entry(c).or_default().push((bq.clone(), expand));
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
                    if !self.in_template() {
                        // DER-1: a class is a direct base once, and a final
                        // class is no base.
                        if self.classes[c].bases.iter().any(|x| x.class == Some(b)) {
                            self.error(bl, "DER-1", format!("'{}' is a direct base twice", self.classes[b].name));
                        } else if self.final_classes.contains(&b) {
                            self.error(bl, "DER-1", format!("'{}' is final and cannot be a base", self.classes[b].name));
                        }
                    }
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
        self.stack = base_stack;
        self.expect("{")?;
        let saved = self.stack.clone();
        if let Some(Ok(s)) = qual {
            // NAM-21 (Q-0377): the body of a class defined with a
            // qualified name sees the members of the qualifier's scope.
            if self.stack.last() != Some(&s) {
                self.stack.push(s);
            }
        }
        self.stack.push(class_scope);
        // injected class name
        if !name.is_empty() && self.scopes[class_scope].names.get(&name).is_none() {
            let id = self.add_decl_in(class_scope, &name, DK::Class(c), line);
            if let Some(&t) = self.class_tid.get(&c) {
                self.injected.insert(id, t);
            } else if self.last_partial_class == Some(c) {
                // TARG-11: in a partial specialization, the name with an
                // argument list names the template.
                if let Some(crate::templ::Last::Partial(t, _)) = &self.last_templ {
                    self.injected.insert(id, *t);
                }
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
        if !is_template && class_align > 0 && !self.in_template() && !self.class_uncertain(c) {
            // ALN-2: the alignas is not weaker than the natural alignment
            // of the class.
            let own = std::mem::replace(&mut self.classes[c].align, 0);
            let natural = self.layout(c).map(|l| l.align);
            self.classes[c].align = own;
            if natural.is_some_and(|n| n > class_align) {
                self.error(line, "ALN-2", "alignas weaker than the natural alignment of the class");
            }
        }
        if !is_template {
            self.inherit_ctors(c, self.line());
            self.check_overrides(c);
            self.prime_eligible(c);
        } else if self.classes[c].template {
            self.check_pattern_overrides(c);
        }
        if self.class_depth == 0 {
            self.run_deferred();
            if !is_template && !self.in_template() {
                self.check_delegation_cycles(c);
            }
        }
        if pending.is_some() {
            self.templ_class_defined = true;
        }
        if is_template {
            return Ok((QTy::unknown(), true));
        }
        Ok((QTy::new(Ty::Class(c)), true))
    }

    /// CTOR-14: a constructor does not delegate to itself, directly or
    /// through other constructors.
    fn check_delegation_cycles(&mut self, c: usize) {
        let ctors = self.classes[c].ctors.clone();
        let target = |p: &P, f: DeclId| -> Option<DeclId> {
            let fi = p.func(f)?;
            let m = fi.mem_inits.iter().find(|m| m.base == Some(c))?;
            match &m.init.as_ref()?.k {
                EK::Construct(t, _) => Some(*t),
                _ => None,
            }
        };
        for &start in &ctors {
            let mut cur = start;
            let mut seen = vec![start];
            while let Some(t) = target(self, cur) {
                if t == start {
                    let line = self.decls[start].line;
                    self.error(line, "CTOR-14", "a constructor delegates to itself through a cycle of constructors");
                    return;
                }
                if seen.contains(&t) {
                    break;
                }
                seen.push(t);
                cur = t;
            }
        }
    }

    fn elaborated_class(&mut self, key: u8, name: &str, qual: Option<Result<usize, ()>>, spec_args: bool, friend: bool, line: u32) -> PR<QTy> {
        if spec_args {
            return Ok(QTy::unknown());
        }
        // TYN-4: `friend class S;` refers to the class that lookup finds,
        // such as an enclosing class, and declares `S` in the enclosing
        // namespace only when lookup finds no class.
        let decl_only = self.at(";") && !friend;
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
    /// NS-15: a qualified definition of a namespace member is in a
    /// namespace that encloses the named one, and not in that namespace.
    pub fn check_ns_enclosing(&mut self, q: usize, line: u32) {
        let here = self.decl_scope();
        if !matches!(self.scopes[here].kind, SK::Namespace) {
            return;
        }
        if here == q {
            self.error(line, "NS-15", "a qualified name for a member of the namespace in which the declaration appears");
        } else if !self.scope_encloses(here, q) {
            self.error(line, "NS-15", "a qualified definition in a namespace that does not enclose the member's namespace");
        }
    }
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
        if existing.is_none() && self.inst_depth == 0 {
            // CNS-13: an explicit specialization satisfies the constraints.
            self.cns13_ok(t, &args, line);
        }
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
        // NAM-21 (Q-0377): a body after a qualified name sees the members
        // of the template's namespace.
        let saved = self.stack.clone();
        let ts = self.templs[t].scope;
        if self.stack.last() != Some(&ts) && matches!(self.scopes[ts].kind, SK::Namespace) {
            self.stack.push(ts);
        }
        let r = self.class_body(c, key, line);
        self.stack = saved;
        r?;
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
        let head_align = std::mem::take(&mut self.head_align);
        self.classes[c].align = self.classes[c].align.max(head_align);
        let name = self.classes[c].name.clone();
        let class_scope = self.classes[c].scope;
        if self.at("final") || (self.pos > 0 && self.toks[self.pos - 1].text == "final" && self.toks[self.pos - 1].kind == TK::Ident) {
            self.final_classes.insert(c);
        }
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
                // PACK-14: `Bs...` gives one base for each element.
                let (bqs, expanded) = self.parse_expansion(|p| {
                    Ok(if p.at("decltype") {
                        p.pos += 1;
                        p.parse_decltype()?
                    } else if p.has_fact(p.pos, crate::templ::FACT_LESS) {
                        // DEP-24: the pattern read `<` as the less-than
                        // operator.
                        p.in_base_clause += 1;
                        let r = p.parse_typename_type();
                        p.in_base_clause -= 1;
                        r?
                    } else if p.qualified_names_type(0) || p.at("typename") {
                        p.eat("typename");
                        p.in_base_clause += 1;
                        let r = p.parse_type_name();
                        p.in_base_clause -= 1;
                        r?
                    } else {
                        p.skip_qualified_name()?;
                        QTy::unknown()
                    })
                })?;
                let expand = !expanded && self.eat("...");
                for bq in bqs {
                self.class_base_types.entry(c).or_default().push((bq.clone(), expand));
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
                    if !self.in_template() {
                        // DER-1: a class is a direct base once, and a final
                        // class is no base.
                        if self.classes[c].bases.iter().any(|x| x.class == Some(b)) {
                            self.error(bl, "DER-1", format!("'{}' is a direct base twice", self.classes[b].name));
                        } else if self.final_classes.contains(&b) {
                            self.error(bl, "DER-1", format!("'{}' is final and cannot be a base", self.classes[b].name));
                        }
                    }
                    if self.classes[b].has_virtual {
                        self.classes[c].has_virtual = true;
                    }
                    if self.binding_hits > hits && self.inst_of.contains_key(&c) {
                        // DEP-11: a base whose type depends on a parameter.
                        self.dep_bases.insert((c, b));
                    }
                }
                self.classes[c].bases.push(BaseInfo { class: bc, virt, access });
                }
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
        self.inherit_ctors(c, self.line());
        self.check_overrides(c);
        self.prime_eligible(c);
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
            if let (Some(v), _) = self.lookup_level_k(s, name, &noms, false) {
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
        let had_alignas = self.at("alignas") || (self.at("[") && self.toks[self.pos..].iter().take_while(|t| t.text != "{" && t.text != ";").any(|t| t.text == "alignas"));
        self.skip_attributes();
        if self.at("alignas") || had_alignas {
            // ALN-7 (Q-0073): no alignas on an enumeration.
            self.error(line, "ALN-7", "alignas applied to an enumeration");
            self.skip_attributes();
        }
        let mut name = String::new();
        let mut qual = None;
        if self.is_ident() || self.at("::") {
            self.last_qual_templ = None;
            qual = self.parse_nested_name()?;
            name = self.ident()?;
        }
        let qual_templ = if matches!(qual, Some(Err(()))) { self.last_qual_templ.take() } else { None };
        if scoped && name.is_empty() {
            // TR-12: a scoped enumeration has a name.
            self.error(line, "TR-12", "the grammar does not derive this: a scoped enumeration without a name");
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
            Some(Err(())) => {
                if is_def {
                    // TPL-42: a member enumeration of a class template,
                    // defined outside the class. Each specialization takes
                    // the enumerators from here.
                    let st = self.pos;
                    self.skip_balanced();
                    if let Some((t, _)) = qual_templ {
                        let toks = self.copy_toks(st, self.pos);
                        self.member_enum_defs.insert((t, name.clone()), toks);
                    }
                }
                return Ok((QTy::unknown(), false));
            }
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
                let parent = self.enum_parent(e);
                let of_instance = matches!(self.scopes[parent].kind, SK::Class(pc) if self.inst_of.contains_key(&pc));
                if is_def && ei.complete && ei.scoped && self.explicit_spec && of_instance && self.inst_depth == 0 {
                    // TSP-30, INST-5: a use of the class specialization made
                    // only the declaration of the member enumeration, so an
                    // explicit specialization gives its enumerators.
                    let es = self.enums[e].scope;
                    self.scopes[es].names.clear();
                    for v in self.scopes[parent].names.values_mut() {
                        v.retain(|&d| !matches!(self.decls[d].k, DK::Enumerator(x, _) if x == e));
                    }
                    self.enums[e].enumerators.clear();
                    self.enums[e].complete = false;
                } else if is_def && ei.complete {
                    self.error(line, "TYN-7", "enumeration defined twice");
                }
                e
            }
            None => {
                if name == "align_val_t" && self.scopes[cur].kind == SK::Namespace && self.scopes[cur].name == "std" && self.inst_depth == 0 {
                    // LSUP-5: a scoped enumeration with the underlying type
                    // std::size_t.
                    if !scoped || fixed.as_ref().is_some_and(|t| *t != Ty::size_t()) || fixed.is_none() {
                        self.error(line, "LSUP-5", "std::align_val_t must be a scoped enumeration with the underlying type std::size_t");
                    }
                }
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
            if let SK::Class(c) = self.scopes[cur].kind {
                if let Some(&(t, _)) = self.inst_of.get(&c) {
                    if let Some(toks) = self.member_enum_defs.get(&(t, name.clone())).cloned() {
                        if !self.enums[e].complete {
                            let save = self.pos;
                            let saved_toks = std::mem::replace(&mut self.toks, (*toks).clone());
                            self.pos = 0;
                            let r = self.parse_enumerators(e, line);
                            self.toks = saved_toks;
                            self.pos = save;
                            r?;
                        }
                    }
                }
            }
            return Ok((QTy::new(Ty::Enum(e)), false));
        }
        // NAM-21: the enumerators of an enumeration defined with a
        // qualified name see the members of the qualifier's scope.
        let saved = self.stack.clone();
        if let Some(Ok(s)) = qual {
            if self.stack.last() != Some(&s) {
                self.stack.push(s);
            }
        }
        let r = self.parse_enumerators(e, line);
        self.stack = saved;
        r?;
        Ok((QTy::new(Ty::Enum(e)), true))
    }

    /// The scope that declares enumeration `e`.
    fn enum_parent(&self, e: usize) -> usize {
        let mut s = self.scopes[self.enums[e].scope].parent;
        while let Some(x) = s {
            if !matches!(self.scopes[x].kind, SK::Template | SK::Enum(_)) {
                return x;
            }
            s = self.scopes[x].parent;
        }
        self.decl_scope()
    }

    fn parse_enumerators(&mut self, e: usize, _line: u32) -> PR<()> {
        self.expect("{")?;
        let escope0 = self.enums[e].scope;
        // INST-5: the definition of a scoped enumeration member of a class
        // specialization is made only when a use needs it. The front end
        // parses it at once and drops its errors (an approximation).
        let lazy = self.enums[e].scoped && self.inst_depth > 0 && !self.in_template() && self.stack.last().is_some_and(|&s| matches!(self.scopes[s].kind, SK::Class(_)));
        if lazy {
            self.d.muted += 1;
        }
        self.stack.push(escope0);
        let r = self.parse_enumerators_inner(e);
        self.stack.pop();
        if lazy {
            self.d.muted -= 1;
        }
        r
    }

    fn parse_enumerators_inner(&mut self, e: usize) -> PR<()> {
        let scoped = self.enums[e].scoped;
        let fixed = self.enums[e].fixed.clone();
        let escope = self.enums[e].scope;
        // The enumerators of an unscoped enumeration go to the scope that
        // declares it, also for a definition with a qualified name.
        let outer = self.enum_parent(e);
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
                    self.error(el, "CE-11", "enumerator value must have integer or enumeration type");
                }
                match self.const_int(&ex) {
                    Ok(x) => v = x,
                    Err(crate::consteval::CeErr::NotConst(r, m)) => {
                        if !self.in_template() {
                            self.error_because(el, "ENUM-14", r, format!("enumerator value is not a constant expression ({}: {})", r, m));
                        }
                    }
                    Err(crate::consteval::CeErr::Unsupported(m)) => {
                        // A value that depends on a parameter is known in
                        // a specialization.
                        if !self.in_template() {
                            self.unsupported(el, format!("enumerator value: {}", m));
                        }
                    }
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
                    // NAM-39: an enumerator can share its name with a class
                    // or enumeration.
                    let own_class = matches!(self.scopes[outer].kind, SK::Class(oc) if self.classes[oc].name == name);
                    if own_class || prev.iter().any(|&p| !matches!(self.decls[p].k, DK::Class(_) | DK::Enum(_))) {
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

impl P {
    /// The virtual functions of the bases of `c` that the member function
    /// `f` of `c` overrides (VIRT-1). None when the front end cannot tell.
    pub fn overridden(&self, c: usize, f: DeclId, depth: u32) -> Option<Vec<DeclId>> {
        let fi = self.func(f)?;
        if fi.is_static || fi.template || fi.kind == FnKind::Ctor || depth > 16 {
            return Some(vec![]);
        }
        if fi.fty.params.iter().any(|p| p.is_unknown() || matches!(p.t, Ty::Dep(_))) {
            return None;
        }
        let mut out = Vec::new();
        let mut stack: Vec<usize> = self.classes[c].bases.iter().map(|b| b.class).collect::<Option<Vec<_>>>()?;
        let mut seen = std::collections::HashSet::new();
        while let Some(b) = stack.pop() {
            if !seen.insert(b) {
                continue;
            }
            if self.class_uncertain(b) {
                return None;
            }
            let cands: Vec<DeclId> = if fi.kind == FnKind::Dtor { self.classes[b].dtor.into_iter().collect() } else { self.own_names(self.classes[b].scope, &self.decls[f].name) };
            let mut found = false;
            for g in cands {
                let Some(gi) = self.func(g) else { continue };
                if gi.is_static || gi.template || gi.class != Some(b) {
                    continue;
                }
                let same = fi.kind == FnKind::Dtor
                    || (gi.fty.params == fi.fty.params && gi.fty.variadic == fi.fty.variadic && gi.fty.cv_c == fi.fty.cv_c && gi.fty.cv_v == fi.fty.cv_v && gi.fty.refq == fi.fty.refq);
                if same && self.fn_virtual(b, g, depth + 1)? {
                    out.push(g);
                    found = true;
                }
            }
            if !found {
                for x in &self.classes[b].bases {
                    stack.push(x.class?);
                }
            }
        }
        Some(out)
    }

    /// Whether a member function is virtual: declared so, or an override.
    pub fn fn_virtual(&self, c: usize, f: DeclId, depth: u32) -> Option<bool> {
        let fi = self.func(f)?;
        if fi.is_virtual || self.pure_fns.contains(&f) {
            return Some(true);
        }
        Some(!self.overridden(c, f, depth)?.is_empty())
    }

    /// VIRT-3 and VIRT-15 at the end of a class definition.
    pub fn check_overrides(&mut self, c: usize) {
        if self.in_template() || self.class_uncertain(c) {
            return;
        }
        if self.classes[c].key == 2 {
            // UNI-8: one variant member has a default member initializer.
            let n = self.classes[c].fields.iter().filter(|&&f| matches!(&self.decls[f].k, DK::Field(fi) if fi.init.is_some()) || self.deferred.iter().any(|d| d.field == Some(f))).count();
            if n > 1 {
                let line = self.classes[c].line;
                self.error(line, "UNI-8", "default member initializers for two members of a union");
            }
        }
        if self.classes[c].dtor.is_none() {
            // VIRT-3, VIRT-7: an implicit destructor overrides a virtual
            // destructor of a base, and must not override a final one.
            let fin = self.classes[c].bases.iter().filter_map(|b| b.class).any(|b| self.classes[b].dtor.is_some_and(|d| self.virt_specs.get(&d).is_some_and(|s| s.1)));
            if fin {
                let line = self.classes[c].line;
                self.error(line, "VIRT-3", format!("the implicit destructor of '{}' overrides a final destructor", self.classes[c].name));
            }
        }
        let mut fns: Vec<DeclId> = self.scopes[self.classes[c].scope].names.values().flatten().copied().filter(|&f| self.func(f).is_some_and(|fi| fi.class == Some(c))).collect();
        fns.sort();
        fns.dedup();
        // VIRT-13: an override with neither `override` nor `final`, in a
        // class where another override has `override`.
        let marked = fns.iter().any(|f| self.virt_specs.get(f).is_some_and(|s| s.0));
        if marked {
            for &f in &fns {
                let (ov, fin) = self.virt_specs.get(&f).copied().unwrap_or((false, false));
                // A destructor without `override` gets no warning.
                let dtor = self.func(f).is_some_and(|fi| fi.kind == FnKind::Dtor);
                if !ov && !fin && !dtor && self.overridden(c, f, 0).is_some_and(|o| !o.is_empty()) {
                    let line = self.decls[f].line;
                    self.warn(line, "VIRT-13", format!("'{}' overrides a virtual function and is not marked 'override', as another override of the class is", self.decls[f].name));
                }
            }
        }
        if !self.in_template() && !self.class_uncertain(c) {
            self.check_final_overriders(c, &fns);
        }
        if self.wflag(&["-Wall", "-Woverloaded-virtual"]) && !self.in_template() {
            // VIRT-13: a function that hides a virtual function of a direct
            // base with the same name and overrides none of that name.
            let mut warned = std::collections::HashSet::new();
            for &f in &fns {
                let Some(fi) = self.func(f) else { continue };
                if fi.kind != FnKind::Normal || fi.template {
                    continue;
                }
                let name = self.decls[f].name.clone();
                if warned.contains(&name) {
                    continue;
                }
                let same_name: Vec<DeclId> = fns.iter().copied().filter(|&g| self.decls[g].name == name).collect();
                let mut overrides_any = false;
                let mut uncertain = false;
                for &g in &same_name {
                    match self.overridden(c, g, 0) {
                        Some(v) if !v.is_empty() => overrides_any = true,
                        Some(_) => {}
                        None => uncertain = true,
                    }
                }
                if overrides_any || uncertain {
                    continue;
                }
                let bases: Vec<usize> = self.classes[c].bases.iter().filter_map(|b| b.class).collect();
                let hidden = bases.iter().any(|&b| !self.class_uncertain(b) && self.own_names(self.classes[b].scope, &name).iter().any(|&g| self.func(g).is_some_and(|gi| !gi.template && gi.class == Some(b)) && self.fn_virtual(b, g, 0) == Some(true)));
                if hidden {
                    warned.insert(name.clone());
                    let line = self.decls[f].line;
                    self.warn(line, "VIRT-13", format!("'{}' hides a virtual function of a base and does not override it", name));
                }
            }
        }
        for f in fns {
            let line = self.decls[f].line;
            let (ov, fin) = self.virt_specs.get(&f).copied().unwrap_or((false, false));
            let Some(over) = self.overridden(c, f, 0) else { continue };
            let virt = self.func(f).is_some_and(|fi| fi.is_virtual) || self.pure_fns.contains(&f) || !over.is_empty();
            let name = self.decls[f].name.clone();
            if ov && over.is_empty() {
                self.error(line, "VIRT-3", format!("'{}' is marked override but overrides no virtual function of a base", name));
            } else if fin && !virt {
                self.error(line, "VIRT-3", format!("'{}' is marked final but is not virtual", name));
            }
            // VIRT-19: an explicit object member function does not
            // override; the front end's search matched it by parameters.
            let explicit_obj = self.func(f).is_some_and(|fi| fi.explicit_obj);
            if over.is_empty() || explicit_obj {
                if let Some(rule) = self.hides_virtual(c, f) {
                    let what = if rule == "VIRT-21" { "a static member function" } else { "an explicit object member function" };
                    self.error(line, rule, format!("'{}' is {} with the name and parameters of a virtual function of a base", name, what));
                }
            }
            for &g in &over {
                let (Some(fi), Some(gi)) = (self.func(f), self.func(g)) else { continue };
                if fi.consteval != gi.consteval {
                    self.error(line, "VIRT-14", format!("'{}' and the function it overrides differ in consteval", name));
                    break;
                }
                if fi.deleted != gi.deleted {
                    self.error(line, "VIRT-19", format!("'{}' and the function it overrides differ in being deleted", name));
                    break;
                }
                if self.virt_specs.get(&g).is_some_and(|s| s.1) {
                    self.error(line, "VIRT-3", format!("'{}' overrides a final function", name));
                    break;
                }
                if gi.fty.noexcept && !fi.fty.noexcept && !gi.deleted {
                    // EXS-3: an override of a non-throwing virtual function
                    // is non-throwing.
                    self.error(line, "EXS-3", format!("'{}' can throw, and the function it overrides is non-throwing", name));
                    break;
                }
                if self.func(f).is_some_and(|fi| fi.kind == FnKind::Dtor) {
                    continue;
                }
                if let Some(msg) = self.covariance(c, f, g) {
                    self.error(line, "VIRT-15", format!("the return type of '{}' {}", name, msg));
                    break;
                }
            }
        }
    }

    /// VIRT-3 and DEP-30 in a class template whose bases do not depend on
    /// a template parameter: a function marked `override` whose name no
    /// virtual function of a base has can override nothing in any
    /// specialization.
    fn check_pattern_overrides(&mut self, c: usize) {
        let bases: Vec<Option<usize>> = self.classes[c].bases.iter().map(|b| b.class).collect();
        if bases.is_empty() || bases.iter().any(|b| b.is_none_or(|b| self.classes[b].template || !self.classes[b].complete || self.class_uncertain(b))) {
            return;
        }
        let mut virtual_names: Vec<String> = Vec::new();
        let mut stack: Vec<usize> = bases.iter().flatten().copied().collect();
        let mut n = 0;
        while let Some(b) = stack.pop() {
            n += 1;
            if n > 64 || self.class_uncertain(b) {
                return;
            }
            for (name, ids) in &self.scopes[self.classes[b].scope].names {
                if ids.iter().any(|&f| self.func(f).is_some_and(|fi| fi.is_virtual) || self.pure_fns.contains(&f)) {
                    virtual_names.push(name.clone());
                }
            }
            for x in &self.classes[b].bases {
                match x.class {
                    Some(y) => stack.push(y),
                    None => return,
                }
            }
        }
        let mut fns: Vec<DeclId> = self.scopes[self.classes[c].scope].names.values().flatten().copied().filter(|&f| self.func(f).is_some_and(|fi| fi.class == Some(c) && fi.kind != FnKind::Dtor)).collect();
        fns.sort();
        fns.dedup();
        for f in fns {
            if self.virt_specs.get(&f).is_some_and(|s| s.0) && !virtual_names.contains(&self.decls[f].name) {
                let line = self.decls[f].line;
                self.error(line, "VIRT-3", format!("'{}' is marked override, and no base has a virtual function of that name", self.decls[f].name));
            }
        }
    }

    /// VIRT-4, VIRT-15: what is wrong with the return type of the override
    /// `f` of `g`, if anything.
    fn covariance(&self, c: usize, f: DeclId, g: DeclId) -> Option<String> {
        let rf = self.func(f)?.fty.ret.clone();
        let rg = self.func(g)?.fty.ret.clone();
        if rf.is_unknown() || rg.is_unknown() || matches!(rf.t, Ty::Dep(_)) || matches!(rg.t, Ty::Dep(_)) {
            return None;
        }
        if rf == rg {
            return None;
        }
        let parts = |q: &QTy| -> Option<(u8, QTy)> {
            match &q.t {
                Ty::Ptr(x) => Some((0, (**x).clone())),
                Ty::LRef(x) => Some((1, (**x).clone())),
                Ty::RRef(x) => Some((2, (**x).clone())),
                _ => None,
            }
        };
        let (Some((kf, cf)), Some((kg, cg))) = (parts(&rf), parts(&rg)) else {
            return Some("differs from the overridden function and is not covariant".into());
        };
        if kf != kg || rf.c != rg.c || rf.v != rg.v {
            return Some("differs from the overridden function and is not covariant".into());
        }
        let (Ty::Class(df), Ty::Class(bg)) = (&cf.t, &cg.t) else {
            return Some("differs from the overridden function and is not covariant".into());
        };
        let (df, bg) = (*df, *bg);
        if (cf.c && !cg.c) || (cf.v && !cg.v) {
            return Some("has more cv-qualification than the overridden return type".into());
        }
        if df == bg {
            return None;
        }
        if self.class_uncertain(df) || self.class_uncertain(bg) {
            return None;
        }
        if !self.classes[df].complete && df != c {
            return Some("names a class that is incomplete at the override".into());
        }
        if !self.is_base_of(bg, df) {
            return Some("names a class that is not derived from the overridden return class".into());
        }
        let (paths, open) = self.base_paths(bg, df);
        if paths > 1 {
            return Some("names a class with an ambiguous base of the overridden return class".into());
        }
        if !open && df != c {
            return Some("names a class with an inaccessible base of the overridden return class".into());
        }
        None
    }

    /// VIRT-21, VIRT-19: a static or explicit object member function with
    /// the name and parameter list of a virtual function of a base.
    fn hides_virtual(&self, c: usize, f: DeclId) -> Option<&'static str> {
        let fi = self.func(f)?;
        if fi.template || !(fi.is_static || fi.explicit_obj) {
            return None;
        }
        // The function type of an explicit object member function leaves
        // out the object parameter when `params` has one more entry.
        let ps: &[QTy] = if fi.explicit_obj && fi.params.len() != fi.fty.params.len() + 1 { fi.fty.params.get(1..)? } else { &fi.fty.params };
        let mut stack: Vec<usize> = self.classes[c].bases.iter().filter_map(|b| b.class).collect();
        let mut seen = std::collections::HashSet::new();
        while let Some(b) = stack.pop() {
            if !seen.insert(b) || self.class_uncertain(b) {
                continue;
            }
            for g in self.own_names(self.classes[b].scope, &self.decls[f].name) {
                let Some(gi) = self.func(g) else { continue };
                if gi.is_static || gi.template || gi.explicit_obj || gi.fty.params != ps {
                    continue;
                }
                if self.fn_virtual(b, g, 1) == Some(true) {
                    return Some(if fi.is_static { "VIRT-21" } else { "VIRT-19" });
                }
            }
            stack.extend(self.classes[b].bases.iter().filter_map(|x| x.class));
        }
        None
    }

    /// Whether the destructor of class `c` is virtual: declared so, or
    /// the destructor of a base is.
    pub fn dtor_virtual(&self, c: usize, depth: u32) -> Option<bool> {
        if depth > 16 || self.class_uncertain(c) {
            return None;
        }
        if let Some(d) = self.classes[c].dtor {
            if self.func(d)?.is_virtual {
                return Some(true);
            }
        }
        for b in &self.classes[c].bases {
            if self.dtor_virtual(b.class?, depth + 1)? {
                return Some(true);
            }
        }
        Some(false)
    }
}

impl P {
    /// The functions that `f` of class `x` overrides, directly or through
    /// other overrides, that override nothing themselves.
    fn override_roots(&self, x: usize, f: DeclId, depth: u32) -> Option<Vec<DeclId>> {
        if depth > 16 {
            return None;
        }
        let over = self.overridden(x, f, 0)?;
        if over.is_empty() {
            return Some(vec![f]);
        }
        let mut out = Vec::new();
        for g in over {
            let gc = self.func(g)?.class?;
            for r in self.override_roots(gc, g, depth + 1)? {
                if !out.contains(&r) {
                    out.push(r);
                }
            }
        }
        Some(out)
    }

    /// VIRT-5: each virtual function of a shared virtual base has one final
    /// overrider in class `c`.
    fn check_final_overriders(&mut self, c: usize, own: &[DeclId]) {
        let mut all = Vec::new();
        self.all_bases(c, &mut all, 0);
        all.sort();
        all.dedup();
        if all.iter().any(|&b| self.class_uncertain(b)) {
            return;
        }
        // The classes that are a virtual base somewhere in the lattice.
        let vbases: Vec<usize> = all.iter().flat_map(|&x| self.classes[x].bases.iter().filter(|b| b.virt).filter_map(|b| b.class).collect::<Vec<_>>()).collect();
        if vbases.is_empty() {
            return;
        }
        // The overriders in the bases of each root that a virtual base
        // declares.
        let mut by_root: Vec<(DeclId, Vec<(usize, DeclId)>)> = Vec::new();
        let mut own_roots: Vec<DeclId> = Vec::new();
        for &f in own {
            match self.override_roots(c, f, 0) {
                Some(v) => own_roots.extend(v),
                None => return,
            }
        }
        for &x in &all {
            let fns: Vec<DeclId> = self.scopes[self.classes[x].scope].names.values().flatten().copied().filter(|&f| self.func(f).is_some_and(|fi| fi.class == Some(x) && !fi.template && fi.kind != FnKind::Ctor)).collect();
            for f in fns {
                if self.fn_virtual(x, f, 0) != Some(true) {
                    continue;
                }
                let Some(roots) = self.override_roots(x, f, 0) else { return };
                for r in roots {
                    let rc = match self.func(r).and_then(|g| g.class) {
                        Some(rc) => rc,
                        None => continue,
                    };
                    if !vbases.contains(&rc) {
                        continue;
                    }
                    match by_root.iter_mut().find(|e| e.0 == r) {
                        Some(e) => {
                            if !e.1.iter().any(|y| y.1 == f) {
                                e.1.push((x, f));
                            }
                        }
                        None => by_root.push((r, vec![(x, f)])),
                    }
                }
            }
        }
        for (r, overs) in by_root {
            if own_roots.contains(&r) {
                continue;
            }
            let finals: Vec<&(usize, DeclId)> = overs.iter().filter(|(x, _)| !overs.iter().any(|(y, _)| y != x && self.is_base_of(*x, *y))).collect();
            if finals.len() > 1 {
                let line = self.classes[c].line;
                self.error(line, "VIRT-5", format!("'{}' has no unique final overrider of '{}': two bases override it and neither overrides the other", self.classes[c].name, self.decls[r].name));
                return;
            }
        }
    }
}
