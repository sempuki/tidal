# Style guide

This guide tells contributors how to write Tidal specification text. It
applies to every file in `specification/`. Decision 0016 records why.

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

The colon that holds back the point,
"not X but Y", and the buried lede are close relatives, and the table
above bans them too.

### Pronouns

Keep "it", "this", "that", and "they" rare, and make each one's
referent obvious.

- Prefer the noun again: "the operand", "the rule", "the conversion".
- Use "it" or "they" only when exactly one noun in the same or the
  previous sentence can be the referent.
- Never use a bare "this" or "that" as a subject. Write "This rule
  means", not "This means".

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
- A principle is one to three paragraphs in plain prose. The principle
  says what the construct is for and what model the rules follow from,
  and can say why. Where a principle names three or more items, cases,
  or steps, the principle uses a list, as everywhere else (section 1).
- Prefer a table or a diagram to prose wherever the table or diagram
  says the same thing in less text. A principle that compares cases,
  kinds, or operators gives the comparison as a table. A principle that
  describes an order, a flow, a hierarchy, or an object layout draws
  that structure as a plain-text diagram in a `text` code block. The
  prose that remains says why.
- `:rules:` lists the rules on the same page that are the principle's
  evidence. Every
  rule on a page with principles must appear in one `:rules:` or
  `:exceptions:` list.
- `:exceptions:` lists the rules that depart from the principle. State
  the principle plainly and put its exceptions here, instead of hedging
  the wording.
- A principle comes before the grammar. The reader meets the model, then
  the syntax, then the rules.
- Write a principle from the rules and the tests, not from memory of any
  text. The similarity check covers principles.
- A principle that claims a case no test checks gets a test for that
  case, and the principle's `:rules:` list names a rule that cites the
  test.

### When a principle gives the answer to a question

A question settles by review only when a principle gives the same answer
(decision 0054). A principle gives the answer when a reader can reach
the answer from the principle alone:

1. The reader takes the program of the question.
2. The reader applies the principle's model to the program. The model
   is the principle's prose, tables, and diagrams. The reader does not
   read the rule text, the Evidence, or the question's answer.
3. The reader reaches the provisional answer, and the model rules out
   each other option of the question.

A principle fails the test in these cases:

| Case | Example |
|---|---|
| The principle states the general model, and the answer turns on a detail that only a rule states. | The principle says that a literal gets the first type in its list. The question asks which types the list holds. |
| The principle names the question's rule in `:exceptions:`. | The exception says that the case departs from the model. |
| The principle sends the reader to a rule for the case. | A table cell names a rule instead of a result. |
| Each option of the question fits the model. | The model gives an order of steps, and the question asks which step a part belongs to. |

A principle on another page counts when a principle or a rule of the
affected page cites the principle with `` {principle}`ID` ``. A
citation in the Evidence segment does not count.

A principle that a phase writes or widens gives no answer in that phase
until a reader or a reviewer has used the principle (decision 0055).
The question records the test in a dated note. The note names the
principle and says which step of the test the principle passes or
fails.

## 4. Rules

Write each rule in a `rule` directive:

    :::{rule} IF-3
    :kind: required
    :tests: statements/if/condition-converted-to-bool.cpp

    The condition is converted to `bool` by {term}`contextual conversion`.
    :::

- The ID is the section prefix and a number. Do not reuse an ID.
- `:kind:` is one of the classes in `documentation/specification-format.md`. The kind must
  agree with the segment that contains the rule.
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
  `tools/check_specification.py`, which lists the topics with no page.

## 5. Word list

Use these words with these meanings only.

| Word | Meaning |
|---|---|
| must | A requirement on a program. If the program breaks the requirement, the program is not valid. |
| must not | A prohibition on a program. |
| rejects | The implementation issues a diagnostic and does not translate the program. |
| accepts | The implementation translates the program. |
| can | The language permits it. |
| is undefined | Tidal sets no requirements on the behavior. |
| is implementation-defined | Each implementation chooses the behavior and documents its choice. |
| is unspecified | Each implementation chooses the behavior from a set that the rule gives. The implementation need not document the choice. |
| is conditionally supported | An implementation can reject the construct. If the implementation accepts the construct, the implementation documents the behavior. |
| should | Recommended practice only. |

Do not use `shall`, `may`, `might`, or `will` in rules.

## 6. Examples

- Each example is a short `cpp` code block in the `Examples` segment.
- A comment on a line gives the result, for example `// int` or
  `// rejected`.
- Use names that show the purpose, such as `limit` or `make`.
- Do not copy an example from another source.
- `tools/check_examples.py` compiles each block alone. A block that does
  not compile alone carries a marker
  (`documentation/specification-format.md`, Examples).

## 7. Evidence

The `Evidence` segment is informative. The segment records compiler results, links
to questions in `questions/`, and notes divergences. Each divergence names
the compilers and the question that holds the provisional answer.

## 8. Grammar

A Syntax segment holds a `productionlist` with the group `tidal`
(decision 0040):

    ```{eval-rst}
    .. productionlist:: tidal
       if_stmt: "if" ["constexpr"] "(" [`init_stmt`] `condition` ")"
              :    `statement` ["else" `statement`]
    ```

| Form | Meaning |
|---|---|
| `"if"` | the terminal `if` |
| `` `condition` `` | a nonterminal, defined once in the whole specification |
| `[x]` | `x` is optional |
| `{x}` | `x` repeats zero or more times |
| `( x )` | grouping |
| `x \| y` | `x` or `y` |
| a line that starts with `:` | continues the right-hand side above; `: \|` starts a new alternative |
| words without quotes or backticks | a set too large to list, such as "any letter" or "any token other than a bracket" |

Shared nonterminals have these names and defining pages:

| Nonterminal | Page |
|---|---|
| `statement`, `labeled_stmt`, `compound_stmt`, `expression_stmt` | `statements/index.md`, `statements/blocks.md` |
| `init_stmt`, `condition` | `statements/if.md` |
| `declaration`, `block_decl`, `simple_decl`, `decl_specifiers`, `decl_specifier`, `type_specifier` | `declarations/index.md` |
| `declarator`, `type_id`, `cv`, `ref_qualifier`, `parameters`, `parameter` | `declarations/declarators.md`, `declarations/functions/default-arguments.md` |
| `init`, `braced_init` | `declarations/initialization/forms.md`, `declarations/initialization/list.md` |
| `expression`, `constant_expr` | `expressions/comma.md`, `foundations/constant-evaluation.md` |
| `primary`, `literal`, `identifier`, `id_expression`, `qualified_name`, `nested_name` | `expressions/primary.md` |
| one level of the expression chain each: `postfix_expr`, `unary_expr`, `cast_expr`, `pm_expr`, `mul_expr`, `add_expr`, `shift_expr`, `three_way_expr`, `relational_expr`, `equality_expr`, `and_expr`, `xor_expr`, `or_expr`, `logical_and_expr`, `logical_or_expr`, `conditional_expr`, `assignment_expr` | the page of that operator |
| `class_def`, `member_decl`, `class_name` | `classes/members.md` |
| `attribute_spec` | `declarations/attributes.md` |

## 9. Guides

The specification is reference material only. A tutorial, a how-to
guide, or an explanation is not part of it (decision 0045). Guides live
in `guides/`, outside `specification/`, and the site does not publish them
(decision 0046). A guide has no rules. It walks through one task in
these parts:

- the task, in one or two sentences,
- the steps, each with a short example and the rule IDs it relies on,
- the mistakes that the rules forbid or make undefined, each with the
  rule that says so.

Each example compiles on the matrix compilers. A guide adds no fact that
a rule does not state. Where a guide needs one, the gap is a finding for
the rule's page.

