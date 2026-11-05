# I2 notes: steps the text did not give

Written in phase 1, before reading any test.

Inputs: `specification/templates/ordering.md` (all of it), and from
`templates/deduction.md` DED-P1, DED-P3, DED-6 to DED-15, DED-20 to
DED-23, DED-30 to DED-34, DED-39; from `templates/constraints.md`
CNS-P1, CNS-P4, CNS-1 to CNS-4, CNS-22 to CNS-24, CNS-34; from
`overloading/overload-resolution.md` OVL-13, OVL-19, OVL-27 to OVL-30,
OVL-41; TSP-11, TSP-14 to TSP-16 (`templates/specialization.md`);
TPL-20; ICS-P3 (`overloading/conversion-sequences.md`). A script
removed Evidence segments and `:tests:` lines before I read each page.

One slip: my first look at the structure of `ordering.md` was a `grep`
for headings that also printed its `:tests:` lines. So I saw the test
file names (for example `repeated-parameter.cpp`,
`reference-removed-forwarding.cpp`), but no test content. The page
text itself also names some tests (`reference-pairs-disagree.cpp` and
others in ORD-P3 and ORD-P4).

## Steps the text did not give

N1. **Do the values of all pairs combine?** ORD-1 says "for every
pair, deduction of `F2` from these types succeeds", and ORD-P2 step 3
says "pair by pair". Neither says that the values of one template
parameter must agree across pairs. DED-21 says so for a call, and
DED-P1 lists partial ordering as a deduction context. Choice: each
pair is matched on its own (for its success flag, which ORD-10 and
ORD-11 use), and then the values combine by DED-21. If the values
disagree, the whole direction fails. Without this step `f(T, T)` and
`f(T, U)` would tie, and ORD-16's `typename Id<U>::type` example
would have no effect.

N2. **How pairs form when the lists differ in length because of a
pack.** Not given. Choice: pair by position. An expansion pairs with
each remaining entry of the other list. Two expansions at one
position make one pair. An entry with nothing against it takes no
part (ORD-4).

N3. **Pack values across pairs.** ORD-4 says the number of call
arguments for a pack takes no part. Choice: pack values never
conflict. The model does not check that a pack has the same elements
in two places.

N4. **When a function parameter pack takes part in a call.** ORD-5
says only parameters "for which the call has arguments" take part, and
ORD-4 says a pack takes no part when nothing is "against it". Choice:
a trailing pack takes part as one entry if the call gives it at least
one argument; otherwise it takes no part. ORD-12 then reads the
declared lists, not the lists that take part.

N5. **Two members, both with implicit object parameters.** ORD-6 covers
one member against a non-member. ORD-P1 says "an object parameter for
each member". Choice: each member gets an object parameter. With
ref-qualifier `&&` it is an rvalue reference, with `&` an lvalue
reference. With no ref-qualifier it takes the kind of the other
member's object parameter (ORD-P1, last paragraph), and two members
without ref-qualifiers both get lvalue references.

N6. **"The other template's first parameter" in ORD-6, with ORD-7.**
Not given which list this is when the other template is swapped.
Choice: the other template's list as it takes part, after its own
reversal, without an object parameter.

N7. **The number of call arguments for a member.** Choice: the
context gives the number of arguments with the object operand
included. A member with an implicit object parameter uses one fewer
explicit parameter.

N8. **Forwarding references (DED-8) in partial ordering.** ORD-3 names
DED-6, DED-14, and DED-15 as rules that do not apply. It does not name
DED-8. An invented type has no value category, so DED-8 has nothing to
act on. Choice: DED-8 does not apply. `T&&` is stripped to `T` like
any reference, and the kind of reference matters only in ORD-10.

N9. **A template parameter of F2 that appears only in non-deduced
positions.** ORD-3 says a parameter that appears in none of the types
needs no value. ORD-16 says a non-deduced position gives no value.
Neither says whether a parameter seen only in non-deduced positions
fails (as DED-23 would make it fail in a call). Choice: no failure.

N10. **"The whole function types" (ORD-9).** Not given whether the
function type is one pair or one pair per parameter and result, and so
whether ORD-10 and ORD-11 can act inside it. Choice: one pair, the
function type, matched exactly. ORD-2 strips only its outer reference
and qualifiers, so ORD-10 and ORD-11 never act in these contexts.
Parameters in a function type lose their top-level `const` (my
choice; the ordering pages do not say it). Member function templates
in these contexts get no object parameter.

N11. **Placement deallocation (ORD-9 with DED-39).** DED-39 deduces
from the parameters after the first, but ORD-9 says "whole function
types". Choice: whole function types, as ORD-9 says.

N12. **The type of an invented value against a constant parameter.**
Not given. Choice: DED-12 by analogy. F2's parameter declared `auto`
takes any value, `auto*` takes a pointer-typed value, and any other
type must equal the type of F1's parameter.

N13. **Partial ordering with incomparable qualifiers (ORD-11).** "More
`const` and `volatile`" for `const T&` against `volatile T&`. Choice:
a proper superset counts as more. Neither is more, so ORD-11 does not
act.

N14. **Pairs for ORD-10 and ORD-11 when the values do not combine.**
"Deduction succeeds both ways" is read per pair (N1), so ORD-10 and
ORD-11 can still act on a pair in a direction that later fails on
combination.

N15. **ORD-12 positions.** "The pack's position" is read as the index
of the pack in the declared function parameter list, without an object
parameter and before any ORD-7 reversal. ORD-12 is checked in every
function context, also address and conversion contexts.

N16. **ORD-13 for a member against a non-member.** The condition
"the function parameter lists have the same length" is read on the
declared lists, without object parameters. The same choice for the
types "position by position": compared after the CNS-34 renaming, with
top-level `const` of a parameter removed, references and inner
qualifiers kept.

N17. **ORD-13 when ordering gives "neither".** ORD-P2 runs the
tie-breakers only when each template is at least as specialized as the
other. OVL-41 reads as if it applies whenever OVL-28 does not decide.
Choice: ORD-P2's order. Constraints act only after a tie.

N18. **ORD-14 constraints.** ORD-14 says "Constraints compare by
CNS-23" with no conditions. CNS-P4's table says "two partial
specializations with the same pattern". Choice: constraints decide
only if the heads are equivalent (TPL-20) and the argument lists are
the same under the CNS-34 position correspondence. The primary
template counts with its parameters as its argument list.

N19. **Atomic constraint identity.** CNS-4 identifies an atom by its
place in the source and its mapping. Choice: an atom is a tuple of a
name (standing for the source expression or concept body) and its
argument names. F2's parameter names are renamed to F1's by position
(CNS-34) before comparing.

N20. **The template head check in ORD-13.** TPL-20 compares template
template parameters by their heads. Choice: the model compares kind,
pack, and constant type, and does not look into template template
heads.

N21. **Non-trailing packs in a list.** DED-20 makes a pack before the
end non-deduced. Choice: in a class argument list, a pack expansion
that is not last makes the rest of the list non-deduced and unchecked.

N22. **A qualified name through an invented type** (ORD-16) is a
distinct opaque type. A type parameter of F2 can take that type as its
value. Only its use against a second, different value fails.
