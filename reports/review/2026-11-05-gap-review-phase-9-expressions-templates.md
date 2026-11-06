# Gap review: the text phase 9 wrote on expressions and templates

Date: 2026-11-05

Role: source reader

Reference: the C++23 working draft sources. This review covers part of
item RV3 of decision 0057. It reads the rules and principles of the
expressions chapter (comparison, conditional, lambdas, new and delete,
postfix) and the templates chapter (arguments, class template argument
deduction, concepts, declarations, deduction, instantiation, ordering,
specialization) whose prose changed after commit 1b16115. It also
applies eleven principles to the programs of their questions, and reads
ten open questions against the reference.

## Result

Most changed rules agree with the reference. Six items read against it,
and several others carry a doubt. The gaps found in Part A:

- CMP-P3 and CMP-8 let constant evaluation give a result for a
  relational comparison of two members of a union, or of a member that
  occupies no storage. The reference leaves the order of such pointers
  unspecified, and an unspecified comparison result is not a constant
  expression.
- CMP-8 can be read to reject a comparison of two members of one base
  class subobject. The reference orders those members.
- COND-16 says that a base class operand never converts to the derived
  class type of the other operand. The reference sends that pair to its
  last case, where a converting constructor of the derived class can
  form the conversion.
- COND-16 says that a reference does not bind directly to a class
  prvalue in the xvalue step (Q-0401). The reference counts that binding
  as direct, and only the lvalue step asks for a glvalue.
- LAM-P3 and LAM-6 limit only the capture default outside a block. The
  reference puts the same limit on a lambda with any named capture or a
  capture of the enclosing object.
- PFX-P3 lists the places where the class is complete and leaves out a
  default template argument of a member template.
- TARG-27 forbids the ambiguous injected class name as a template
  template argument. The reference treats the name there as the
  template itself, with no ambiguity.
- CTAD-16 says that a derived parameter unused by the base takes its
  default in an inherited guide. The reference's deducibility test
  matches a partial specialization, which has no defaults, so the guide
  fails also when the parameter has a default.
- The ORD-P1 row for an address with no target type says that no types
  take part. The reference still orders two function template
  specializations there by their function types, and it removes a less
  constrained function that is not a template with no condition on the
  parameter lists.
- ORD-13 leaves out the extra conditions for a swapped rewritten
  candidate, where the template parameters must have one reordering that
  makes the two heads and parameter types match.
- ORD-4 no longer says that an invented pack in a class template
  argument list with nothing against it takes no part.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| CMP-P3 | contradicts | Union members and members with no storage have no fixed order, so constant evaluation rejects a relational comparison of them. |
| CMP-P5 | agrees | The reference lets an integer to floating-point conversion through and gives a partial ordering. |
| CMP-8 | contradicts | Same gap as CMP-P3, and the base class wording can reach two members of one base subobject. |
| COND-P2 | agrees with a doubt | The C++23 text lets a same-class operand with fewer qualifiers fall through to the last case; the result for the cited forms still matches. |
| COND-16 | contradicts | The new base class row and the Q-0401 sentence both read against the reference. |
| LAM-P2 | agrees | Capture order follows the member order, which is unspecified. The trailing return type sees captures and the parameter list does not. |
| LAM-P3 | missing case | Named captures share the limit on capture defaults. |
| LAM-6 | missing case | Same as LAM-P3. |
| LAM-7 | agrees | |
| LAM-15 | agrees | |
| LAM-28 | agrees | The reference makes both the member order and the initialization order unspecified. |
| NEW-25 | agrees with a doubt | The new sentence agrees. The reference checks access of the deallocation function only when the new expression creates class objects. |
| PFX-P3 | missing case | A default template argument is also a place where the class is complete. |
| PFX-22 | agrees | |
| PFX-11 | agrees with a doubt | The rule omits that a different result from the complete class makes the program not valid with no diagnostic. The page notes this. |
| PFX-15 | agrees with a doubt | The reference limits the case to a potentially evaluated argument and to eligible special members. |
| TARG-27 | contradicts | As a template template argument, the name means the template and is not ambiguous. |
| TARG-20 | agrees | |
| TARG-13 | agrees | The reference's list of conversions for a constant argument has no floating-point promotion or conversion. |
| TARG-16 | agrees with a doubt | The C++23 grammar of a template argument has no braced list. |
| CTAD-6 | agrees | |
| CTAD-11 | agrees | |
| CTAD-16 | contradicts | The default of an unused derived parameter does not rescue the inherited guide. |
| CPT-35 | agrees | |
| TPL-49 | agrees with a doubt | The reference does not say that the object is never destroyed. It asks for constant destruction. |
| TPL-35 | agrees | |
| DED-15 | agrees | |
| DED-24 | agrees | The reference lists a bound of zero or less among the failures. |
| DED-39 | agrees with a doubt | The reference does not say how a deallocation template is deduced for the match. Exact deduction is the likely reading. |
| INST-P1 | agrees | |
| INST-12 | agrees | The reference keeps the constraints of a local class member out of the eager instantiation. |
| ORD-P1 | contradicts | The new row for an address with no target type. |
| ORD-P2 | agrees with a doubt | Step 6 adds that the values from all pairs agree; the reference states success pair by pair. |
| ORD-P4 | agrees with a doubt | Declared parameter types already lose top-level qualifiers in the function type. |
| ORD-1 | agrees with a doubt | Same doubt as ORD-P2. |
| ORD-17 | agrees | The reference gives a placeholder in an invented value's type a new invented type. |
| ORD-3 | agrees | |
| ORD-4 | missing case | The ignored invented pack in a class template argument list. |
| ORD-6 | agrees with a doubt | The reference does not limit the object parameter to calls or to members with an implicit object parameter, and does not name the class for a member brought in by a using declaration. |
| ORD-9 | agrees with a doubt | One pair agrees. The deduction of a forwarding parameter from an lvalue reference inside the pair has no special basis. |
| ORD-11 | agrees | |
| ORD-12 | agrees with a doubt | The reference does not say whether the object parameter counts for the pack position. |
| ORD-13 | missing case | The swapped rewritten candidate conditions, and the doubt of ORD-P4. |
| ORD-14 | agrees | |
| TSP-11 | agrees | |
| TSP-14 | agrees with a doubt | See Q-0295. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| LAM-P2 | Q-0092 | `capture-visible-in-trailing-return.cpp` | The trailing return type sees the copy, so the type is a reference to const int, and the program is accepted. | yes | supports with no doubt |
| LAM-P2 | Q-0092 | `capture-not-visible-in-parameter-list.cpp` | The parameter list sees the outer variable, so the type is a reference to int. | yes | supports with no doubt |
| TARG-P4 | Q-0168 | `same-specialization-alias-template.cpp` | The alias template is a template of its own, so the two specializations differ. | yes | supports with a doubt |
| ORD-P2 | Q-0217 | `reference-kind-and-qualifiers.cpp` | Step 5a removes one direction and step 5b the other, so there is no order and the initialization is ambiguous. | yes | supports with no doubt |
| ORD-P1 | Q-0218 | `reversed-member-object.cpp` | The member gets its object parameter, then the swapped pair is reversed, and the member is more specialized. | yes | supports with no doubt |
| DEP-P6 | Q-0247 | `base-pack-expanded-twice.cpp` | Every valid argument list leaves the pack empty, so no diagnostic is required, and each compiler's result fits. | yes | supports with no doubt |
| DEP-P3 | Q-0248 | `dependent-base-conversion-function.cpp` | The conversion function name with a dependent target type searches the dependent base at the instantiation, and the call finds the base's function. | yes | supports with no doubt |
| CNS-P2 | Q-0263 | `atomic-not-bool-skipped.cpp` | The left operand is false, so the right one is never substituted or checked, and the program is accepted. | partly: the outcome is one that option 3 allows, and the model does not give "not valid" | supports with a doubt |
| CNS-P5 | Q-0269 | `redeclare-placeholder.cpp` | The constraint on the placeholder sits in another place, so the two declarations are two templates, and the call is ambiguous. | yes, with a doubt about what "place" means for an invented parameter | supports with no doubt |
| ORD-P2, ORD-P4 | Q-0293 | `constraints-ellipsis.cpp` | The ellipsis is no pack and does not count, the lists tie, and the constraint picks the constrained template. | yes | supports with a doubt |
| ORD-P2, ORD-P4 | Q-0293 | `constraints-ellipsis-on-second.cpp` | The same steps, with the constraint on the template that has the ellipsis. | yes | supports with a doubt |
| COND-P2 | Q-0378 | `class-deleted-conversion-counts.cpp` | The deleted conversion counts, both operands convert, and the expression is rejected. | yes | supports with no doubt |
| COND-P2 | Q-0378 | `class-const-and-volatile-lvalues.cpp` | Neither set of qualifiers includes the other, so two class lvalues are rejected. | yes | supports with a doubt |
| CMP-P5 | Q-0386 | `builtin-three-way-mixed.cpp` | Each use yields one class (strong for two integers, partial for an integer against a floating-point value), and only that class is needed. | yes | supports with a doubt |
| CMP-P5 | Q-0386 | `ordering-strong-only-builtin.cpp` | Two integers yield the strong ordering class, the only class declared and needed. | yes | supports with a doubt |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0295 | supports with a doubt | Deduction fails when a constant's type differs from the type of the matching parameter of the named template. The reference does not say plainly what that type is when the primary parameter is a placeholder. |
| Q-0318 | supports with a doubt | No rule names a member call through a null pointer as undefined. The answer rests on the lack of any object, and constant evaluation rejects only undefined operations that the text specifies. |
| Q-0325 | silent | The reference leaves the integer to pointer mapping to the implementation and says nothing about objects at such addresses. |
| Q-0392 | supports with no doubt | The reference names a member function of a local class as a case where the requires clause is not instantiated with the function. The member is templated, so the clause is allowed. |
| Q-0396 | supports with a doubt | Outside a call, the function type is the one type that takes part, so only the top level is stripped. The forwarding parameter part has no special basis. |
| Q-0397 | supports with no doubt | The invented value of a placeholder parameter has a new invented type, so one direction fails and the int partial specialization is more specialized. |
| Q-0398 | supports with no doubt | The object parameter is a reference to the class with the member's qualifiers. |
| Q-0399 | supports with no doubt | The reference lists an array with a size of zero or less among the substitution failures. Whether the bound is computed does not matter. |
| Q-0400 | supports with no doubt | The no-diagnostic rule covers a requirement whose substitution always fails. A valid requirement that is always false is outside it. |
| Q-0401 | reads against | The xvalue step asks only that the reference bind directly. Binding an rvalue reference to a class prvalue of a derived class counts as direct, so the result is an xvalue of the base. |

## CMP-P3

The reference orders two pointers only for elements of one array and
for members of one object. It drops the member order when the class is a
union or when a member has zero size. Any other relational result is
unspecified. Constant evaluation rejects a comparison whose result is
unspecified. So a relational comparison of two union members, or of a
member with no storage and another member, is rejected during constant
evaluation. CMP-P3 says that constant evaluation still gives a result.
The equality comparisons of such members are defined, and they stay
valid. The page notes this split and cites a pending decision.

The new example of two rows of an array of arrays agrees. A pointer past
the end of a row is not past a complete object, so the equality has a
fixed result. The example of two separate arrays also agrees.

## CMP-8

Same gap as CMP-P3 for union members and members with no storage.

The new base class case agrees for two base subobjects and for a base
subobject against a member. The last sentence says that a pointer to a
member of a base subobject involves the subobject. Two pointers to two
members of the same base subobject are ordered by the reference, because
they point to members of one object. The sentence can be read to reject
that comparison during constant evaluation. The rule should make clear
that it covers two different base subobjects, or a base subobject
against a member of the derived object.

## COND-P2

For two operands of one class type, the C++23 text gives the target
type of the other operand only when that operand has at least the same
qualifiers. Otherwise it falls through to the last case, where the
target is the other operand's type and a copy constructor can form the
conversion. A later defect resolution changes this case so that no
conversion forms. COND-P2 follows the later text. For the const and
volatile class lvalues of Q-0378, both texts reject the expression. The
C++23 path gets there through a copy of a volatile object, which no
implicit copy constructor can make.

## COND-16

The new row for a base class operand against an operand of a derived
class says that the base operand does not convert. The reference has
three cases: the same class, the other type is a base of this one, and
everything else. A base operand against its derived class falls into
the last case. Its target is the derived class type, and a converting
constructor of the derived class from the base can form the conversion.
The page notes that no newest compiler converts there. So the row
follows the compilers and reads against the reference.

The paragraph added for Q-0401 also reads against the reference. See
the Q-0401 section.

## LAM-P3

The reference forbids a capture default and also any named capture,
including a capture of the enclosing object, unless the lambda's
innermost enclosing scope is a block, or the lambda is in a default
member initializer. LAM-P3 and LAM-6 state the limit for the capture
default alone. A named capture of an automatic variable in a default
argument of a function declared in a block is one case that LAM-6 does
not reject: the variable has automatic storage, and the lambda is not
inside a block.

## LAM-6

See LAM-P3.

## NEW-25

The new sentence about a deleted function beside a matching template
agrees. The reference calls a deallocation function only when lookup
finds a single match. Two matches leave none chosen, so the deleted one
is never referred to.

The rule says that the function must be accessible in every case. The
reference asks for the access check of the deallocation function only
when the new expression creates an object or array of class type. For
other types, only a global function can be found, so the difference may
have no program that shows it.

## PFX-P3

The reference lists five places where a class is complete: function
bodies, default arguments, default template arguments, exception
specifications, and default member initializers. PFX-P3 lists four and
leaves out a default template argument of a member template. CLS-2 has
the same gap.

## PFX-11

The rule agrees. The reference adds that the program is not valid if
the member access gives a different result than it would with the
complete class. The page's notes record this condition as not yet
stated.

## PFX-15

The reference limits the case to a potentially evaluated argument. An
argument in an unevaluated operand is outside it. The class case also
looks only at eligible copy and move constructors.

## TARG-27

The reference treats an injected class name as the template itself in
three uses: with an argument list, as the argument for a template
template parameter, and in a friend template declaration. When lookup
finds injected class names of several specializations of one template
and the name is used as the template, there is no ambiguity. So a class
with two bases of one template can pass the base name as a template
template argument. TARG-27 forbids that use. The part about a use as a
type agrees.

## TARG-16

The C++23 grammar of a template argument has a constant expression, a
type, and a name. It has no braced list. A later defect resolution may
add one. The part about floating-point parameters agrees.

## CTAD-16

For a class template that inherits constructors, the reference builds
the inherited guides through an alias whose parameters are those of the
derived class and whose type is the base. Each such guide carries a
constraint that the alias arguments are deducible from the result. That
test matches a partial specialization whose parameters are those of the
derived class. A partial specialization has no defaults, so a derived
parameter that the base type does not use is never deduced, and the
guide fails. The reference's own example shows the failure without a
default. CTAD-16 says that a default rescues the guide. The reference
gives no basis for that. The passage is hard to follow, so this reading
carries a small doubt.

## TPL-49

The reference gives the object static storage and asks for constant
destruction. It does not say that the object is never destroyed. With
constant destruction, the destruction has no observable effect, so the
two statements differ only in wording.

## DED-39

The reference says that a placement deallocation function matches when
its parameters after the first are the same as those of the allocation
function, after parameter adjustments. It does not say how a template is
deduced for this match. Exact deduction with no call adjustments is the
natural reading, and DED-39 takes it.

## ORD-P1

The new row for an address of an overload set with no target type reads
against the reference in two ways.

First, with no target, deduction runs with no types, so a function
template still gives a specialization when explicit template arguments
fix all its parameters. If two such specializations remain, the
reference removes the one whose template is less specialized, by
partial ordering of the function types. So types do take part. The row
says none do.

Second, for functions that are not templates, the reference removes a
function when another one is more constrained. It states no condition
on the parameter lists in this context. The row applies the conditions
of ORD-13. Test `address-no-target-constraints-different-parameters.cpp`
and Q-0142 follow the compilers here, and the reference reads against
them.

## ORD-P2

Step 6 says that the values that all pairs give to one parameter must
agree. The reference describes deduction one pair at a time, and says
that each pair that deduces makes its type at least as specialized. It
treats the template parameters as a whole only when it says that a
parameter may stay without a value if the compared types do not use
it. Combining the pairs is the reading that all compilers take, so this
is a doubt only.

Step 6d says that the types compare as declared. See ORD-P4.

## ORD-P4

The reference compares the function parameter types of the two
templates. A function parameter type already loses its top-level
qualifiers in the function type. So a constrained template whose
parameter is a const value and another whose parameter is a plain value
meet the condition, and the constraint decides. The words "as the
templates declare them" can be read to keep that qualifier. The cases
that the page gives (a reference to const against a value, and two
result types) agree with the reference.

## ORD-1

Same doubt as ORD-P2 for the agreement of the values from all pairs.

## ORD-4

The reference says that, during partial ordering, an argument of a class
template argument list that was a pack expansion is ignored when the
other list has no argument at its position. It fails only when the
other list has an argument there that is not a pack expansion. The old
text of ORD-4 said that a pack with nothing against it takes no part,
for both kinds of list. The new text keeps that statement for call
arguments only.

## ORD-6

The reference gives the first parameter to each member function
template. It does not limit this to a call, and it does not name the
kind of object parameter. ORD-6 limits the parameter to a call and to a
member with an implicit object parameter. For a static member, or a
member with an explicit object parameter, the two texts can differ.

For a member that a using declaration brings into a derived class, the
reference does not say which class the reference names in partial
ordering. Overload resolution treats such a member as a member of the
derived class for its object parameter. ORD-6 follows that, which is a
reasonable reading.

The qualifiers of the member on the object parameter agree.

## ORD-9

One pair of whole function types agrees, with only the top level
stripped (Q-0396). The reference uses the general deduction from a type
inside that pair. It has no special case that deduces a forwarding
parameter from an lvalue reference by collapsing. The page notes that
every compiler fails that deduction for the address of a function
outside partial ordering. The reference uses the same deduction in both
places. So either both succeed or both fail. If both fail, the
reference-kinds example has no order, which is GCC 14's result.

The last sentence, that a member gets no object parameter in these
contexts, has the doubt of ORD-6.

## ORD-12

The reference speaks of a trailing pack for which the other template
has no corresponding parameter. It does not say whether the object
parameter of a member counts for the position. The page notes that no
program separates the two readings. The rest agrees, including the
default argument (Q-0216) and the ellipsis.

## ORD-13

When exactly one template is a swapped rewritten candidate, the
reference adds two conditions before constraints decide. Every template
parameter of both templates must be deducible from the function
parameters. There must also be exactly one reordering of the template
parameters that makes the parameters equivalent and the function
parameter types the same. ORD-13 states neither. The constraints page
notes the case and leaves it to ORD-7.

The "as declared" wording has the doubt of ORD-P4.

## TSP-14

See Q-0295. The reference fails deduction when a constant's type differs
from the type of the matching parameter of the named template. For a
primary template with a placeholder parameter, the reference does not
spell out that this type is the type of the argument.

## Q-0168

The reference makes two template arguments the same when they refer to
the same template, and an alias template is a template of its own. It
does not say whether an alias that passes its parameters through refers
to the class template. Implementers have long treated that as an open
point.

## Q-0263

The reference requires an atomic constraint to be a constant of type
bool only when satisfaction substitutes into it. A short circuit skips
the substitution. The no-diagnostic answer rests on the general rule for
a template that no argument list makes valid. Applying that rule to a
concept that is never instantiated is a reading, so the support is weak.

The CNS-P2 model gives acceptance at this use, because its steps never
reach the right operand. The principle does not state the no-diagnostic
rule, which sits in CNS-39 as an exception.

## Q-0269

The reference states that an abbreviated function template and the
same template written with a head are functionally equivalent but not
equivalent. Two such declarations of one function make the program not
valid, with no diagnostic required. That is option 2. The CNS-P5 model
reaches option 2, with a doubt about whether the constraint on a
placeholder counts as "another place".

## Q-0293

The reference compares the lengths of the function parameter lists and
the types of the parameters at each position. An ellipsis is not a
parameter, so two templates that differ only in an ellipsis meet the
conditions. The reference does not say this outright. For functions
that are not templates it uses the parameter type list, which includes
the ellipsis, so the split of ORD-13 has a basis.

## Q-0295

See TSP-14. The reference also shows that the type of a constant
parameter, when it uses template parameters, takes their values from
the argument's own type. So a pointer to a
const placeholder is not deduced from a pointer to a non-const type,
which agrees with the answer. The doubt is only about the case of a
plain int parameter against a placeholder primary.

## Q-0318

The reference has no rule that names a member call through a null
pointer. The rule on class member access makes the access undefined when
the object has a dissimilar type, and a null pointer gives no object at
all. Constant evaluation rejects operations that the text makes
undefined. The answer is the common reading, with a doubt because the
undefined behavior is implicit.

## Q-0325

The reference leaves the mapping from integers to pointers to the
implementation. It says nothing about objects that a platform provides
at fixed addresses. The provisional answer's claim that the pointer is
an invalid pointer value has no basis in the reference either.

## Q-0378

Form 1 agrees with no doubt. The reference states that access, a
bit-field operand, and a deleted conversion function are ignored when
it asks whether a conversion sequence can be formed. Form 2 agrees with
the doubt of COND-P2.

## Q-0386

The reference forbids a use of an ordering class, also an implicit use
through the built-in operator or a defaulted comparison, before the
class is declared. It names the type that the use needs, which supports
option 3. A doubt remains for a defaulted comparison with a deduced
return type, whose common category the reference computes from the
member results. A further doubt is that the reference asks for a
standard library declaration, and the tests declare the classes
themselves.

## Q-0396

See ORD-9. One pair with no inner stripping is supported. The case of
the forwarding parameter in `address-reference-kinds.cpp` depends on a
deduction that the reference treats the same way inside and outside
partial ordering.

## Q-0401

The reference tries a conversion to an rvalue reference to the other
type when the other operand is an xvalue, and allows it only if the
reference binds directly. Only the lvalue step adds that the binding
must be to a glvalue. The reference defines binding directly as every
case of reference initialization apart from the last one, where the
initializer is converted to a temporary of the referenced type. Binding
an rvalue reference to a base from a class prvalue of a derived class
materializes the prvalue and binds to the base subobject. That is not
the last case, so it binds directly. So the derived prvalue converts to
an xvalue of the base, and the result is an xvalue, as GCC says. A later
defect resolution may add the glvalue condition to the xvalue step as
well. If it does, the answer of Clang and EDG holds.
