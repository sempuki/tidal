"""The conditional operator c ? X : Y with class operands, from the Tidal rules.

Written from specification/expressions/conditional.md (COND-3, COND-4,
COND-5, COND-8, COND-11, COND-12, COND-13, COND-16) and the rules they
cite: REF-1/2/3/10 (direct binding), ICS-* (conversion sequences),
OVL-9/11/14/15 (candidates), CVF-* (conversion functions), CTOR-9/10
(converting constructors), CONV-17/ACC-12 (base access), DEL-1.

Entry point: conditional(x, y, model) -> Result or Reject.
"""
from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import Optional

LV, XV, PRV = "lvalue", "xvalue", "prvalue"

# ---------------------------------------------------------------- types

ARITH = [
    "bool", "char", "signed char", "unsigned char", "wchar_t", "char8_t",
    "char16_t", "char32_t", "short", "unsigned short", "int", "unsigned int",
    "long", "unsigned long", "long long", "unsigned long long",
    "float", "double", "long double",
]
FLOATS = {"float": 1, "double": 2, "long double": 3}
# Integer conversion rank (TYP-8), with signedness. Outside knowledge for
# the character types (see notes.md).
INT_RANK = {
    "bool": 0, "char": 1, "signed char": 1, "unsigned char": 1, "char8_t": 1,
    "short": 2, "unsigned short": 2, "char16_t": 2,
    "int": 3, "unsigned int": 3, "wchar_t": 3, "char32_t": 3,
    "long": 4, "unsigned long": 4, "long long": 5, "unsigned long long": 5,
}
UNSIGNED = {"unsigned char", "unsigned short", "unsigned int", "unsigned long",
            "unsigned long long", "char8_t", "char16_t", "char32_t", "bool"}
SIGNED_OF = {"unsigned int": "int", "unsigned long": "long",
             "unsigned long long": "long long"}
UNSIGNED_OF = {v: k for k, v in SIGNED_OF.items()}
PROMOTED_ARITH = ["int", "unsigned int", "long", "unsigned long", "long long",
                  "unsigned long long", "float", "double", "long double"]


@dataclass(frozen=True)
class Type:
    kind: str                     # arith enum ptr mptr nullptr void class array func
    name: str = ""                # arith name, enum name, class name
    cv: frozenset = frozenset()   # top-level cv-qualifiers
    inner: Optional["Type"] = None  # pointee, array element, member type
    scoped: bool = False          # enum
    bound: Optional[int] = None   # array

    def unqual(self) -> "Type":
        return replace(self, cv=frozenset())

    def with_cv(self, cv) -> "Type":
        return replace(self, cv=frozenset(cv))

    def __str__(self) -> str:
        q = " ".join(c for c in ("const", "volatile") if c in self.cv)
        if self.kind == "ptr":
            s = f"{self.inner}*"
            return f"{s} {q}" if q else s
        if self.kind == "mptr":
            return f"{self.inner} {self.name}::*" + (f" {q}" if q else "")
        if self.kind == "array":
            return f"{self.inner}[{'' if self.bound is None else self.bound}]"
        base = {"nullptr": "std::nullptr_t", "void": "void"}.get(self.kind, self.name)
        return f"{q} {base}" if q else base


def cvset(*names):
    return frozenset(names)


@dataclass
class Ctor:
    """A constructor whose first parameter is `param` (other parameters defaulted)."""
    param: Type
    ref: Optional[str] = None     # None, "&", "&&"
    explicit: bool = False
    deleted: bool = False
    access: str = "public"


@dataclass
class Conv:
    """A conversion function `operator result()` (result with ref)."""
    result: Type
    ref: Optional[str] = None     # None, "&", "&&"
    explicit: bool = False
    deleted: bool = False
    access: str = "public"
    cv: frozenset = frozenset()   # cv-qualifiers of the function
    refqual: Optional[str] = None  # None, "&", "&&"


@dataclass
class ClassDef:
    name: str
    bases: list = field(default_factory=list)   # (name, access, virtual)
    ctors: list = field(default_factory=list)
    convs: list = field(default_factory=list)
    copy: str = "ok"    # ok, deleted, private
    move: str = "ok"    # ok, deleted, private, absent (falls back to copy)


class Model:
    def __init__(self, classes=(), enums=None):
        self.classes = {c.name: c for c in classes}
        self.enums = dict(enums or {})   # name -> scoped (bool)

    # -- parsing a type string such as "const Box", "int*", "const char[3]"
    def ty(self, s: str) -> Type:
        s = s.strip()
        if s.endswith("]"):
            i = s.rindex("[")
            b = s[i + 1:-1].strip()
            return Type("array", inner=self.ty(s[:i]), bound=int(b) if b else None)
        if s.endswith("*") or s.endswith("* const") or s.endswith("* volatile") \
                or s.endswith("* const volatile"):
            i = s.rindex("*")
            top = s[i + 1:].split()
            return Type("ptr", inner=self.ty(s[:i]), cv=frozenset(top))
        words = s.split()
        cv = frozenset(w for w in words if w in ("const", "volatile"))
        base = " ".join(w for w in words if w not in ("const", "volatile"))
        if base in ("std::nullptr_t", "nullptr_t"):
            return Type("nullptr", cv=cv)
        if base == "void":
            return Type("void", cv=cv)
        if base == "unsigned":
            base = "unsigned int"
        if base in ARITH:
            return Type("arith", base, cv)
        if base in self.enums:
            return Type("enum", base, cv, scoped=self.enums[base])
        if base in self.classes:
            return Type("class", base, cv)
        raise ValueError(f"unknown type {s!r}")

    # -- class relations
    def base_paths(self, d: str, b: str):
        """Every path from class d up to class b, as lists of (name, access, virtual)."""
        if d == b:
            return [[]]
        out = []
        for (n, acc, virt) in self.classes[d].bases:
            for p in self.base_paths(n, b):
                out.append([(n, acc, virt)] + p)
        return out

    def is_base(self, b: str, d: str) -> bool:
        """b is a proper base class of d."""
        return b != d and bool(self.base_paths(d, b))

    def base_fault(self, d: str, b: str) -> Optional[str]:
        """A fault of the derived-to-base conversion (CONV-17), or None."""
        paths = self.base_paths(d, b)
        # Distinct subobjects: a path that ends in a virtual link to b is shared.
        nonvirt = [p for p in paths if not (p and p[-1][2])]
        virt = [p for p in paths if p and p[-1][2]]
        subobjects = len(nonvirt) + (1 if virt else 0)
        if subobjects > 1:
            return "ambiguous base (CONV-17, DER-10)"
        if not any(all(acc == "public" for (_, acc, _) in p) for p in paths):
            return "base not accessible (CONV-17, ACC-12)"
        return None

    def conv_functions(self, c: str):
        """Conversion functions of c, with those inherited and not hidden
        (CVF-5). A function to c itself or a base of c is never used (CVF-4)."""
        return [f for f in self._convs(c)
                if not (f.result.kind == "class"
                        and (f.result.name == c or self.is_base(f.result.name, c)))]

    def _convs(self, c: str):
        own = list(self.classes[c].convs)
        seen = {(str(f.result), f.ref) for f in own}
        for (n, _, _) in self.classes[c].bases:
            for f in self._convs(n):
                if (str(f.result), f.ref) not in seen:
                    own.append(f)
        return own


@dataclass
class Operand:
    type: Type
    cat: str = PRV
    throw: bool = False
    null_constant: bool = False   # the literal 0 (or another null pointer constant)
    bitfield: bool = False


@dataclass
class Result:
    type: Type
    cat: str
    rule: str = ""
    note: str = ""

    def __str__(self):
        return f"{self.cat} {self.type}"


@dataclass
class Reject:
    rule: str
    why: str = ""

    def __str__(self):
        return f"rejected ({self.rule}): {self.why}"


def is_class(t: Type) -> bool:
    return t.kind == "class"


# ---------------------------------------------------- standard conversions
EXACT, PROMO, CONV = 0, 1, 2


def promote(name: str) -> str:
    """Integer promotion (CONV-1). Outside knowledge for the character types."""
    if name in FLOATS:
        return name
    if INT_RANK[name] < 3:
        return "int"
    if name == "wchar_t":
        return "int"
    if name == "char32_t":
        return "unsigned int"
    return name


def promote_type(t: Type) -> Type:
    if t.kind == "enum" and not t.scoped:
        return Type("arith", "int")
    if t.kind == "arith":
        return Type("arith", promote(t.name))
    return t.unqual()


def usual_arith(a: Type, b: Type) -> Optional[Type]:
    """CONV-19. None if not valid."""
    if a.kind == "enum" and a.scoped or b.kind == "enum" and b.scoped:
        if a.unqual() == b.unqual():
            return a.unqual()
        return None
    a, b = promote_type(a), promote_type(b)
    if a.name in FLOATS or b.name in FLOATS:
        fa, fb = FLOATS.get(a.name, 0), FLOATS.get(b.name, 0)
        return a if fa >= fb else b
    if a.name == b.name:
        return a
    ua, ub = a.name in UNSIGNED, b.name in UNSIGNED
    ra, rb = INT_RANK[a.name], INT_RANK[b.name]
    if ua == ub:
        return a if ra >= rb else b
    u, s = (a, b) if ua else (b, a)
    if INT_RANK[u.name] >= INT_RANK[s.name]:
        return u
    # the signed type can hold every value of the unsigned type (LP64 model)
    width = {"unsigned int": 32, "unsigned long": 64, "unsigned long long": 64,
             "unsigned short": 16, "unsigned char": 8}
    swidth = {"int": 32, "long": 64, "long long": 64}
    if swidth.get(s.name, 0) > width.get(u.name, 64):
        return s
    return Type("arith", UNSIGNED_OF.get(s.name, u.name))


def decay(t: Type) -> Type:
    if t.kind == "array":
        return Type("ptr", inner=t.inner)
    if t.kind == "func":
        return Type("ptr", inner=t)
    return t


def std_conv(src: Type, dst: Type, model: Model, null_constant=False) -> Optional[int]:
    """Rank of a standard conversion sequence from a value of src to dst (both
    without references), or None. Top-level cv-qualifiers are ignored (ICS-1)."""
    s, d = decay(src).unqual(), dst.unqual()
    if s.kind == "class" or d.kind == "class":
        if s.kind == d.kind == "class":
            if s.name == d.name:
                return EXACT
            if model.is_base(d.name, s.name):
                return CONV   # derived to base (ICS-2)
        return None
    if s == d:
        return EXACT
    if d.kind == "arith" and d.name == "bool":
        if s.kind in ("arith", "ptr", "mptr") or (s.kind == "enum" and not s.scoped):
            return CONV
        return None
    if d.kind == "arith":
        if s.kind == "arith" or (s.kind == "enum" and not s.scoped):
            if promote_type(s) == d or (s.kind == "arith" and s.name == "float"
                                        and d.name == "double"):
                return PROMO
            return CONV
        return None
    if d.kind == "ptr":
        if s.kind == "nullptr" or (null_constant and s.kind == "arith"):
            return CONV
        if s.kind == "ptr":
            sp, dp = s.inner, d.inner
            if sp.unqual() == dp.unqual() and sp.cv <= dp.cv:
                return EXACT   # qualification adjustment
            if not (sp.cv <= dp.cv):
                return None
            if dp.kind == "void":
                return CONV
            if sp.kind == dp.kind == "class" and model.is_base(dp.name, sp.name):
                return CONV
        return None
    if d.kind == "mptr":
        if s.kind == "nullptr" or (null_constant and s.kind == "arith"):
            return CONV
        return None
    if d.kind == "nullptr":
        return None
    return None


# ---------------------------------------------------- reference binding

def binds_directly_plain(rtype: Type, ref: str, x: Operand, model: Model):
    """A reference `rtype ref` binds directly to the operand x itself (no
    conversion function). Returns (rank, fault) or None. REF-1, REF-2,
    REF-3, REF-10. A fault such as an inaccessible base does not stop the
    binding (ICS-8)."""
    t = x.type
    if not (t.unqual() == rtype.unqual()
            or (is_class(t) and is_class(rtype) and model.is_base(rtype.name, t.name))):
        return None
    if not (t.cv <= rtype.cv):
        return None
    rank = EXACT if t.unqual() == rtype.unqual() else CONV
    fault = model.base_fault(t.name, rtype.name) if rank == CONV else None
    if ref == "&":
        if x.cat == LV:
            if x.bitfield and not (rtype.cv == cvset("const")):
                return None
            return (rank, fault)
        # rvalue: only a const, non-volatile lvalue reference, and only to a
        # class rvalue or an xvalue (REF-2) binds directly
        if rtype.cv == cvset("const") and (x.cat == XV or is_class(t)):
            return (rank, fault)
        return None
    if ref == "&&":
        if x.cat == LV:
            return None
        if x.cat == XV or is_class(t):
            return (rank, fault)
        return None
    return None


def object_binding(f: Conv, x: Operand) -> Optional[int]:
    """The implicit object parameter of conversion function f takes x.
    Returns the number of cv-qualifiers on the object parameter (ICS-30
    key), or None. CVF-9, ICS-13, ICS-40."""
    if not (x.type.cv <= f.cv):
        return None
    if f.refqual == "&" and x.cat != LV and f.cv != cvset("const"):
        return None
    if f.refqual == "&&" and x.cat == LV:
        return None
    return len(f.cv)


@dataclass
class Conversion:
    """The outcome of trying to convert one operand for the other."""
    target: Type
    cat: str
    via: str = ""                  # description of the function used
    faults: list = field(default_factory=list)   # (rule, why), checked after
    ambiguous: bool = False


def pick_best(cands):
    """cands: list of (key1, key3, fn_id, info). Lower keys are better.
    key1 compares first (the argument's sequence to the first parameter or
    object parameter, ICS-7). key3 decides between two conversion functions
    when key1 ties (ICS-37). Returns (info, ambiguous)."""
    best = []
    for c in cands:
        better_than_all = True
        for o in cands:
            if o is c:
                continue
            if not better(c, o):
                better_than_all = False
                break
        if better_than_all:
            best.append(c)
    if len(best) == 1:
        return best[0][3], False
    return (cands[0][3] if cands else None), bool(cands)


def better(c, o) -> bool:
    if c[0] != o[0]:
        return c[0] < o[0]
    # key1 ties: two conversion functions compare their third parts (ICS-37)
    if c[1] is not None and o[1] is not None and c[1] != o[1]:
        return c[1] < o[1]
    return False


def check_fn(f, what: str):
    faults = []
    if f.deleted:
        faults.append(("DEL-1", f"{what} is deleted"))
    if f.access != "public":
        faults.append(("ACC-2", f"{what} is {f.access}"))
    return faults


def bind_direct_via_conv(t: Type, ref: str, x: Operand, model: Model):
    """Step 1 or 2 of COND-16 through a conversion function of x (OVL-15):
    a `t ref` binds directly to the glvalue result of a conversion function.
    COND-16 says a class prvalue never binds directly in these steps, and a
    non-class prvalue result needs a temporary, so only functions that
    return a reference of the same kind take part."""
    if not is_class(x.type):
        return None
    cands = []
    for f in model.conv_functions(x.type.name):
        if f.explicit:
            continue  # copy context (outside knowledge, see notes.md)
        if f.ref != ref:
            continue
        r = f.result
        if not (r.unqual() == t.unqual() or (is_class(r) and is_class(t)
                                              and model.is_base(t.name, r.name))):
            continue
        if not (r.cv <= t.cv):
            continue
        k1 = object_binding(f, x)
        if k1 is None:
            continue
        k3 = EXACT if r.unqual() == t.unqual() else CONV
        cands.append(((EXACT, 0, k1), (k3, len(t.cv - r.cv)), id(f), f))
    if not cands:
        return None
    f, amb = pick_best(cands)
    return f, amb


def try_convert(x: Operand, y: Operand, model: Model) -> Optional[Conversion]:
    """COND-16: does x convert for y? Returns a Conversion or None."""
    T = y.type
    # Step 1
    if y.cat == LV:
        r = binds_directly_plain(T, "&", x, model) if x.cat != PRV or not is_class(x.type) else None
        if r is not None and x.cat == LV:
            rank, fault = r
            return Conversion(T, LV, "direct binding",
                              [("COND-16", fault)] if fault else [])
        if r is not None:
            # An xvalue x: a const T& binds directly to it (REF-2).
            rank, fault = r
            return Conversion(T, LV, "direct binding of an xvalue",
                              [("COND-16", fault)] if fault else [])
        got = bind_direct_via_conv(T, "&", x, model)
        if got:
            f, amb = got
            c = Conversion(T, LV, f"conversion function to {f.result}&", check_fn(f, "the conversion function"))
            c.ambiguous = amb
            return c
    # Step 2
    if y.cat == XV:
        if x.cat == XV:
            r = binds_directly_plain(T, "&&", x, model)
            if r is not None:
                rank, fault = r
                return Conversion(T, XV, "direct binding",
                                  [("COND-16", fault)] if fault else [])
        got = bind_direct_via_conv(T, "&&", x, model)
        if got:
            f, amb = got
            c = Conversion(T, XV, f"conversion function to {f.result}&&", check_fn(f, "the conversion function"))
            c.ambiguous = amb
            return c
    # Step 3: the target table
    X = x.type
    if is_class(X) and is_class(T) and X.name == T.name:
        if X.cv <= T.cv:
            return Conversion(T, PRV, "copy", copy_faults(x, model))
        return None
    if is_class(X) and is_class(T) and model.is_base(T.name, X.name):
        tgt = Type("class", T.name, X.cv)
        faults = []
        fault = model.base_fault(X.name, T.name)
        if fault:
            faults.append(("COND-16", fault))
        faults += copy_faults(replace(x, type=tgt), model)
        return Conversion(tgt, PRV, "derived to base", faults)
    if is_class(X) and is_class(T) and model.is_base(X.name, T.name):
        return None
    tgt = decay(T)
    if not is_class(tgt):
        tgt = tgt.unqual()
    return implicit_to(x, tgt, model)


def copy_faults(x: Operand, model: Model, cls: Optional[str] = None):
    """The copy (lvalue) or move (xvalue) that makes a prvalue from a class
    glvalue (COND-8). A prvalue needs no copy (COND-12)."""
    if x.cat == PRV or not is_class(x.type):
        return []
    c = model.classes[cls or x.type.name]
    status = c.copy
    what = "copy constructor"
    if x.cat == XV and c.move != "absent":
        status, what = c.move, "move constructor"
    if status == "deleted":
        return [("COND-8", f"the {what} of {c.name} is deleted")]
    if status == "private":
        return [("COND-8", f"the {what} of {c.name} is not accessible")]
    return []


def ctor_first_part(p: Ctor, x: Operand, model: Model):
    """The argument x to the first parameter of constructor p, by a standard
    sequence only (ICS-10). Returns a key (lower is better) or None."""
    if p.ref is None:
        r = std_conv(x.type, p.param, model, x.null_constant)
        return None if r is None else (r, 0, 0)
    rt = p.param
    direct = binds_directly_plain(rt, p.ref, x, model)
    if direct is not None:
        rank, _ = direct
        rref = 0 if (p.ref == "&&") else 1
        return (rank, rref if x.cat != LV else 0, len(rt.cv))
    # bind to a temporary: only const& (not volatile) or &&
    if p.ref == "&" and rt.cv != cvset("const"):
        return None
    if p.ref == "&&" and x.cat == LV and (x.type.unqual() == rt.unqual()):
        return None
    if is_class(x.type) and is_class(rt):
        return None   # would need a user-defined conversion
    r = std_conv(x.type, rt, model, x.null_constant)
    if r is None:
        return None
    return (r, 0 if p.ref == "&&" else 1, len(rt.cv))


def implicit_to(x: Operand, tgt: Type, model: Model) -> Optional[Conversion]:
    """Step 3 of COND-16 for "any other pair": an implicit conversion
    sequence from x to tgt, in a copy context. Access and deleted functions
    do not stop the sequence (ICS-8); they are faults checked afterwards."""
    X = x.type
    if not is_class(X) and not is_class(tgt):
        r = std_conv(X, tgt, model, x.null_constant)
        return Conversion(tgt, PRV, "standard") if r is not None else None
    cands = []
    if is_class(tgt):
        for p in model.classes[tgt.name].ctors:
            if p.explicit:
                continue   # CTOR-9: copy-initialization
            k1 = ctor_first_part(p, x, model)
            if k1 is not None:
                cands.append((k1, None, id(p), ("ctor", p)))
    if is_class(X):
        for f in model.conv_functions(X.name):
            if f.explicit:
                continue   # CVF-11
            r = f.result
            if is_class(tgt):
                # OVL-14: conversion functions to T or a class derived from T
                if not (is_class(r) and (r.name == tgt.name or model.is_base(tgt.name, r.name))):
                    continue
                k3 = EXACT if r.name == tgt.name else CONV
            else:
                k3 = std_conv(r, tgt, model)
                if k3 is None:
                    continue
            k1 = object_binding(f, x)
            if k1 is None:
                continue
            cands.append(((EXACT, 0, k1), k3, id(f), ("conv", f)))
    if not cands:
        return None
    (kind, fn), amb = pick_best(cands)
    if kind == "ctor":
        faults = check_fn(fn, "the constructor")
        via = f"constructor {tgt.name}({fn.param}{fn.ref or ''})"
    else:
        faults = check_fn(fn, "the conversion function")
        via = f"conversion function to {fn.result}{fn.ref or ''}"
    c = Conversion(tgt, PRV, via, faults)
    c.ambiguous = amb
    return c


# ---------------------------------------------------- the rows of COND-3

def same_ignoring_cv(a: Type, b: Type) -> bool:
    return a.unqual() == b.unqual()


def rows(x: Operand, y: Operand, model: Model, class_rows=True):
    """COND-3, the rows before the last. Returns Result, Reject, or the
    string "LAST" when the last row (COND-16) must decide."""
    a, b = x.type, y.type
    # Row 1 and row 2: two lvalues of one type apart from cv-qualifiers
    if x.cat == LV and y.cat == LV and same_ignoring_cv(a, b):
        if a.cv <= b.cv or b.cv <= a.cv:
            big = a if b.cv <= a.cv else b
            note = "bit-field" if (x.bitfield or y.bitfield) else ""
            return Result(big, LV, "COND-3 row 1", note)
        if not is_class(a):
            return Result(a.unqual(), PRV, "COND-3 row 2")
        return "LAST"
    # Row 3: two xvalues. Read as: one type apart from cv-qualifiers, where
    # one set includes the other (by analogy with row 1). For class types
    # the types must match exactly, else COND-16 applies.
    if x.cat == XV and y.cat == XV and same_ignoring_cv(a, b):
        if a == b or (not is_class(a) and (a.cv <= b.cv or b.cv <= a.cv)):
            big = a if b.cv <= a.cv else b
            return Result(big, XV, "COND-3 row 3")
        if is_class(a):
            return "LAST"
    # Row 4: an lvalue and an xvalue of one type
    if {x.cat, y.cat} == {LV, XV} and same_ignoring_cv(a, b):
        if not is_class(a):
            return Result(a.unqual(), PRV, "COND-3 row 4")
        if a == b:
            return finish_class_prvalue(a, x, y, model, "COND-3 row 4")
        return "LAST"
    if is_class(a) or is_class(b):
        if is_class(a) and a == b:
            return finish_class_prvalue(a, x, y, model, "COND-3 row 5")
        return "LAST"
    # Rows 5 to 9 for non-class operands: decay, drop cv
    da, db = decay(a).unqual(), decay(b).unqual()
    if da == db:
        if da.kind == "void":
            return Result(da, PRV, "COND-4")
        return Result(da, PRV, "COND-3 row 5")
    ar = lambda t: t.kind == "arith" or t.kind == "enum"
    if ar(da) and ar(db):
        if da.kind == "enum" and da.scoped or db.kind == "enum" and db.scoped:
            return Reject("COND-5", f"no common type for {da} and {db}")
        u = usual_arith(da, db)
        if u is None:
            return Reject("COND-5", f"no common type for {da} and {db}")
        return Result(u, PRV, "COND-3 row 6")
    # Row 7: pointers
    if da.kind == "ptr" or db.kind == "ptr":
        p = composite_pointer(da, x, db, y, model)
        if p is None:
            return Reject("COND-5", f"no common type for {da} and {db}")
        return Result(p, PRV, "COND-3 row 7")
    # Row 8
    if da.kind == "nullptr" and (db.kind == "nullptr" or y.null_constant):
        return Result(da, PRV, "COND-3 row 8")
    if db.kind == "nullptr" and x.null_constant:
        return Result(db, PRV, "COND-3 row 8")
    # Row 9
    if da.kind == "mptr" and (db.kind == "nullptr" or y.null_constant):
        return Result(da, PRV, "COND-3 row 9")
    if db.kind == "mptr" and (da.kind == "nullptr" or x.null_constant):
        return Result(db, PRV, "COND-3 row 9")
    return Reject("COND-5", f"no common type for {da} and {db}")


def composite_pointer(a: Type, x: Operand, b: Type, y: Operand, model: Model):
    """COND-3 row 7: a pointer and a pointer or null pointer constant."""
    if a.kind != "ptr":
        a, x, b, y = b, y, a, x
    if b.kind == "nullptr" or (y.null_constant and b.kind == "arith"):
        return a
    if b.kind != "ptr":
        return None
    pa, pb = a.inner, b.inner
    cv = pa.cv | pb.cv
    if pa.unqual() == pb.unqual():
        return Type("ptr", inner=pa.with_cv(cv))
    if pa.kind == "void" or pb.kind == "void":
        return Type("ptr", inner=Type("void", cv=cv))
    if is_class(pa) and is_class(pb):
        if model.is_base(pa.name, pb.name):
            return Type("ptr", inner=pa.with_cv(cv))
        if model.is_base(pb.name, pa.name):
            return Type("ptr", inner=pb.with_cv(cv))
    return None


def finish_class_prvalue(t: Type, x: Operand, y: Operand, model, rule):
    """A prvalue of class type t. COND-8: a glvalue operand is copied (an
    lvalue) or moved (an xvalue), and the copy must be valid. COND-12: two
    prvalues need no copy."""
    for o in (x, y):
        f = copy_faults(o, model, t.name)
        if f:
            return Reject(f[0][0], f[0][1])
    return Result(t, PRV, rule)


# ---------------------------------------------------- COND-11

def class_seq(x: Operand, p: Type, model: Model):
    """The sequence of class operand x to the built-in parameter p, through
    a conversion function (OVL-P4). Returns (fn or None, k3, ambiguous) or
    None. Explicit functions take no part (CVF-11)."""
    cands = []
    for f in model.conv_functions(x.type.name):
        if f.explicit:
            continue
        k3 = std_conv(f.result, p, model)
        if k3 is None:
            continue
        k1 = object_binding(f, x)
        if k1 is None:
            continue
        cands.append(((EXACT, 0, k1), k3, id(f), f))
    if not cands:
        return None
    f, amb = pick_best(cands)
    k3 = None if amb else std_conv(f.result, p, model)
    return (None if amb else f, k3, amb)


def builtin_candidates(x: Operand, y: Operand, model: Model):
    types = []
    for o in (x, y):
        if is_class(o.type):
            for f in model.conv_functions(o.type.name):
                if not f.explicit:
                    types.append(decay(f.result).unqual())
        else:
            types.append(decay(o.type).unqual())
    cands = [(Type("arith", l), Type("arith", r)) for l in PROMOTED_ARITH for r in PROMOTED_ARITH]
    ptrs = []
    for t in types:
        if t.kind in ("ptr", "mptr") and t not in ptrs:
            ptrs.append(t)
    for i, p in enumerate(ptrs):
        for q in ptrs[i + 1:]:
            if p.kind == q.kind == "ptr":
                c = composite_pointer(p, Operand(p), q, Operand(q), model)
                if c is not None and c not in ptrs:
                    ptrs.append(c)
    for p in ptrs:
        cands.append((p, p))
    for t in types:
        if t.kind == "enum" and t.scoped and (t, t) not in cands:
            cands.append((t, t))
    return cands


def arg_seq(o: Operand, p: Type, model: Model):
    if is_class(o.type):
        return class_seq(o, p, model)
    r = std_conv(o.type, p, model, o.null_constant)
    return None if r is None else ("std", r)


def seq_better(s1, s2) -> Optional[bool]:
    """True if s1 is better than s2, False if worse, None if neither."""
    if s1[0] == "std":
        if s1[1] < s2[1]:
            return True
        if s1[1] > s2[1]:
            return False
        return None
    f1, k1, a1 = s1
    f2, k2, a2 = s2
    if a1 or a2 or f1 is not f2:
        return None   # ICS-25: different functions, or ambiguous, tie
    if k1 < k2:
        return True
    if k1 > k2:
        return False
    return None


def cond11(x: Operand, y: Operand, model: Model):
    viable = []
    for (l, r) in builtin_candidates(x, y, model):
        s1, s2 = arg_seq(x, l, model), arg_seq(y, r, model)
        if s1 is None or s2 is None:
            continue
        viable.append(((l, r), s1, s2))
    if not viable:
        return Reject("COND-5", "neither operand converts (COND-16), and COND-11 finds no viable candidate")

    def beats(a, b):
        some = False
        for i in (1, 2):
            v = seq_better(a[i], b[i])
            if v is False:
                return False
            if v is True:
                some = True
        return some

    best = [a for a in viable if all(a is b or beats(a, b) for b in viable)]
    if len(best) != 1:
        return Reject("COND-11", "no built-in candidate is better than all others (COND-5)")
    (l, r), s1, s2 = best[0]
    for s in (s1, s2):
        if s[0] != "std":
            f, _, amb = s
            if amb:
                return Reject("ICS-8", "the conversion of an operand is ambiguous (CVF-12)")
            for rule, why in check_fn(f, "the conversion function"):
                return Reject(rule, why)
    if l.kind == "arith":
        t = usual_arith(l, r)
    else:
        t = l
    return Result(t, PRV, "COND-11", f"candidate ({l}, {r})")


# ---------------------------------------------------- the operator

def conditional(x: Operand, y: Operand, model: Model):
    """The type and value category of c ? x : y, or a Reject."""
    # COND-4, COND-13
    if x.throw and y.throw:
        return Result(Type("void"), PRV, "COND-4")
    if x.throw:
        return Result(y.type, y.cat, "COND-4")
    if y.throw:
        return Result(x.type, x.cat, "COND-4")
    vx, vy = x.type.kind == "void", y.type.kind == "void"
    if vx and vy:
        return Result(Type("void"), PRV, "COND-4")
    if vx or vy:
        return Reject("COND-13", "a void operand with a value operand")

    r = rows(x, y, model)
    if r != "LAST":
        return r

    # The last row: COND-16
    a, b = x.type, y.type
    cx = try_convert(x, y, model)
    cy = try_convert(y, x, model)
    if cx and cy:
        return Reject("COND-5", "each operand converts for the other (COND-16)")
    if not cx and not cy:
        if is_class(a) and is_class(b) and a.name == b.name:
            return Reject("COND-5", "neither operand converts, one class type (COND-P2)")
        return cond11(x, y, model)
    conv, src, other = (cx, x, y) if cx else (cy, y, x)
    if conv.ambiguous:
        return Reject("COND-16", "the implicit conversion sequence is ambiguous")
    for rule, why in conv.faults:
        return Reject(rule, why)
    new = Operand(conv.target, conv.cat, bitfield=False)
    r = rows(new, other, model) if src is x else rows(other, new, model)
    if r == "LAST":
        return Reject("COND-5", f"after the conversion ({conv.via}), no row of COND-3 gives one type")
    if isinstance(r, Result):
        r.note = (r.note + "; " if r.note else "") + f"{'second' if src is x else 'third'} operand converts by {conv.via} (COND-16)"
    return r


# ---------------------------------------------------- small helpers for callers

def op(model: Model, t: str, cat: str = PRV, **kw) -> Operand:
    return Operand(model.ty(t), cat, **kw)


def describe(res) -> str:
    return str(res)
