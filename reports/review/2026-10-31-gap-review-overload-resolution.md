<!--
Gap review summary, 2026-10-31: overload resolution (item M, decision
0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's page spec/foundations/overload-resolution.md.
It did not write or change any file in this repository; the integrator
copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository, and the integrator's own check gives 0
flagged. No code, no quotes, no section names. Each item ends with the
outcome a probe should show; it is a suggestion to probe, not a rule.
Labels: M = missing, E = possible error, O = order, S = structure.
-->

Role: source reader

Gap review of item M, overload resolution. Each item ends with the
outcome that a probe should show.

M-1 (nearest OVL-13, also DCMP-6). The three-way comparison operator
itself gets reversed candidates. The page and DCMP-6 list reversed
candidates only for equality and the relational operators. A class with
only a non-member three-way comparison that takes the class first and an
integer second can be compared with an integer on the left. The result
is the ordering that the function gives, flipped, because zero is
compared against the call's result. Probe: accepted, and an object that
the function reports as greater compares as less from the reversed side.

M-2 (nearest OVL-13). After a rewritten three-way candidate wins for a
relational operator, the comparison of its result with zero is resolved
with no rewritten candidates. If the three-way comparison returns a class
type that has its own three-way comparison against an integer and no
less-than operator, a less-than between two objects needs a rewritten
candidate for the inner step. Probe: rejected.

M-3 (nearest OVL-13, also DCMP-12). A reversed equality candidate that
wins for a plain equality expression must also return bool, possibly
cv-qualified. DCMP-12 states the bool requirement only for the not-equal
rewrite. A member equality of class A that takes a B and returns an
integer, used with the B object on the left, picks the reversed candidate.
Probe: rejected. With the A object on the left the comparison is accepted.

M-4 (nearest OVL-2). The current object is the implied object argument
wherever the current object can be used. Those places include more than
member function bodies. A default member
initializer that calls a non-static member function of the same class by
an unqualified name is one such place. Probe: accepted.

M-5 (nearest OVL-6). A conversion function template gives no surrogate
candidate, since nothing fixes its result type for a call. A class whose
only conversion is a template that could produce a pointer to function,
called like a function with one argument. Probe: rejected.

M-6 (nearest OVL-6). A conversion function with a reference qualifier
can still give a surrogate, but the object must then bind to that
qualifier when it converts. A class with one conversion to a pointer to
function, qualified for rvalues only. Probe: a call on an lvalue object
is rejected, and a call on a prvalue object is accepted.

M-7 (nearest OVL-18). When the target is a pointer to member function,
the class that declares the member plays no part in matching. Only the
function type matters, and the usual conversion from a pointer to member
of a base to a pointer to member of a derived class follows. A base with
two overloads of one name, taken by address with the base's qualified
name, initializes a pointer to member of a derived class whose function
type matches one overload. Probe: accepted, and the matching overload is
the one chosen.

M-8 (nearest OVL-9). The built-in candidates exist only inside operator
resolution. A program cannot call one by the operator function's name.
Calling the addition operator function by name with two integer
arguments. Probe: rejected.

M-9 (nearest OVL-27). A built-in candidate counts as a function that is
not a template specialization, so it beats a template specialization when
all sequences tie. An enumeration type and a non-member function template
for equality that takes two parameters of one deduced type and returns a
distinct class type. Equality between two enumerators keeps both
candidates, because a template does not remove the built-in candidate.
Probe: accepted, and the result has type bool, which shows the built-in
operator won.

M-10 (nearest OVL-15). For direct binding through a conversion function,
the result type must also be reference-compatible with the referenced
type. A conversion function that returns an lvalue reference to long is
no candidate for direct binding of a reference to const int, and the
binding then goes through a temporary. Probe: accepted, and at run time
the reference does not refer to the long object. In direct initialization
of a reference, an explicit conversion function also takes part if its
result differs from the referenced type only in cv-qualifiers. Probe: a
direct initialization of a reference to const int from an object whose
only conversion is explicit and returns an lvalue reference to int is
accepted. The copy initialization is rejected.

E-1 (OVL-21). Parentheses around a function name do not change the
call. A call through a parenthesized name keeps the implied object
argument, as a call by the bare name does. Only the address form drops
it. OVL-21 groups the parenthesized name with the address form. Probe:
inside a non-static member function, a call through the parenthesized
name of a member with an implicit object parameter and no arguments is
accepted and acts on the current object.

E-2 (OVL-2). The current object becomes the implied object argument only
if the current class is the class whose members lookup found, or derives
from that class. Otherwise the argument is a made-up object of that
class, and a win by a member with an implicit object parameter is an
error. An enclosing class has a static member of one name that takes an
int and a non-static member of the same name that takes a long. A member
function of a nested class calls the name with a long argument. Under
the page, the current object of the nested class cannot meet the
non-static member, so the static member would win through a conversion.
Probe: rejected.

E-3 (OVL-17). The exclusion of an inherited constructor depends on the
argument count only. Exactly one argument is needed, of any type. The
argument need not have the derived type. A base with only implicitly
declared constructors and a derived class that inherits them. Direct
initialization of a derived object from a base lvalue. Probe: rejected,
because the base's copy constructor is excluded and nothing else is
viable. A base constructor whose first parameter is a reference to the
base and whose second parameter has a default is also excluded for a
one-argument call. Probe: the same initialization with a second
argument added is accepted.

E-4 (OVL-38). Default arguments from two scopes make the call
ill-formed even when the two default values are equal. The test for
equal values expects acceptance. Probe: rejected for the equal-value
case as well. GCC alone matches this outcome (Q-0140).

E-5 (OVL-20). With no target type, a function template still adds a
specialization when explicit template arguments fix every template
parameter. A single template, taken by address with an explicit argument
list and stored in a deduced variable. Probe: accepted. The rule requires
the set to have exactly one function that is not a template, which
rejects the case.

E-6 (OVL-20). With no target type, the plain functions are all selected
and the later reduction steps still run. Constraints that are not
satisfied remove a function, and a more constrained plain function
removes a less constrained one. A class template with two non-template
members of one name and opposite constraints. The address of the member
for one specialization stored in a deduced variable. Probe: accepted. A
second probe with both constraints satisfied and one subsuming the other
also accepted, and it picks the more constrained member.

E-7 (OVL-21, OVL-3). An address can be formed from an overload set that
holds members with an implicit object parameter only by the plain
qualified name. With a parenthesized qualified name or an unqualified
name, such members cannot have their address taken, so the expression is
an error even if a static member of the set would win the later call.
Inside a member function, a set of one reference-qualified non-static
member with no parameters and one static member with a defaulted
parameter. Probe: a call with no arguments through the address of the
parenthesized qualified name is rejected. Through the address of the
unparenthesized qualified name it is accepted and calls the static
member. Q-0143 records related splits.

E-8 (OVL-9, the table of OVL-P4). Two rows use J, an integer type. In
the reference treatment these rows range over every integral type, which
includes bool and the character types. Tidal's types page lists bool and
the character types apart from the integer types. If J excludes them,
two results change. First, a char variable as the left operand of a
remainder or bitwise compound assignment, with a class object on the
right that converts to int, has no candidate. Probe: accepted. Second,
three-way comparison between an object that converts to char and an
object that converts to bool has candidates for char, for bool, and for
int that do not order. Probe: rejected as ambiguous.

O-1 (OVL-27, OVL-29, OVL-P3). The reference treatment puts the
function-reference-kind tie-breaker before the plain-over-template
tie-breaker. A plain conversion function that returns an lvalue
reference to function and a conversion function template that returns
an rvalue reference to function. An rvalue reference to function is
bound to the object. Probe: the template is chosen. The page's order
picks the plain function, as Clang and EDG do (Q-0148).

O-2 (OVL-30). The constraint tie-breaker needs the two parameter lists
to be the same in full, including parameters whose default arguments
the call uses and a trailing ellipsis. The page says "same parameter
types", which can be read as only the parameters the call uses, the way
OVL-31 reads. Two plain functions, one taking an int, the other taking
an int and a defaulted second parameter, where the constraint of one
subsumes the other. A call with one int. Probe: rejected as ambiguous.
The same with a trailing ellipsis in place of the defaulted parameter.
Probe: rejected as ambiguous.

O-3 (OVL-26, OVL-28, OVL-P3). The page says the result tie-breaker never
separates two templates, because the results match exactly. Exact-match
rank still has more than one sequence. An identity sequence beats a
sequence with only a function pointer conversion. Two conversion
function templates, one that deduces a pointer to function and one that
deduces a pointer to a function that cannot throw, initialize a pointer
to a function that can throw. The result tie-breaker would choose the
first, and the order of the result step against the specialization step
then becomes observable. Probe: accepted, and the first template is
called.

S-1 (OVL-21). The rule mixes two behaviours. A call through the address
of an overload set is one. The handling of an argument that is itself an
overload set is the other, and it applies to every call by name, not only
calls through an address. Probe: a call by name of an overloaded function
with an overloaded function name as argument, where exactly one pairing
fits, is accepted.

S-2 (OVL-14). The rule mixes the candidate sets for each form of
initialization with the special case for explicit conversion functions
that make the temporary for a copy or move constructor's parameter. The
second part depends on an outer constructor call and is easier to test on
its own. Probe: the existing explicit-conversion tests split cleanly.

S-3 (OVL-28). The rule mixes the partial ordering of templates, the
constraint ordering of templates with equal parameters, and a claim about
the order of OVL-27 and OVL-28. Probe: none beyond the existing tests and
Q-0146.

S-4 (OVL-17, OVL-P2). The candidate sets have a second exclusion that
applies in every context. A defaulted move constructor or move assignment
that is deleted is not a candidate. The copy and move page states the
behaviour, but the overload page does not list it beside the inherited
constructor exclusion or point to it. A class whose defaulted move
constructor is deleted because of a member with a deleted move, while
the member's copy works. Initialization from an rvalue. Probe: accepted,
and the copy constructor runs.
