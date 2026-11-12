# Q-0445: Can an elaborated type specifier that is not a forward declaration have an attribute after its class key?

**Status:** provisional
**Affects:** TYN-10, TYN-6, `tests/declarations/type-names/elaborated-attribute-not-forward-declaration.cpp`, `tests/declarations/type-names/explicit-instantiation-attribute.cpp`
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
| `template <> struct [[deprecated]] S<int>;`, a specialization with no body | accepts | accepts | accepts |
| `template <class T> struct [[deprecated]] S<T*>;`, a partial specialization with no body | accepts | accepts | accepts |
| `template struct [[deprecated]] S<int>;`, an explicit instantiation | rejects: an attribute list cannot appear here | accepts | rejects: attributes are not allowed on explicit instantiations |

## Evidence

The tests in **Affects**, `forward-class-attributes.cpp`,
`explicit-specialization-attributes.cpp`,
`partial-specialization-attributes.cpp`, and the probe in **Source**.

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

Rule text on 2026-11-11 (decision 0064, item F6): the shape
`specialization_class` takes the template-id form in a specialization
with no body, and TYN-6 and TYN-10 accept an attribute or an alignment
specifier there, as every compiler does
(`reports/clean-room/roadmap-13/writer-probes/q0445-template-id.json`).
`explicit-specialization-attributes.cpp` and
`partial-specialization-attributes.cpp` hold the form. An explicit
instantiation is not such a declaration, and
`explicit-instantiation-attribute.cpp` expects its rejection, with
GCC as an override. The answer for the other declarations is
unchanged, and the question waits for a review of GCC's acceptance.

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The gap review of the phase 16 text supports the answer with a doubt on the qualified form (RV9-4), and GCC 16 alone differs. Every compiler accepts the qualified forms that the tests hold.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-28), and GCC 16 alone differs. The doubt is only the qualified template-id form of an explicit specialization, which every compiler accepts.

## Opinions

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) gives the first reading of the question: supports with a doubt (option 1). An attribute after the class key is allowed only in a declaration that holds the elaborated type and nothing else, so a variable, a function, or a friend declaration cannot carry one. The doubt is RV9-R-4. The reference allows the attribute in one more such declaration: a class key with a template-id, inside an explicit specialization or a partial specialization. The reference's own grammar for an elaborated type with a template-id gives no place for the attribute, so the reference is not consistent on that form. The read calls the doubt small. It finds that the `elaborated_type` hunk and the TYN-10 hunk agree with this doubt, and that the TYN-6 hunk and the Q-0445 line agree.

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) reads the question: supports with a doubt (RV9-4). Only a declaration that holds the elaborated type and nothing else can carry an attribute after the class key. The template-id form inside an explicit specialization or a partial specialization is one such declaration, and an explicit instantiation can carry no attribute at all. So TYN-10's item, its last sentence, and `explicit-instantiation-attribute.cpp` agree, and GCC reads against. The friend forms have no place for an attribute. The doubt is the qualified form. The reference allows the template-id form with an attribute only without a qualifier, and a qualified partial specialization with no body fits none of its listed forms. Every compiler accepts the qualified forms of `explicit-specialization-attributes.cpp`, and the review calls the doubt small.

### Read of the RV9 repair diff, 2026-11-11

The source-reader read of the RV9 repair diff (`reports/review/2026-11-11-read-of-rv9-repair-diff.md`) reads the dated note and the gap review's entry of decision 0064, item RV9: agrees as a record. Both state RV9-4 correctly, with its doubt on the qualified template-id form, and the doubt stays. The read gives no new reading of the question's answer.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-28). An attribute after the class key is allowed only when the elaborated type is the whole declaration. The elaborated type must then be a class key with an identifier, or a class key with a template-id inside an explicit specialization or a partial specialization. No other elaborated type can carry an attribute. So the variable declaration of `elaborated-attribute-not-forward-declaration.cpp` and the explicit instantiation are not valid, and GCC reads against. The doubt is now only the qualified template-id form, such as the last declaration of `explicit-specialization-attributes.cpp`. The allowed forms have no qualifier. The grammar of an elaborated type gives an attribute no place before any template-id, qualified or not, so the reference is not consistent on that form. Every compiler accepts the form.
