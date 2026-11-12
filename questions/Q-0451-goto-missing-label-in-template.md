# Q-0451: Is a `goto` to a missing label rejected in a templated function that no specialization uses or in a discarded statement?

**Status:** provisional
**Affects:** JMP-14, JMP-7, DEP-30, `tests/statements/jumps/goto-missing-label-template-never-used.cpp`, `tests/statements/jumps/goto-missing-label-class-template-member.cpp`, `tests/statements/jumps/goto-missing-label-generic-lambda.cpp`, `tests/statements/jumps/goto-missing-label-discarded-branch.cpp`
**Source:** X24-3 (`reports/clean-room/X24/report.md`, probe `t1.cpp`), rerun on the matrix in `reports/clean-room/roadmap-14/writer-probes/x24-compile.json`.
**Waits for:** a review (doubt); a principle (JMP-P1)

## Context

A function body names a label in a `goto`, and the function has no
label of that name. The body is in a templated context:

```cpp
template <class T> void step() { goto nowhere; }
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| a function template that no specialization uses | rejects | accepts | rejects |
| a member function of a class template that no specialization uses | rejects | accepts | rejects |
| a generic lambda that the program never calls | rejects | accepts | rejects |
| a discarded branch of `if constexpr` in a function template, used or not | rejects | accepts | rejects |
| a function template with an explicit instantiation | rejects | rejects | rejects |
| a discarded branch of `if constexpr` outside a template | rejects | rejects | rejects |

Every compiler also rejects, in a function template that no
specialization uses, a label name that occurs twice and a `continue`
outside a loop. Every compiler rejects a `break` outside a loop in a
discarded branch of a used function template.

## Evidence

The tests in **Affects** and the probe in **Source**.
`tests/statements/jumps/duplicate-label-template-never-used.cpp` and
`continue-outside-loop-template-never-used.cpp` hold the forms that
every compiler rejects.

## Options

1. The implementation checks the labels of a function at its
   definition, so it rejects each form (Clang, EDG).
2. The check waits for a specialization, as for the constructs of
   DEP-30, so an implementation need not diagnose a template that no
   specialization uses (GCC).

## Provisional answer

A template that no specialization uses needs no diagnostic, and a
discarded statement of a used template is rejected. A label of a
function does not depend on a template parameter, so JMP-14 leaves the
first three forms to DEP-30, and Clang's and EDG's rejection and GCC's
acceptance each conform there (option 2). For the discarded statement,
JMP-14 rejects the program, as Clang and EDG do (option 1). GCC alone
accepts that form.

Opened on 2026-11-11 (decision 0065, item F7). No principle gives the answer. JMP-P1 says that the place a jump names must exist in the same function, and it does not say when an implementation checks the place in a template.

Kept open on 2026-11-11 (decision 0065, item RV10). The gap review of the phase 17 text reads against the answer for the three forms with no specialization (RV10-14). A broken label condition does not depend on the parameters, so the program is not valid and needs no diagnostic there, and GCC's acceptance conforms. The review gives no clear answer for the discarded statement of a used template. Clang 22 and EDG 7.0 keep the matrix's answer, and GCC 16 gives the review's reading. So the question joins `pending.md` row 4 with the item "JMP-14, Q-0451".

Answer changed on 2026-11-11 (decision 0065, item RV10). Before this note, the answer was option 1 for every form, and JMP-14 and JMP-7 said that the implementation checks their conditions at the definition, also in a template that no specialization uses. DEP-30 already takes any construct that does not depend on the parameters, and Q-0223 settled that such a construct in an unused template needs no diagnostic. So JMP-14 and JMP-7 now leave a template that no specialization uses to DEP-30, and DEP-30 lists the jump conditions. The tests of the three forms keep the expectation `reject` and cite DEP-30, and the GCC override records the other result that DEP-30 allows. The duplicate label and the `continue` outside a loop, which every compiler rejects, also cite DEP-30. The review supports the new answer for those forms. It gives no clear answer for the discarded statement, so the question waits for a review (doubt), and `pending.md` row 4 no longer holds it. GCC 16 alone differs in the discarded form. No principle gives the answer for the discarded statement. JMP-P1 does not say when an implementation checks the place that a jump names in a template, so JMP-P1 fails step 3 of the principle test.

Kept open on 2026-11-11 (decision 0065, item RV10). The read of the RV10 repair diff reads the changed answer. The read supports the part on a template that no specialization uses: no diagnostic is required, and a check before any instantiation also conforms. Option 2's text says that the check waits for a specialization, which is GCC's model only, and the C++23 text asks no implementation to wait. The answer's words, that each result conforms, give the reference's outcome, so the difference is in wording only. For the discarded statement of a used template, the C++23 text leaves two readings open (RV11-R-6). The label rule binds either the template's written body, and the program needs a diagnostic as JMP-14 says, or only the specialization, which holds no `goto`, and GCC's acceptance conforms. The read finds that the **Waits for:** line, a review (doubt), fits, so the line does not change. GCC 16 alone differs in the discarded form, and JMP-P1 still fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) gives the first reading of the question: reads against for three forms (RV10-14). The reference lets an implementation check a template before any instantiation. A program is not valid, with no diagnostic required, when a construct that does not depend on the parameters would make a specialization right after the definition fail, or when a template that is never instantiated has no valid specialization. A `goto` to a missing label does not depend on the parameters. So in a function template that no specialization uses, in a member function of a class template that is never used, and in a generic lambda that is never called, the program is not valid, and the implementation need not diagnose it. Option 1's rejection is allowed, and GCC's acceptance also conforms. DEP-30 is Tidal's form of that rule. For a discarded statement in a template that is used, the review finds no clear answer. The template is instantiated, the discarded statement is not, and the reference does not say whether the label rule binds such a statement. Outside a template, every compiler rejects the discarded form.

### Read of the RV10 repair diff, 2026-11-11

The source-reader read of the RV10 repair diff (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`) reads the changed answer, JMP-14, JMP-7, DEP-30's item on jumps, the Q-0451 lines of the jumps and dependent-names pages, the split rows, and the jump tests that cite DEP-30: supports the part on a template that no specialization uses (option 2), and finds that the reference leaves the discarded statement of a used template open (RV11-R-6). A missing label does not depend on the parameters. The reference makes a template that would fail for such a construct not valid with no diagnostic required, and lets an implementation check a template before any instantiation. So in the three forms with no specialization, a rejection conforms and GCC's acceptance conforms. The answer's words, that each result conforms, give the reference's outcome. Option 2's text says that the check waits for a specialization, which describes only GCC's model, and the reference asks no implementation to wait. The read calls the difference one of wording only. In the discarded form, the template is instantiated, so the rule on a template that is never instantiated does not apply. The specialization holds no `goto`, because the discarded statement is not instantiated. The reference states the label rule for a function and does not say whether the rule binds the written body of a template or only its specializations. Under the first reading, the program needs a diagnostic, as JMP-14 says. Under the second, the program is valid, or at most not valid with no diagnostic required, and GCC's acceptance conforms. So `goto-missing-label-discarded-branch.cpp` holds a form that the reference does not decide. The note of the changed answer and the Q-0451 lines record the doubt correctly, and the read finds that the **Waits for:** line, a review (doubt), fits. The gap review's entry above states RV10-14 correctly.
