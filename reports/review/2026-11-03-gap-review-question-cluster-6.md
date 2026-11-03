# Gap review: question cluster 6

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. The draft already contains the rewrite of scope and lookup that
also changed when a less-than sign after a name starts a template
argument list, which matters for Q-0225 and Q-0235. Later changes are
mentioned only where they matter, and they are not in the reference.

## Result

Three of the ten answers conflict with the reference in some form.

- Q-0225 contradicts the reference. The draft reads a less-than sign
  as the start of an argument list after the last name of a
  typename-qualified name, a base, or an alias target, with no keyword.
  The draft supports option 2, and it goes further than GCC for the
  alias target.
- Q-0226 contradicts the reference in part. The keyword rule agrees
  with option 1. The form that deduces class template arguments fails
  on a separate rule, so the draft's results match Clang's in every
  column. The draft supports option 3 for the results.
- Q-0235 contradicts the reference for the variable form, with the
  same doubt as the class form. The function form agrees. The draft
  supports option 2 for a variable.

One answer agrees with a doubt.

- Q-0346 agrees for a wrong element type and for a bound that differs
  from a stated bound. For a bound where the template states none, the
  draft does not say clearly which type the stated type is compared
  with, and EDG's reading is possible.

The other six answers agree with the reference. Q-0233 carries a side
note on the thread-local form.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0225 | The keyword is needed (option 1). | contradicts | option 2 | See below. |
| Q-0226 | The keyword without a list is valid only before a template template argument or a class or alias template (option 1). | contradicts, in part | option 3 for the results | See below. |
| Q-0227 | The keyword is not valid in a declarator (option 1). | agrees | option 1 | The draft forbids the keyword right after a qualifier that names the scope of the entity being declared. The rule covers every declaration, so a member function, a static member function, and a member class template are all covered. The rule is not exempt from diagnosis. |
| Q-0233 | A storage class other than thread-local and an attribute are not valid, and thread-local is valid (option 1). | agrees | option 1 | The draft allows no storage class on an explicit instantiation except thread-local, and lets no attribute apply to an explicit instantiation. An attribute before the keyword has no place in the grammar. An attribute after the class key in the class form is also not allowed, since that form is not one of the two that may carry attributes. As a side note, the draft requires thread-local on every declaration of a variable once one has it, and the explicit instantiation is a declaration of the specialization. So the draft may even require the keyword to be repeated. That depends on whether the template's declaration counts as a declaration of the specialization, the doubt already raised for Q-0203. |
| Q-0234 | Not valid (option 1). | agrees | option 1 | The draft forbids an explicit instantiation declaration that names a specialization whose template has internal linkage. A template declared static and a template in an unnamed namespace both have internal linkage, and the rule makes no distinction among a function, a class, and a variable. The rule is not exempt from diagnosis. An explicit instantiation definition is not limited this way. |
| Q-0235 | The name must be qualified (option 1). | contradicts for a variable, with doubt; agrees for a function | option 2 for a variable, option 1 for a function | See below. |
| Q-0236 | The bound can be left out (option 1). | agrees | option 1 | The draft treats the explicit instantiation and the declaration made from the template as two declarations of one entity, which must have the same type. Declarations of one array object may differ in whether the first bound is present. A note with an example in the draft accepts an explicit instantiation of a variable template with the bound left out. The rule applies alike to both kinds of explicit instantiation and whether or not the template states the bound. |
| Q-0254 | The definition can follow the explicit instantiation (option 1). | agrees | option 1 | For a variable template, the draft requires only that a declaration be reachable from the explicit instantiation, and that the definition be present somewhere in the unit. A definition must be reachable only for a class template and its member classes. A unit that never defines the variable template breaks the presence rule, which is not exempt from diagnosis. |
| Q-0255 | A stated exception specification must match (option 1). | agrees | option 1 | The draft lets an explicit instantiation leave out the exception specification. If it states one, the specification must be the same as on every other declaration of the function. A diagnostic is required when the mismatch is in one translation unit, so GCC's acceptance is a defect. The rule covers both kinds of explicit instantiation and members of class templates. |
| Q-0346 | The stated type must be the specialization's type (option 1). | agrees, with doubt | option 1 | See below. |

### Q-0225 in more detail

- The draft lists cases in which a less-than sign after a name starts
  a template argument list. One case is a name that is the last name
  in a context where only a type can appear, as long as that name is
  not itself part of a qualifier. Another is the last name of the
  declared name in a declarator.
- The draft's list of type-only contexts includes the name after the
  typename keyword, a base, the type in an alias declaration, and
  parameter types in several kinds of declaration.
- So after the typename keyword and in a base, the draft reads the
  sign as the start of an argument list with no template keyword. This
  is GCC's result, and Clang and EDG apply the older rule. The same
  holds for a function parameter written with the typename keyword.
- The alias target without the typename keyword is also a type-only
  context, so the draft reads the argument list there too. Every
  compiler rejects that form, GCC included.
- The friend form falls under the declarator case, so the draft reads
  the sign as an argument list there too. Whether such a friend
  declaration is otherwise valid is a separate matter that the draft
  does not settle clearly. The draft has a rule for a template friend
  that names a member of a dependent type, and the test is not a
  template friend. I mark the friend form as unclear.
- The draft supports option 2, and it extends option 2 to the alias
  target without the typename keyword.

### Q-0226 in more detail

- The draft requires a name after the template keyword to be followed
  by an argument list, or else to name a class or alias template.
  The second case is deprecated. A data member after a
  dot and a static data member after a qualifier fit neither case, so
  both are invalid and need a diagnostic. This agrees with Clang and
  EDG, and GCC's acceptance after a dot is a defect.
- The form that deduces class template arguments from the initializer
  meets the keyword rule, since the name refers to a class template.
  It fails on two other rules. The grammar for a placeholder for a
  deduced class type has no place for the template keyword. The draft
  also requires the qualifier of such a placeholder to be
  non-dependent, and here the qualifier depends on a template
  parameter. The member alias template probe fails the same way.
- So the draft rejects every form in the question's table, which is
  Clang's result. The wording of option 1 matches the keyword rule
  itself, but the deduced form is not valid. In practice the draft
  leaves the template template argument as the only place where the
  keyword without a list names a class template, which is option 3.
- I have not checked whether a later change lifts the limit on a
  dependent qualifier for a deduced class type.

### Q-0235 in more detail

- The cluster 5 review covered the class form. It found no rule in
  the draft that places an explicit instantiation in its template's
  namespace or makes the name qualified, and ordinary lookup through a
  using-directive finds the template.
- For a function template, the draft keeps a limit. When an explicit
  instantiation or a specialization names a function template
  specialization with an unqualified name, only templates declared as
  members of the namespace where the declaration stands are
  considered. A template in another namespace that a using-directive
  makes visible is not one of them. So the function form is not valid,
  as every compiler finds. This agrees with option 1.
- The limit is stated for function template specializations only. A
  variable template specialization is named by a template-id that
  ordinary lookup resolves, and the using-directive makes the template
  visible. So the variable form is valid, which is Clang's and EDG's
  result. This supports option 2.
- The doubt is the one from the class form. The draft may have
  dropped the older placement rule without intent. Clang now accepts
  the variable form while it still rejects the class form, so
  implementations do not apply the old rule consistently.

### Q-0346 in more detail

- The draft treats the explicit instantiation and the declaration made
  from the template as two declarations of one entity, which must have
  the same type. Only the first array bound may be present in one and
  absent in the other. A note in the draft gives an example that
  rejects an explicit instantiation with a type that differs from the
  specialization's.
- A different element type, or a whole type that differs, is
  therefore not valid. When the template states a bound and the
  explicit instantiation states another, both bounds are present and
  the types differ, so that form is not valid either. The template's
  declaration is reachable, so a diagnostic is required. GCC's
  acceptance of these forms is a defect.
- The doubt is the form where the template states no bound and the
  initializer fixes it. The draft does not say whether the declaration
  made from the template has the type it is written with, which has no
  bound, or the complete type that the initializer gives. Read the
  first way, a bound of four differs from no bound only by presence,
  and the form is valid, as EDG finds. Read the second way, the types
  have different bounds, and the form is not valid. The draft's rule
  that an initializer defines the array's bound points to the second
  reading, which is option 1.
