// Overload resolution for operator expressions
// (specification/overloading/overload-resolution.md, OVL-7 to OVL-13, OVL-25,
// OVL-32, OVL-33, OVL-35, OVL-36; classes/operators.md, OPR-6, OPR-15;
// classes/comparisons.md, DCMP-6).
//
// The candidates are the member operator functions of the left operand's
// class, the non-member operator functions that lookup and
// argument-dependent lookup find, the built-in candidates of the table in
// OVL-P4, and the rewritten and swapped candidates of DCMP-6. The best
// candidate is chosen by the sequences of the operands (OVL-25), then by
// OVL-32 and OVL-33.

use crate::ast::*;
use crate::ics::{Ics, IK, UKey};
use crate::parser::*;
use crate::types::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum CK {
    Written,
    /// A rewritten candidate in the written order (DCMP-6).
    Rewritten,
    /// A rewritten candidate with the operands swapped.
    Swapped,
    Builtin,
}

#[derive(Clone, Debug)]
struct OCand {
    fid: Option<DeclId>,
    member: bool,
    kind: CK,
    /// One sequence per operand, in the written order.
    seqs: Vec<Ics>,
    /// The parameter types of a built-in candidate.
    params: Vec<QTy>,
}

/// The result of resolving an operator.
pub enum OpRes {
    /// An operator function or a rewritten candidate won; the expression
    /// is its call, or the call is not valid and an error was reported.
    Done(Expr),
    /// A built-in candidate won (OVL-11). The operands are given after
    /// the conversion functions of their sequences.
    Builtin(Vec<Expr>),
    /// The front end does not model a candidate or an operand.
    NotHandled,
}

const PROMOTED_INT: [Ty; 6] = [Ty::Int, Ty::UInt, Ty::Long, Ty::ULong, Ty::LLong, Ty::ULLong];
const PROMOTED_FLOAT: [Ty; 3] = [Ty::Float, Ty::Double, Ty::LDouble];
const ARITH: [Ty; 19] = [
    Ty::Bool, Ty::Char, Ty::SChar, Ty::UChar, Ty::WChar, Ty::Char8, Ty::Char16, Ty::Char32, Ty::Short, Ty::UShort, Ty::Int, Ty::UInt, Ty::Long, Ty::ULong, Ty::LLong, Ty::ULLong, Ty::Float, Ty::Double, Ty::LDouble,
];

impl P {
    /// Ordinary lookup of an operator function with members skipped
    /// (OVL-7).
    fn lookup_nonmember(&self, name: &str) -> Option<Vec<DeclId>> {
        let noms = self.nominations();
        for &s in self.stack.iter().rev() {
            if matches!(self.scopes[s].kind, SK::Class(_)) {
                continue;
            }
            let (r, amb) = self.lookup_level(s, name, &noms);
            if let Some(r) = r {
                if amb {
                    return None;
                }
                return Some(r);
            }
        }
        Some(Vec::new())
    }

    /// The operator functions named `name`: members of class `member_of`
    /// and the non-members. None when the front end cannot tell.
    fn op_functions(&self, name: &str, member_of: Option<usize>, ops: &[Expr]) -> Option<(Vec<DeclId>, Vec<DeclId>)> {
        let mut members = Vec::new();
        if let Some(c) = member_of {
            if let Some(ids) = self.lookup_member(c, name) {
                for d in ids {
                    self.func(d)?;
                    members.push(d);
                }
            }
        }
        let mut others: Vec<DeclId> = Vec::new();
        for d in self.lookup_nonmember(name)? {
            if self.func(d).is_some_and(|f| f.class.is_none()) && !others.contains(&d) {
                others.push(d);
            } else if self.func(d).is_none() {
                return None;
            }
        }
        let (adl, certain) = self.adl_lookup(name, ops);
        if !certain {
            return None;
        }
        for d in adl {
            if self.func(d).is_some_and(|f| f.class.is_none()) && !others.contains(&d) {
                others.push(d);
            }
        }
        // A member of a class template defined after the class (TPL-23) is
        // not a function of the namespace, so it is not a candidate.
        others.retain(|&d| !self.qual_templ_defs.contains(&d));
        let others = self.same_entities_removed(&others);
        Some((members, others))
    }

    /// The types that the built-in candidates can take from an operand:
    /// its own type, or the result types of its conversion functions.
    fn op_type_pool(&self, ops: &[Expr]) -> Option<Vec<QTy>> {
        let mut pool: Vec<QTy> = Vec::new();
        let mut add = |t: QTy| {
            let mut t = nonref(&t).unqual();
            match &t.t {
                Ty::Array(e, _) => t = QTy::new(Ty::Ptr(e.clone())),
                Ty::Func(_) => t = QTy::new(Ty::Ptr(Box::new(t.clone()))),
                _ => {}
            }
            if !pool.contains(&t) {
                pool.push(t);
            }
        };
        for o in ops {
            match nonref(&o.ty).t {
                Ty::Class(c) => {
                    let mut fns = Vec::new();
                    self.conv_fns(c, 0, &mut fns);
                    for f in fns {
                        let fi = self.func(f)?;
                        if fi.template || fi.fty.ret.is_unknown() {
                            return None;
                        }
                        add(fi.fty.ret.clone());
                    }
                }
                _ => add(o.ty.clone()),
            }
        }
        Some(pool)
    }

    /// The parameter lists of the built-in candidates for `op` (OVL-9, the
    /// table of OVL-P4). Only the operators that this module handles.
    fn builtin_params(&self, op: &str, arity: usize, ops: &[Expr]) -> Option<Vec<Vec<QTy>>> {
        let pool = self.op_type_pool(ops)?;
        let q = QTy::new;
        let ptrs: Vec<QTy> = pool.iter().filter(|t| matches!(t.t, Ty::Ptr(_))).cloned().collect();
        let obj_ptrs: Vec<QTy> = ptrs.iter().filter(|t| matches!(&t.t, Ty::Ptr(p) if !matches!(p.t, Ty::Func(_) | Ty::Void))).cloned().collect();
        let enums: Vec<QTy> = pool.iter().filter(|t| matches!(t.t, Ty::Enum(_))).cloned().collect();
        let memptrs: Vec<QTy> = pool.iter().filter(|t| matches!(t.t, Ty::MemPtr(..))).cloned().collect();
        let a: Vec<Ty> = PROMOTED_INT.iter().chain(PROMOTED_FLOAT.iter()).cloned().collect();
        let i: Vec<Ty> = PROMOTED_INT.to_vec();
        let pairs = |x: &[Ty], y: &[Ty]| -> Vec<Vec<QTy>> { x.iter().flat_map(|l| y.iter().map(move |r| vec![QTy::new(l.clone()), QTy::new(r.clone())])).collect() };
        let pd = Ty::ptrdiff_t();
        let mut out: Vec<Vec<QTy>> = Vec::new();
        match (op, arity) {
            ("*" | "/", 2) => out = pairs(&a, &a),
            ("+" | "-", 2) => {
                out = pairs(&a, &a);
                for p in &obj_ptrs {
                    out.push(vec![p.clone(), q(pd.clone())]);
                    if op == "+" {
                        out.push(vec![q(pd.clone()), p.clone()]);
                    } else {
                        out.push(vec![p.clone(), p.clone()]);
                    }
                }
            }
            ("%" | "&" | "^" | "|" | "<<" | ">>", 2) => out = pairs(&i, &i),
            ("==" | "!=" | "<" | ">" | "<=" | ">=", 2) => {
                out = pairs(&a, &a);
                for p in ptrs.iter().chain(enums.iter()) {
                    out.push(vec![p.clone(), p.clone()]);
                }
                if op == "==" || op == "!=" {
                    for m in &memptrs {
                        out.push(vec![m.clone(), m.clone()]);
                    }
                    out.push(vec![q(Ty::NullPtr), q(Ty::NullPtr)]);
                }
            }
            ("<=>", 2) => {
                out = pairs(&a, &a);
                for p in ptrs.iter().chain(enums.iter()) {
                    out.push(vec![p.clone(), p.clone()]);
                }
            }
            ("&&" | "||", 2) => out.push(vec![q(Ty::Bool), q(Ty::Bool)]),
            ("[]", 2) => {
                for p in &obj_ptrs {
                    out.push(vec![p.clone(), q(pd.clone())]);
                    out.push(vec![q(pd.clone()), p.clone()]);
                }
            }
            ("&" | ",", _) => {}
            ("*=" | "/=" | "+=" | "-=" | "%=" | "&=" | "^=" | "|=" | "<<=" | ">>=", 2) => {
                let ints = matches!(op, "%=" | "&=" | "^=" | "|=" | "<<=" | ">>=");
                let rights: &[Ty] = if ints { &i } else { &a };
                for x in ARITH.iter().filter(|t| !ints || !t.is_floating()) {
                    for v in [false, true] {
                        for r in rights {
                            out.push(vec![q(Ty::LRef(Box::new(QTy { t: x.clone(), c: false, v }))), q(r.clone())]);
                        }
                    }
                }
                if op == "+=" || op == "-=" {
                    for p in &obj_ptrs {
                        for v in [false, true] {
                            out.push(vec![q(Ty::LRef(Box::new(QTy { t: p.t.clone(), c: false, v }))), q(pd.clone())]);
                        }
                    }
                }
            }
            ("!", 1) => out.push(vec![q(Ty::Bool)]),
            ("+" | "-", 1) => {
                for t in &a {
                    out.push(vec![q(t.clone())]);
                }
                if op == "+" {
                    for p in &ptrs {
                        out.push(vec![p.clone()]);
                    }
                }
            }
            ("~", 1) => {
                for t in &i {
                    out.push(vec![q(t.clone())]);
                }
            }
            ("*", 1) => {
                for p in &ptrs {
                    if let Ty::Ptr(x) = &p.t {
                        if x.t != Ty::Void {
                            out.push(vec![p.clone()]);
                        }
                    }
                }
            }
            ("++" | "--", 1) => {
                for t in ARITH.iter().filter(|t| **t != Ty::Bool) {
                    for v in [false, true] {
                        out.push(vec![q(Ty::LRef(Box::new(QTy { t: t.clone(), c: false, v })))]);
                    }
                }
                for p in &obj_ptrs {
                    for v in [false, true] {
                        out.push(vec![q(Ty::LRef(Box::new(QTy { t: p.t.clone(), c: false, v })))]);
                    }
                }
            }
            ("->*", 2) => {
                // OVL-9: the built-in ->* needs a pointer operand; a class
                // operand with a conversion function is not modeled.
                if ops.iter().any(|o| matches!(nonref(&o.ty).t, Ty::Class(c) if self.any_conv(c, 0))) {
                    return None;
                }
            }
            _ => return None,
        }
        Some(out)
    }

    /// The sequences of one operator function as a candidate, in the
    /// written order of the operands. `swapped` takes the operands in
    /// the reverse order. Ok(None): not viable. Err: not modeled.
    fn op_fn_seqs(&mut self, fid: DeclId, member: bool, ops: &[Expr], swapped: bool, postfix: bool) -> Result<Option<Vec<Ics>>, ()> {
        let mut f = self.func(fid).ok_or(())?.clone();
        if f.defaulted && f.body.is_none() {
            // DCMP: a defaulted comparison gets its body at its first use.
            if !self.synth_defaulted_cmp(fid) {
                if let Some(&rule) = self.dcmp_deleted.get(&fid) {
                    // DCMP-9, DCMP-11: the use of a deleted defaulted
                    // comparison.
                    if !self.in_template() {
                        let line = ops.first().map_or(0, |o| o.line);
                        self.error(line, rule, format!("use of the defaulted '{}', which is deleted", self.decls[fid].name));
                    }
                }
                return Err(());
            }
            f = self.func(fid).ok_or(())?.clone();
        }
        if f.template || f.opaque || f.constrained || f.fty.variadic || f.defaults > 0 || f.is_static {
            return Err(());
        }
        match self.fn_sat(fid) {
            crate::cons::Sat::Yes => {}
            crate::cons::Sat::Unknown => return Err(()),
            _ => return Ok(None),
        }
        let explicit_obj_ty = if f.explicit_obj { Some(f.params.first().and_then(|&p| self.var(p)).map(|v| v.ty.clone()).ok_or(())?) } else { None };
        let order: Vec<usize> = if swapped { vec![1, 0] } else { (0..ops.len()).collect() };
        let extra = postfix as usize;
        let want = if member { ops.len() - 1 + extra } else { ops.len() + extra };
        if f.fty.params.len() != want {
            return Ok(None);
        }
        let mut seqs: Vec<Option<Ics>> = vec![None; ops.len()];
        let mut k = 0;
        for (pos, &oi) in order.iter().enumerate() {
            let o = &ops[oi];
            let s = if member && pos == 0 {
                match &explicit_obj_ty {
                    // ICS-41: an explicit object parameter is an ordinary
                    // parameter.
                    Some(pt) => match self.ics_form(o, pt, false)? {
                        Some(s) => s,
                        None => return Ok(None),
                    },
                    // The object takes only a standard sequence (ICS-10).
                    None => match self.ics_object(o, &f) {
                        Some(s) => s,
                        None => return Ok(None),
                    },
                }
            } else {
                let p = f.fty.params[k].clone();
                k += 1;
                match self.ics_form(o, &p, false)? {
                    Some(s) => s,
                    None => return Ok(None),
                }
            };
            seqs[oi] = Some(s);
        }
        Ok(Some(seqs.into_iter().map(|s| s.unwrap()).collect()))
    }

    /// DCMP-6: the scope of operator== `fid` declares an operator!= with
    /// the same parameters, so `fid` gives no rewritten candidates.
    fn eq_excluded(&self, fid: DeclId) -> bool {
        let Some(f) = self.func(fid) else { return false };
        let sc = self.decls[fid].scope;
        // DCMP-13: for a member, member lookup (which finds a base's !=);
        // for a non-member, a search of its namespace, which does not find
        // a friend declared only in a class.
        let ids: Vec<DeclId> = match f.class {
            Some(c) => self.lookup_member(c, "operator!=").unwrap_or_default(),
            None => self.scopes[sc].names.get("operator!=").map(|v| v.iter().copied().filter(|d| !self.hidden_friends.contains_key(d)).collect()).unwrap_or_default(),
        };
        let member = f.class.is_some();
        ids.iter().any(|&d| self.func(d).is_some_and(|g| g.fty.params == f.fty.params && g.fty.cv_c == f.fty.cv_c && g.fty.cv_v == f.fty.cv_v && g.fty.refq == f.fty.refq && g.class.is_some() == member))
    }

    /// OVL-25, OVL-32, OVL-33: is `x` better than `y`?
    fn op_better(&self, x: &OCand, y: &OCand, more: &[((DeclId, CK), (DeclId, CK))]) -> bool {
        let mut some = false;
        for (a, b) in x.seqs.iter().zip(y.seqs.iter()) {
            let c = self.ics_cmp(a, b);
            if c < 0 {
                return false;
            }
            if c > 0 {
                some = true;
            }
        }
        if some {
            return true;
        }
        // OVL-27: a function that is not a template specialization (or a
        // built-in candidate) beats a specialization.
        let spec = |c: &OCand| c.fid.is_some_and(|f| self.spec_of.contains_key(&f));
        if spec(x) != spec(y) {
            return !spec(x);
        }
        // OVL-28: the more specialized template, for two candidates of the
        // same kind.
        if let (Some(a), Some(b)) = (x.fid, y.fid) {
            if more.contains(&((a, x.kind), (b, y.kind))) {
                return true;
            }
            if more.contains(&((b, y.kind), (a, x.kind))) {
                return false;
            }
        }
        let rewritten = |k: CK| matches!(k, CK::Rewritten | CK::Swapped);
        // OVL-32: a candidate that is not rewritten wins.
        if !rewritten(x.kind) && rewritten(y.kind) {
            return true;
        }
        // OVL-33: the written order wins over the swapped one.
        x.kind == CK::Rewritten && y.kind == CK::Swapped
    }

    /// Overload resolution for the operator `op` with these operands, at
    /// least one of class or enumeration type (OVL-7).
    pub fn op_resolve(&mut self, op: &str, ops: &[Expr], postfix: bool, line: u32) -> OpRes {
        let inner = std::mem::take(&mut self.inner_rewrite);
        if self.in_template() || !matches!(ops.len(), 1 | 2) {
            return OpRes::NotHandled;
        }
        let mut has_class = false;
        for o in ops {
            if o.ty.is_unknown() || matches!(o.k, EK::InitList(_) | EK::Overloads(_) | EK::BoundMember(..)) {
                return OpRes::NotHandled;
            }
            if let Ty::Class(c) = nonref(&o.ty).t {
                if !self.class_ok(c) || self.init_list_elem(c).is_some() {
                    return OpRes::NotHandled;
                }
                has_class = true;
            }
        }
        let class_of = |e: &Expr| match nonref(&e.ty).t {
            Ty::Class(c) => Some(c),
            _ => None,
        };
        let name = format!("operator{}", op);
        let Some((members, others)) = self.op_functions(&name, class_of(&ops[0]), ops) else { return OpRes::NotHandled };
        // DED-1: an operator function template adds its specialization for
        // the operands.
        let rev: Vec<Expr> = ops.iter().rev().cloned().collect();
        let Some(members) = self.expand_fn_templates(&members, &ops[1..], line) else { return OpRes::NotHandled };
        let Some(others) = self.expand_fn_templates(&others, ops, line) else { return OpRes::NotHandled };
        // OVL-8: with no class operand, a non-member needs a parameter of
        // the type of the matching enumeration operand.
        let enum_ok = |p: &P, fid: DeclId, order: &[usize]| -> bool {
            if has_class {
                return true;
            }
            let f = p.func(fid).unwrap();
            order.iter().enumerate().any(|(k, &oi)| matches!(nonref(&ops[oi].ty).t, Ty::Enum(_)) && f.fty.params.get(k).is_some_and(|q| nonref(q).t == nonref(&ops[oi].ty).t))
        };
        let mut cands: Vec<OCand> = Vec::new();
        let direct: Vec<usize> = (0..ops.len()).collect();
        for &fid in &members {
            match self.op_fn_seqs(fid, true, ops, false, postfix) {
                Ok(Some(seqs)) => cands.push(OCand { fid: Some(fid), member: true, kind: CK::Written, seqs, params: vec![] }),
                Ok(None) => {}
                Err(()) => return OpRes::NotHandled,
            }
        }
        for &fid in &others {
            if !enum_ok(self, fid, &direct) {
                continue;
            }
            match self.op_fn_seqs(fid, false, ops, false, postfix) {
                Ok(Some(seqs)) => cands.push(OCand { fid: Some(fid), member: false, kind: CK::Written, seqs, params: vec![] }),
                Ok(None) => {}
                Err(()) => return OpRes::NotHandled,
            }
        }
        // DCMP-6: the rewritten and swapped candidates.
        let mut rewritten_nonmember_params: Vec<Vec<QTy>> = Vec::new();
        let mut excluded_any = false;
        if ops.len() == 2 && !inner {
            let src = match op {
                "==" | "!=" => Some("operator=="),
                "<" | ">" | "<=" | ">=" | "<=>" => Some("operator<=>"),
                _ => None,
            };
            if matches!(op, "==" | "!=") {
                // A defaulted <=> also declares an ==, which lookup does not
                // show here; a declared != changes which == is used.
                for c in ops.iter().filter_map(class_of) {
                    if self.lookup_member(c, "operator<=>").is_some_and(|v| v.iter().any(|&d| self.func(d).is_none_or(|f| f.defaulted))) || self.class_friends.get(&c).is_some_and(|v| v.iter().any(|&d| self.decls[d].name == "operator<=>" && self.func(d).is_some_and(|f| f.defaulted))) {
                        // DCMP: the == that the defaulted <=> declares.
                        if self.in_template() || !self.implicit_eq(c) {
                            return OpRes::NotHandled;
                        }
                    }
                }
            }
            if let Some(src) = src {
                let Some((m_left, o_all)) = self.op_functions(src, class_of(&ops[0]), ops) else { return OpRes::NotHandled };
                let m_right = class_of(&ops[1]).and_then(|c| self.lookup_member(c, src)).unwrap_or_default();
                let Some(m_left) = self.expand_fn_templates(&m_left, &ops[1..], line) else { return OpRes::NotHandled };
                let Some(m_right) = self.expand_fn_templates(&m_right, &ops[..1], line) else { return OpRes::NotHandled };
                let Some(o_written) = self.expand_fn_templates(&o_all, ops, line) else { return OpRes::NotHandled };
                let Some(o_swapped) = self.expand_fn_templates(&o_all, &rev, line) else { return OpRes::NotHandled };
                let normal = src != format!("operator{}", op);
                let mut add = |p: &mut P, fid: DeclId, member: bool, swapped: bool| -> bool {
                    let kind = if swapped { CK::Swapped } else { CK::Rewritten };
                    if src == "operator==" && p.eq_excluded(fid) {
                        // DCMP-6: the scope of this == declares a matching !=.
                        excluded_any = true;
                        return true;
                    }
                    match p.op_fn_seqs(fid, member, ops, swapped, false) {
                        Ok(Some(seqs)) => {
                            if !member && !p.spec_of.contains_key(&fid) {
                                rewritten_nonmember_params.push(p.func(fid).unwrap().fty.params.clone());
                            }
                            cands.push(OCand { fid: Some(fid), member, kind, seqs, params: vec![] });
                            true
                        }
                        Ok(None) => true,
                        Err(()) => false,
                    }
                };
                if normal {
                    for &fid in &m_left {
                        if !add(self, fid, true, false) {
                            return OpRes::NotHandled;
                        }
                    }
                    for &fid in &o_written {
                        if enum_ok(self, fid, &direct) && !add(self, fid, false, false) {
                            return OpRes::NotHandled;
                        }
                    }
                }
                for &fid in &m_right {
                    if !add(self, fid, true, true) {
                        return OpRes::NotHandled;
                    }
                }
                for &fid in &o_swapped {
                    if enum_ok(self, fid, &[1, 0]) && !add(self, fid, false, true) {
                        return OpRes::NotHandled;
                    }
                }
            }
        }
        // OVL-9, OVL-10: the built-in candidates.
        let Some(lists) = self.builtin_params(op, ops.len(), ops) else { return OpRes::NotHandled };
        let written_params: Vec<Vec<QTy>> = others.iter().filter(|d| !self.spec_of.contains_key(d)).filter_map(|&d| self.func(d)).filter(|f| !f.template).map(|f| f.fty.params.clone()).collect();
        for params in lists {
            if written_params.contains(&params) || rewritten_nonmember_params.contains(&params) {
                continue;
            }
            let mut seqs = Vec::new();
            let mut ok = true;
            for (k, (o, p)) in ops.iter().zip(params.iter()).enumerate() {
                // ICS-10, OVL-P4: the left operand of a built-in `=` takes
                // only a standard sequence; a compound assignment does not.
                let std_only = k == 0 && op == "=";
                match self.ics_form(o, p, std_only) {
                    Ok(Some(s)) => seqs.push(s),
                    Ok(None) => {
                        ok = false;
                        break;
                    }
                    Err(()) => return OpRes::NotHandled,
                }
            }
            if ok {
                cands.push(OCand { fid: None, member: false, kind: CK::Builtin, seqs, params });
            }
        }
        let fn_cands = cands.iter().any(|c| c.kind != CK::Builtin);
        if !fn_cands && matches!(op, "!" | "&&" | "||") {
            // The operands convert to bool by the operator's own page.
            return OpRes::Builtin(ops.to_vec());
        }
        if cands.is_empty() && matches!(op, "&" | ",") {
            // OVL-12: the built-in meaning.
            return OpRes::Builtin(ops.to_vec());
        }
        if cands.is_empty() && excluded_any {
            // DCMP-12: the comparison needs a candidate that DCMP-6
            // excludes.
            self.error(line, "DCMP-12", format!("no viable function for operator '{}': the == that would be rewritten has a matching !=", op));
            return OpRes::Done(Expr::unknown(line, "bad operator"));
        }
        if cands.is_empty() && inner {
            // OVL-43: the comparison with 0 has no viable function.
            self.error(line, "OVL-43", format!("the comparison of the '<=>' result with 0 in a rewritten '{}' has no viable function", op));
            return OpRes::Done(Expr::unknown(line, "bad operator"));
        }
        if cands.is_empty() {
            // OVL-35, OPR-15.
            self.error_because(line, "OVL-35", "OPR-15", format!("no viable function for operator '{}'", op));
            return OpRes::Done(Expr::unknown(line, "bad operator"));
        }
        // A function that two lookups find is one candidate.
        let mut uniq: Vec<OCand> = Vec::new();
        for c in cands {
            if !uniq.iter().any(|u| u.fid.is_some() && u.fid == c.fid && u.kind == c.kind && u.member == c.member) {
                uniq.push(c);
            }
        }
        let cands = uniq;
        // OVL-28: the order of the templates of two specializations of the
        // same kind.
        let mut op_more: Vec<((DeclId, CK), (DeclId, CK))> = Vec::new();
        let specs: Vec<(DeclId, CK, bool)> = cands.iter().filter_map(|c| c.fid.filter(|f| self.spec_of.contains_key(f)).map(|f| (f, c.kind, c.member))).collect();
        // A pair of specializations that the front end does not order.
        let mut unordered = false;
        for &(a, ka, ma) in &specs {
            for &(b, kb, mb) in &specs {
                if a == b {
                    continue;
                }
                let (sa, sb) = (ka == CK::Swapped, kb == CK::Swapped);
                if ma == mb && sa == sb {
                    if self.spec_more_specialized(a, b, ops.len() - ma as usize) {
                        op_more.push(((a, ka), (b, kb)));
                    }
                } else if ma != mb || (!ma && ops.len() == 2) {
                    // ORD-6: a member against a non-member takes part with
                    // its object parameter. ORD-7: a swapped candidate
                    // takes part with its two parameters reversed, after
                    // the object parameter of a member is added.
                    if sa != sb && ops.len() != 2 {
                        unordered = true;
                        continue;
                    }
                    self.order_rev = (sa, sb);
                    if self.spec_more_specialized(a, b, ops.len()) {
                        op_more.push(((a, ka), (b, kb)));
                    }
                    self.order_rev = (false, false);
                } else {
                    unordered = true;
                }
            }
        }
        let best = (0..cands.len()).find(|&i| (0..cands.len()).all(|j| i == j || self.op_better(&cands[i], &cands[j], &op_more)));
        if best.is_none() && unordered {
            // OVL-27, OVL-28, ORD-6 for operator templates: left to the
            // older selection.
            return OpRes::NotHandled;
        }
        let Some(bi) = best else {
            // OVL-36, OPR-15.
            self.error_because(line, "OVL-36", "OPR-15", format!("ambiguous operator '{}': no viable candidate is better than each of the others", op));
            return OpRes::Done(Expr::unknown(line, "bad operator"));
        };
        let w = cands[bi].clone();
        match w.kind {
            CK::Builtin => {
                // OVL-11: each class operand converts by the conversion
                // function of its sequence, and stops there.
                let mut out = Vec::new();
                for (o, s) in ops.iter().zip(w.seqs.iter()) {
                    if matches!(nonref(&o.ty).t, Ty::Class(_)) {
                        match (&s.kind, &s.user) {
                            (IK::User, Some(UKey::Fn(cf))) => {
                                let cf = *cf;
                                if self.func(cf).is_some_and(|f| f.deleted) {
                                    self.error_because(line, "ICS-8", "DEL-1", format!("operator '{}' needs the deleted function '{}'", op, self.decls[cf].name));
                                }
                                if self.in_unevaluated == 0 && self.internal_fns.contains(&cf) {
                                    self.internal_uses.push((cf, line));
                                }
                                let f = self.func(cf).unwrap().clone();
                                let (t, cat) = crate::expr2::ret_cat(&f.fty.ret);
                                out.push(Expr::new(EK::Call(Box::new(o.clone()), vec![], Some(cf)), t, cat, line));
                            }
                            (IK::Ambig, _) => {
                                self.error_because(line, "ICS-8", "CVF-12", format!("operator '{}' needs an ambiguous conversion", op));
                                return OpRes::Done(Expr::unknown(line, "bad operator"));
                            }
                            _ => {
                                let _ = &w.params;
                                return OpRes::NotHandled;
                            }
                        }
                    } else {
                        out.push(o.clone());
                    }
                }
                OpRes::Builtin(out)
            }
            _ => {
                let fid = w.fid.unwrap();
                let swapped = w.kind == CK::Swapped;
                let mut args: Vec<Expr> = if swapped { vec![ops[1].clone(), ops[0].clone()] } else { ops.to_vec() };
                if postfix {
                    args.push(Expr::new(EK::Int(0), QTy::int(), Cat::PR, line));
                }
                let saved = std::mem::take(&mut self.call_adl);
                let call = if w.member {
                    let obj = args.remove(0);
                    self.call_overloads_pub(Some(obj), vec![fid], args, line)
                } else {
                    self.call_overloads_pub(None, vec![fid], args, line)
                };
                self.call_adl = saved;
                if w.kind == CK::Written {
                    return OpRes::Done(call);
                }
                let ret = self.func(fid).map(|f| f.fty.ret.clone()).unwrap_or(QTy::unknown());
                if matches!(op, "==" | "!=") && ret.t != Ty::Bool && !ret.is_unknown() {
                    // DCMP-12: an == that rewrites != or wins reversed for ==
                    // returns bool, perhaps cv-qualified (Q-0158).
                    self.error(line, "DCMP-12", format!("the operator== used for a rewritten '{}' returns {}, not bool", op, show(&ret)));
                }
                // OVL-13: what a rewritten candidate evaluates.
                let zero = Expr::new(EK::Int(0), QTy::int(), Cat::PR, line);
                let r = match op {
                    "==" => call,
                    "!=" => self.build_unary(UOp::Not, call, line),
                    "<=>" => self.build_binary(BOp::Spaceship, zero, call, line),
                    _ => {
                        let b = BOp::from_str(op).unwrap();
                        self.inner_rewrite = true;
                        let r = if swapped { self.build_binary(b, zero, call, line) } else { self.build_binary(b, call, zero, line) };
                        self.inner_rewrite = false;
                        r
                    }
                };
                OpRes::Done(r)
            }
        }
    }
}

/// The result of choosing a constructor (OVL-14, CTOR-25).
pub enum CtorRes {
    Chosen(DeclId, Vec<Ics>),
    Ambiguous,
    NoViable,
    Unknown,
}

impl P {
    /// OVL-14, ILC-2, ILC-3, CTOR-25: the constructor of class `c` for
    /// these arguments. `list` is list-initialization, whose arguments are
    /// the elements. `copy` is copy-initialization, which takes only the
    /// converting constructors. The implicit copy and move constructors
    /// count; the front end gives up when one of them could be viable.
    pub fn ctor_resolve(&mut self, c: usize, args: &[Expr], list: bool, copy: bool, copy_list: bool) -> CtorRes {
        if self.in_template() || !self.class_ok(c) || self.classes[c].ctors.is_empty() {
            return CtorRes::Unknown;
        }
        if args.iter().any(|a| (a.ty.is_unknown() && !matches!(a.k, EK::InitList(_))) || matches!(&a.k, EK::Unknown(s) if s.starts_with("designator"))) {
            return CtorRes::Unknown;
        }
        let mut ctors = self.classes[c].ctors.clone();
        if args.len() == 1 {
            ctors.retain(|&cid| !self.inherited_copy_shape(cid, c));
        }
        if list && args.is_empty() {
            // A constructor template whose parameters all need arguments
            // gives no candidate for empty braces.
            ctors.retain(|&cid| self.func(cid).is_none_or(|f| !f.template || f.fty.variadic || f.params.len() <= f.defaults || self.decl_tid.get(&cid).is_some_and(|&t| self.templs[t].fn_pack.is_some())));
            if ctors.is_empty() {
                return CtorRes::Unknown;
            }
        }
        // DED-1: a constructor template adds its specialization.
        let ctors = if ctors.iter().any(|&cid| self.func(cid).is_some_and(|f| f.template)) {
            if list && !args.is_empty() {
                // ILC-3: an initializer-list constructor that is not a
                // template and takes the elements comes first.
                let mut list_viable = Vec::new();
                for &cid in &ctors {
                    let Some(f) = self.func(cid).cloned() else { return CtorRes::Unknown };
                    let Some(&p0) = f.params.first() else { continue };
                    let pt = nonref(&self.var(p0).map(|v| v.ty.clone()).unwrap_or(QTy::unknown()));
                    let Ty::Class(lc) = pt.t else { continue };
                    let Some(el) = self.init_list_elem(lc) else { continue };
                    if f.template || f.params.len() - 1 > f.defaults || el.is_unknown() {
                        return CtorRes::Unknown;
                    }
                    let mut all = true;
                    for a in args {
                        match self.ics_form(a, &el, false) {
                            Ok(Some(_)) => {}
                            Ok(None) => {
                                all = false;
                                break;
                            }
                            Err(()) => return CtorRes::Unknown,
                        }
                    }
                    if all {
                        list_viable.push(cid);
                    }
                }
                if list_viable.len() == 1 {
                    return CtorRes::Chosen(list_viable[0], Vec::new());
                }
            }
            if list {
                return CtorRes::Unknown;
            }
            match self.expand_fn_templates(&ctors, args, 0) {
                Some(v) if !v.is_empty() => v,
                _ => return CtorRes::Unknown,
            }
        } else {
            ctors
        };
        for &cid in &ctors {
            let Some(f) = self.func(cid) else { return CtorRes::Unknown };
            if f.template || f.opaque || f.constrained {
                return CtorRes::Unknown;
            }
        }
        // OVL-24, CNS-16: a constructor whose constraints are not satisfied
        // is not viable, and still counts as declared.
        let mut kept = Vec::new();
        for &cid in &ctors {
            match self.fn_sat(cid) {
                crate::cons::Sat::Yes => kept.push(cid),
                crate::cons::Sat::Unknown => return CtorRes::Unknown,
                _ => {}
            }
        }
        let ctors = kept;
        if list {
            if args.is_empty() {
                // ILC-2: empty braces value-initialize a class with a
                // default constructor, and otherwise take an
                // initializer-list constructor. With neither, no
                // constructor is viable.
                let zero = ctors.iter().any(|&cid| self.func(cid).is_some_and(|f| f.fty.variadic || f.params.len() <= f.defaults));
                let list_ctor = ctors.iter().any(|&cid| match self.func(cid).and_then(|f| f.params.first().copied()) {
                    None => false,
                    Some(p) => self.var(p).is_none_or(|v| matches!(nonref(&v.ty).t, Ty::Class(lc) if self.init_list_elem(lc).is_some()) || v.ty.is_unknown()),
                });
                if (list_ctor && !zero) || !self.classes[c].bases.is_empty() {
                    return CtorRes::Unknown;
                }
                if !zero {
                    if self.classes[c].ctors.len() != ctors.len() {
                        return CtorRes::Unknown;
                    }
                    return CtorRes::NoViable;
                }
                // A default constructor: overload resolution with no
                // arguments chooses it (ILC-2, CTOR-25).
            } else {
            // ILC-2: an initializer-list constructor that takes the list.
            let mut list_viable: Vec<DeclId> = Vec::new();
            for &cid in &ctors {
                let f = self.func(cid).unwrap().clone();
                let Some(&p0) = f.params.first() else { continue };
                let pt = nonref(&self.var(p0).map(|v| v.ty.clone()).unwrap_or(QTy::unknown()));
                let Ty::Class(lc) = pt.t else {
                    if pt.is_unknown() {
                        return CtorRes::Unknown;
                    }
                    continue;
                };
                let Some(el) = self.init_list_elem(lc) else { continue };
                if f.params.len() - 1 > f.defaults || el.is_unknown() {
                    return CtorRes::Unknown;
                }
                let mut all = true;
                for a in args {
                    match self.ics_form(a, &el, false) {
                        Ok(Some(_)) => {}
                        Ok(None) => {
                            all = false;
                            break;
                        }
                        Err(()) => return CtorRes::Unknown,
                    }
                }
                if all {
                    list_viable.push(cid);
                }
            }
            match list_viable.len() {
                0 => {}
                1 => return CtorRes::Chosen(list_viable[0], Vec::new()),
                _ => return CtorRes::Unknown,
            }
            }
        }
        if args.len() == 1 && !copy && !copy_list {
            if let Ty::Class(ac) = nonref(&args[0].ty).t {
                // OVL-40, ICS-45: direct initialization can use an explicit
                // conversion function for the parameter of a copy or move
                // constructor, which ics_form does not model.
                // Copy-list-initialization cannot (Q-0145).
                let mut fns = Vec::new();
                self.conv_fns(ac, 0, &mut fns);
                // Only a function that gives `c`, a class derived from it,
                // or an unknown type can reach such a parameter.
                let reaches = |p: &Self, t: &QTy| match nonref(t).t {
                    Ty::Class(r) => r == c || p.is_base_of(c, r),
                    Ty::Unknown | Ty::Dep(_) => true,
                    _ => false,
                };
                if fns.iter().any(|&f| self.func(f).is_none_or(|fi| fi.explicit && (fi.template || reaches(self, &fi.fty.ret)))) {
                    return CtorRes::Unknown;
                }
            }
        }
        // The implicit copy and move constructors, which the front end does
        // not list as candidates. A class that declares a copy or move
        // constructor has no implicit copy or move constructor of that kind
        // (CPY-5); a declared copy constructor also removes the implicit
        // move constructor.
        let mut implicit_user = false;
        if args.len() == 1 {
            let own = list && matches!(args[0].k, EK::InitList(_));
            let ct = QTy::new(Ty::Class(c));
            let declared = |rv: bool| {
                ctors.iter().any(|&cid| {
                    self.func(cid).and_then(|f| f.params.first().copied()).and_then(|p| self.var(p)).is_some_and(|v| match &v.ty.t {
                        Ty::LRef(x) => !rv && x.t == Ty::Class(c),
                        Ty::RRef(x) => rv && x.t == Ty::Class(c),
                        _ => false,
                    })
                })
            };
            let (has_copy, has_move) = (declared(false), declared(true));
            let mut implicit = Vec::new();
            if !has_copy {
                implicit.push(QTy::new(Ty::LRef(Box::new(QTy { t: ct.t.clone(), c: true, v: false }))));
            }
            if !has_copy && !has_move {
                implicit.push(QTy::new(Ty::RRef(Box::new(ct.clone()))));
            }
            for p in implicit {
                match self.ics_form(&args[0], &p, own) {
                    Ok(None) => {}
                    // A user-defined sequence to an implicit copy or move
                    // constructor loses to a candidate whose sequences are
                    // all standard (ICS-21).
                    Ok(Some(s)) if matches!(s.kind, crate::ics::IK::User | crate::ics::IK::Ambig) && !matches!(nonref(&args[0].ty).t, Ty::Class(_)) && !own => implicit_user = true,
                    _ => return CtorRes::Unknown,
                }
            }
        }
        let cands: Vec<DeclId> = ctors.iter().copied().filter(|&cid| !(copy && self.func(cid).unwrap().explicit)).collect();
        self.ics_no_viable = false;
        let r = self.ics_select(None, &cands, args);
        match r {
            Ok(Some((cid, seqs))) if implicit_user && !seqs.iter().all(|q| q.kind == crate::ics::IK::Std) => {
                let _ = cid;
                CtorRes::Unknown
            }
            Ok(Some((cid, seqs))) => CtorRes::Chosen(cid, seqs),
            Ok(None) if implicit_user => CtorRes::Unknown,
            Ok(None) => CtorRes::Ambiguous,
            Err(()) if std::mem::take(&mut self.ics_no_viable) => {
                if cands.len() != ctors.len() {
                    // An explicit constructor might fit (CTOR-9).
                    return CtorRes::Unknown;
                }
                CtorRes::NoViable
            }
            Err(()) => CtorRes::Unknown,
        }
    }
}

impl P {
    /// OVL-17: an inherited constructor whose first parameter is a
    /// reference to its base, to the class `d`, or to a class between them.
    /// A call with one argument does not consider it.
    pub fn inherited_copy_shape(&self, cid: DeclId, d: usize) -> bool {
        let Some(&bc) = self.inherited_from.get(&cid) else { return false };
        let Some(b) = self.func(bc).and_then(|f| f.class) else { return false };
        let Some(t) = self.func(cid).and_then(|f| f.fty.params.first().cloned()) else { return false };
        match &t.t {
            Ty::LRef(x) | Ty::RRef(x) => match x.t {
                Ty::Class(k) => k == d || k == b || (self.is_base_of(k, d) && self.is_base_of(b, k)),
                _ => false,
            },
            _ => false,
        }
    }
}

/// A candidate of a call on a class object (OVL-5).
#[derive(Clone)]
enum ObjCand {
    /// A function call operator.
    Op(DeclId),
    /// A surrogate from a conversion function, with the function type.
    Surrogate(DeclId, FnTy),
}

impl P {
    /// The conversion functions of class `c` and its bases that a call on
    /// an object can use. A function of a derived class hides one of a
    /// base with the same name (OVL-6).
    fn surrogate_fns(&self, c: usize, depth: u32, out: &mut Vec<DeclId>) -> bool {
        if depth > 16 || self.class_uncertain(c) {
            return false;
        }
        let sc = self.classes[c].scope;
        let mut names: Vec<(&String, &Vec<DeclId>)> = self.scopes[sc].names.iter().collect();
        names.sort_by_key(|x| x.0.clone());
        let mut own: Vec<DeclId> = Vec::new();
        for (_, ids) in names {
            for &d in ids {
                if self.decls[d].scope == sc && self.func(d).is_some_and(|f| f.kind == FnKind::Conversion) && !out.iter().any(|&o| self.decls[o].name == self.decls[d].name) {
                    own.push(d);
                }
            }
        }
        out.extend(own);
        for b in self.classes[c].bases.clone() {
            match b.class {
                Some(bc) => {
                    if !self.surrogate_fns(bc, depth + 1, out) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }

    /// OVL-5, OVL-6: a call `c(args)` on an object of class `c` whose
    /// class has a conversion function. None when the front end cannot
    /// decide.
    pub fn object_call(&mut self, callee: &Expr, c: usize, args: &[Expr], line: u32) -> Option<Expr> {
        if self.in_template() || !self.class_ok(c) || args.iter().any(|a| a.ty.is_unknown() && !matches!(a.k, EK::InitList(_))) {
            return None;
        }
        let ops = self.lookup_member(c, "operator()").unwrap_or_default();
        let mut convs = Vec::new();
        if !self.surrogate_fns(c, 0, &mut convs) {
            return None;
        }
        let obj_t = nonref(&callee.ty);
        let mut cands: Vec<(ObjCand, Vec<Ics>)> = Vec::new();
        for &fid in &ops {
            let f = self.func(fid)?.clone();
            if f.template || f.opaque || f.constrained || f.explicit_obj {
                return None;
            }
            let Some(os) = self.ics_object(callee, &f) else { continue };
            let np = f.fty.params.len();
            if args.len() + f.defaults.min(np) < np || (args.len() > np && !f.fty.variadic) {
                continue;
            }
            let mut seqs = vec![os];
            let mut ok = true;
            for (i, a) in args.iter().enumerate() {
                if i >= np {
                    seqs.push(Ics::ellipsis());
                    continue;
                }
                match self.ics_form(a, &f.fty.params[i], false).ok()? {
                    Some(s) => seqs.push(s),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                cands.push((ObjCand::Op(fid), seqs));
            }
        }
        for &cf in &convs {
            let f = self.func(cf)?.clone();
            if f.template {
                // No result type is fixed for the call, so no surrogate.
                continue;
            }
            if f.explicit {
                continue;
            }
            if f.fty.ret.is_unknown() {
                return None;
            }
            // The cv-qualifiers of the function include those of the
            // object (Q-0144).
            if (obj_t.c && !f.fty.cv_c) || (obj_t.v && !f.fty.cv_v) {
                continue;
            }
            let r = nonref(&f.fty.ret);
            let ft = match &r.t {
                Ty::Ptr(p) => match &p.t {
                    Ty::Func(ft) => (**ft).clone(),
                    _ => continue,
                },
                Ty::Func(ft) => (**ft).clone(),
                _ => continue,
            };
            let Some(os) = self.ics_object(callee, &f) else { continue };
            // The object converts by the conversion function: a
            // user-defined sequence (ICS-25 makes two of them tie).
            let first = Ics { kind: IK::User, s: os.s.clone(), user: Some(UKey::Fn(cf)), list: crate::ics::LT::No };
            if args.len() < ft.params.len() || (args.len() > ft.params.len() && !ft.variadic) {
                continue;
            }
            let mut seqs = vec![first];
            let mut ok = true;
            for (i, a) in args.iter().enumerate() {
                if i >= ft.params.len() {
                    seqs.push(Ics::ellipsis());
                    continue;
                }
                match self.ics_form(a, &ft.params[i], false).ok()? {
                    Some(s) => seqs.push(s),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                cands.push((ObjCand::Surrogate(cf, ft), seqs));
            }
        }
        if cands.is_empty() {
            self.error(line, "OVL-35", format!("no viable function for the call of an object of class '{}'", self.classes[c].name));
            return Some(Expr::unknown(line, "bad call"));
        }
        let better = |p: &P, x: &[Ics], y: &[Ics]| {
            let mut some = false;
            for (a, b) in x.iter().zip(y.iter()) {
                let k = p.ics_cmp(a, b);
                if k < 0 {
                    return false;
                }
                some |= k > 0;
            }
            some
        };
        let best = (0..cands.len()).find(|&i| (0..cands.len()).all(|j| i == j || better(self, &cands[i].1, &cands[j].1)));
        let Some(bi) = best else {
            self.error(line, "OVL-36", format!("ambiguous call of an object of class '{}'", self.classes[c].name));
            return Some(Expr::unknown(line, "bad call"));
        };
        let (w, seqs) = cands[bi].clone();
        match w {
            ObjCand::Op(fid) => Some(self.call_overloads_pub(Some(callee.clone()), vec![fid], args.to_vec(), line)),
            ObjCand::Surrogate(cf, _) => {
                self.check_chosen_seqs(&seqs[1..], cf, line);
                if self.func(cf).is_some_and(|f| f.deleted) {
                    self.error_because(line, "ICS-8", "DEL-1", format!("call through the deleted conversion function '{}'", self.decls[cf].name));
                }
                if self.in_unevaluated == 0 && self.internal_fns.contains(&cf) {
                    self.internal_uses.push((cf, line));
                }
                let f = self.func(cf).unwrap().clone();
                let (t, cat) = crate::expr2::ret_cat(&f.fty.ret);
                let conv = Expr::new(EK::Call(Box::new(callee.clone()), vec![], Some(cf)), t, cat, line);
                Some(self.build_call(conv, args.to_vec(), line))
            }
        }
    }
}

impl P {
    /// The functions of an overload set `f` or `&f`, and whether `&` was
    /// written.
    pub fn overload_set_of(e: &Expr) -> Option<(Vec<DeclId>, bool)> {
        match &e.k {
            EK::Overloads(fs) => Some((fs.clone(), false)),
            EK::Unary(UOp::AddrOf, x) => match &x.k {
                EK::Overloads(fs) => Some((fs.clone(), true)),
                _ => None,
            },
            _ => None,
        }
    }

    /// OVL-18, OVL-19: the functions of set `fs` that match the target
    /// type `to`. Err when the front end cannot tell (a template, a
    /// constraint, or a target that is not a function type).
    #[allow(clippy::type_complexity)]
    pub fn overload_matches(&self, fs: &[DeclId], to: &QTy) -> Result<Vec<(DeclId, FnTy, Option<usize>)>, ()> {
        let (ft, memclass) = match &to.t {
            Ty::Ptr(p) | Ty::LRef(p) | Ty::RRef(p) => match &p.t {
                Ty::Func(ft) => ((**ft).clone(), None),
                _ => return Err(()),
            },
            Ty::MemPtr(c, p) => match &p.t {
                Ty::Func(ft) => ((**ft).clone(), Some(*c)),
                _ => return Err(()),
            },
            _ => return Err(()),
        };
        let mut matches = Vec::new();
        let mut has_template = false;
        for &fid in fs {
            let f = self.func(fid).ok_or(())?;
            if f.template {
                has_template = true;
                continue;
            }
            if (f.constrained && !self.cons_settled.contains(&fid)) || f.opaque {
                return Err(());
            }
            let mut fty = f.fty.clone();
            if f.explicit_obj && f.params.len() == fty.params.len() + 1 {
                // The object parameter is an ordinary parameter of the
                // pointer type.
                fty.params.insert(0, self.var(f.params[0]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown()));
            }
            let implicit_obj = f.class.is_some() && !f.is_static && !f.explicit_obj && f.kind != FnKind::Ctor;
            match memclass {
                None if implicit_obj => continue,
                Some(_) if !implicit_obj => continue,
                Some(c) => {
                    let fc = f.class.unwrap();
                    if fc != c && !self.is_base_of(fc, c) {
                        continue;
                    }
                }
                None => {}
            }
            // OVL-18: the target's function type; a noexcept function also
            // matches a target without noexcept.
            let mut a = fty.clone();
            a.noexcept = ft.noexcept;
            if a != ft || (ft.noexcept && !fty.noexcept) {
                continue;
            }
            matches.push((fid, fty, memclass.and(f.class)));
        }
        // OVL-19 step 3: a plain function removes the specializations.
        if matches.is_empty() && has_template {
            return Err(());
        }
        Ok(matches)
    }

    /// OVL-19 steps 2 and 4 for the functions of a set that are not
    /// templates: a function whose constraints are not satisfied leaves
    /// the set, and so does a function when another with the same
    /// parameters is more constrained (Q-0142). None when the front end
    /// cannot tell. The set comes back unchanged when no function has
    /// constraints.
    pub fn constraint_reduced(&mut self, fs: &[DeclId]) -> Option<Vec<DeclId>> {
        let has = |p: &P, f: DeclId| p.func(f).is_some_and(|x| !x.template && (x.constrained || p.fn_cons.contains_key(&f)));
        if !fs.iter().any(|&f| has(self, f)) {
            return Some(fs.to_vec());
        }
        let mut kept = Vec::new();
        for &f in fs {
            let x = self.func(f)?;
            if x.template || !has(self, f) {
                kept.push(f);
                continue;
            }
            if !self.fn_cons.contains_key(&f) {
                return None;
            }
            match self.fn_sat(f) {
                crate::cons::Sat::Yes => kept.push(f),
                crate::cons::Sat::No | crate::cons::Sat::Error => {}
                crate::cons::Sat::Unknown => return None,
            }
        }
        let plain: Vec<DeclId> = kept.iter().copied().filter(|&f| self.func(f).is_some_and(|x| !x.template)).collect();
        let mut out = Vec::new();
        for &a in &kept {
            let beaten = plain.iter().any(|&b| {
                if a == b || self.func(a).is_some_and(|x| x.template) {
                    return false;
                }
                let (fa, fb) = (self.func(a).cloned().unwrap(), self.func(b).cloned().unwrap());
                if fa.fty.params != fb.fty.params || fa.fty.cv_c != fb.fty.cv_c || fa.fty.refq != fb.fty.refq {
                    return false;
                }
                let (ca, cb) = (self.fn_cons.get(&a).cloned(), self.fn_cons.get(&b).cloned());
                self.more_constrained(&cb, u32::MAX, &[], &ca, u32::MAX, &[])
            });
            if !beaten {
                out.push(a);
            }
        }
        // The rest of the checks do not look at the constraints again.
        for &f in &out {
            if self.func(f).is_some_and(|x| x.constrained && !x.template) {
                self.cons_settled.insert(f);
            }
        }
        Some(out)
    }

    /// OVL-18 to OVL-20: the function of an overload set that a target
    /// type selects, converted to the target. None when the front end
    /// cannot decide; an error when no function or more than one matches.
    pub fn overload_target(&mut self, e: &Expr, to: &QTy, line: u32) -> Option<Expr> {
        // A function template named alone, or with explicit arguments.
        let mut explicit: Option<std::rc::Rc<Vec<crate::templ::RawArg>>> = None;
        let set = match &e.k {
            EK::Func(f) if self.func(*f).is_some_and(|x| x.template) => Some(vec![*f]),
            EK::TemplId(None, fs, raw) => {
                explicit = Some(raw.clone());
                Some(fs.clone())
            }
            EK::Unary(UOp::AddrOf, x) => match &x.k {
                EK::Func(f) if self.func(*f).is_some_and(|y| y.template) => Some(vec![*f]),
                EK::TemplId(None, fs, raw) => {
                    explicit = Some(raw.clone());
                    Some(fs.clone())
                }
                _ => None,
            },
            _ => None,
        };
        let fs = match set {
            Some(fs) => fs,
            None => Self::overload_set_of(e)?.0,
        };
        if self.in_template() || to.is_unknown() {
            return None;
        }
        // OVL-19, DED-30: each function template adds the specialization
        // that deduction from the target gives.
        let fs = if fs.iter().any(|&f| self.func(f).is_some_and(|x| x.template)) || explicit.is_some() {
            let Some(v) = self.target_specs(&fs, explicit.as_ref(), to, line) else { return None };
            if v.is_empty() {
                // DED-35: no specialization matches the target type.
                let name = fs.first().map(|&f| self.decls[f].name.clone()).unwrap_or_default();
                self.error(line, "DED-35", format!("no specialization of '{}' matches the target type {}", name, show(to)));
                return Some(Expr::unknown(line, "bad overload set"));
            }
            v
        } else {
            fs
        };
        let fs = self.constraint_reduced(&fs)?;
        let mut matches = self.overload_matches(&fs, to).ok()?;
        if matches.len() > 1 {
            // OVL-19 steps 3 and 5: a plain function removes the
            // specializations, and the more specialized template wins.
            if matches.iter().any(|m| !self.spec_of.contains_key(&m.0)) {
                matches.retain(|m| !self.spec_of.contains_key(&m.0));
            } else {
                let ids: Vec<DeclId> = matches.iter().map(|m| m.0).collect();
                let mut keep: Option<DeclId> = None;
                for &a in &ids {
                    let best = ids.iter().all(|&b| {
                        if a == b {
                            return true;
                        }
                        let (ta, tb) = (self.spec_of[&a].0, self.spec_of[&b].0);
                        if ta == tb {
                            return false;
                        }
                        let (x, y) = self.func_order(ta, tb, None);
                        x && !y
                    });
                    if best {
                        keep = Some(a);
                        break;
                    }
                }
                if let Some(k) = keep {
                    matches.retain(|m| m.0 == k);
                }
            }
        }
        if matches.len() == 1 && !self.in_template() && self.in_unevaluated == 0 {
            // INST-7: taking the address needs the definition.
            self.use_fn(matches[0].0);
        }
        if matches.len() != 1 {
            // OVL-20: exactly one function must remain.
            let name = fs.first().map(|&f| self.decls[f].name.clone()).unwrap_or_default();
            let what = if matches.is_empty() { "no function" } else { "more than one function" };
            self.error(line, "OVL-20", format!("{} of the overload set '{}' matches the target type {}", what, name, show(to)));
            return Some(Expr::unknown(line, "bad overload set"));
        }
        let (fid, fty, memclass) = matches.pop().unwrap();
        let f = self.func(fid).unwrap().clone();
        if f.deleted {
            self.error(line, "DEL-1", format!("reference to deleted function '{}'", self.decls[fid].name));
        }
        if f.is_main {
            self.error(line, "TR-3", "address of 'main' taken");
        }
        let x = if let Some(fc) = memclass {
            Expr::new(if self.in_template() { EK::Unknown("pointer to member function".into()) } else { EK::MemberPtr(fid) }, QTy::new(Ty::MemPtr(fc, Box::new(QTy::new(Ty::Func(Box::new(fty)))))), Cat::PR, line)
        } else if f.explicit_obj && (self.in_template() || fty.params.iter().any(|q| q.is_unknown())) {
            Expr::new(EK::Unknown("address of an explicit object member function".into()), QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Func(Box::new(fty)))))), Cat::PR, line)
        } else {
            Expr::new(EK::Func(fid), QTy::new(Ty::Func(Box::new(fty))), Cat::L, line)
        };
        Some(match self.implicit(x, to, false) {
            Ok(x) => x,
            Err(m) => {
                self.error(line, "OVL-20", m);
                Expr::unknown(line, "bad overload set")
            }
        })
    }
}
