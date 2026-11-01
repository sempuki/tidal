# Q-0311: Does an argument that is a template-id for a function template add the namespaces of its template arguments?

**Status:** provisional
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

EDG has not run the probes yet.

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

## Provisional answer

Option 1, which GCC follows. NAM-27 already adds the type arguments of
a class template specialization, and option 1 gives a function template
specialization the same treatment. The test expects GCC's result and
records Clang's result with an override.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
