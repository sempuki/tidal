# PU3: principle-use batch

Date: 2026-11-05
Role: clean-room reader
Kind: principle use (decision 0057, item PU3)

## Summary

I predicted the 13 programs from the principles of the PU3 copy of the
specification. No program was compiled. The answers are in
`answers.json`.

| Program | Outcome | Principle |
|---|---|---|
| p01 | reject (doubtful) | SB-P2 |
| p02 | compiles | SB-P2 |
| p03 | not-decided | LSUP-P2, LSUP-P1 |
| p04 | not-decided | LSUP-P2, LSUP-P1 |
| p05 | reject | DEFARG-P2 |
| p06 | compiles | CNS-P2 |
| p07 | runs (doubtful) | CNS-P3, CNS-P2 |
| p08 | reject | DEFARG-P2 |
| p09 | compiles | SB-P2 |
| p10 | reject (doubtful) | SB-P2 |
| p11 | reject | DEFARG-P2 |
| p12 | not-decided | LSUP-P2, LSUP-P1 |
| p13 | runs (doubtful) | CNS-P3, CNS-P2 |

DEFARG-P2 decided p05, p08, and p11 directly. SB-P2 decided p02 and
p09, but its member-form sentence gave a doubtful answer for p01 and
p10. LSUP-P2 decided none of the three programs on `std::align_val_t`.
CNS-P2 and CNS-P3 gave answers for p06, p07, and p13, but they do not
say when the bool-type requirement on an atom is checked.

## Findings

### PU3-1: SB-P2 does not say which kind of member template selects the member form

Programs: p01, p10.

SB-P2 says the call is the member form `e.get<i>()` when a search for
`get` in the class finds "a member `get` template that takes a constant
as its leading template parameter". The glossary entry "member template"
covers function templates, nested class templates, and static variable
templates. p01 has a nested class template `get` and p10 has a static
variable template `get`, each with a constant parameter. Read
literally, the model picks the member form for both, and that call
cannot work, so I answered `reject`. The namespace has a matching free
`get` that argument-dependent lookup would find.

Outside knowledge: I believe only a member function template that takes a constant
as its leading template parameter selects the member form. Under
that reading p01 and p10 fall back to the free `get`, and both compile.

Suggestion: say "a member function template named `get`" in SB-P2, and
add an example where a nested class or variable template named `get`
leaves the call to argument-dependent lookup.

### PU3-2: SB-P2 does not say how `i` meets a member `get` whose parameter is not `std::size_t`

Programs: p02, p09.

SB-P2 says `i` is a constant of type `std::size_t`. p02's member `get`
takes `auto I`, and p09's takes `int I`. The model does not say what
happens to `i` in either case. I used outside knowledge: an `auto`
parameter takes the type of the argument (so `I` is `std::size_t` in
p02), and a constant of type `std::size_t` converts to `int` when the
value fits (p09).

Suggestion: add a sentence that `i` passes to the template parameter as
any constant template argument does, so an `auto` parameter gets type
`std::size_t` and an `int` parameter gets the converted value. Link the
page on template arguments.

### PU3-3: CNS-P2 does not say when an atom whose type is never bool is diagnosed

Programs: p06, p07, p13.

CNS-P2 says an atomic constraint must be a constant of type `bool`, and
its diagram reaches the error branch only while checking the atom
against arguments. In p06, p07, and p13 the second atom is
`(sizeof(T))`, whose type is `std::size_t` whatever `T` is. The type
does not depend on `T`, so an implementation could see the error at
the definition. The model does not say whether it does. I answered
from the short circuit (p06) and from "checked when a use needs them"
in CNS-P3 (p07, p13), so p06 compiles and p07 and p13 run. p07 puts the
atom in a requires clause and p06 and p13 put it in a concept, and the
model does not say whether the two places differ.

Suggestion: state whether an atom whose type does not depend on any
template parameter and is not `bool` makes the program not valid at its
definition, with no use. If the compilers differ, or differ between a
concept and a requires clause, record that in the principle.

### PU3-4: LSUP-P2 requires agreement on the kind of each name but gives no kind

Programs: p03, p04, p12.

LSUP-P2 says the program and the library must agree on each name and
its kind, and its table gives only the job of `std::align_val_t`. No
principle says whether `std::align_val_t` is a class, a scoped
enumeration, or an unscoped enumeration, or what its underlying type
is. The examples on the new and delete page write
`std::align_val_t(64)`, which fits several kinds. LSUP-P1 says the
implementation checks a shape where it uses the type. p03, p04, and
p12 declare the name and never use it, so the model also does not say
whether a wrong shape is caught without a use. I answered
`not-decided` for all three.

Outside knowledge: I believe the library declares `std::align_val_t`
as a scoped enumeration with `std::size_t` as its underlying type, and
some compilers declare the name themselves in every translation unit.

Suggestion: give the required shape of `std::align_val_t` in the
LSUP-P2 table (kind and underlying type), as the `std::byte` example
does for its type, and say whether a declaration with another shape is
rejected on its own or only at a `new` expression that passes an
alignment.
