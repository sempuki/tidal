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
| Use a table to compare cases on more than one property. | A table with one row per case | Parallel sentences that repeat their structure |
| Put the condition first. | `If the value is zero, the behavior is undefined.` | `The behavior is undefined if, after conversion, the value is zero.` |
| Do not use cleft sentences. | `The cache makes this fast.` | `It is the cache that makes this fast.` |
| Do not define a thing by what it is not. | `This is a design decision.` | `This is not a bug but a design decision.` |
| Do not use a colon to hold back the point. | `The index is rebuilt on every write.` | `There is one catch: the index is rebuilt on every write.` |
| Do not use dashes for asides. | Write a new sentence. | `The parser — which is old — still works.` |
| Do not set up a puzzle and then answer it. Say the thing. | `` `decltype(x)` gives the declared type, and `decltype((x))` gives the type of the expression. `` | `` `decltype` answers one of two questions, and the parentheses choose which. `` |
| Do not end a passage with a line that restates it. | Stop after the last fact. | `So the rule is simple.` |

### Backwards sentences

A *backwards sentence* puts its meaning somewhere other than its plain
place, for emphasis. It makes the reader hold a gap open until the
meaning arrives. Technical text never uses one. These are the kinds:

| Kind | What it does | Backwards | Plain |
|---|---|---|---|
| Cleft | Front-loads a wh-clause or "it is" | `What makes this fast is the cache.` | `The cache makes this fast.` |
| Cataphora | Points forward to content not yet given | `` `decltype` answers one of two questions, and the parentheses choose which. `` | `` `decltype(x)` gives the declared type, and `decltype((x))` gives the type of the expression. `` |
| Hypophora | Asks a question, then answers it | `Why is this undefined? Because the value is unknown.` | `This is undefined because the value is unknown.` |
| Rhetorical anaphora | Repeats an opening across sentences for rhythm | `It checks the type. It checks the value. It checks the range.` | `It checks the type, the value, and the range.` |

Grammatical back-reference ("it", "this rule", "the operand") is not
rhetorical anaphora, and stays. The colon that holds back the point,
"not X but Y", and the buried lede are close relatives, and the table
above bans them too.

STE limits writers to an approved dictionary. Tidal keeps that rule in a
relaxed form:

- C++ terms are STE *technical names*. Examples are `lvalue`, `translation
  unit`, `enumerator`, and every keyword. Define each term once in the
  glossary.
- Use the verbs in the word list (section 5) for requirements.
- Do not use a synonym for a defined term.

## 2. Page structure

Each section is one page. A page contains these parts in this order. Leave
out a part that has no content.

| Part | Contents | Normative |
|---|---|---|
| Title and summary | One or two sentences that say what the page covers. | No |
| `Principles` | Prose that states the model the rules follow from. Each principle lists the rules that are its evidence. | Yes |
| `Syntax` | A `productionlist` grammar block. | Yes |
| `Constraints` | Rules that an implementation checks before the program runs. | Yes |
| `Runtime Behavior` | What the program does when it runs. | Yes |
| `Undefined Behavior` | Conditions that have no requirements. | Yes |
| `Implementation Requirements` | Implementation-defined and unspecified behavior, and errors that an implementation need not diagnose. | Yes |
| `Recommended Practice` | Diagnostics and other behavior that an implementation should provide. | Yes, as advice |
| `Examples` | Code, with short comments. | No |
| `Evidence` | Compiler results and divergences that support the rules. | No |

An implementation rejects a program that does not obey a rule in the
`Constraints` segment. A rule in that segment need not repeat this
requirement.

## 3. Principles

A principle is a `principle` directive:

    :::{principle} CPY-P2
    :rules: CPY-4 CPY-5 CPY-6

    The implementation writes the copy and move functions for a class
    unless the class writes them. ...
    :::

- The ID is the page prefix, `P`, and a number.
- A principle is one to three paragraphs in plain prose. It says what the
  construct is for and what model the rules follow from. It can say why.
  Where it names three or more items, cases, or steps, it uses a list,
  as everywhere else (section 1).
- Prefer a table or a diagram to prose wherever it says the same thing
  in less text. A principle that compares cases, kinds, or operators
  gives the comparison as a table. One that describes an order, a flow,
  a hierarchy, or an object layout draws it as a plain-text diagram in a
  `text` code block. The prose that remains says why.
- `:rules:` lists the rules on the same page that are its evidence. Every
  rule on a page with principles must appear in one `:rules:` or
  `:exceptions:` list.
- `:exceptions:` lists the rules that depart from the principle. State
  the principle plainly and put its exceptions here, instead of hedging
  the wording.
- A principle comes before the grammar. The reader meets the model, then
  the syntax, then the rules.
- Write a principle from the rules and the tests, not from memory of any
  text. The similarity check covers principles.

## 4. Rules

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
- An `impl-defined` rule can give the matrix targets' choices in a table,
  one row per type and one column per target (decision 0036). A test
  backs each cell.
- Link a rule on another page with `` {rule}`ID` ``. Mark a topic that
  Tidal does not specify yet with *(TOPIC referenced)*, for example
  *(templates referenced)*. TOPIC is one of `REFERENCED_TOPICS` in
  `tools/check_spec.py`, which lists the topics with no page.

## 5. Word list

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
| is conditionally supported | An implementation can reject the construct. If it accepts it, it documents the behavior. |
| should | Recommended practice only. |

Do not use `shall`, `may`, `might`, or `will` in rules.

## 6. Examples

- Each example is a short `cpp` code block in the `Examples` segment.
- A comment on a line gives the result, for example `// int` or
  `// rejected`.
- Use names that show the purpose, such as `limit` or `make`.
- Do not copy an example from another source.

## 7. Evidence

The `Evidence` segment is informative. It records compiler results, links
to questions in `questions/`, and notes divergences. Each divergence names
the compilers and the question that holds the provisional answer.
