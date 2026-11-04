"""Class layout on the Tidal matrix targets, from the rules alone.

Rules used: CLS-10, CLS-11, CLS-31, CLS-33, CLS-34, CLS-35 (members.md),
BF-1, BF-3, BF-9 (unions.md), ALN-1, ALN-5 (alignment.md),
TYP-1, TYP-2, TYP-12 (types.md), ATTR-9, ATTR-16 (attributes.md).

A class description is a Cls object:

    Cls(name, bases=[Base(cls, virtual=False)], members=[Mem(...)],
        union=False, virtual_funcs=False, alignas=None,
        user_ctor=False, user_dtor=False, user_copy_assign=False,
        ctor_template=False, defaulted_ctor=False, deleted_ctor=False,
        user_move_assign=False)

    Mem(name, type, bounds=(), width=None, alignas=None, nua=False,
        access='public', init=False)

A type is a string naming a fundamental type ('char', 'int', 'long',
'ptr', 'double', 'long double', ...), a Cls, or ('enum', 'int') for an
enumeration with that underlying type. An unnamed bit-field has name None.
An anonymous union is a member whose type is a union Cls.

layout(cls, target='LP64', compiler='gcc16') returns a Layout with
size, align, offsets (member name -> byte offset), bitoffsets (bit-field
name -> bit offset from the start of the class), and bases
(base class name -> offset; virtual bases included).
"""

from dataclasses import dataclass, field
from typing import Any, Optional

# TYP-1 and TYP-2.
_SIZES = {
    'LP64': {'bool': 1, 'char': 1, 'signed char': 1, 'unsigned char': 1,
             'char8_t': 1, 'short': 2, 'unsigned short': 2, 'int': 4,
             'unsigned': 4, 'unsigned int': 4, 'long': 8, 'unsigned long': 8,
             'long long': 8, 'unsigned long long': 8, 'wchar_t': 4,
             'char16_t': 2, 'char32_t': 4, 'float': 4, 'double': 8,
             'long double': 16, 'ptr': 8, 'nullptr_t': 8},
    'ILP32': {'bool': 1, 'char': 1, 'signed char': 1, 'unsigned char': 1,
              'char8_t': 1, 'short': 2, 'unsigned short': 2, 'int': 4,
              'unsigned': 4, 'unsigned int': 4, 'long': 4, 'unsigned long': 4,
              'long long': 8, 'unsigned long long': 8, 'wchar_t': 4,
              'char16_t': 2, 'char32_t': 4, 'float': 4, 'double': 8,
              'long double': 12, 'ptr': 4, 'nullptr_t': 4},
}
# Types the text does not size: references and pointers to members.
# Not given by the text; the simplest choice is the size of a pointer.
for _t in _SIZES.values():
    _t['ref'] = _t['ptr']
    _t['memptr'] = _t['ptr']
    _t['memfnptr'] = _t['ptr']

_INT_LADDER = ['char', 'short', 'int', 'long long']  # for U in BF-9


def _fund_align(t, target, compiler):
    s = _SIZES[target][t]
    if target == 'LP64':
        return s
    if t in ('long long', 'unsigned long long'):
        return 8 if compiler == 'edg' else 4
    if t in ('double', 'long double'):
        return 4
    return s


@dataclass(eq=False)
class Cls:
    name: str
    bases: list = field(default_factory=list)
    members: list = field(default_factory=list)
    union: bool = False
    virtual_funcs: bool = False
    alignas: Optional[int] = None
    user_ctor: bool = False
    user_dtor: bool = False
    user_copy_assign: bool = False
    ctor_template: bool = False
    defaulted_ctor: bool = False
    deleted_ctor: bool = False
    user_move_assign: bool = False


@dataclass(eq=False)
class Base:
    cls: Cls
    virtual: bool = False


@dataclass(eq=False)
class Mem:
    name: Optional[str]
    type: Any
    bounds: tuple = ()
    width: Optional[int] = None
    alignas: Optional[int] = None
    nua: bool = False
    access: str = 'public'
    init: bool = False


@dataclass
class Layout:
    size: int
    align: int
    dsize: int            # end of the data, in bytes
    nvdsize: int          # end of the data of the part a base subobject holds
    nvsize: int
    dynamic: bool         # has a pointer for virtual calls
    empty: bool
    keeps_end_padding: bool
    offsets: dict
    bitoffsets: dict
    bases: dict
    nv_empties: list      # (type name, offset) inside the base-subobject part
    empties: list         # (type name, offset) inside a complete object
    vbase_order: list


def _round(x, a):
    return (x + a - 1) // a * a


def _is_class(t):
    return isinstance(t, Cls)


def _scalar_name(t):
    if isinstance(t, tuple) and t[0] == 'enum':
        return t[1]
    return t


def _has_vbase(c):
    return any(b.virtual or _has_vbase(b.cls) for b in c.bases)


def _dynamic(c):
    return c.virtual_funcs or _has_vbase(c) or any(_dynamic(b.cls) for b in c.bases)


def is_empty(c):
    """Not given by the text: no definition of an empty class.
    Choice: no non-static data member other than unnamed bit-fields of
    width 0, no virtual function, no virtual base, and every base empty."""
    if c.union:
        return False
    if c.virtual_funcs or _has_vbase(c):
        return False
    for m in c.members:
        if not (m.name is None and m.width == 0):
            return False
    return all(is_empty(b.cls) for b in c.bases)


def keeps_end_padding(c, compiler='gcc16'):
    """CLS-35, the conditions for GCC 16, Clang 22, Clang 18, plus the
    extra conditions of GCC 14 and EDG 7.0."""
    if c.bases or c.virtual_funcs:
        return False
    for m in c.members:
        if m.access != 'public' or m.init:
            return False
    if c.ctor_template or c.user_ctor or c.user_dtor or c.user_copy_assign:
        return False
    if compiler == 'gcc14' and (c.defaulted_ctor or c.deleted_ctor):
        return False
    if compiler == 'edg' and (c.deleted_ctor or c.user_move_assign):
        return False
    for m in c.members:
        t = m.type
        if _is_class(t):
            if not keeps_end_padding(t, compiler):
                return False
        elif _scalar_name(t) == 'ref':
            return False
    return True


def _vbases_dfs(c):
    """CLS-35 order row: depth-first walk of the base lists, left to
    right, first meeting. Preorder chosen (the text says 'first meets')."""
    seen = []

    def walk(k):
        for b in k.bases:
            if b.virtual and b.cls not in seen:
                seen.append(b.cls)
            walk(b.cls)
    walk(c)
    return seen


def _type_info(t, target, compiler):
    """Size, alignment and complete-object empties of a type."""
    if _is_class(t):
        L = layout(t, target, compiler)
        return L.size, L.align, L.empties, L
    s = _scalar_name(t)
    return _SIZES[target][s], _fund_align(s, target, compiler), [], None


def layout(c, target='LP64', compiler='gcc16'):
    if c.union:
        return _layout_union(c, target, compiler)
    return _layout_struct(c, target, compiler)


def _layout_union(c, target, compiler):
    size = 0
    align = 1
    offsets, bitoffsets = {}, {}
    empties = []
    for m in c.members:
        if m.width is not None:
            t = _scalar_name(m.type)
            ts, ta = _SIZES[target][t], _fund_align(t, target, compiler)
            if m.width > 8 * ts:
                ta = _fund_align(_u_type(m.width, target), target, compiler)
            if m.name is not None:
                align = max(align, ta)
                bitoffsets[m.name] = 0
            size = max(size, (m.width + 7) // 8)
            continue
        s, a, e, _ = _type_info(m.type, target, compiler)
        n = 1
        for b in m.bounds:
            n *= b
        if m.alignas:
            a = max(a, m.alignas)
        align = max(align, a)
        size = max(size, s * n)
        offsets[m.name] = 0
        if m.type is not None and _is_class(m.type):
            for i in range(n):
                empties += [(tn, o + i * s) for tn, o in e]
    if c.alignas:
        align = max(align, c.alignas)
    dsize = size
    size = max(1, _round(size, align))
    kp = keeps_end_padding(c, compiler)
    return Layout(size, align, dsize, dsize, size, False, False, kp,
                  offsets, bitoffsets, {}, empties, empties, [])


def _u_type(width, target):
    """BF-9: U is the largest integer type with at most N bits."""
    best = 'char'
    for t in _INT_LADDER:
        if 8 * _SIZES[target][t] <= width:
            best = t
    return best


def _layout_struct(c, target, compiler):
    ptr = _SIZES[target]['ptr']
    ptra = _fund_align('ptr', target, compiler)
    offsets, bitoffsets, bases = {}, {}, {}
    placed_empties = []          # (type name, offset)
    bits = 0                     # data so far, in bits
    end = 0                      # end of every subobject, in bytes
    align = 1
    if c.alignas:
        align = c.alignas

    def conflicts(cands, at):
        have = set(placed_empties)
        return any((tn, o + at) in have for tn, o in cands)

    def place(cands, start, a):
        """CLS-35, two empty subobjects of one type."""
        off = start
        if not conflicts(cands, off):
            return off
        off = _round((bits + 7) // 8, a)
        while conflicts(cands, off):
            off += a
        return off

    nv = [b for b in c.bases if not b.virtual]
    primary = None
    for b in nv:
        if _dynamic(b.cls):
            primary = b
            break
    order = []
    if primary is not None:
        order.append(primary)
        order += [b for b in nv if b is not primary]
    else:
        order = list(nv)
        if c.virtual_funcs or _has_vbase(c):
            bits = 8 * ptr
            end = ptr
            align = max(align, ptra)

    def place_base(b, is_virtual):
        nonlocal bits, end, align
        L = layout(b.cls, target, compiler)
        align = max(align, L.align)
        cands = L.nv_empties
        if L.empty:
            off = place(cands, 0, L.align)
            end = max(end, off + L.size)
        else:
            off = place(cands, _round((bits + 7) // 8, L.align), L.align)
            if L.keeps_end_padding:
                d = off + L.size
            elif L.vbase_order:
                d = off + L.nvdsize
            else:
                d = off + L.dsize
            bits = 8 * d
            end = max(end, off + (L.nvsize if L.vbase_order else L.size))
        bases[b.cls.name] = off
        placed_empties.extend((tn, o + off) for tn, o in cands)

    for b in order:
        place_base(b, False)

    for m in c.members:
        if m.width is not None:
            t = _scalar_name(m.type)
            ts, ta = _SIZES[target][t], _fund_align(t, target, compiler)
            N = m.width
            if N == 0:
                bits = _round(bits, 8 * ta)
                end = max(end, bits // 8)
                continue
            if N > 8 * ts:
                ua = _fund_align(_u_type(N, target), target, compiler)
                start = _round(bits, 8 * ua)
                fa = ua
            else:
                k = bits // (8 * ta)
                if bits + N <= (k * ta + ts) * 8:
                    start = bits
                else:
                    start = _round(bits, 8 * ta)
                fa = ta
            if m.name is not None:
                align = max(align, fa)
                bitoffsets[m.name] = start
            bits = start + N
            end = max(end, (bits + 7) // 8)
            continue
        s, a, e, L = _type_info(m.type, target, compiler)
        n = 1
        for bd in m.bounds:
            n *= bd
        if m.alignas:
            a = max(a, m.alignas)
        align = max(align, a)
        cands = []
        if L is not None:
            for i in range(n):
                cands += [(tn, o + i * s) for tn, o in L.empties]
        start = _round((bits + 7) // 8, a)
        if L is not None and m.nua and L.empty and not m.bounds:
            off = place(cands, start, a)
            # takes no bytes (CLS-31)
            end = max(end, off + s)
        else:
            off = place(cands, start, a)
            if L is not None and m.nua and not m.bounds and not L.keeps_end_padding:
                d = off + L.dsize
            else:
                d = off + s * n
            bits = 8 * d
            end = max(end, off + s * n)
        offsets[m.name] = off
        placed_empties.extend((tn, o + off) for tn, o in cands)

    nvdsize = (bits + 7) // 8
    nvend = end
    nv_empties = list(placed_empties)
    vorder = _vbases_dfs(c)
    for vb in vorder:
        place_base(Base(vb, True), True)

    dsize = (bits + 7) // 8
    empty = is_empty(c)
    total = max(end, dsize)
    size = _round(total, align)
    if size == 0:
        size = 1          # CLS-11
    nvsize = _round(max(nvend, nvdsize), align) if vorder else size
    if empty:
        nv_empties = [(c.name, 0)] + nv_empties
        placed_empties = [(c.name, 0)] + placed_empties
    return Layout(size, align, dsize, nvdsize, nvsize, _dynamic(c), empty,
                  keeps_end_padding(c, compiler), offsets, bitoffsets, bases,
                  nv_empties, placed_empties, vorder)
