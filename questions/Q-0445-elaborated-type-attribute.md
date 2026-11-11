# Q-0445: Can an elaborated type specifier that is not a forward declaration have an attribute after its class key?

**Status:** provisional
**Affects:** TYN-10, `tests/declarations/type-names/elaborated-attribute-not-forward-declaration.cpp`
**Source:** ledger row RM12-2 (`reports/clean-room/roadmap-12/editor.md`, from the grammar finding of commit 9359cc42), with the writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json`.
**Waits for:** a first review

## Context

The `forward_class` shape lets a forward declaration have attributes
and an alignment specifier after the class key, as in
`struct [[deprecated]] Old;`. The same attribute can stand after the
class key of an elaborated type specifier in another declaration:

```cpp
struct Node;
struct [[maybe_unused]] Node* head = nullptr;
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `struct [[deprecated]] Fwd;` | accepts | accepts | accepts |
| `struct alignas(8) Fwd;` | accepts | accepts | accepts |
| `struct [[maybe_unused]] Fwd* p = nullptr;` | rejects: an attribute list cannot appear here | accepts | rejects: attributes are not allowed here |
| `struct [[maybe_unused]] Fwd* f();` | rejects | accepts | rejects |
| `friend struct [[maybe_unused]] Fwd;` in a class | rejects | accepts | rejects |

## Evidence

The test in **Affects**, `forward-class-attributes.cpp`, and the probe
in **Source**.

## Options

1. Only a forward declaration can have an attribute or an alignment
   specifier after the class key (Clang, EDG).
2. Every elaborated type specifier can have one (GCC).

## Provisional answer

Only a forward declaration can have one (option 1). Clang and EDG
agree, and GCC alone differs. The grammar of the type names page gives
the attributes to `elaborated_type`, and TYN-10 rejects them outside a
forward declaration.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
