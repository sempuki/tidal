# Q-0311: Does an argument that is a template-id for a function template add the namespaces of its template arguments?

**Status:** settled
**Affects:** NAM-27, `foundations/names/adl-template-id-argument.cpp`

## Context

NAM-27 gives the associated namespaces of each kind of argument. For a
name of an overload set, each function adds what its type adds. A
function template in the set adds what its parameter types and return
type add, apart from the types that depend on its template parameters.
An argument can also be a template-id such as `h<M::B>`, which names a
specialization through explicit template arguments.

| Probe | GCC 16.2 | Clang 22 |
|---|---|---|
| `take(h<M::B>)` with `template <class T> void h(T)` and `M::take` | finds `M::take` | does not find `take` |
| the same with a plain `void h()` added to the set | finds `M::take` | does not find `take` |
| `take(h)` with `template <class T> void h(T)`, no template arguments | does not find `take` | does not find `take` |

EDG 7.0 accepts the first probe, as GCC does (`tests/RESULTS.md`).

## Evidence

`tests/foundations/names/adl-template-id-argument.cpp` and
`tests/foundations/names/adl-function-template-parameter-adds-nothing.cpp`.

## Options

1. A template-id adds what each type template argument adds. A class
   template specialization adds its template arguments in the same way
   (NAM-27), so the two kinds of template-id behave alike. GCC follows
   this option.
2. A function template in the set adds nothing for its template
   parameters, also when a template-id gives explicit arguments for
   them. Clang follows this option.

## Answer

Yes; a template-id adds what each type template argument adds (NAM-27, option 1), as GCC does. NAM-27 already adds the type arguments of
a class template specialization, and option 1 gives a function template
specialization the same treatment. The test expects GCC's result and
records Clang's result with an override.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of names and lookup (`reports/review/2026-11-03-gap-review-names.md`) supports this answer with no doubt, and NAM-P3 gives the same answer. Its first step follows each argument type to the classes it is built from, and the type template arguments of `h<M::B>` are part of the specialization the argument names. GCC 16 and EDG 7.0 follow the answer. Clang 22 still differs, and the test keeps its override for Clang.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of names and lookup (`reports/review/2026-11-03-gap-review-names.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The reference adds, for an overload set named with a template-id, the entities of the type template arguments and the template template arguments, and value arguments add nothing. The review describes the statement as present since the first standard. GCC follows it, and Clang conflicts with it. For the third probe, a plain template name with no explicit arguments, the review says the reference does not state what parameter types that depend on the template's own parameters add. The review calls Tidal's reading, that they add nothing, plausible.
