// Class layout on the matrix targets: CLS-10, CLS-11, CLS-31, CLS-33,
// CLS-34, CLS-35, BF-9, STG-25, ALN-1, TYP-2.
//
// The order (CLS-35): the pointer for virtual calls if the class has its
// own, the bases that are not virtual in the order of the base list (a
// first base with a pointer goes to offset 0 and lends its pointer), the
// members, and, in a complete object, the virtual bases. Each part starts
// at the next multiple of its alignment. An empty base, or an empty
// `[[no_unique_address]]` member, goes to offset 0, and moves to the next
// free offset when it would put two subobjects of one type at one
// address. A base or a `[[no_unique_address]]` member of a class that
// does not keep its end padding lets the next parts start in that
// padding.

use crate::parser::*;
use crate::types::*;
use crate::ast::*;

/// Where a non-static data member lies: its byte offset, and for a
/// bit-field its first bit (from the start of the class) and width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldPos {
    pub off: u64,
    pub bits: Option<(u64, u32)>,
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
    pub size: u64,
    pub align: u64,
    /// The data size: the size without the padding at the end.
    pub dsize: u64,
    /// The size and alignment of the part without the virtual bases.
    pub nvsize: u64,
    pub nvalign: u64,
    /// CLS-35: the class keeps its end padding.
    pub keeps: bool,
    pub empty: bool,
    /// The class has its own pointer for virtual calls at offset 0.
    pub own_vptr: bool,
    /// The offset of each direct base, by its index in the base list. A
    /// virtual base has its offset in a complete object of the class.
    pub bases: Vec<u64>,
    /// The place of each member, by `FieldInfo::index`.
    pub fields: Vec<Option<FieldPos>>,
    /// Every virtual base of a complete object, with its offset.
    pub vbases: Vec<(usize, u64)>,
    /// The subobjects of empty class type in the part without virtual
    /// bases, with their offsets (the class itself first, if empty).
    pub empties: Vec<(usize, u64)>,
    /// The same for a complete object.
    pub all_empties: Vec<(usize, u64)>,
}

struct St {
    size: u64,
    dbits: u128,
    align: u64,
    empties: Vec<(usize, u64)>,
}

impl St {
    fn dsize(&self) -> u64 {
        self.dbits.div_ceil(8) as u64
    }
    fn conflicts(&self, comp: &[(usize, u64)], off: u64) -> bool {
        comp.iter().any(|&(t, x)| self.empties.contains(&(t, x + off)))
    }
    /// The offset of a part: an empty one tries offset 0 first, and a
    /// part moves by its alignment past a conflict.
    fn place(&self, comp: &[(usize, u64)], empty: bool, al: u64) -> u64 {
        let al = al.max(1);
        if empty && !self.conflicts(comp, 0) {
            return 0;
        }
        let mut off = self.dsize().div_ceil(al) * al;
        if empty && off == 0 {
            off = al;
        }
        while self.conflicts(comp, off) {
            off += al;
        }
        off
    }
}

const EMPTIES_CAP: usize = 256;

fn shifted(v: &[(usize, u64)], by: u64) -> impl Iterator<Item = (usize, u64)> + '_ {
    v.iter().map(move |&(t, x)| (t, x + by))
}

impl P {
    fn ptr_size(&self) -> u64 {
        if ilp32() {
            4
        } else {
            8
        }
    }

    /// A class with a virtual function or a virtual base, its own or a
    /// base's: it has a pointer for virtual calls (CLS-35).
    pub fn is_dynamic(&self, c: usize) -> bool {
        self.is_dynamic_d(c, 0)
    }
    fn is_dynamic_d(&self, c: usize, depth: u32) -> bool {
        let ci = &self.classes[c];
        depth < 40 && (ci.has_virtual || ci.bases.iter().any(|b| b.virt || b.class.is_some_and(|bc| self.is_dynamic_d(bc, depth + 1))))
    }

    /// An empty class: no non-static data members other than unnamed
    /// bit-fields of width 0, no pointer for virtual calls, and only
    /// empty bases.
    pub fn is_empty_class(&self, c: usize) -> bool {
        self.is_empty_d(c, 0)
    }
    fn is_empty_d(&self, c: usize, depth: u32) -> bool {
        let ci = &self.classes[c];
        if depth > 40 || ci.has_virtual || ci.key == 2 {
            return false;
        }
        let no_fields = ci.fields.iter().all(|&f| match &self.decls[f].k {
            DK::Field(fi) => fi.bits == Some(0) && self.decls[f].name.is_empty(),
            _ => true,
        });
        no_fields && ci.bases.iter().all(|b| !b.virt && b.class.is_some_and(|bc| self.is_empty_d(bc, depth + 1)))
    }

    /// Every virtual base of a class, in the order of a walk of the base
    /// lists, each once.
    pub fn virtual_bases(&self, c: usize) -> Vec<usize> {
        fn walk(p: &P, c: usize, out: &mut Vec<usize>, depth: u32) {
            if depth > 40 {
                return;
            }
            for b in &p.classes[c].bases {
                let Some(bc) = b.class else { continue };
                if b.virt && !out.contains(&bc) {
                    out.push(bc);
                }
                walk(p, bc, out, depth + 1);
            }
        }
        let mut out = Vec::new();
        walk(self, c, &mut out, 0);
        out
    }

    /// CLS-35: whether a class keeps the padding at its end, so that the
    /// next parts after a base or a `[[no_unique_address]]` member of the
    /// class start after all of its bytes.
    pub fn keeps_padding(&self, c: usize) -> bool {
        self.keeps_d(c, 0)
    }
    fn keeps_d(&self, c: usize, depth: u32) -> bool {
        let ci = &self.classes[c];
        if depth > 40 || !ci.bases.is_empty() || ci.has_virtual {
            return false;
        }
        let user = |d: DeclId| self.func(d).is_some_and(|f| !f.deleted && !f.first_defaulted);
        if ci.ctors.iter().any(|&d| user(d)) || ci.dtor.is_some_and(user) {
            return false;
        }
        // A constructor template is a user-provided constructor.
        if self.scopes[ci.scope].names.get(&ci.name).is_some_and(|v| v.iter().any(|&d| matches!(self.decls[d].k, DK::Template(..)))) {
            return false;
        }
        if let Some(v) = self.scopes[ci.scope].names.get("operator=") {
            for &d in v {
                let Some(f) = self.func(d) else { continue };
                if !user(d) || f.fty.params.len() != 1 {
                    continue;
                }
                // A copy assignment operator: the parameter is the class,
                // or an lvalue reference to it.
                let pt = match &f.fty.params[0].t {
                    Ty::LRef(i) => &i.t,
                    Ty::RRef(_) => continue,
                    t => t,
                };
                if *pt == Ty::Class(c) {
                    return false;
                }
            }
        }
        for &f in &ci.fields {
            let DK::Field(fi) = &self.decls[f].k else { continue };
            if fi.init.is_some() || self.member_access.get(&f).copied().unwrap_or(0) != 0 {
                return false;
            }
            let mut t = &fi.ty.t;
            while let Ty::Array(el, _) = t {
                t = &el.t;
            }
            match t {
                Ty::LRef(_) | Ty::RRef(_) => return false,
                Ty::Class(m) => {
                    if !self.keeps_d(*m, depth + 1) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }

    /// The layout of a class, or None when the front end does not know it.
    pub fn layout(&self, c: usize) -> Option<Layout> {
        self.layout_d(c, 0)
    }

    /// The size, alignment, and empty subobjects of a member type.
    fn member_parts(&self, t: &Ty, depth: u32) -> Option<(u64, u64, Vec<(usize, u64)>)> {
        match t {
            Ty::LRef(_) | Ty::RRef(_) => Some((self.ptr_size(), self.ptr_size(), vec![])),
            Ty::Class(m) => {
                let l = self.layout_d(*m, depth + 1)?;
                Some((l.size, l.align, l.all_empties))
            }
            Ty::Array(el, Some(n)) => {
                let (s, a, e) = self.member_parts(&el.t, depth + 1)?;
                let mut all = Vec::new();
                if !e.is_empty() {
                    for i in 0..*n {
                        if all.len() > EMPTIES_CAP {
                            break;
                        }
                        all.extend(shifted(&e, i * s));
                    }
                }
                Some((s * n, a, all))
            }
            t => Some((self.sizeof_ty(t)?, self.alignof_ty(t)?, vec![])),
        }
    }

    fn layout_d(&self, c: usize, depth: u32) -> Option<Layout> {
        let ci = &self.classes[c];
        if depth > 40 || !ci.complete || ci.opaque || ci.template || ci.bases.iter().any(|b| b.class.is_none()) {
            return None;
        }
        let ps = self.ptr_size();
        let union = ci.key == 2;
        let dynamic = self.is_dynamic(c);
        let mut st = St { size: 0, dbits: 0, align: 1, empties: Vec::new() };
        let mut l = Layout { bases: vec![0; ci.bases.len()], ..Default::default() };
        let empty = self.is_empty_class(c);
        if empty {
            st.empties.push((c, 0));
        }
        // The first base that is not virtual and has a pointer for
        // virtual calls goes to offset 0.
        let primary = if dynamic {
            ci.bases.iter().position(|b| !b.virt && b.class.is_some_and(|bc| self.is_dynamic(bc)))
        } else {
            None
        };
        let mut order: Vec<usize> = Vec::new();
        if let Some(pi) = primary {
            order.push(pi);
        } else if dynamic {
            l.own_vptr = true;
            st.size = ps;
            st.dbits = 8 * ps as u128;
            st.align = ps;
        }
        order.extend((0..ci.bases.len()).filter(|&i| !ci.bases[i].virt && Some(i) != primary));
        for i in order {
            let bc = ci.bases[i].class?;
            let bl = self.layout_d(bc, depth + 1)?;
            let off = if bl.empty {
                let off = st.place(&bl.empties, true, bl.nvalign);
                st.size = st.size.max(off + bl.size);
                off
            } else {
                let off = st.place(&bl.empties, false, bl.nvalign);
                st.dbits = 8 * (off + if bl.keeps { bl.size } else { bl.nvsize }) as u128;
                st.size = st.size.max(off + bl.nvsize);
                off
            };
            st.align = st.align.max(bl.nvalign);
            if st.empties.len() < EMPTIES_CAP {
                st.empties.extend(shifted(&bl.empties, off));
            }
            l.bases[i] = off;
        }
        // The members.
        let mut nfields = 0;
        let mut fields: Vec<(DeclId, FieldInfo)> = Vec::new();
        for &f in &ci.fields {
            if let DK::Field(fi) = &self.decls[f].k {
                nfields = nfields.max(fi.index + 1);
                fields.push((f, fi.clone()));
            }
        }
        l.fields = vec![None; nfields];
        for (f, fi) in &fields {
            if let Some(w) = fi.bits {
                // BF-9.
                let bt = match fi.ty.t.clone() {
                    Ty::Enum(e) => self.enum_underlying(e),
                    t => t.clone(),
                };
                let ts = self.sizeof_ty(&bt)?;
                let ta = self.alignof_ty(&bt)?;
                let w = w as u64;
                let named = !self.decls[*f].name.is_empty();
                if w > 8 * ts {
                    return None;
                }
                if union {
                    l.fields[fi.index] = Some(FieldPos { off: 0, bits: Some((0, w as u32)) });
                    st.size = st.size.max(w.div_ceil(8));
                    if named {
                        st.align = st.align.max(ta);
                    }
                    continue;
                }
                let unit = 8 * ta;
                if w == 0 {
                    st.dbits = st.dbits.div_ceil(unit as u128) * unit as u128;
                    st.size = st.size.max((st.dbits / 8) as u64);
                    continue;
                }
                let start = st.dbits as u64;
                let k = start / unit * unit;
                let pos = if start + w <= k + 8 * ts { start } else { start.div_ceil(unit) * unit };
                l.fields[fi.index] = Some(FieldPos { off: pos / 8, bits: Some((pos, w as u32)) });
                st.dbits = (pos + w) as u128;
                st.size = st.size.max(st.dsize());
                if named {
                    st.align = st.align.max(ta);
                }
                continue;
            }
            let (s, a, e) = self.member_parts(&fi.ty.t, depth)?;
            let a = a.max(fi.align);
            st.align = st.align.max(a);
            if union {
                l.fields[fi.index] = Some(FieldPos { off: 0, bits: None });
                st.size = st.size.max(s);
                if st.empties.len() < EMPTIES_CAP {
                    st.empties.extend(e);
                }
                continue;
            }
            let class = match fi.ty.t {
                Ty::Class(m) => Some(m),
                _ => None,
            };
            let off = if fi.nua && class.is_some_and(|m| self.is_empty_class(m)) {
                // CLS-31, ATTR-9: no bytes of its own.
                let off = st.place(&e, true, a);
                st.size = st.size.max(off + s);
                off
            } else {
                let off = st.place(&e, false, a);
                let used = match class {
                    Some(m) if fi.nua && !self.keeps_padding(m) => self.layout_d(m, depth + 1)?.dsize,
                    _ => s,
                };
                st.dbits = 8 * (off + used) as u128;
                st.size = st.size.max(off + s);
                off
            };
            if st.empties.len() < EMPTIES_CAP {
                st.empties.extend(shifted(&e, off));
            }
            l.fields[fi.index] = Some(FieldPos { off, bits: None });
        }
        st.align = st.align.max(ci.align);
        l.nvalign = st.align;
        l.nvsize = st.dsize();
        l.empties = st.empties.clone();
        // The virtual bases of a complete object.
        for v in self.virtual_bases(c) {
            let vl = self.layout_d(v, depth + 1)?;
            let off = if vl.empty {
                let off = st.place(&vl.empties, true, vl.nvalign);
                st.size = st.size.max(off + vl.size);
                off
            } else {
                let off = st.place(&vl.empties, false, vl.nvalign);
                st.dbits = 8 * (off + if vl.keeps { vl.size } else { vl.nvsize }) as u128;
                st.size = st.size.max(off + vl.nvsize);
                off
            };
            st.align = st.align.max(vl.nvalign);
            if st.empties.len() < EMPTIES_CAP {
                st.empties.extend(shifted(&vl.empties, off));
            }
            l.vbases.push((v, off));
            if let Some(i) = ci.bases.iter().position(|b| b.virt && b.class == Some(v)) {
                l.bases[i] = off;
            }
        }
        l.all_empties = st.empties.clone();
        l.align = st.align;
        l.dsize = st.dsize();
        let mut size = st.size.max(l.dsize);
        if size == 0 {
            // CLS-11: an empty class has size 1.
            size = 1;
        }
        l.size = size.div_ceil(l.align) * l.align;
        l.keeps = self.keeps_padding(c);
        l.empty = empty;
        if l.keeps || l.empty {
            l.dsize = l.size;
            l.nvsize = l.size;
        }
        Some(l)
    }
}
