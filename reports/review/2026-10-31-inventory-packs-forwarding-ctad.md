<!--
Behaviour inventory, 2026-10-31: packs, forwarding references, and
class template argument deduction (roadmap item R).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository; the integrator's own check gives 0
flagged. No code, no quotes, no section names. Each item is a behaviour
to probe, not a rule.
-->

Role: source reader

# Inventory R: packs, forwarding references, class template argument deduction

The draft read for this inventory has no pack indexing, no structured binding packs, and no expansion statements. Those features are absent from the inventory.

## 1. Categories (probe grid axes)

- Pack kind: type pack, constant (non-type) pack, template template pack, function parameter pack, capture pack.
- Pack length: zero, one, several; two packs of equal or unequal length in one expansion.
- Where the expansion sits: call arguments, braced list, template argument list, base list, member initializer list, function parameter list, template parameter list, capture list, attribute list, alignment specifier, using-declaration, fold, pack-size operator.
- Position: trailing or non-trailing in its list.
- Nesting: one expansion inside another; a pack named in both the inner and the outer pattern.
- Deduction source: call argument, explicit argument, function type, class specialization argument list, partial ordering.
- Argument value category for forwarding: lvalue, xvalue, prvalue; const or not.
- Reference formation path: alias, template argument, decltype, placeholder.
- CTAD guide source: constructor, constructor template, no constructor, copy candidate, user guide, aggregate candidate, alias template, inherited constructor.
- CTAD initializer form: parentheses, braces, copy with equals, copy-list, empty, single element of the same template.
- CTAD site: variable declaration, new expression, functional cast, constant template parameter type.
- Explicit marking: explicit constructor or explicit guide, with copy versus direct initialization.

## 2. Behaviours

### A. Capture packs in lambdas
- A1 Capture by copy of a pack: a lambda can capture every element of a function parameter pack by copy, written as the pack name with a trailing ellipsis.
- A2 Capture by reference of a pack: the same works by reference, with an ampersand before the pack name.
- A3 Default capture and pack capture: with an equals default, a pack capture must be by reference; with an ampersand default, a pack capture must be by copy.
- A4 Init-capture pack by reference: the ellipsis can follow an ampersand, giving a pack of reference captures.
- A5 Init-capture pack count: an init-capture with a leading ellipsis makes one new member for each element its initializer expands to, and the new name is itself a pack.
- A6 Init-capture pack use: inside the body the new capture pack must itself be expanded, like any other pack.
- A7 Zero-element capture pack: an empty pack gives a lambda with no members from that capture.

### B. Class template argument deduction: alias templates
- B1 Alias constraint kept: constraints on the alias's own parameters also apply to the derived guides.
- B2 Alias form needed: an alias template can drive deduction only if it stands for a single specialization of another deducible template, possibly through a non-dependent qualifier.
- B3 Copy and guide status carried: a guide derived from the copy candidate, or from a user guide, keeps that status in the alias's set.
- B4 Deducibility constraint: each derived guide gets an extra constraint that the result type really is some specialization of the alias; a call whose result is not reachable through the alias fails.
- B5 Explicit status carried: a derived guide keeps the explicit marking of its source.
- B6 Guide derivation: each guide of the underlying template is adjusted by matching its result type against the alias's right side; deduction here may leave some parameters open without failing.
- B7 Leftover parameters: the derived guide's template parameters are the alias parameters that this matching used, then the source guide's parameters that it left open.
- B8 Nested alias: an alias of an alias works by the same steps, and the constraints of both layers apply.

### C. Class template argument deduction: aggregate candidate
- C1 Array element from braces: an array member given a nested braced list gets an rvalue reference to the array type as its parameter, so the bound can be deduced.
- C2 Array element from string literal: an array member given a string literal gets an lvalue reference to the const array type, so the element type deduces without const and the bound includes the terminator.
- C3 Brace elision limits: braces are not elided for an element whose type is dependent and not an array, for an array with a value-dependent bound, or for an array with a dependent element type when the initializer is a string literal.
- C4 Conditions for the candidate: the class must be defined, meet the aggregate rules (dependent bases are assumed to bring no virtual functions or virtual bases), have no user guides, and be given a non-empty parenthesized or braced initializer.
- C5 Designated initializers: a designated braced list can also produce the aggregate candidate.
- C6 Element types as parameters: otherwise each parameter is the declared type of the member that its initializer element would fill.
- C7 No match drops the candidate: if some initializer element matches no member, the aggregate candidate is not formed.
- C8 Non-trailing pack member: a pack expansion member before the end is taken to match no initializer element, and a pack parameter is still placed at its spot.
- C9 Parenthesized aggregate: a parenthesized list counts the same as a braced list for this candidate.
- C10 Trailing pack length: the length of a trailing pack in this candidate comes from the count of remaining arguments only when nothing else fixes it.
- C11 Trailing pack member: a pack expansion as the last member takes all remaining initializer elements and becomes one trailing function parameter pack.
- C12 Using later members: a member whose type is written in terms of an earlier member's declared type still deduces from the earlier element.

### D. Class template argument deduction: implicit guides
- D1 Constructor guides: each constructor of a defined class template yields a guide whose template parameters are the class's parameters followed by the constructor's own (with defaults), whose parameters are the constructor's parameters, and whose result is the class specialization over its own parameters.
- D2 Copy candidate: one more guide comes from a notional constructor taking the class by value; it is called the copy deduction candidate.
- D3 Default guide: if the class is only declared, or declares no constructors, a guide from a notional constructor with no parameters is added.
- D4 Explicit constructors: a guide from an explicit constructor is treated as explicit.
- D5 Parameters not deducible: a class parameter that appears in a constructor only through a member alias or a non-deduced position cannot be deduced through that guide.
- D6 Rvalue reference to class parameter: in a constructor guide, an rvalue reference to a class template parameter is an ordinary rvalue reference and does not forward.

### E. Class template argument deduction: inherited constructors
- E1 Guides from a base: when a class template inherits constructors from a direct base, the guides of that base, seen through a notional alias of the derived parameters to the base type, are added with their results mapped back to the derived template.
- E2 No inheritance without using: a derived class that does not inherit constructors gets no guides from the base.
- E3 Underivable parameters: if the base type does not mention the derived parameters in a deducible way, those guides add nothing useful and deduction fails.
- E4 User guides of the base: a user guide written for the base also reaches the derived class through this path.

### F. Class template argument deduction: selection
- F1 Each initializer kind: the guides act as the constructors of a notional class, and the usual rules for direct, copy, and list initialization pick one of them.
- F2 Explicit guide in copy context: if an explicit guide or constructor guide is the best match under copy initialization or copy-list initialization, the program is ill-formed.
- F3 Guides are public: the notional constructors are all public, whatever the access of the real constructor.
- F4 Initializer-list phase skipped: for a braced list holding one element whose type is, or derives from, a specialization of the same template, the initializer-list constructor phase is skipped, so the copy candidate wins.
- F5 Result replaces placeholder: the deduced type is the result type of the chosen guide; real initialization then runs again with that type.
- F6 Second step can fail: deduction can succeed and the following real initialization can still be ambiguous or fail.
- F7 Tie: inherited versus own: a guide from the class's own constructors beats one inherited from a base when both have the same parameter types for the given arguments.
- F8 Tie: non-template constructor: a guide from a non-template constructor beats one from a constructor template.
- F9 Tie: order of preferences: after the usual overload ranking ties, a user guide beats a guide that is not one; then the copy candidate beats the rest; then the non-template constructor rule applies.
- F10 Tie: user guide over copy candidate: a user guide as specialized as the copy candidate wins over it, so a user guide can wrap instead of copy.

### G. Class template argument deduction: where it applies
- G1 Constant template parameter: a deducible template name can be the type of a constant template parameter, deduced from the argument.
- G2 Declared type exact: in a variable declaration the declared type must be the placeholder with only cv-qualifiers; pointers, references, and arrays around it are not allowed.
- G3 Functional cast: a deducible template name can be used as the type in a functional cast with parentheses or braces.
- G4 Multiple declarators: when one declaration has several declarators, each must deduce the same type.
- G5 New expression: a deducible template name can be the type in a new expression.
- G6 No other site: a placeholder for a deduced class type is not valid anywhere else, such as a function parameter, a return type, or a member without an initializer.
- G7 Qualifier non-dependent: a scope qualifier before the template name must not be dependent.
- G8 Template name only: deduction happens only when the name has no argument list at all; an empty angle-bracket list is a different thing.

### H. Class template argument deduction: user guides
- H1 Access for member templates: a guide for a member class template must have the same access as that member.
- H2 Explicit guides: a guide can be marked explicit, with the effect in F2.
- H3 Form: a user guide names the template, gives a parameter list, and gives a result that is a specialization of that same template.
- H4 Guide parameters deducible: each template parameter of a guide without a default must be deducible from the guide's parameter list.
- H5 Not found by lookup: guides are not found by name lookup; every reachable guide for the template takes part.
- H6 Parameter restrictions: the guide's parameters obey the same rules as a function's parameters.
- H7 Rvalue reference forwards: in a user guide, an rvalue reference to a guide template parameter is a forwarding reference.
- H8 Same scope: a guide must be declared in the scope where the template lives.
- H9 Two equivalent guides: two guides for one template with equivalent parameter lists are ill-formed when one can reach the other.

### I. Declaring packs
- I1 Default arguments: no pack of any kind can have a default argument, and a function parameter pack can follow parameters that have default arguments.
- I2 Ellipsis parse at the end: an ellipsis written right after a parameter type with no comma joins the parameter as a pack when the type names an unexpanded pack or contains a placeholder; otherwise it is the C-style variable argument marker.
- I3 Function pack form: the pattern can be any declarator, including a pointer to function.
- I4 Function parameter pack: a function parameter declared with the ellipsis in its declarator is a function parameter pack, and the declaration is itself an expansion of its type.
- I5 Generic lambda or abbreviated pack: a placeholder parameter with an ellipsis invents a type parameter pack.
- I6 Order in class, variable, and alias templates: a pack must be the last template parameter of a primary class, variable, or alias template.
- I7 Order in function templates: a pack may come earlier in a function template if later parameters are deducible or have defaults.
- I8 Pack of constant parameters typed by an outer pack: a constant parameter pack whose type names an outer type pack is itself an expansion; the outer pack must not be in the same parameter list.
- I9 Template parameter pack kinds: a pack can hold types, constant values, or templates.
- I10 Template template pack: a template template parameter can be a pack.

### J. Deduction with packs
- J1 Explicit then deduced: explicit arguments fill the front of a pack and deduction may add more elements after them.
- J2 Function type deduction: a pack in a function type's parameter list deduces from the remaining parameter types of the matching function type.
- J3 Non-trailing function pack: a function parameter pack that is not last is not deduced.
- J4 Non-trailing in argument list: a pack expansion in a template argument list that is not the last argument makes the whole list a non-deduced context.
- J5 Partial ordering independence: partial ordering does not depend on how many elements a pack received.
- J6 Partial ordering pack versus single: a template whose matching position is a pack is less specialized than one with a plain parameter there, and a trailing pack with no counterpart loses a tie.
- J7 Trailing function pack: each remaining call argument is deduced against the pack's pattern, element by element.
- J8 Trailing pack defaults to empty: a trailing template parameter pack that nothing deduces and nothing gives is empty.
- J9 Trailing template-argument pack: a pack expansion at the end of a template argument list deduces from the remaining arguments of the matching specialization.
- J10 Zero-length deduction: deduction can produce a pack with no elements.

### K. Expansion mechanics
- K1 Element form: each element of a type pack acts as a type name, each element of a constant pack as a value, each element of a template pack as a template name, and each element of a function or capture pack as the name of that parameter or capture.
- K2 Empty expansion keeps the parse: an expansion with zero elements leaves the surrounding construct read the same way, even where removing the text would change or break the parse.
- K3 Empty list results: an empty expansion in a list gives an empty list, such as no base classes or no call arguments.
- K4 Innermost owner: a pack name is expanded by the nearest enclosing expansion only.
- K5 Lengths must match: all packs expanded together must have the same length.
- K6 Must contain a pack: an expansion pattern must name at least one pack that a nested expansion does not already expand.
- K7 Nested expansion: one pattern can hold an inner expansion over one pack and an outer expansion over another, or over the same pack again.
- K8 Pack treated as its result: when a rule talks about something other than a pack, a pack counts as what the pattern would produce for one element.
- K9 Pack-expansion argument to fixed parameters: an expansion can supply arguments for template parameters that are not packs, if the lengths fit after substitution.
- K10 Unexpanded pack is an error: naming a pack outside any expansion is ill-formed.
- K11 Zip of two packs: expanding two packs together pairs their elements by position.

### L. Expansion sites
- L1 Alignment specifier: an alignment specifier can be expanded over a pack, and the strictest alignment wins.
- L2 Attribute list: an attribute can be expanded, where the attribute allows it.
- L3 Base classes: a base list can expand a pack of types, giving one base per element.
- L4 Braced lists: an element of a braced list can be expanded, and the resulting elements are evaluated left to right.
- L5 Call arguments: a call argument can be expanded, and the elements become separate arguments.
- L6 Capture list: see group A.
- L7 Member initializers for bases: a member initializer that names a base can be expanded over a pack of bases.
- L8 Pack-size operator: see group N.
- L9 Template argument list: a template argument can be expanded, and an expansion in a partial specialization's argument list must come last.
- L10 Template parameter list: a template parameter can be an expansion of an outer pack (I8).
- L11 Using-declaration: see group O.

### M. Folds over packs
- M1 Binary fold extra operand: in a fold with two operands, exactly one holds an unexpanded pack; the other becomes the starting or ending value.
- M2 Empty unary fold: with no elements, a fold over logical and gives true, over logical or gives false, and over the comma gives a void value; any other operator is ill-formed.
- M3 Fold pattern: the pack part of a fold is the operand that holds an unexpanded pack, and it may contain other, nested expansions.
- M4 One pack per side: a unary fold's operand must hold an unexpanded pack, and a binary fold must not have packs on both sides.
- M5 Several packs in one operand: an operand can expand two packs of the same length together.

### N. Pack-size operator
- N1 Any pack: the operator applies to any kind of pack, including a function parameter pack and a capture pack.
- N2 Constant result: the result is a constant expression of the size type equal to the element count.
- N3 Counts as expansion: the operator is classed as an expansion, so the pack inside it counts as expanded and needs no ellipsis after it.
- N4 Name only: the operand must be the name of a pack, not an expression or pattern.

### O. Packs in using-declarations
- O1 Empty pack removes the names: if the pack is empty, the using-declaration brings in nothing, and a name found through it in the template may then be missing in an instantiation.
- O2 Expansion of base members: a using-declarator can be expanded over a pack of bases, bringing in a member of each, for example to merge call operators.
- O3 Inherited constructors per base: an expanded using-declarator naming constructors inherits the constructors of each base.

### P. Reference collapsing and forwarding references
- P1 Collapsing through alias and decltype: adding a reference to a type that is already a reference gives an lvalue reference unless both are rvalue references; cv-qualifiers on the alias are dropped.
- P2 Const blocks forwarding: an rvalue reference to a const type parameter is not a forwarding reference and cannot bind lvalues.
- P3 Forwarding in placeholder declarations: a placeholder declared as an rvalue reference deduces like a forwarding reference.
- P4 Forwarding pack: a function parameter pack whose pattern is a forwarding reference forwards each element separately.
- P5 Forwarding reference definition: a parameter whose type is a bare, unqualified type parameter of the function template or user guide, followed by a double ampersand, forwards.
- P6 Function type matching: when matching a function type for an address or a declaration, a forwarding reference matched against an lvalue reference is treated as the bare parameter, so it deduces an lvalue reference type.
- P7 Lvalue argument: with an lvalue argument the parameter is deduced as an lvalue reference type, and the parameter collapses to an lvalue reference.
- P8 Not in class guides: an rvalue reference to a class template parameter in a constructor guide is not a forwarding reference (D6).
- P9 Ordering lvalue versus rvalue reference: in partial ordering, between an lvalue reference and an rvalue reference with matching referred types, the lvalue reference template is more specialized.
- P10 Rvalue argument: with an rvalue argument the parameter is deduced as the plain argument type.

### Q. Template-level checks for packs
- Q1 Always-empty pack: a variadic template where every valid specialization needs an empty pack is ill-formed, no diagnostic required.
- Q2 Head argument list: in a template's own argument list for itself, a pack parameter appears as its expansion.
- Q3 Partial specialization argument checks: no unexpanded pack in an argument, and any expansion must be the last argument.
- Q4 Redeclaration match: a pack in one declaration's head must be a pack in every redeclaration.
- Q5 Template template matching: a template template parameter with a pack accepts a template with any number of matching parameters.

## 3. Gaps and doubts

- D1 with defaults: whether the constructor's own default template arguments are always copied into the guide, and how compilers treat a constructor template with a default argument that names a class parameter.
- B4, B6, B7: alias template deduction is recent; older releases of each compiler lacked it, and the leftover-parameter and constraint details may differ.
- C3 and C10: brace elision during the aggregate candidate and trailing pack length are recent repairs; compilers may still disagree.
- C9: parenthesized aggregate deduction depends on parenthesized aggregate initialization, which arrived late in some compilers.
- E1 to E4: CTAD through inherited constructors is very recent and may be missing in some compilers.
- F2: some compilers have accepted an explicit guide chosen in copy-list initialization.
- F4: whether a single braced element derived from a specialization (not the same specialization) skips the list phase in each compiler.
- G1: deduced class type as the type of a constant template parameter needs class-type constant parameters; older compilers lacked it.
- G6: some compilers accept a deduced class type in odd places, such as a static member with an in-class initializer.
- I2: the ellipsis parse rule leads to differing warnings, and placeholder parameters may differ.
- K2: some compilers have rejected an empty expansion in contexts where the empty form would be a parse error.
- K9: an expansion supplying fixed parameters of an alias template is a known area of disagreement and of open issues.
- L1, L2: expansion in alignment and attribute lists is rarely used and may be unsupported for some attributes.
- M2 with assignment operators: whether an empty fold over other operators is rejected at the point of use or only when instantiated.
- O1: lookup of a name brought in by an empty pack expansion may differ between compilers in timing of the error.
- P6 and P9: ordering between forwarding references and lvalue references has a history of compiler differences.
- Q1: compilers diagnose the always-empty case rarely and differently.
- Pack indexing, packs in structured bindings, and expansion statements are not in the draft read. If Tidal targets a later language, those need a separate reading.

## 4. Already covered in Tidal

- AUTO-4 covers P3 for the placeholder forwarding case.
- DCL-4 covers P1.
- DED-2 covers J1.
- DED-8 covers P2, P5, P7, P10, and P8 in part.
- DED-20 covers J3 and J7.
- DED-23 covers J8 and J10.
- DED-30 covers P6.
- DED-37 covers J2 and J9.
- DEP-8 notes a pack expansion in call arguments (L5 in part).
- DEP-33 covers Q1.
- ORD-4 and ORD-12 cover J5 and J6.
- ORD-10 covers P9.
- OVL-22 covers the viability side of J7.
- PRIM-27, PRIM-28, PRIM-29 cover M1 to M5.
- TARG-6 and TARG-21 cover how a pack takes template arguments.
- TARG-17 covers Q5.
- TPL-13 and TPL-34 cover I1 for template parameters; TPL-14 covers the default-after-pack case.
- TPL-20 covers Q4.
- TPL-30 covers I6.
- TPL-35 covers I7.
- TPL-36 covers I8.
- TPL-43 covers part of Q2.
- TSP-9 and TSP-11 cover Q3.

Not covered: all of the capture, CTAD, expansion-site, pack-size, and using-declaration groups; most of the expansion mechanics; and items I2 to I5, I9, I10, and P4. The overload-resolution page lists CTAD and its tie-breaks as not specified. The deduction page defers two items to packs: length mismatch between packs expanded together (K5), and an expansion before the end of a class template's argument list (J4).
