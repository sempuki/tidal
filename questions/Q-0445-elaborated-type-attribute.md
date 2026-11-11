# Q-0445: Can an elaborated type specifier that is not a forward declaration have an attribute after its class key?

**Status:** provisional
**Affects:** TYN-10, `tests/declarations/type-names/elaborated-attribute-not-forward-declaration.cpp`
**Source:** ledger row RM12-2 (`reports/clean-room/roadmap-12/editor.md`, from the grammar finding of commit 9359cc42), with the writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json`.
**Waits for:** a review (doubt)

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

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The read of the RV8 repair diff supports the answer with a doubt (RV9-R-4), and GCC 16 alone differs. The read finds one more declaration that can carry the attribute in the reference, a class key with a template-id alone in an explicit specialization or a partial specialization. TYN-10 allows the attribute only in `forward_class`, which ends in an identifier. No test holds the template-id form, and no rule changes.

## Opinions

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) gives the first reading of the question: supports with a doubt (option 1). An attribute after the class key is allowed only in a declaration that holds the elaborated type and nothing else, so a variable, a function, or a friend declaration cannot carry one. The doubt is RV9-R-4. The reference allows the attribute in one more such declaration: a class key with a template-id, inside an explicit specialization or a partial specialization. The reference's own grammar for an elaborated type with a template-id gives no place for the attribute, so the reference is not consistent on that form. The read calls the doubt small. It finds that the `elaborated_type` hunk and the TYN-10 hunk agree with this doubt, and that the TYN-6 hunk and the Q-0445 line agree.
