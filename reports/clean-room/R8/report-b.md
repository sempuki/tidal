# Predict batches: constraints and instantiation, reader B

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R8-constraints (131 programs), R8-instantiation (137 programs)

Both answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R8-constraints/answers.json
- /home/ryanm/Code/tidal/build/exercises/R8-instantiation/answers.json

I answered `depends` where an Evidence segment records that the compilers differ and the program falls on the split:
- constraints: p08, p33, p40, p50, p72, p77, p80, p81, p94, p104, p116, p120, p121, p126, p129
- instantiation: p02, p06, p08, p12, p21, p33, p34, p37, p39, p41, p50, p71, p79, p81, p85, p93, p98, p102, p121, p128

Where only EDG (or only Clang 18) differs on a settled question, I gave the outcome of the rule and named the compiler that differs in `why`. Examples are constraints p01, p18, p34, p35, p45, p46, p63, p74, p105, p115 and instantiation p13, p48.

## R8B-1

- Pages and rules: constraints, CNS-27, CNS-40
- Kind: missing evidence
- What happened: Constraints p112 declares `template <class T> int f(T);` and then defines `int f(auto)`, with no constraints, and calls `f(1)`. CNS-27 says a function with a placeholder parameter declares a template of its own. Read strictly, that gives two templates and an ambiguous call, under CNS-40 with no diagnostic required. The Evidence covers only the constrained form (Q-0269). I answered compiles from outside knowledge, because I expect every compiler to treat the two as one template when there are no constraints.
- Suggestion: Add one line to the CNS-27 or CNS-40 Evidence on what the compilers do with the unconstrained pair. Or add an example that shows the pair.

## R8B-2

- Pages and rules: constraints, CNS-4, CNS-27
- Kind: unclear
- What happened: Three programs depend on when two concept arguments count as equivalent.
  - p04 compares `Answer<42>` with `Answer<40 + 2>`.
  - p27 compares mappings `sizeof(T) + 1` with `1 + sizeof(T)`.
  - p85 uses the same tokens twice.

  CNS-27 says equivalence needs "the same tokens". The same rule also says that two concept-ids naming the same specialization of a concept are equivalent. Under the second sentence, p04 is one template and compiles. Under the first, p27 is ambiguous. I could not tell which sentence applies to a value argument and which to a dependent one.
- Suggestion: State that tokens decide for arguments that depend on template parameters and values decide for arguments that do not, if that is the rule. Add p04 and p27 as a pair of examples.

## R8B-3

- Pages and rules: constraints, CNS-10; concepts, CPT-26
- Kind: hard to find
- What happened: Constraints p19 and p24 check `Peek<Box>` on a private member, once inside a member and once at namespace scope, in each order. The constraints page mentions only that "every compiler keeps the first result". The rule that access is judged from the place of the check is in the CPT-26 Evidence on the concepts page. Without that rule I would have predicted false in both programs and rejected p24.
- Suggestion: In CNS-10, or in CNS-7 where the point of the check is defined, say that access is judged at the place of the check and link to CPT-26.

## R8B-4

- Pages and rules: constraints, CNS-35
- Kind: needs an example
- What happened: In constraints p47 and p131, a concept's requirement calls `use(t)` or `a + a`. A template declared after the concept has that concept as its constraint. I predicted reject because argument-dependent lookup at the check finds the later template, and checking that template needs the same concept again. CNS-35 says only that such a check "can arise through overload resolution inside a requirement". I had to work out the lookup step from DEP-9.
- Suggestion: Add an example to CNS-35 with a later-declared constrained function that argument-dependent lookup finds.

## R8B-5

- Pages and rules: constraints, CNS-34, ORD-13
- Kind: needs an example
- What happened: Constraints p89 and p124 swap the order of the template parameters between two overloads. I had to apply the positional correspondence by hand to see that the function parameter types differ in p89 (ambiguous) and agree in p124 (the second is more constrained). The rule text is short, and it is easy to misapply.
- Suggestion: Add both cases as examples under CNS-34.

## R8B-6

- Pages and rules: constraints, CNS-37
- Kind: unclear
- What happened: In constraints p91, one member's clause is `requires requires { typename T::type; }`. For `int` that clause is false, though its form is valid. CNS-37 says a member whose constraints are "not valid after that substitution" does not match. I could not tell whether a requires expression that becomes false counts as not valid. The outcome did not change, because the specialization's clause matched the other member.
- Suggestion: Say whether "not valid" in CNS-37 covers a requires expression that becomes false, or only a substitution that forms an invalid expression.

## R8B-7

- Pages and rules: instantiation, INST-30, INST-31, INST-33
- Kind: needs an example
- What happened: In instantiation p58, a `static constexpr` member's initializer calls `probe`, which is declared after the use. The page's example shows only the valid order (p73). I predicted reject from the Q-0257 Evidence, which is written for a function. I could not tell whether the class rule INST-33 or the function and static member rule INST-31 and INST-32 applies to the member's initializer.
- Suggestion: Add the failing order as a second example next to `Price`, and name the rule that rejects it.

## R8B-8

- Pages and rules: instantiation, INST-31, INST-32 (Q-0310)
- Kind: unclear
- What happened: The Evidence says a generic lambda gives the same split as a `constexpr` function template. In instantiation p36, the lambda is defined before `probe`, and its first call is in a function after `probe`. I predicted runs, on the view that the point of the call operator follows the function that calls it. The view that the point follows the lambda's own declaration fits the text equally well.
- Suggestion: Say where the point of instantiation of a generic lambda's call operator is, and give an example that names the point.

## R8B-9

- Pages and rules: instantiation, INST-7 (Q-0252)
- Kind: missing evidence
- What happened: The Evidence says GCC instantiates a `constexpr` function in a `sizeof` operand without braces (p128). Instantiation p99 has the same call in a `decltype` operand without braces. The page does not say what GCC does there. I answered compiles.
- Suggestion: Record GCC's result for `decltype` without braces in the INST-7 Evidence.

## R8B-10

- Pages and rules: instantiation, INST-22, INST-25
- Kind: unclear
- What happened:
  - Instantiation p74 names a function with a deduced return type by its deduced type `int`. INST-22 says such a function is named with `auto`, and a variable template is named with its deduced type. I answered reject, but the contrast between the two allowances is easy to miss.
  - Instantiation p106 has an explicit instantiation declaration of a `constexpr` variable template. INST-25 forbids `constexpr` there, and the page does not say that the declaration writes `const int`. I took that from the type of a `constexpr` variable.
- Suggestion: Add one example of each to INST-22: a function with a deduced return type, and a `constexpr` variable template.
