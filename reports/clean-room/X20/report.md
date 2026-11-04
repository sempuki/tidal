# X20: P17, opaque type aliases

Date: 2026-11-04. Commit read: `dad8bf8`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P17 has no description beyond its row in section 2 of `reports/clean-room/roadmap-3/designer.md` ("an alias that makes a distinct type with the same representation and operations") and its rows in roadmap-4 and roadmap-5 section 1.7. So the syntax, the term and the rules are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `python3 tools/propose_check.py`, run before reading any earlier Propose report, and `reports/clean-room/X12/report.md` as the model for this report's form.
- `reports/clean-room/roadmap-5/designer.md` sections 1.5 to 1.7 and items 1 to 6; the P17 rows of roadmap-3 and roadmap-4; the P1 to P16 overview of `reports/clean-room/roadmap/designer.md`.
- In full: `specification/declarations/type-names.md`, `specification/templates/arguments.md`, and `specification/declarations/enumerations.md`.
- The cited rules of these pages: types (TYP-P1, TYP-5 to TYP-9), copy-move (CPY-P6, CPY-19), storage (STG-P7, STG-12, STG-14, STG-23), casts (CAST-P1, CAST-1, CAST-2, CAST-6, CAST-14), overload resolution (OVL-P4, OVL-9 to OVL-12), operators (OPR-2, OPR-6), names (NAM-27, NAM-28), template declarations (the TPL-P1 table, TPL-6, TPL-11, TPL-26 to TPL-31, TPL-46), deduction (DED-P3, DED-P4, DED-10 to DED-13), specialization (TSP-7), class template deduction (the alias rows of CTAD-14), dependent names (DEP-15), odr (ODR-7, ODR-10, ODR-12), storage-linkage (STO-P3, LNK-1 to LNK-10), conversions (CONV-20), if (IF-3, IF-4), switch (SW-2), unions (BF-2), handlers (TRY-2), postfix (PFX-15 and the ellipsis text), list initialization (LIST-P1), glossary (arithmetic type, scalar type, structural type, contextual conversion, linkage).
- Tests: the TYN-1, TYN-8, TARG-19, TARG-22, TPL-28, ENUM-3, ENUM-8, ENUM-10, ENUM-16, ENUM-19, NAM-28, OPR-2, OVL-9, STG-14 and ODR-10 tests named below, and the grep scans of section 4.
- `tests/programs/`: all 80 files of the 67 program tests, scanned by grep, with `W7/money.cpp` and `W15/largest.cpp` read in full.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`, with `-O0` and `-O2` for one run and `-Wall` for one run) under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X20/p`. Each probe that prints calls `fflush` before it returns. The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 57 rule changes (6 new, 23 amended of which 1 is a Syntax segment, 28 unchanged but must state their effect). No existing test changes its expected outcome. 11 findings, X20-1 to X20-11.

## 1. Design

### 1.1 The feature

Today a type alias is a second name for a type that already exists (TYN-1, TYN-P1). `using Meters = double;` makes `Meters` and `double` one type. A function that takes `Meters` takes any `double`, and `Box<Meters>` is `Box<double>` (probe `a1.cpp`). A program that wants a distinct type today writes a class with one member, or a scoped enumeration with no enumerators and a fixed type (probe `e1.cpp`). The class needs every operator written by hand. The enumeration has comparisons but no arithmetic (probe `e2.cpp`), and only integer underlying types.

P17 adds the *explicit alias*. The word `explicit` after the `=` of an alias declaration makes a new type, an *explicit type*. The explicit type has the size, alignment, values and built-in operations of its *underlying type*. It converts to and from the underlying type only by a cast.

```cpp
using Meters  = explicit double;
using Seconds = explicit double;

Meters run{100.0};                    // direct-list-initialization (X20:new-3)
Meters lap = run + run;               // Meters + Meters gives Meters (X20:new-4)
bool longer = lap > run;              // comparisons give bool
double d = static_cast<double>(run);  // a cast goes back

Seconds t{9.58};
auto mixed = run + t;                 // rejected: no candidate for (Meters, Seconds)
double e = run;                       // rejected: no implicit conversion
Meters f = 5.0;                       // rejected: no implicit conversion
Meters g = run * 2.0;                 // rejected here: no candidate for (Meters, double)

Meters operator*(Meters m, double k) { return Meters{static_cast<double>(m) * k}; }
Meters operator/(Meters, Meters) = delete;   // replaces the built-in candidate (OVL-10)
Meters h = run * 2.0;                 // valid after the declaration above
auto ratio = run / run;               // rejected: the deleted function wins

template <class Tag> using Id = explicit int;   // an explicit alias template
struct User; struct Order;
Id<User> u{1};
Id<Order> o = u;                      // rejected: two explicit types
template <class Tag> Tag* owner(Id<Tag>);       // Tag is deduced from Id<User>
```

Code that has no explicit alias keeps today's meaning. Section 4 shows that no existing test changes.

### 1.2 The declaration (new rules X20:new-1, X20:new-2; amended Syntax, TYN-P1, TYN-1, TYN-2, TYN-8)

The syntax adds one optional keyword to `alias_decl`:

```text
alias_decl: "using" identifier {attribute_spec} "=" ["explicit"] type_id ";"
```

`explicit` is a keyword, so it can name nothing else. Both compilers reject `explicit` in that place today, at namespace scope, in a class and in an alias template (probes `s1.cpp`, `s4.cpp`, `s5.cpp`). They also reject `explicit using M = double;` and `using explicit M = double;` (probes `s2.cpp`, `s3.cpp`). The `typedef` form gets no explicit version, and `typedef explicit int M6;` stays rejected (probe `s6.cpp`). `explicit` takes no parenthesized condition here. A `type_id` cannot start with `(`, so the grammar has no ambiguity with `explicit(bool)`.

X20:new-1 states what the declaration makes:
- The name denotes a new type, the explicit type. Each explicit alias declaration makes a different type, also when two of them name one underlying type.
- The explicit type has the size, alignment and values of its underlying type. An object of the explicit type has the object representation that an object of the underlying type with the same value has.
- The explicit type is a scalar type. It is not an arithmetic, integral, floating-point or pointer type. Rules that ask for one of those kinds do not accept it. So an explicit type cannot be the type of a bit-field (BF-2), the condition of a `switch` (SW-2), an array bound, or the fixed underlying type of an enumeration (ENUM-17).
- cv-qualifiers apply to the explicit type as to any type. `const Meters` is valid.

X20:new-2 limits the underlying type to a pointer to an object type, or to an arithmetic type except `bool`, with no cv-qualifiers at the top level. These are not valid as underlying types: `bool`, an enumeration, an explicit type, a class, an array, a reference, a function type, `void`, a pointer to a function, a pointer to member, and `std::nullptr_t`. Section 1.7 gives the reasons. A pointer to an incomplete class is valid, so `using Handle = explicit struct Node*;` works before `Node` is defined.

An explicit alias defines its type, as a class definition does. So an explicit alias is declared once in its scope (amended TYN-2). A second `using Meters = explicit double;` in the same scope is rejected, and so is `using Meters = double;` after it. TYN-1's "the declaration does not make a new type or a new overload" gets the exception, and TYN-8's second item ("a function declared with an alias and a function declared with its type are one function") no longer holds for an explicit alias. TYN-P1 states the exception in its first sentence.

### 1.3 Conversions (new rule X20:new-3; amended CAST-1)

No implicit conversion exists between an explicit type and any other type. Each context of CONV-20 rejects an operand of the wrong type. These are the only ways in and out:

| Form | Valid? | Model today |
|---|---|---|
| `Meters m{1.0};`, `Meters{1.0}` | yes; a narrowing conversion to the underlying type is rejected | ENUM-8, ENUM-19 |
| `Meters m{};` | yes; the value is zero, or a null pointer | value-initialization of a scalar |
| `Meters m = {1.0};` | no | ENUM-19 |
| `Meters m(1.0);` | no | probe `t3.cpp`: `E e(1);` is rejected for a scoped enumeration |
| `static_cast<Meters>(1.0)`, `Meters(1.0)`, `(Meters)1.0` | yes | CAST-1, probe `t4.cpp` |
| `static_cast<double>(m)` | yes, to the underlying type only | CAST-1 for enumerations |
| `static_cast<int>(m)` | no; a program casts twice | |
| `if (m)`, `!m`, `m && n` | no; no contextual conversion to `bool` | ENUM-11, IF-4 |
| an argument for an ellipsis parameter | passes as the explicit type, with no promotion | no rule today (X20-5) |

The brace form takes any value that converts implicitly to the underlying type, so `Meters{1}` is valid and `Meters{n}` for an `int` variable `n` narrows and is rejected. The value is the value that the conversion to the underlying type gives.

CAST-1 gains the item "an explicit type to its underlying type, and a value that `static_cast` can convert to the underlying type to the explicit type". `static_cast<Meters>(1)` therefore converts `1` to `double` first. CAST-6 does not change. An explicit type over a pointer is not a pointer type, so `reinterpret_cast` cannot convert it to another pointer. `reinterpret_cast<Meters&>(x)` for a `double` object `x` is the reference item of CAST-6, and amended STG-14 makes the access defined (1.6).

### 1.4 Operations (new rule X20:new-4; amended OVL-P4, OVL-9, OPR-2, OPR-6)

The built-in operations reach an explicit type through built-in candidates, in the way OVL-P4 gives candidates for enumerations today. For an explicit type `O` with underlying type `U`:

| Operators | `U` arithmetic | `U` integral | `U` is `T*` | Result |
|---|---|---|---|---|
| binary `*`, `/` | `(O, O)` | | | `O` |
| binary `+`, `-` | `(O, O)` | | `(O, std::ptrdiff_t)`; `(std::ptrdiff_t, O)` for `+` | `O` |
| binary `-` of two pointers | | | `(O, O)` | `std::ptrdiff_t` |
| `%`, `&`, `^`, `\|` | | `(O, O)` | | `O` |
| `<<`, `>>` | | `(O, I)` for each promoted integer type `I` | | `O` |
| unary `+`, `-` | `(O)` | | `(O)` for `+` | `O` |
| `~` | | `(O)` | | `O` |
| unary `*` | | | `(O)` | `T&` |
| `[]` | | | `(O, std::ptrdiff_t)` | `T&` |
| `->` | | | as for `T*` | |
| `==`, `!=`, `<`, `>`, `<=`, `>=` | `(O, O)` | | `(O, O)` | `bool` |
| `<=>` | `(O, O)` | | `(O, O)` | the ordering type that `U` gives |
| `++`, `--` | `(V O&)` | | `(V O&)` | as for `U` |
| `=` | `(V O&, O)` | | `(V O&, O)` | the left operand |
| `*=`, `/=`, `+=`, `-=` | `(V O&, O)` | | `(V O&, std::ptrdiff_t)` for `+=`, `-=` | the left operand |
| `%=`, `&=`, `^=`, `\|=` | | `(V O&, O)` | | the left operand |
| `<<=`, `>>=` | | `(V O&, I)` | | the left operand |

`!`, `&&` and `||` have no candidate, because an explicit type does not convert to `bool`. `?:` with two operands of one explicit type has that type (COND-3). With an explicit type and its underlying type, `?:` has no common type and is rejected.

The value of each operation is the value of the same operation on the underlying values. If that result has the underlying type or its promoted type, the result converts to the underlying type as `static_cast` does, and then has the explicit type. So for `using Small = explicit short;`, `a + b` adds in `int` and converts the sum back to `short`, as `a += b` does for two `short` objects today. Overflow, division by zero and shifts follow the rules of the underlying type unchanged.

The candidates follow OVL-10 and OVL-11 like other built-in candidates. So:
- A class that converts to `O` can use them (OVL-11).
- A non-member function with the parameter types of a candidate replaces it (OVL-10). A deleted `Meters operator/(Meters, Meters)` therefore removes division from `Meters`. Both compilers already treat a deleted `operator==(E, E)` this way for an enumeration (probe `e4.cpp`).

For this to work, OPR-2's first limit becomes "a parameter of class, enumeration or explicit type, or a reference to one", and OPR-6 lists an operand of explicit type with the class and enumeration operands. A program can then declare the mixed operations it wants, such as `Meters operator*(Meters, double)`.

### 1.5 Explicit alias templates and argument equivalence (new rule X20:new-5; amended TPL-P1, TPL-28, TARG-P4, TARG-19, TPL-6, DED-P3)

The roadmap asks for a check of class templates and argument equivalence. This section is that check. Today an alias template is transparent. TPL-28 says a specialization "is not a new type", TARG-P4 compares types "after aliases are replaced", and deduction sees through the alias. A tag parameter that the aliased type does not use is lost (probe `t2.cpp`). Both compilers make `Id<User>` and `Id<Order>` the same type, and neither deduces `Tag` from a call with `Id<User>`.

X20:new-5 makes an *explicit alias template*, a template whose declaration is an explicit alias, behave like a class template for identity:
- Each specialization is an explicit type. Two template-ids of one explicit alias template name the same explicit type if their arguments are the same by TARG-P4. Otherwise they name two types. This holds also when the underlying type does not use a parameter, as in `Id<Tag>`.
- A specialization is not replaced by its underlying type, in TARG-P4 or anywhere else.
- In deduction, a parameter written `Id<T>` gives `T` from an argument of type `Id<User>`, as `Box<T>` does for a class template. DED-P3 gets a row for this form.
- An explicit alias template is a template of its own as a template template argument (TARG-17, TARG-22). Its name without arguments needs an argument list (TARG-10). It has no class template deduction, no explicit specialization, and no partial specialization (TSP-7).
- The underlying type can depend on a parameter, as in `template <class T> using Tagged = explicit T;`. X20:new-2 is then checked for each specialization. In deduction, a violation is a substitution failure (DED-24).
- An explicit alias that is a member of a class template gives one explicit type for each specialization of the class, as a nested class does.

The other rules of the two template pages need no change in text, only in effect:

| Question | Answer under P17 | Rule |
|---|---|---|
| Are `Box<Meters>` and `Box<double>` the same specialization? | No, two types | TARG-P4 (amended), TARG-19 |
| Does `template <> struct Unit<Meters>` change `Unit<double>`? | No. Today it does, because the two are one type (probe `t6.cpp`) | TARG-19 |
| Can an explicit type be a type argument? | Yes, also one declared in a block | TARG-25 |
| Can a constant parameter have an explicit type? | Yes. TPL-6 gets the item "an explicit type" | TPL-6 (amended) |
| Is `Take<1.0>` valid for `template <Meters M>`? | No. The conversion needs a cast. Both compilers reject the scoped enumeration form today (probe `e3.cpp`) | TARG-12, TARG-13 |
| When are two constant arguments of explicit type the same? | When the values are the same by the row of the underlying type: `0.0` and `-0.0` differ, two pointers to one object are the same | TARG-P4 (amended) |
| Does `template <auto V>` give `Take<Meters{1.0}>` and `Take<1.0>` one specialization? | No. The types differ (TPL-11) | TARG-19 |
| Does an explicit alias of a type parameter count as that parameter in a class template? | No. It names another type | DEP-15 |

TARG-P4's type row becomes "they are the same type, after aliases other than explicit aliases are replaced". A new row says that two explicit types are the same when they come from one explicit alias, or from one explicit alias template with the same arguments. TARG-19 adds that its rule also gives the identity of explicit alias template specializations. In the TPL-P1 table, the alias template row gets a second line, "`using` name `=` `explicit` type: a new explicit type for each argument list". TPL-28 states the same exception.

### 1.6 Identity across translation units, lookup and storage (new rule X20:new-6; amended NAM-27, NAM-28, ODR-7, ODR-12, LNK-1, STG-P7, STG-14)

**Linkage and one definition.** X20:new-6 gives an explicit alias the linkage that a class name has in the same place. At namespace scope its type is one entity in every translation unit, and in an unnamed namespace it belongs to one unit (LNK-5). In a block it has no linkage. Several units can define the same explicit alias, and the definitions must match (ODR-7 and ODR-12 amended, ODR-10 unchanged). `void f(Meters)` and `void f(double)` are two functions with two names at link time. Tidal has no rule that gives a class name its linkage, so the amendment to LNK-1 has no rule to follow (X20-3).

**Argument-dependent lookup.** An argument of explicit type adds the class that the explicit alias is a member of and the innermost namespace that contains it, as an enumeration does (amended NAM-27). NAM-28's item "the namespace that declares a type alias" gets the exception. Today a function next to an ordinary alias is not found by such a call (probe `a1.cpp`, and test `foundations/names/adl-alias-namespace.cpp`). A user-declared `operator*` next to `Meters` must be found, or 1.4 does not work outside its namespace.

**Access through a glvalue.** STG-14 gains the item "the explicit type whose underlying type is `T`, or the underlying type of `T` if `T` is an explicit type, with any cv-qualifiers". STG-P7's table gets the same row. A program can then view an array of `double` as `Meters` values through a reference cast. This follows the signed and unsigned pair, which differs in type and shares a representation. Both compilers today assume that an `enum class E : long` glvalue does not refer to a `long` object (probe `t1.cpp`, section 4.5). The choice for explicit types is the opposite one, and section 1.7 gives the cost.

**Copying and constant evaluation.** An explicit type is a scalar type, so it is trivially copyable (CPY-P6, CPY-28), a literal type and a structural type. The glossary entry for scalar type adds "an explicit type".

### 1.7 Alternatives considered

- **Another place for the keyword.** `explicit using M = double;` and `using explicit M = double;` are free too (probes `s2.cpp`, `s3.cpp`). After the `=`, the keyword sits next to the type it changes, and the alias template form reads the same. Not chosen.
- **A new word, such as `strong` or `newtype`.** A new keyword breaks programs that use the word as a name. A contextual word after `=` would need a lookup rule for a type named `strong`. `explicit` already means "no implicit conversion" for constructors and conversion functions (CTOR-9 in W7). Rejected.
- **The name "opaque alias".** Tidal uses "opaque" for an enumeration declared without its enumerators (ENUM-7, ENUM-18, TYN-P3, the `enum_opaque` production). A second meaning would break the style guide's rule of one meaning for each word (X20-7). Rejected in favor of "explicit alias" and "explicit type".
- **An attribute, such as `[[distinct]]`.** An implementation ignores an attribute it does not recognize (ATTR-1). P17 changes which programs are valid and which functions overload. Rejected.
- **Class types as underlying types.** The member functions of the class take a `this` of the class type, so each member would need a rule for the explicit type. A derived class with inherited constructors gives most of this today, with an implicit conversion to the base. Left for later.
- **`bool`, enumerations and explicit types as underlying types.** An enumeration is already a distinct type, and its enumerators would not be values of the explicit type without a cast. `bool` has only the logical operators, which need the contextual conversion that P17 does not give. An explicit type of an explicit type needs a rule for casts across two levels. Left for later.
- **Function pointers, member pointers and `std::nullptr_t`.** Few operations, and no request in the corpus. Left for later.
- **An implicit conversion from the underlying type.** `run * 2.0` would work with no user operator. `run + some_double` would also work, which defeats the purpose. A literal-only form, as some languages give their untyped constants, does not fit C++, where every literal has a type. Rejected.
- **An implicit conversion to the underlying type.** Library functions such as `sqrt` would take an explicit type. Then `run + t` converts both operands to `double` and is valid. Rejected.
- **A contextual conversion to `bool`.** `if (handle)` would read well. The glossary defines contextual conversion as `bool t(e);`, so the conversion would also make `bool t(m);` valid, which is an implicit way out for one type. Rejected, as ENUM-11 does for scoped enumerations.
- **Unit-aware results,** such as `Meters / Seconds` giving a speed type. That is the work of a units library. P17 gives the parts, and a program can delete `operator/` or declare mixed operators. Rejected.
- **No access across the explicit type and its underlying type,** as both compilers treat an enumeration and its underlying type (probe `t1.cpp`). The optimizer could then keep a `Meters` value in a register across a write to a `double`. A program could not view existing `double` storage as `Meters`, and a reader of STG-14 would expect the signed and unsigned model. Rejected, at the cost of that optimization.
- **A marked scoped enumeration,** such as arithmetic for `enum class Meters : long {}`. The enumeration idiom exists already (probe `e1.cpp`). But an enumeration has only integer underlying types, and floating-point and pointer types are common uses. Rejected.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X20:new-1 | new | none | required | An alias declaration with `explicit` declares a new explicit type with the size, alignment, values, representation and operations of its underlying type. It is a scalar type and no other kind of type (1.2). | none |
| X20:new-2 | new | none | reject | The underlying type is a pointer to an object type, or an arithmetic type except `bool`, with no top-level cv-qualifiers. A dependent underlying type is checked for each specialization (1.2, 1.5). | none |
| X20:new-3 | new | none | reject | No implicit conversion to or from an explicit type. Direct-list-initialization without narrowing, a cast to or from the underlying type, and `{}` are the only conversions. No contextual conversion to `bool`. An ellipsis argument is not promoted (1.3). | none |
| X20:new-4 | new | none | required | The built-in candidates of the table in 1.4, with the value computed on the underlying values and converted back (1.4). | none |
| X20:new-5 | new | none | required | Each specialization of an explicit alias template is an explicit type, identified by its arguments, never replaced, and deducible like a class template specialization (1.5). | none |
| X20:new-6 | new | none | required | An explicit alias has the linkage of a class name in its place, and several units can define it under ODR-10 (1.6). | none |
| `alias_decl` syntax (type-names.md) | amended | syntax | syntax | An optional `explicit` after `=`. | none |
| TYN-P1 | amended | principle | principle | An explicit alias makes a new type and a new overload. | none |
| TYN-1 | amended | required | required | "does not make a new type or a new overload" gets the exception for an explicit alias. | none |
| TYN-2 | amended | reject | reject | An explicit alias must not be declared again in its scope, also for the same type. | none |
| TYN-8 | amended | reject | reject | The second item does not apply to an explicit alias. | none |
| TPL-P1 | amended | principle | principle | The alias template row of the table gets the explicit form. | none |
| TPL-28 | amended | required | required | A specialization of an explicit alias template is a new type (X20:new-5). | none |
| TPL-6 | amended | required | required | An explicit type is a structural type (X20-9). | none |
| TARG-P4 | amended | principle | principle | The type row keeps explicit aliases. A new row gives the identity of explicit types, and constant values of explicit type compare by the row of the underlying type. | none |
| TARG-19 | amended | required | required | Also gives the identity of explicit alias template specializations. | none |
| DED-P3 | amended | principle | principle | A row for `Id<T>` with `Id` an explicit alias template (X20-1). | none |
| OVL-P4 | amended | principle | principle | A set letter for explicit types and the rows of 1.4. | none |
| OVL-9 | amended | required | required | Lists the candidates of X20:new-4. | none |
| OPR-2 | amended | reject | reject | A non-member operator function can have a parameter of explicit type in place of a class or enumeration. | none |
| OPR-6 | amended | required | required | An operand of explicit type selects operator functions too. | none |
| NAM-27 | amended | required | required | A row for an explicit type, as for an enumeration. | none |
| NAM-28 | amended | reject | reject | The type alias item excludes an explicit alias. | none |
| CAST-1 | amended | required | required | `static_cast` between an explicit type and its underlying type. | none |
| STG-P7 | amended | principle | principle | A table row for an explicit type and its underlying type. | none |
| STG-14 | amended | required | required | A glvalue of the explicit type or of the underlying type can access an object of the other (X20-2). | none |
| ODR-7 | amended | required | required | Lists explicit aliases with classes and enumerations. | none |
| ODR-12 | amended | required | required | Lists explicit alias templates. | none |
| LNK-1 | amended | required | required | An explicit alias at namespace scope has external linkage (X20-3). | none |
| TYN-10 | unchanged, state effect | reject | reject | A class key with the name of an explicit alias is not valid. | none |
| TSP-7 | unchanged, state effect | reject | reject | An explicit alias template has no explicit or partial specialization. | none |
| TARG-10 | unchanged, state effect | reject | reject | An explicit alias template name needs an argument list. | none |
| TARG-12 | unchanged, state effect | required | required | No conversion in its list reaches an explicit type from another type. | none |
| TARG-13 | unchanged, state effect | reject | reject | The "explicit conversion" item rejects `Take<1.0>` for a parameter of type `Meters`. | none |
| TARG-17 | unchanged, state effect | required | required | An explicit alias template is a valid template template argument. | none |
| TARG-22 | unchanged, state effect | required | required | An explicit alias template is a template of its own; it never passes through to a class template. | none |
| TARG-25 | unchanged, state effect | required | required | An explicit type, also one in a block, is a valid type argument. | none |
| DED-10 | unchanged, state effect | required | required | The match compares explicit types by identity. | none |
| DED-24 | unchanged, state effect | required | required | A specialization with an underlying type that X20:new-2 rejects is a substitution failure. | none |
| DEP-15 | unchanged, state effect | required | required | An explicit alias of a type parameter does not count as that parameter. | none |
| ENUM-8 | unchanged, state effect | required | required | The model for direct-list-initialization of an explicit type. | none |
| ENUM-10 | unchanged, state effect | required | required | The model for a distinct type with an underlying type. | none |
| ENUM-11 | unchanged, state effect | reject | reject | The model for no contextual conversion to `bool`. | none |
| ENUM-17 | unchanged, state effect | reject | reject | An explicit type is not an integer type, so it cannot be a fixed underlying type. | none |
| ENUM-19 | unchanged, state effect | reject | reject | The model for rejecting `= {v}` and narrowing. | none |
| ENUM-20 | unchanged, state effect | reject | reject | `using enum` with an explicit alias is rejected: it names no enumeration. | none |
| OVL-10 | unchanged, state effect | required | required | A user function with the parameter types of an explicit-type candidate replaces it. | none |
| OVL-11 | unchanged, state effect | reject | reject | A class that converts to an explicit type reaches its candidates. | none |
| CAST-6 | unchanged, state effect | required | required | No pointer conversion for an explicit type; the reference item applies. | none |
| CONV-20 | unchanged, state effect | required | required | No context converts to or from an explicit type. | none |
| COND-3 | unchanged, state effect | required | required | Two operands of one explicit type give that type; an explicit type and its underlying type have no common type. | none |
| IF-4 | unchanged, state effect | reject | reject | A condition of explicit type is rejected. | none |
| SW-2 | unchanged, state effect | reject | reject | A `switch` condition of explicit type is rejected. | none |
| BF-2 | unchanged, state effect | reject | reject | A bit-field of explicit type is rejected. | none |
| STG-23 | unchanged, state effect | undefined | undefined | Access across two explicit types with one underlying type stays undefined. | none |
| CPY-28 | unchanged, state effect | required | required | An explicit type is scalar and so trivially copyable. | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | Two definitions of an explicit alias must have the same tokens (X20-4). | none |
| TRY-2 | unchanged, state effect | required | required | A handler for the underlying type does not catch an explicit type, and the reverse. | none |

Counts: 57 changes. 6 new, 23 amended (1 of them a Syntax segment), 28 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome. The glossary needs two new entries (explicit alias, explicit type) and an amended "scalar type". Glossary entries are not rules and are not counted.

Every program that P17 affects declares an explicit alias, and no valid program does that today. A grep of `tests/` for `= explicit` after `using`, for `explicit using` and for `using explicit` finds no file. So no existing test changes. The tests nearest the boundary keep their outcome:
- `declarations/type-names/typedef-same-type.cpp` and `alias-overload-same.cpp`: plain aliases stay transparent.
- `templates/declarations/alias-template.cpp` and `alias-template-deduce.cpp`: plain alias templates still give the aliased type.
- `templates/arguments/same-specialization-alias-template.cpp`: TARG-22 is unchanged.
- `foundations/names/adl-alias-namespace.cpp`: a plain alias still adds nothing.
- `classes/operators/nonmember-no-class-param.cpp`: `int operator+(int, int)` is still rejected.
- `foundations/storage/access-signed-counterpart-and-cv.cpp`: unchanged.

The proposal needs these new tests.

Under `tests/declarations/type-names/`:
- `explicit-alias-distinct.cpp` (accept; `Meters` and `double` differ, two explicit aliases of `double` differ, size and alignment equal),
- `explicit-alias-overload.cpp` (run; `f(Meters)` and `f(double)` are two functions),
- `explicit-alias-redeclared.cpp` (reject; TYN-2),
- `explicit-alias-bad-underlying.cpp` (reject; `bool`, an enumeration, a class),
- `explicit-alias-pointer-incomplete.cpp` (accept),
- `explicit-alias-conversions.cpp` (accept; brace, cast and functional forms) and `explicit-alias-no-implicit.cpp`, `explicit-alias-paren-init.cpp`, `explicit-alias-narrowing.cpp`, `explicit-alias-condition.cpp` (reject).

Under `tests/foundations/overload-resolution/`:
- `builtin-explicit-arithmetic.cpp` (accept; result types of the table in 1.4, `short` converts back),
- `builtin-explicit-mixed.cpp` (reject; `Meters + double`),
- `builtin-explicit-pointer.cpp` (run; `*`, `->`, `[]`, `+`, difference),
- `builtin-explicit-deleted.cpp` (reject; a deleted `operator/` wins).

Under `tests/templates/arguments/`:
- `explicit-alias-specializations.cpp` (accept; `Box<Meters>` and `Box<double>` differ; `Id<User>` and `Id<Order>` differ; `Id<User>` named twice is one type),
- `explicit-alias-constant-parameter.cpp` (accept) and `explicit-alias-constant-needs-cast.cpp` (reject),
- `explicit-alias-template-template.cpp` (accept).

Under `tests/templates/deduction/`: `explicit-alias-template-deduce.cpp` (accept; `Tag` from `Id<User>`).

Under `tests/foundations/names/`: `adl-explicit-alias.cpp` (run). Under `tests/foundations/odr/`: `explicit-alias-two-units.cpp` (run). Under `tests/foundations/storage/`: `access-explicit-underlying.cpp` (run).

## 3. Interactions

**Type names (TYN-P1, TYN-1, TYN-2, TYN-8, TYN-10).** P17 changes the first sentence of the page's model. An alias is a second name for a type, except an explicit alias, which defines a type. The rest of the page applies with "class" in place of "alias" for redeclaration. The elaborated forms of TYN-4 and TYN-10 do not reach an explicit alias. The page's word "opaque" for enumerations made the roadmap's name for P17 unusable (X20-7).

**Template arguments (TARG-P4, TARG-10, TARG-12, TARG-13, TARG-17, TARG-19, TARG-22, TARG-25).** The page needs two changes, both in TARG-P4's table, and a sentence in TARG-19. The rest works unchanged, because an explicit type takes no implicit conversion and TARG-13 already rejects a conversion that needs a cast. The page's grid has no column for a parameter of scoped enumeration type, the nearest model (X20-8).

**Template declarations, specialization and deduction (TPL-P1, TPL-6, TPL-28, TSP-7, DED-P3, DED-10, DED-24, DEP-15).** An explicit alias template is an alias template by its syntax and a class template by its identity. TPL-6 and the glossary list structural types in two places (X20-9). The deduction page does not say how a plain alias template behaves in a parameter, so the contrast that P17 draws has no rule on one side (X20-1).

**Enumerations (ENUM-8, ENUM-10, ENUM-11, ENUM-17, ENUM-19, ENUM-20).** A scoped enumeration with a fixed type is the closest existing model, and P17 copies its conversions. The enumerations page does not test the parenthesized form that P17 copies (X20-6). The two kinds of type part in two places. An explicit type has built-in arithmetic and no enumerators. Access through the underlying type is defined for an explicit type and undefined for an enumeration (X20-2).

**Overload resolution and operators (OVL-P4, OVL-9, OVL-10, OVL-11, OPR-2, OPR-6).** The candidate model carries the operations with no new mechanism. OVL-10 gives the way to remove an operation.

**Names (NAM-27, NAM-28).** An explicit type takes the enumeration row.

**Casts and conversions (CAST-1, CAST-6, CONV-20, COND-3, IF-4, SW-2, BF-2, TRY-2).** Only CAST-1 changes. The other rules reject an explicit type through their existing lists of types. Tidal has no rule for an argument that matches an ellipsis, so P17 states its own (X20-5).

**Storage and copying (STG-P7, STG-14, STG-23, STG-12, CPY-28).** One new row in STG-14. Byte copies and trivially copyable status follow from the scalar classification.

**Linkage and the one-definition rule (LNK-1, LNK-5, ODR-7, ODR-10, ODR-12).** An explicit alias needs the linkage of a class name, which Tidal does not state (X20-3). ODR-10 does not say how a plain type alias takes part in a match (X20-4).

**Not touched.** Arithmetic on the fundamental types, the usual arithmetic conversions, enumerations, classes, constant evaluation and lambda closure types keep today's rules.

## 4. Impact on existing programs

The claims below come from the rules, from compiler runs, and from a count over `tests/programs/`. Clean-room readers wrote those 67 programs (80 files, 7712 lines) from the specification, so they show how such readers write code. They are not a sample of code in use.

### 4.1 Rejected becomes valid

- `using X = explicit T;` at namespace scope, in a class and in a block, and `template <...> using X = explicit T;`. Both compilers reject each form today (probes `s1.cpp`, `s4.cpp`, `s5.cpp`).
- A non-member operator function whose only parameters have explicit types. The form is new, because no explicit type exists today.

### 4.2 Undefined becomes defined

None for an existing program. Amended STG-14 defines access only between an explicit type and its underlying type, and no object has an explicit type today.

### 4.3 Valid programs that change meaning

None. Only explicit aliases are affected, and no valid program declares one.

A test of validity cannot change its answer for an existing type, because no existing type is an explicit type. A program that adopts P17 changes some answers on purpose. These cases come from the corpus:
- `W15/largest.cpp` uses `struct Meters { double v; }` as a type that its `Arithmetic` concept refuses. That concept lists the fundamental types by name, so `using Meters = explicit double;` is refused too. A concept written as `requires (T a, T b) { a < b; }` would accept the explicit type and refuse the class.
- `W7/money.cpp` keys `Money` by a currency, and about 30 lines of class text give the operators. `template <Currency C> using Money = explicit long long;` gives `+`, `-`, unary `-`, the comparisons and the compound assignments, and keeps `Usd + Eur` and `Usd + int` invalid. The program still declares scaling by `long long` as an operator template, which deduces `C` from `Money<C>` by X20:new-5. Two changes in meaning follow. `Money<C> * Money<C>` and `Money<C> / Money<C>` become valid unless the program deletes them. The member function `currency()` has no place, because an explicit type has no members.

### 4.4 Valid programs that become invalid

None. Each new rejection applies to an explicit type.

### 4.5 Evidence

Corpus counts over `tests/programs/` (grep):

| Construct | Count | Changes under P17 |
|---|---|---|
| alias declarations `using X = ...;` | 44, of which 15 declare `size_t` | none |
| `typedef` | 0 | none |
| alias templates at namespace scope | 3 (`push_front`, `concat`, `Common`) | none |
| `explicit` after the `=` of an alias | 0 | none |
| scoped enumerations | 15; 1 has no enumerators and a fixed type (`align_val_t`, the library shape that LSUP-5 needs) | none |
| classes with one scalar data member | 5: 3 exception payloads, W10's `Cheap`, and W15's `Meters` | none; `Meters` could adopt P17 |
| class templates keyed by a tag around one scalar | 1 (W7's `Money`) | none; could adopt P17 |
| aliases of fixed-width integers (`u8`, `u16`, `u32` twice, `Word`) | 5 | none; P17 would make literals need casts there |

Compiler probes, all with g++ 16.2.1 and clang++ 22.1.8:
- `s1.cpp` to `s6.cpp`: `explicit` after `=` in an alias, before `using`, after `using`, in an alias template, in a member alias, and in a `typedef`. Both compilers reject all six.
- `a1.cpp`: `Box<Meters>` is `Box<double>` for a plain alias. A partial specialization for `T*` matches a plain alias of `int*`. A call deduces through `template <class T> using Tag = T;`. A call `f(i)` with `i` of a namespace member alias type does not find `n::f`.
- `a2.cpp`: `f(double)` and `f(Meters)` for a plain alias are one function. Both compilers report a redefinition.
- `e1.cpp` (run): for `enum class Meters : long {}`, `Box<Meters>` and `Box<long>` differ, size and alignment equal `long`, `Meters m{5}` is valid, a user `operator+` works, `Take<Meters{3}>` is valid, and `f(Meters)` and `f(long)` overload. Both print `10 1 2`.
- `e2.cpp`, `e3.cpp`, `e5.cpp`: the same enumeration has no `+`, takes no `3` as an argument for a `Meters` parameter, and takes no `1` in copy-initialization. Both reject each.
- `e4.cpp`: a deleted `operator==(E, E)` for an unscoped enumeration is chosen over the built-in candidate. Both reject the use.
- `t1.cpp` (run): a function writes `1` through a `long*`, then `E{2}` through an `E*` to the same object, with `enum class E : long`, and returns the `long`. At `-O0` both return 2. At `-O2` both return 1. The same function with `unsigned long*` returns 2 at both levels.
- `t2.cpp`: with `template <class Tag> using Id = int;`, `Id<User>` and `Id<Order>` are one type, and `which(Id<User>{})` for `template <class Tag> int which(Id<Tag>)` fails deduction on both.
- `t3.cpp`, `t4.cpp`: for `enum class E : int {}`, `E e(1);` is rejected by both, and `E(1)`, `static_cast<E>(2)` and `E g{3}` are accepted by both.
- `t5.cpp`: a class template keyed by a tag deduces the tag. An alias template whose type ignores its parameter is a valid template template argument on both.
- `t6.cpp`: `template <> struct Unit<Meters>` for a plain alias of `double` is the specialization for `double`. A second one for `double` is a redefinition on both.
- `v1.cpp` (run, `-Wall`): a `float` argument for an ellipsis reads back as `double`, a `short` as `int`, and an `enum class E : short` value as `E`. Neither compiler warns. Both print `1.5 9 7`.

## 5. Findings

**X20-1** (missing; templates/deduction.md, templates/declarations.md; DED-P3, DED-13, TPL-28). The deduction page never mentions alias templates. TPL-28 says a specialization of an alias template is the aliased type, and `alias-template-deduce.cpp` shows a call that deduces `T` through `Pointer<T>`. No rule says what happens to a parameter of an alias template that the aliased type does not use. With `template <class Tag> using Id = int;`, a parameter `Id<Tag>` gives `Tag` no value, and both compilers reject the call (probe `t2.cpp`). Suggestion: a row in DED-P3's table of positions that give no value, "a parameter of an alias template that the aliased type does not use", citing TPL-28, and a reject test.

**X20-2** (missing; foundations/storage.md; STG-14, STG-23, STG-P7, ENUM-10). STG-14 and STG-23 do not mention an enumeration and its underlying type. The two share a representation, as a signed and unsigned pair does, and a reader can expect the same row. Both compilers assume at `-O2` that a write through `enum class E : long` does not change a `long` object (probe `t1.cpp`). The function returns 1 at `-O2` and 2 at `-O0`. So STG-23 applies, and nothing on the page says so. Suggestion: an item in STG-23, "an enumeration is not listed for its underlying type, or the reverse", and an Evidence line with the `-O2` result. A sanitizer does not see this case, so the test can be a `run` test that accepts either value, as `inline-same-tokens-different-meaning.cpp` does for ODR-10.

**X20-3** (missing; declarations/storage-linkage.md; LNK-1, LNK-5, STO-P3, TYN-3, ODR-7). LNK-1 gives external linkage to "a variable or function at namespace scope". No rule gives a class or an enumeration its linkage. LNK-5 gives internal linkage to "each name in an unnamed namespace", which includes classes, and TYN-3 speaks of a class's "name for linkage". So the pages imply that classes have linkage and do not say which. P17 needs to give an explicit alias "the linkage of a class name" and has no rule to cite. Suggestion: LNK-1 says "a variable, function, class or enumeration", or a separate rule for types, with a test of a class in an unnamed namespace used in a function signature in two units.

**X20-4** (unclear; foundations/odr.md; ODR-10, ODR-7, ODR-12, TYN-1). ODR-10 asks that "each name in them refers to the same entity". A type alias names a type and is not a kind of entity in ODR-7 or ODR-12. The page does not say whether an `inline` function that uses `Count` matches across two units where `Count` is `int` in one and `long` in the other. No ODR test uses an alias (a grep of `tests/foundations/odr/` for `using` and `typedef` finds none). Suggestion: ODR-10 says that a type alias refers to the type that it names, so the definitions match only if those types are the same, with a two-unit `run` test in the form of `inline-same-tokens-different-meaning.cpp`.

**X20-5** (missing; expressions/postfix.md; PFX-6, PFX-15, CONV-20). No rule gives the conversions of an argument that matches an ellipsis parameter. PFX-15 covers only a class with a non-trivial copy, move or destructor. Both compilers pass a `float` as `double`, a `short` as `int`, and a scoped enumeration with its own type (probe `v1.cpp`). CONV-20 does not list the ellipsis as a context either. P17 had to state its own rule for explicit types with nothing to cite. Suggestion: a `required` rule on the postfix page that lists the conversions for an ellipsis argument (integer promotion, `float` to `double`, array and function to pointer, no promotion of a scoped enumeration), and a `run` test for each.

**X20-6** (needs example; declarations/enumerations.md; ENUM-3, ENUM-8, ENUM-16, ENUM-19). ENUM-3 allows a conversion "with an explicit cast", ENUM-8 allows `F f{5};`, and ENUM-19 rejects `F f = {5};`. No rule or test covers `F f(5);`, which both compilers reject, or `F(5)`, which both accept as a cast (probes `t3.cpp`, `t4.cpp`). A reader can take the parenthesized form for direct initialization, which for other types does what a cast does (CAST-1, first item). Suggestion: ENUM-16 adds "initialize a scoped enumeration from an integer with parentheses", with a reject test, and an accept test for the functional form under ENUM-3.

**X20-7** (unclear; declarations/enumerations.md, declarations/type-names.md, glossary; ENUM-7, ENUM-18, ENUM-P2, ENUM-20, TYN-P3, TYN-6, the `enum_opaque` production). "Opaque" names an enumeration declaration without its enumerators. The glossary uses "opaque enumeration declaration" inside another entry (line 113) and has no entry of its own. The designer's portfolio names P17 "opaque type aliases", which gives the word a second meaning. The style guide asks for one meaning for each word. Suggestion: a glossary entry for "opaque enumeration declaration", and a note in the roadmap that P17's term is "explicit alias" or another word.

**X20-8** (needs example; templates/arguments.md; TARG-P3, TARG-12, TARG-13). The probe grid in Evidence has a row for a scoped enumerator and no column for a parameter of scoped enumeration type. `constant-scoped-enum.cpp` tests a scoped enumerator for an `int` parameter. No test gives an integer for a scoped enumeration parameter, which both compilers reject (probe `e3.cpp`), or an enumerator of another enumeration. Suggestion: a column for a scoped enumeration parameter in `tests/templates/arguments/grids/`. Only its own enumerators, a braced value of its type, and the `auto` column accept.

**X20-9** (hard to find; templates/declarations.md, glossary; TPL-6, structural type, scalar type). TPL-6 lists the structural types item by item. The glossary entry says "Scalar types and lvalue references are structural". The two lists agree today. A change to the set of scalar types, such as P17's, must change both, and TPL-6 does not cite the glossary. Suggestion: TPL-6's first five items become "a scalar type", citing the glossary, so one list holds.

**X20-10** (outside knowledge). These come from general experience:
- the motivation, that distinct numeric types prevent unit mix-ups and that other languages give a type with the representation of another and no implicit conversion,
- the term "untyped constant" for literals that take the type of their context, used in 1.7,
- the expectation that compilers give a distinct link name to a function whose parameter has a distinct type.

**X20-11** (outside the spec; Q-0007, roadmap-5 section 1.6). Two needs fall outside the spec:
- Version marking. P17 takes the place after `=` in an alias declaration. A designer cannot tell from Tidal whether a later version gives that place, or the phrase "explicit alias", a meaning.
- Cost. X20:new-4 adds a candidate family to overload resolution, X20:new-5 adds an identity rule to every template-id of an explicit alias template, and amended STG-14 removes one optimization. Tidal has no model to state these costs in (Q-0183).
