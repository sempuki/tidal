# Gap review: names and lookup

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the live rules and principles of
specification/foundations/names.md, and the one open question that
names a rule on that page, with that draft only. Later defect
resolutions are mentioned only where they matter.

## Result

The one provisional answer, Q-0311, agrees with the reference. The
reference states the template-id case directly, and the statement is
old. Clang conflicts with the reference.

Most rules agree. The gaps are these:

- NAM-14 makes every result with two entities ambiguous. In the
  reference, functions found along different paths of directives form
  one overload set.
- NAM-13 leaves out using directives that sit in an inline namespace of
  the named namespace.
- NAM-12 has no exception for the lookups that consider only types or
  only namespaces. Across scopes, an inner variable does not hide an
  outer class after a class key or in a list of bases. NAM-15 gives the
  exception for a qualifier only.
- NAM-16 asks for a complete class. The reference asks for a class and
  lets lookup find the members declared so far inside a class that is
  still being defined. A fundamental type used as a qualifier is also
  less clear-cut in the reference than NAM-16 says.
- NAM-38 always adds argument-dependent lookup for an operator. The
  reference routes operator lookup through the rule for calls, and a
  strict reading lets a block-scope function declaration turn
  argument-dependent lookup off.
- NAM-27 leaves out one associated class for a class template
  specialization, and its lambda row is open to a wrong reading.
- NAM-23 does not say that a directive in a namespace also covers the
  out-of-namespace definitions of that namespace's members.
- NAM-39 does not cover a member of an anonymous union, which cannot
  share a name with a class.
- NAM-1 says the point of declaration comes before the initializer.
  The reference places an enumerator's point of declaration after its
  initializer.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| NAM-P1 | agrees, loosely | The list of scopes leaves out the scopes of an enumeration, of template parameters, and of a lambda. The reference's outermost scope covers the whole program. The page uses the translation unit, which gives the same results without modules. |
| NAM-P2 | agrees | The search stops at the first scope that has any declaration of the name. The kind-restricted lookups are the exception. See "Hiding and restricted lookups". |
| NAM-P3 | agrees | |
| NAM-P4 | agrees | The table names function bodies and default member initializers. The reference also treats default arguments, default template arguments, and exception specifications as places where the class is complete. CLS-2 lists all but default template arguments. |
| NAM-P5 | agrees | The member-access row needs one addition. See NAM-37. |
| NAM-P6 | agrees | |
| NAM-P7 | agrees | A type found by ordinary lookup makes the construct a conversion and not a call, so the "does not happen" entry is right in effect. |
| NAM-P8 | agrees | |
| NAM-P9 | agrees, one missing condition | For structured bindings, the reference uses the member form only if the member search finds a member template whose leading template parameter takes a value. The table says only "member function template". The detail belongs to SB-5. |
| NAM-P10 | agrees | |
| NAM-1 | agrees, one case not covered | A declarator's point of declaration comes right after the complete declarator, so the variable is in scope in its own initializer, and the reference calls a read there undefined. An enumerator's point of declaration comes after its whole definition, so an enumerator's initializer sees an outer entity of the same name. The rule's general sentence about initializers reads as if it covered enumerators too. |
| NAM-8 | agrees | Modules can make a declaration from another translation unit visible. The page does not cover modules, so the third bullet holds for the language the page covers. |
| NAM-2 | agrees | |
| NAM-12 | missing condition | See "Hiding and restricted lookups". |
| NAM-3 | agrees | The reference treats the outermost block of a function as its own scope, but makes a conflict with a parameter ill-formed. The effect is the one NAM-3 states. |
| NAM-39 | agrees, one case not covered | The reference exempts a class or enumeration paired with a variable, a non-static data member, an enumerator, a function, or a function template. A non-static data member that belongs to an anonymous union is left out of the exemption, so it conflicts with a class of the same name. A non-normative note in the template clause says a template can share a name only with functions. The normative rule does let a function template share a name with a class, as NAM-39 says. |
| NAM-40 | agrees, one place not listed | The reference also restricts lookup to types for the names inside the type of a conversion function. A class that shares its name with a function is found there. |
| NAM-41 | agrees | |
| NAM-42 | agrees | A class template, alias template, or variable template does not count as declaring a type for the exemption. |
| NAM-4 | agrees | |
| NAM-9 | agrees, loosely | Members of an inline namespace are visible in the enclosing namespace without qualification, and directives, using declarations, and argument-dependent lookup also reach namespace members. The rule reads as absolute. |
| NAM-38 | doubt | See "Operator lookup". |
| NAM-13 | agrees, one missing source | See "Qualified lookup in a namespace". |
| NAM-14 | missing condition | See "Qualified lookup in a namespace". |
| NAM-15 | agrees | |
| NAM-16 | extra condition | See "Qualifiers". |
| NAM-17 | agrees | |
| NAM-18 | agrees | |
| NAM-37 | agrees, one case not covered | For the name after the member operator, the search covers the class and its bases only. When a qualifier sits between the operator and the member name, and the class search for that qualifier finds nothing, the reference then looks the qualifier up from the point of use as an unqualified name. NAM-37 says the search covers only the class. |
| NAM-19 | agrees | |
| NAM-20 | agrees | |
| NAM-21 | agrees | The reference extends the namespace or class scope over every part of the declaration after the qualified name, so array bounds, default arguments, and exception specifications there see the members too. A friend declaration with a qualified name searches the named scope first in the same way. |
| NAM-22 | agrees | |
| NAM-23 | agrees, one missing region | See "Directive regions". |
| NAM-24 | agrees | |
| NAM-25 | agrees | |
| NAM-26 | agrees | The reference does not let a class declaration be hidden by a variable found in a different search, so a class and a variable from two directives are ambiguous, as the rule says. |
| NAM-5 | agrees | |
| NAM-27 | agrees, one missing item and one doubt | See "Associated entities". |
| NAM-28 | agrees | A braced list has no type, so it adds nothing. The reference says outright that a lambda's parameter types do not affect its associated namespaces. |
| NAM-29 | agrees | The reference finds a friend of an associated class only if the class has a reachable definition. Associated namespaces are the innermost enclosing namespaces that are not inline, together with all their inline namespaces. That gives NAM-29's last sentence. |
| NAM-30 | agrees | The reference searches each associated namespace without following directives. For a dependent call, later declarations visible at the point of instantiation also count. The template pages cover that case. |
| NAM-31 | agrees | A block-scope using declaration names functions that do not inhabit the block, so argument-dependent lookup still happens. |
| NAM-32 | agrees | |
| NAM-33 | agrees | The injected class name is declared just after the opening brace of the class. |
| NAM-34 | agrees | The reference lets the constructor name appear only in the declarator of a constructor declaration (friend declarations included) and in a using declaration. |
| NAM-35 | agrees | The reference replaces a found injected class name by the type it names before it compares results from two bases, so two paths to one base type agree. |
| NAM-36 | agrees | |
| NAM-6 | agrees | The rule names only member function bodies. The other places where the class is complete come from CLS-2, which NAM-6 cites. |
| NAM-10 | agrees, one overlap | When a name used before its member declaration also has an outer declaration, the use finds the outer one. The later member then changes the meaning, which the reference makes ill-formed with no diagnostic required. TYN-5 states that case. NAM-10 holds when no outer declaration exists. |
| NAM-7 | agrees | |
| NAM-11 | agrees | |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0311 | A template-id adds what each type template argument adds (option 1). | agrees | option 1 | See below. |

### Q-0311 in more detail

- The reference says that when an argument is an overload set named
  with a template-id, the associated entities also include the
  template template arguments and the entities associated with the
  type template arguments. Value arguments add nothing.
- The statement goes back to the first standard. I know of no later
  resolution that removes it.
- GCC follows the reference. Clang's result conflicts with it.
- The third probe in Q-0311, the plain template name with no explicit
  arguments, is a separate point. The reference says a member of the
  set adds what its parameter types and return type add. It does not say
  what a function template's parameter types add when they depend on
  its own template parameters. Tidal's reading, that such types add
  nothing, is plausible, and both compilers agree with it. The
  reference does not state it.

### Hiding and restricted lookups

- The reference stops an unqualified search at the first scope that has
  a declaration. Some lookups consider only certain kinds of
  declaration, and the restriction applies before the stop. The lookup
  of a name before a qualifier considers only namespaces and types. The
  lookup after a class key with no qualifier, and the lookup of a name
  in a list of bases, consider only types. The lookup in a using
  directive or namespace alias considers only namespaces.
- So an inner variable does not hide an outer class at those places.
  The lookup passes over the inner scope and finds the outer class.
- NAM-12 says the use fails when the inner declaration does not fit.
  NAM-15 gives the exception for a qualifier. Neither rule gives it for
  a class key or a list of bases when the two names live in different
  scopes. NAM-40 covers only the same-scope case.

### Qualified lookup in a namespace

- The reference searches the named namespace and its inline namespaces.
  If that finds nothing, it follows the directives that sit in the
  named namespace or in any of its inline namespaces. NAM-13 mentions
  only directives in the named namespace. A directive must also come
  before the point of lookup.
- When the paths end at different entities, the reference applies its
  general rule. If all the declarations are functions or function
  templates, they form an overload set and overload resolution
  chooses. The result is ambiguous only when the entities differ and
  are not all functions. NAM-14 makes any two entities ambiguous.
  NAM-25 states the function exception for unqualified lookup only.
- The reference keeps a class found along one path alongside a variable
  found along another, so the two are ambiguous. NAM-14's last sentence
  agrees.

### Qualifiers

- The reference requires the name before a qualifier to denote a
  namespace, a class, an enumeration, or a dependent type. It no
  longer requires the class to be complete. A lookup in a class that
  is only declared finds nothing, so the use still fails. A lookup
  inside a class that is being defined finds the members declared
  earlier, and the reference shows such a use as valid in an array
  bound. NAM-16 asks for a complete class, which rejects that case.
- For an opaque enumeration, a qualified lookup finds no enumerators,
  so the use fails as NAM-16 says, by a different route.
- The reference's general rule rejects a fundamental type as a
  qualifier. Its rule for a destructor name after a qualifier asks only
  that the qualifier name a type, and its own illustration uses an
  alias for a fundamental type there. The two rules conflict. A
  destructor call through such an alias is the doubtful case. NAM-16
  rejects it.

### Operator lookup

- The reference builds the non-member candidates from an unqualified
  lookup of the operator function name as in a call, with argument
  dependent lookup, and with member functions ignored. Ignoring member
  functions has the effect NAM-38 states, passing over class scopes.
- The rule for calls turns argument-dependent lookup off when ordinary
  lookup finds a function declared in a block. Read strictly, the same
  holds for operators. NAM-38 says argument-dependent lookup always
  adds its functions.
- The existing test does not separate the two readings, because its
  associated namespace declares no operator. A probe in which the
  block declaration does not fit and the operand's namespace has one
  that does would show the compilers' choice. I expect the compilers
  to keep argument-dependent lookup for operators, but I have not
  checked.

### Associated entities

- For a class template specialization, the reference adds the entities
  of the type arguments, each template used as a template template
  argument, and the class of which such a template is a member when it
  is a member template. NAM-27 leaves out that last class, so friends
  declared in it would be missed.
- The reference declares a closure type in the smallest enclosing
  block, class, or namespace scope. A lambda in a default member
  initializer or a static data member's initializer has the class as an
  associated class. A lambda inside a member function body is in a
  block, so the class is not associated. NAM-27 says "the class whose
  body contains the lambda", which a reader can take to include member
  function bodies. NAM-28 states the matching exclusion for a local
  class but not for a lambda.
- An argument that is the address of an overload set counts like the
  set itself in the reference. NAM-27's overload-set row does not
  mention the address form.

### Directive regions

- A directive is active wherever it comes earlier and sits in the
  scope being searched. The scope of a namespace includes the part of
  each out-of-namespace member definition after the qualified name.
  So a directive in a namespace also applies inside the body of a
  member of that namespace defined later outside it.
- NAM-23 lists the rest of the namespace definition and later
  definitions of the namespace. It does not list those member
  definitions.
