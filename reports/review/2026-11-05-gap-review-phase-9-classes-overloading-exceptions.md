# Gap review: the text phase 9 wrote on classes, overloading, and exceptions

Date: 2026-11-05

Role: source reader

Reference: the C++23 working draft sources. The review covers the
rules and principles on the classes, overloading, and exceptions pages
whose prose changed between commit 1b16115 and now (part A), one use of
UNI-P6 under decision 0056 (part B), and six open questions (part C). A
later core issue is named only where it may change an answer.

## Result

Most of the changed text agrees with the reference. Two rules state the
class-type width of a bit-field too narrowly. One rule keeps a test of
"same default argument" that the reference does not have. VIRT-1
depends on a later core resolution, because the C++23 text on
overriding is defective. The layout questions concern choices that the
reference leaves to the implementation.

Gaps found in part A:

- UNI-P4 and BF-1 ask for exactly one conversion function. The
  reference asks for exactly one target type. Several conversion
  functions to the same target type are allowed, and only the one that
  overload resolution picks must be constexpr.
- OVL-38 lets a call through when the two default arguments are the
  same expression. The reference rejects the call whenever two
  declarations in different scopes each supply a default argument that
  the call uses, equal or not.
- OPR-9 misses a third reserved form. In the deprecated form with a
  space between the quotes and the suffix, a suffix that starts with an
  underscore and then an uppercase letter is a reserved identifier, so
  the declaration is not valid with no diagnostic required.
- VIRT-1 matches only the later core resolution on overriding. Read
  literally, the C++23 text compares object parameter types that always
  differ between a base and a derived class, so the ref-qualifier
  condition comes from the later fix.
- ACC-20 does not say that the qualifier is a pattern. Only the
  specializations that match the pattern give friends, and a member of
  an explicit specialization counts only when its declaration matches
  the substituted friend declaration.
- CTOR-17 treats aggregate initialization as outside the ban on a
  default member initializer that binds a temporary to a reference
  member. The reference states the ban without naming constructors, so
  this reading has a doubt.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ACC-P3 | agrees | Befriending a member of a class template reaches that member in each matching specialization, explicit ones included when the declaration matches. |
| ACC-20 | agrees with a doubt | The qualifier acts as a pattern, and an explicit specialization's member counts only when its declaration matches. The rule says "every specialization". |
| CTOR-17 | agrees with a doubt | The reference bans the binding from a default member initializer without limiting the ban to constructors. Compilers and lifetime extension support the aggregate exception. |
| CVF-P1 | agrees | A conversion function template cannot have a deduced target type. |
| CVF-7 | agrees with a doubt | The reference forbids any deduced target for a template, which also covers a pointer or reference to a placeholder. The rule's words name only a bare placeholder. |
| CLS-P3 | agrees | The mixing ban applies only between functions whose object types match after references are dropped. |
| CLS-26 | agrees with a doubt | The rule matches for implicit object functions. The same ban also catches an explicit object function against an unqualified implicit one with the same object type, and CLS-26 does not say so. |
| CLS-13 | agrees | The deduction table matches forwarding and lvalue reference deduction. |
| CLS-35 | agrees | The reference leaves end padding and empty subobject placement to the implementation. The added lower bound on size follows from subobjects lying inside the object. |
| OPR-P3 | agrees with a doubt | "In either case" can read as allowing mixed case such as a lowercase then uppercase long-long suffix. Only all-lowercase and all-uppercase forms of that suffix exist. |
| OPR-9 | missing case | A suffix with an underscore and then an uppercase letter is reserved in the deprecated spaced form. |
| UNI-P4 | extra condition | The reference asks for one target type, not one conversion function. |
| UNI-P6 | agrees | Unnamed bit-fields take part in the common initial sequence and can end it. Constant evaluation rejects a read of an inactive member. |
| UNI-7 | agrees | A designated list may name only one member of each anonymous union. |
| BF-1 | extra condition | Same gap as UNI-P4. |
| VIRT-P4 | agrees | A call cannot use an abstract class as a parameter or return type. |
| VIRT-8 | agrees | Same reading as VIRT-P4. |
| VIRT-1 | agrees with a doubt | The ref-qualifier condition comes from a later core resolution. The C++23 text alone is defective here. |
| VIRT-13 | agrees | The reference gives no advice on these warnings. A recommended rule of Tidal's own conflicts with nothing. |
| UNW-2 | agrees | Closure members are initialized in an unspecified order, and unwinding destroys completed subobjects in reverse order of completion. |
| OVL-41 | agrees | Constraints decide only after each template deduces against the other and the parameter lists match. |
| OVL-38 | extra condition | The reference has no exception for equal default arguments. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| UNI-P6 | Q-0340 | tests/classes/members/grids/layout-compatible.json, row "overlap mark on both" | Layout-compatible. The first pair has the same type, no bit-field, the same alignment, and no attribute. The second pair has the same empty class type, the same alignment, and the attribute on both. So the sequence holds every member. | yes | supports with no doubt |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0337 | supports with a doubt | The set of conversion results offered to an lvalue reference holds only lvalue references, and the fallback path takes no explicit function. An example in the reference binds a reference to const through a by-value conversion as a direct case, which hints at a wider set. |
| Q-0361 | silent or left to the implementation | The reference leaves the reuse of end padding to the implementation. Both options conform. |
| Q-0372 | silent or left to the implementation | The reference defines padding bits but gives no value for a byte that mixes value bits and padding bits. A library note leans toward padding bits having no fixed value, closer to option 2. |
| Q-0381 | silent or left to the implementation | The attribute permits overlap but does not fix whether the class lends its end padding. |
| Q-0384 | silent or left to the implementation | The reference says nothing on the end padding of a union used as an overlapping member. |
| Q-0394 | supports with a doubt | The C++23 text compares object parameter types that always differ between base and derived, so it cannot be read literally. A later core resolution on overriding requires the same ref-qualifier, as the provisional answer says. |

## ACC-20

The reference reads the qualifier in such a friend declaration as a
pattern. The template parameters must be deducible from the qualifier.
A member of a specialization is a friend only when the specialization
matches the pattern and the friend declaration, after substitution,
matches that member's declaration. Two consequences are missing from
ACC-20. First, a qualifier that fixes part of the argument (a pointer
form, for example) reaches only the matching specializations. Second, a
member function of an explicit specialization with a different return
type is not a friend. ACC-P3 says "same name and type", which covers the
second point, but ACC-20 does not. The reference also requires the
qualifier to end with the class template's own template-id, so a member
of a nested class of the template cannot be named this way.

## CTOR-17

The reference makes a program invalid when the initializer that a class
gives in its member list for a reference member binds that member to a
temporary. The ban's wording does not mention
constructors, though its example uses only constructors. In aggregate
initialization the member is initialized from its default member
initializer, and the general lifetime rules then extend the temporary
to the lifetime of the reference, which is the aggregate's lifetime.
CTOR-17's added sentence matches compilers and the lifetime rules. A
literal reader could still apply the ban to the aggregate case.

## CVF-7

The reference forbids a deduced target type on any conversion function
template. A target that is a pointer to a placeholder, or a reference to
one, is also deduced. CVF-7 lists "a template whose name is auto or
another placeholder". A reader may take that to mean only a bare
placeholder.

## CLS-26

Under the reference, two member functions with the same parameters clash
when exactly one is an implicit object function with no ref-qualifier
and their object types match once references are dropped. The same test
applies when the other function has an explicit object parameter. So an
unqualified implicit object function and an explicit object function
whose parameter is a reference to the same class type clash. CLS-26
names only the ref-qualifier form through CLS-6.

## OPR-P3

The long-long integer suffix has only an all-lowercase and an
all-uppercase spelling. Mixed case is not a suffix of the language.
"In either case" can be read as allowing any mix of case. The single
letter suffixes have no such problem.

## OPR-9

The reference marks two kinds of suffix as reserved: a suffix without a
leading underscore, and one that contains a double underscore. A third
case comes from the general rule on reserved identifiers. In the
deprecated literal operator form, with whitespace between the empty
string and the suffix, the suffix is an ordinary identifier. An
identifier that starts with an underscore and then an uppercase letter
is reserved there, so the declaration is not valid, with no diagnostic
required. Without the space the same suffix is fine. OPR-9 has neither
the case nor a note.

## UNI-P4

The reference turns an expression of literal class type, used where an
integral constant is needed, into an integral or unscoped enumeration
type. It looks at the non-explicit conversion functions whose results
are of such a type, or a reference to one, and asks for exactly one such
target type. It then needs the function that overload resolution picks
to be constexpr. Two consequences differ from Tidal's text:

- A class may have several conversion functions with one target type,
  for example a const and a non-const overload, or one returning the
  type and one returning a reference to it. The width is valid if
  resolution picks one, and only the picked function must be
  constexpr.
- Two functions with different integral target types make the width
  invalid, even if both are constexpr.

UNI-P4 says "exactly one such conversion function" and "the function
must be constexpr". Both are stricter than the reference.

## BF-1

The same gap as UNI-P4. BF-1 says that exactly one non-explicit
conversion function gives an integral or unscoped enumeration type, and
that this function is constexpr. The reference counts target types, not
functions, and needs only the selected function to be constexpr.

## VIRT-1

The C++23 text makes a function override a base virtual function when
the two declarations correspond, with constraints set aside.
Correspondence compares object parameters. The base's object type is
the base class and the derived one's is the derived class, so a literal
reading never finds a match, and every override fails. A reader must
repair the text. One repair compares the object types with references
dropped when one function has no ref-qualifier, and it makes a
function with no ref-qualifier override one with a ref-qualifier. A
later core resolution (the issue on overriding and explicit object
parameters) instead compares the declarations as if both were in the
derived class and requires the same ref-qualifier for implicit object
functions. VIRT-1 matches the later resolution and the newest
compilers. The doubt is that VIRT-1 rests on a change after C++23.

## OVL-38

The reference rejects a call whose best function was found through
declarations in different scopes when two of them supply a default
argument that the call needs. The reference has no test of whether the
two arguments are the same. OVL-38 lets the call through when the
arguments are the same expression, and the page's own notes say that
the test is Tidal's own and follows the compilers. OVL-38 therefore
adds a condition that the reference lacks. The new list of when two
expressions are the same (entities, literals, parentheses) is Tidal's
own, and nothing in the reference speaks to it.

## Q-0337

The set of conversion function results offered to a reference that is
an lvalue reference holds only lvalue references. An explicit function
counts only in direct initialization and only from that set. The
fallback through copy initialization of a temporary takes no explicit
function. That reading gives option 1. But the binding rule for a
reference to const also covers a class value that converts to an
rvalue, and an example in the reference binds such a reference through
a by-value conversion function as part of that case. If the candidate
set is meant to include value results for this case, option 2 follows
for direct initialization. The reference is internally uneven here.

## Q-0361

The reuse of end padding is a layout choice that the reference does
not fix. Both options conform. The question records the platform ABI.

## Q-0372

The reference defines padding bits as the object bits outside the value
representation, and it makes extra bits of a wide bit-field padding
bits. It does not say what value a byte holds when some of its bits
are value bits and some are padding. A library note on atomic compare
and exchange describes padding bits as having an indeterminate value
when they are outside the active member's value representation. That
leans toward option 2. Zero initialization sets padding bits to zero,
so for an object with static storage duration, or for a class that
value initialization zero-initializes, the padding bits have a known
value of zero. Option 1 does not mention that case.

## Q-0381

The no-unique-address attribute lets a member overlap other
subobjects. The reference does not say whether a class that has such a
member lends its own end padding to a class that contains it. Both
options conform.

## Q-0384

The reference says nothing on whether a union lends its end padding
when it is an overlapping member. Both options conform.

## Q-0394

See VIRT-1. The literal C++23 text cannot decide the case, because it
never matches object parameter types across base and derived classes.
The later core resolution supports option 1, the provisional answer.
The prior review's option 2 depends on one particular repair of the
C++23 text.
