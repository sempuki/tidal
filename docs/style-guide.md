# Style guide

This guide tells contributors how to write Tidal specification text. It
applies to every file in `spec/`. Decision 0016 records why.

## 1. Writing rules

Tidal follows the writing rules of ASD-STE100 Simplified Technical English
(STE), a specification maintained by the ASD Simplified Technical English
Maintenance Group. The rules below are the ones that apply most often.

| Rule | Do | Do not |
|---|---|---|
| One statement in each sentence. | `The condition is evaluated once.` | `The condition is evaluated once, and its value, after conversion, selects a branch.` |
| Keep sentences short: at most 20 words in a rule, 25 in a note. | Split the sentence. | Join clauses with `and`, `which`, or `;`. |
| Use the active voice when the actor is known. | `The implementation rejects the program.` | `The program is rejected by the implementation.` |
| Use the present tense. | `The variable is destroyed.` | `The variable will be destroyed.` |
| Use a word with one meaning only. | `rejects` | `rejects`, `refuses`, `does not accept` for the same thing |
| Use a list for three or more items or conditions. | A bullet list | A long sentence with commas |
| Put the condition first. | `If the value is zero, the behavior is undefined.` | `The behavior is undefined if, after conversion, the value is zero.` |
| Do not use cleft sentences. | `The cache makes this fast.` | `It is the cache that makes this fast.` |
| Do not define a thing by what it is not. | `This is a design decision.` | `This is not a bug but a design decision.` |
| Do not use a colon to hold back the point. | `The index is rebuilt on every write.` | `There is one catch: the index is rebuilt on every write.` |
| Do not use dashes for asides. | Write a new sentence. | `The parser — which is old — still works.` |

STE limits writers to an approved dictionary. Tidal keeps that rule in a
relaxed form:

- C++ terms are STE *technical names*. Examples are `lvalue`, `translation
  unit`, `enumerator`, and every keyword. Define each term once in the
  glossary.
- Use the verbs in the word list (section 4) for requirements.
- Do not use a synonym for a defined term.

## 2. Page structure

Each section is one page. A page contains these parts in this order. Leave
out a part that has no content.

| Part | Contents | Normative |
|---|---|---|
| Title and summary | One or two sentences that say what the page covers. | No |
| `Syntax` | A `productionlist` grammar block. | Yes |
| `Static Semantics` | Rules that an implementation checks before the program runs. | Yes |
| `Dynamic Semantics` | What the program does when it runs. | Yes |
| `Undefined Behavior` | Conditions that have no requirements. | Yes |
| `Implementation Requirements` | Implementation-defined and unspecified behavior, and errors that an implementation need not diagnose. | Yes |
| `Recommended Practice` | Diagnostics and other behavior that an implementation should provide. | Yes, as advice |
| `Examples` | Code, with short comments. | No |
| `Evidence` | Compiler results and divergences that support the rules. | No |

The `Static Semantics` segment states a general requirement: an
implementation rejects a program that does not obey a rule in it. A rule
need not repeat it.

## 3. Rules

Write each rule in a `rule` directive:

    :::{rule} IF-3
    :kind: required
    :tests: statements/if/condition-converted-to-bool.cpp

    The condition is converted to `bool` by {term}`contextual conversion`.
    :::

- The ID is the section prefix and a number. Do not reuse an ID.
- `:kind:` is one of the classes in `docs/spec-format.md`. It must agree
  with the segment that contains the rule.
- `:tests:` lists one or more test files, relative to `tests/`, separated
  by spaces.
- A rule contains one to three sentences. Use a list for conditions.
- Link each defined term with `` {term}`...` `` the first time it occurs in
  a rule.
- Do not explain a rule in prose. Put an example in `Examples`.

## 4. Word list

Use these words with these meanings only.

| Word | Meaning |
|---|---|
| must | A requirement on a program. If the program breaks it, the program is not valid. |
| must not | A prohibition on a program. |
| rejects | The implementation issues a diagnostic and does not translate the program. |
| accepts | The implementation translates the program. |
| can | The language permits it. |
| is undefined | Tidal sets no requirements on the behavior. |
| is implementation-defined | Each implementation chooses the behavior and documents its choice. |
| is unspecified | Each implementation chooses the behavior from a set that the rule gives. It need not document the choice. |
| should | Recommended practice only. |

Do not use `shall`, `may`, `might`, or `will` in rules.

## 5. Examples

- Each example is a short `cpp` code block in the `Examples` segment.
- A comment on a line gives the result, for example `// int` or
  `// rejected`.
- Use names that show the purpose, such as `limit` or `make`.
- Do not copy an example from another source.

## 6. Evidence

The `Evidence` segment is informative. It records compiler results, links
to questions in `questions/`, and notes divergences. Each divergence names
the compilers and the question that holds the provisional answer.
