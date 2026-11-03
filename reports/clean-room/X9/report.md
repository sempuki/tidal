# X9: P8, relocation

Date: 2026-11-03. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md, and section 1 of `documentation/style-guide.md`.
- Section 4 of `documentation/exercises.md`.
- `reports/clean-room/X8/report.md`, as the model for this report's form.
- P8 in `reports/clean-room/roadmap/designer.md`. From `reports/clean-room/roadmap-3/designer.md`: item 2, the P8 row of section 2, the D21 row of section 1.5, sections 1.6 and 1.7, and item 8.
- All of `foundations/storage.md`, `foundations/objects.md`, `foundations/types.md`, `classes/virtual.md`, `classes/copy-move.md` and `exceptions/unwinding.md`.
- These parts of `classes/constructors.md`: the principles, the Syntax, the destructor rules and the Evidence (CTOR-P1 to CTOR-P6, DTOR-1 to DTOR-9, CTOR-6, CTOR-23).
- The cited rules of these pages: members (CLS-17, CLS-19, CLS-10, Evidence), keywords (KW-P1, KW-1 to KW-4), blocks (STMT-6, STMT-1), jumps (JMP-P1, JMP-P2, JMP-6, JMP-9), switch (SW-5), unions (UNI-3, UNI-5), storage-linkage (STO-P1, STO-1, STO-2), values (VC-P4, VC-3 to VC-7, VC-10), lambdas (LAM-4), deleted (DEL-P2, DEL-1, DEL-2, DEL-4), exception-specifications (EXS-P1, EXS-4), constant-evaluation (CE-P1, CE-2, CE-5), evaluation-order (SEQ-1, SEQ-2), overload-resolution (OVL-P2, OVL-14, OVL-17), derived (DER-8), declarators (DCL-P1, DCL-13), definitions (FN-P1), new-delete (NEW-5), language-support (LSUP-7), return-deduction (RET-6), and the `unary_expr` production of unary.md.
- The glossary entries "trivial", "eligible" and "implicit-lifetime type". Searches of the specification for "scalar type", "special member" and "dynamic type".
- Q-0304 to Q-0308, Q-0323, Q-0324 and Q-0054.
- The tests that CPY-15, CPY-19, STG-9, STG-21, DTOR-7 and OBJ-3 cite. A search of `tests/` for constructors that take their own class by value and for the name `relocate`.

Compiler runs: small programs built with g++ and clang++ (`-std=c++23 -pedantic-errors`) under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X9/probes`, to see what programs do today. Two run programs also ran under `-fsanitize=address,undefined`. The report gives the results where they support a claim.

Totals: 90 rule changes (13 new, 30 amended, 47 unchanged but must state their effect). One existing test changes its expected outcome (`classes/copy-move/copy-ctor-by-value.cpp`, from reject to accept). 15 findings, X9-1 to X9-15.

## 1. Design

### 1.1 The feature

Today a program can end an object early only by an explicit destructor call (DTOR-6). If the object is a variable, the block then destroys it a second time, and the behavior is undefined (DTOR-7, STG-21). So a program that moves a local into a new object leaves a moved-from object behind, and the block still runs that object's destructor. A growable array that moves its elements calls the move constructor and then the destructor of each old element.

P8 adds *relocation*. A relocation makes a new object from an old one and ends the old one without running its destructor. If the old object is a local variable, the block then skips it.

```cpp
struct Buffer {
  int* data;
  explicit Buffer(int n) : data(new int[n]()) {}
  Buffer(Buffer&& o) noexcept : data(o.data) { o.data = nullptr; }
  Buffer(Buffer src) = default;       // X9: a relocation constructor
  ~Buffer() { delete[] data; }
};

void store(Buffer b);

Buffer make() {
  Buffer b(16);
  return relocate b;                  // b ends here; ~Buffer does not run for b
}

void keep_if(bool keep) {
  Buffer b(8);
  if (keep) store(relocate b);
}                                     // the block destroys b only if keep is false
```

The design has three parts:
- a *relocation constructor* that a class declares (1.2 to 1.4),
- a *relocation expression* `relocate e` (1.5 and 1.6),
- a rule for variables that a relocation has ended (1.7).

Relocation never copies bytes. It always runs a constructor, so it needs neither "trivially copyable" (CPY-19) nor "standard-layout" (CLS-19). X9-1 records what a design that copies bytes would need.

### 1.2 The relocation constructor (new rules X9:new-1, X9:new-2; amended CPY-15, CPY-P1, copy-move Syntax)

A *relocation constructor* of class `T` is a constructor that is not a template and whose only parameter has type `T`. Top-level cv-qualifiers on the parameter do not count.

```cpp
struct S {
  S(S src);              // a relocation constructor
};
```

CPY-15 rejects this form today, and both compilers reject it (`copy-ctor-by-value.cpp`, and probe p9 for `= default`). So the space is free. CPY-15 still rejects a constructor whose first parameter is `T` and whose other parameters all have default arguments (probe p6).

The parameter does not make a new object. Inside the constructor, `src` names the source object, as an lvalue of type `T`. The constructor initializes the new object, and its completion ends the source (1.4).

X9:new-2 gives the limits:
- The constructor has exactly one parameter, with no default argument.
- It is not `explicit`. It is not a template. (CPY-15 already says a template never has this form.)
- It is non-throwing. A `noexcept` specifier whose value is false is not valid.
- A relocation constructor defaulted after its class definition must not be one that X9:new-4 deletes. This follows CPY-17.

A relocation constructor is a special member function. It is neither a copy constructor nor a move constructor (CPY-1, CPY-2), so declaring one removes no implicit copy or move function (CPY-4, CPY-5).

A relocation constructor is never a candidate in overload resolution (amended OVL-P2 and OVL-14). `S b(a);` still calls the copy constructor when `S` declares `S(S)`. Only a relocation calls a relocation constructor (1.6). A relocation constructor is not inherited (amended DER-8).

### 1.3 Implicit and defaulted relocation (new rules X9:new-3, X9:new-4, X9:new-5)

A class gets an implicit relocation constructor when both of these hold:
- The class has no user-declared relocation constructor.
- CPY-5 gives the class an implicit move constructor. So the class declares no copy function, no move function and no destructor.

A class with a user-declared copy function, move function or destructor gets no implicit relocation constructor. It can write `S(S src) = default;` to opt in. P8 described this opt-in.

The implicit or defaulted relocation constructor relocates each part in the order of CTOR-6: the bases, then the members. Each part is relocated by the relocation of its type (X9:new-7, 1.6):
- A scalar part gets the value of the source part.
- An array part is relocated element by element, starting from the first.
- A reference member refers to the object that the source member refers to.
- A part of class type uses its own relocation.

Each source part ends when its relocation completes. The implicit function is non-throwing (amended EXS-4) and counts as `constexpr` (amended CE-2).

X9:new-4 deletes the implicit or defaulted relocation constructor in the cases below. They follow CPY-7 and UNI-3:
- A part of class type has a relocation constructor that is deleted or not accessible.
- A part of class type has no relocation constructor, and the constructor that an xvalue of that type selects (CPY-9) is deleted, not accessible, or not non-throwing.
- A part of class type has no relocation constructor, and its destructor is deleted or not accessible.
- A variant member has a copy constructor or a destructor that is not trivial (CPY-27, DTOR-9). The implementation does not know which member is active, so it cannot relocate the union. A union whose variant members are all trivial is relocated by its copy constructor (UNI-5), and the source union then ends.

A relocation constructor that X9:new-4 deletes steps aside, as a deleted defaulted move function does (CPY-7). A relocation of the class then uses the fallback of 1.6. A relocation constructor that the program deletes by name does not step aside. X9:new-5 rejects a relocation that reaches such a constructor, and also a relocation that reaches a relocation constructor that is not accessible.

### 1.4 What a user-provided relocation constructor does (new rule X9:new-6)

A class whose parts hold the addresses of other parts needs a relocation that fixes those addresses. copy-move.md shows such a class (`Line`, which has a cursor into its own text):

```cpp
struct Line {
  char text[16] = {};
  char* cursor = text;
  Line() = default;
  Line(const Line& o);
  Line& operator=(const Line& o);
  Line(Line src) : cursor(text + (src.cursor - src.text)) {}  // X9
};
```

A user-provided relocation constructor runs these steps:
1. Each base and member that the member initializer list names is initialized as in any constructor.
2. Each base and member that the list does not name is relocated from the matching part of `src` (1.3). Its default member initializer is not used. Its source part ends at that point.
3. The body runs.
4. The parts of `src` that step 2 did not relocate are destroyed, in the reverse of their order. This is the same work as an implicit destructor with an empty body.

The destructor body of `T` never runs for the source. When the relocation constructor completes, the source's lifetime ends and the new object's lifetime begins (amended OBJ-1).

In the `Line` example, step 2 relocates `text` and step 1 sets `cursor`. `src.text` has ended by then, but its address is still valid to use (STG-10). If step 1 copies from a part that step 2 has already relocated, the copy reads an object outside its lifetime (OBJ-2). The parts are built in declaration order (CTOR-6). So the problem arises only when a part reads a later part of `src` that the list does not name.

A delegating relocation constructor initializes the whole object through its target. Step 4 then destroys every part of `src`.

During a relocation constructor, a virtual call on the new object reaches the overriders of `T` (VIRT-6). The source is a complete object of type `T`, so a virtual call through `src` reaches the same overriders. A call that reaches a part that has ended is undefined (STG-22).

### 1.5 The relocation expression (new rules X9:new-8, X9:new-9, X9:new-13; amended `unary_expr` syntax, KW-3)

```text
  unary_expr: ... | "relocate" cast_expr
```

`relocate e` is a prvalue of the type of `e` without cv-qualifiers. Its evaluation relocates the object that `e` refers to into the result object (VC-4). `decltype`, `sizeof` and `noexcept` do not evaluate their operand, so `relocate e` in those operands ends nothing. `noexcept(relocate e)` is true when the relocation is non-throwing.

The operand has two forms (X9:new-9):

| Operand | Form | Limits |
|---|---|---|
| A name of a variable, not in parentheses | tracked (1.7) | The variable has automatic storage duration and is declared in the innermost enclosing function or lambda body. It is not a function parameter, a reference, a structured binding, or a `const` or `volatile` variable. |
| Any other glvalue | untracked | The program makes sure that nothing ends the object again (X9:new-11). |

In both forms the type is a complete scalar or class type. An array or an abstract class (VIRT-8) is not valid. A name of a non-static data member is not valid either, because the destructor of the enclosing object would end the member again. A program that means to relocate a member writes `relocate this->m`.

A function parameter is excluded because the caller ends each parameter object (VC-P4) at a point that the implementation chooses (VC-10). The callee cannot tell the caller to skip the parameter.

`relocate` is not a keyword. X9:new-13 gives the word its meaning only in places where no valid program today gives it a meaning (KW-P1):
- The word stands where a unary expression can begin.
- The next token is an identifier, `*`, or `this`.
- Unqualified lookup of `relocate` at that point finds no declaration.

So `relocate (x)` is never a relocation. A template that calls a function `relocate(t)` by argument-dependent lookup keeps its meaning. A program that declares something named `relocate` keeps that meaning. Probe p10 (`int relocate = 3; ... relocate * x.v`) builds on both compilers today and keeps its meaning. When `relocate` names a type, `T y(relocate x);` stays a function declaration (DCL-13, probe p4).

### 1.6 The relocation of a type (new rule X9:new-7)

To relocate an object of type `T` into a result object, the implementation takes the first of these steps that applies:
1. `T` is a scalar type. The result gets the value of the source, and the source ends.
2. `T` has a relocation constructor that takes part (1.3). The constructor runs with the source as `src`.
3. `T` has no relocation constructor, or only one that X9:new-4 deleted. The *fallback* initializes the result from an xvalue of the source (CPY-9) and then destroys the source with its destructor. If the initialization throws, the source keeps its lifetime and the relocation has not happened.

The fallback lets every movable class be relocated, so generic code needs only one spelling. For a class with a relocation constructor, step 2 never throws (X9:new-2).

With relocation, a growable array moves its elements without a destructor call. For a class without a relocation constructor, the fallback does the same work as a move followed by a destructor call:

```cpp
void grow() {
  T* fresh = static_cast<T*>(::operator new(sizeof(T) * bigger));
  for (int i = 0; i != count; ++i)
    new (fresh + i) T(relocate items[i]);   // untracked: element i ends here
  ::operator delete(items);                  // no ~T call for the old elements
  items = fresh;
}
```

The example depends on STG-28 and Q-0323, which say that the elements of `fresh` exist before they are built. Probe `fb.cpp` writes the fallback by hand in today's language (a move, then an explicit destructor call on the old element). It runs without a report under the address and undefined sanitizers on both compilers.

### 1.7 Variables that a relocation has ended (new rule X9:new-10; amended OBJ-3 and the block-exit rules)

A variable that a tracked relocation has ended is *relocated*. These rules apply:
- When control leaves the block of a relocated variable, by any route, the variable is not destroyed. The routes are the end of the block, `return`, a jump (JMP-9), and unwinding (UNW-1). The other variables of the block end in the reverse of their construction order, as they do today (OBJ-3).
- A jump back to a point before the declaration does not destroy a relocated variable. The declaration then makes a new variable that is not relocated (JMP-9).
- Each pass through a declaration makes a variable that is not relocated. A loop body is included (OBJ-3).

Whether a variable is relocated can depend on the path. In `keep_if` (1.1), `b` is relocated only when `keep` is true. On each path, the implementation records whether the relocation of the variable completed. The program cannot observe how the implementation records it. A fallback that throws leaves the variable not relocated, and unwinding destroys it.

STG-21 keeps its class. The implicit destructor call of STG-21 does not happen for a relocated variable (amended STG-21). X9:new-10 answers the roadmap's D22 (an object whose end depends on the path).

After a relocation, the storage of the variable holds no object. A use of the variable is a use outside its lifetime (OBJ-2, STG-22). If the program creates a new object in that storage with placement `new`, the old name reaches the new object (STG-9). The block does not destroy that new object, because the variable is relocated. The program must end it.

`return relocate x;` initializes the return object from `x`. CPY-12 lets an implementation make a named local the return object. Amended CPY-12 removes that freedom for a variable that is the operand of a relocation expression in the same function. Without the change, the result object could be the source itself.

### 1.8 Untracked relocation (new rule X9:new-11)

An untracked relocation ends an object that another part of the program may end later. The relocation counts as an end of the object for DTOR-7 (amended DTOR-7). So these cases are undefined under existing rules:
- `relocate *p` followed by `delete p`. The program must free the storage with the deallocation function instead.
- `relocate this->m`, followed by the destructor of the enclosing object.
- `relocate *&x` for a variable `x`, followed by the end of the block.

X9:new-11 adds two undefined cases:
- The dynamic type of the operand differs from its static type. A relocation through a `Base&` that refers to a `Derived` would end only the base part. This follows VIRT-12.
- The result object overlaps the storage of the source.

### 1.9 Recommended practice (new rule X9:new-12)

An implementation should warn about these cases:
- a use of a variable that every path to the use has relocated,
- a relocation of a variable that is declared outside the loop that contains the relocation,
- an untracked relocation through a glvalue of a polymorphic class that is not `final`. This follows VIRT-13.

### 1.10 Alternatives considered

- **Relocation by copying bytes.** P8 described this as a second path. It needs complete lists for CPY-19 and CLS-19 and a rule that a byte copy creates an object. X9-1 gives the details. Left for a later proposal.
- **A full keyword `relocate`.** KW-1 now lists the keywords, so the cost is visible. A keyword makes probe p10, which builds today, invalid. Rejected in favor of the lookup rule of X9:new-13.
- **A reserved name, such as a name with two underscores.** KW-4 reserves such names for the implementation, so a language feature with such a name looks like an extension. Rejected.
- **`return x;` relocates when `T` has a relocation constructor.** Many existing classes get an implicit relocation constructor (1.3). For those classes, a relocation would destroy each part of `x` during the return. A move destroys `x` only after the later variables of the block end. The order of destructor calls would change in valid programs. Rejected, and CPY-11 is unchanged.
- **A relocation constructor that ends the source through the source's destructor body.** The destructor body of a resource class releases the resource, so the new object would lose it. Rejected.
- **A user-provided relocation constructor that default-initializes the parts that its list does not name**, as other constructors do. Every user-provided relocation constructor would then have to name every part. Rejected in favor of step 2 of 1.4, so that the class writes only its fix-ups.
- **Relocation of parameters.** The caller ends each parameter object (VC-P4, VC-10). A callee that relocates its parameter would need a new calling rule. Left out. A parameter can still be moved.
- **Relocation of `const` variables.** The source ends anyway, so a `const` source does no harm to the relocation itself. But a placement `new` into the storage afterward would fall under STG-20 and Q-0307. Left out, so that the design does not rest on that provisional answer.
- **A static rule that rejects a use after a relocation.** Such a check needs rules for the flow of control (definite assignment, and paths through `goto` and `switch`), and Tidal has none. Kept as a warning (X9:new-12). A use at run time is undefined under OBJ-2.
- **A relocation constructor that removes the implicit copy and move functions**, as a move function does under CPY-P2. A class that declares a relocation constructor does not say that part-by-part copying is wrong. Rejected.
- **A variable whose destructor is deleted, allowed when every path relocates it.** DTOR-3 rejects such a variable. Allowing it needs the static rule above. Left out.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. Classes are the specification's rule classes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X9:new-1 | new | none | required | The relocation constructor is a constructor, not a template, whose only parameter has type `T`. It is a special member function and neither a copy nor a move constructor. Its parameter names the source (1.2). | none |
| X9:new-2 | new | none | reject | Limits on a relocation constructor: one parameter, no default argument, not `explicit`, not throwing, and not deleted when defaulted after the class definition (1.2). | none |
| X9:new-3 | new | none | required | When a class has an implicit relocation constructor, and what an implicit or defaulted one does, part by part (1.3). | none |
| X9:new-4 | new | none | required | When the implicit or defaulted relocation constructor is deleted, and that a deleted one steps aside (1.3). | none |
| X9:new-5 | new | none | reject | A relocation must not reach a relocation constructor that the program deletes or that is not accessible, or a fallback whose constructor or destructor is not usable (1.3, 1.6). | none |
| X9:new-6 | new | none | required | The steps of a user-provided relocation constructor, and when the source ends (1.4). | none |
| X9:new-7 | new | none | required | The relocation of a type, by one of three steps: scalar, relocation constructor, or fallback (1.6). | none |
| X9:new-8 | new | none | required | `relocate e` is a prvalue whose evaluation relocates the object into the result object. It has no effect in an unevaluated operand (1.5). | none |
| X9:new-9 | new | none | reject | The operand forms and their limits (1.5). | none |
| X9:new-10 | new | none | required | No exit from its block destroys a relocated variable. Each pass through the declaration makes a new variable (1.7). | none |
| X9:new-11 | new | none | undefined | An untracked relocation is undefined if the operand's dynamic type differs from its static type, or if the result overlaps the source (1.8). | none |
| X9:new-12 | new | none | recommended | Warnings for a use after relocation, a relocation in a loop of a variable declared outside it, and an untracked relocation through a polymorphic class (1.9). | none |
| X9:new-13 | new | none | required | When the word `relocate` is the relocation operator: at the start of a unary expression, before an identifier, `*` or `this`, when lookup finds no declaration (1.5). | none |
| OBJ-P1 | amended | principle | principle | An object also goes out of existence when a relocation of it completes. | none |
| OBJ-P2 | amended | principle | principle | The row for automatic objects says that a relocation can end one first, and the block then skips it. | none |
| OBJ-P4 | amended | principle | principle | The list gains "a variable after a relocation". | none |
| OBJ-1 | amended | required | required | The lifetime also ends when a relocation of the object completes. | none |
| OBJ-2 | amended | undefined | undefined | The list gains a use of a variable or object after a relocation. X9:new-11 joins the specific cases. | none |
| OBJ-3 | amended | required | required | A relocated variable is not destroyed when control leaves its block (X9:new-10). | none |
| STO-1 | amended | required | required | "The object ends when control leaves the block" gains "unless a relocation ended it first". | none |
| JMP-P2 | amended | principle | principle | The first two rows of the table skip a relocated variable. | none |
| JMP-9 | amended | required | required | A jump out of a block, or back past a declaration, does not destroy a relocated variable. | none |
| UNW-P1 | amended | principle | principle | Unwinding skips a relocated variable, as any other exit does. | none |
| UNW-1 | amended | required | required | The automatic objects that unwinding destroys exclude relocated variables. | none |
| DTOR-7 | amended | undefined | undefined | A relocation counts as the first end for "destroys an object a second time". | none |
| CTOR-P1 | amended | principle | principle | A relocation constructor begins one object and ends another. | none |
| CPY-P1 | amended | principle | principle | The sentence "A constructor that takes the class by value cannot exist" becomes a pointer to X9:new-1 (see X9-8). | none |
| CPY-P2 | amended | principle | principle | The implementation also writes a relocation constructor under the conditions of CPY-5. A relocation constructor removes no other function. | none |
| CPY-15 | amended | reject | reject | A constructor with exactly one parameter of type `T` is a relocation constructor and is valid. The rule still rejects `T` as the first of several parameters with default arguments. | `classes/copy-move/copy-ctor-by-value.cpp` (reject to accept) |
| CPY-12 | amended | unspecified | unspecified | A variable that is the operand of a relocation expression is not made the return value. | none |
| DEL-P2 | amended | principle | principle | The relocation constructor is among the functions that `= default` can ask for. | none |
| DEL-4 | amended | reject | reject | Lists the relocation constructor among the special member functions. | none |
| EXS-4 | amended | required | required | A relocation constructor is non-throwing. | none |
| CE-2 | amended | reject | reject | An implicit or defaulted relocation constructor counts as `constexpr`, with the same exception for a virtual base. | none |
| OVL-P2 | amended | principle | principle | A relocation constructor joins the list of functions that are never candidates. | none |
| OVL-14 | amended | required | required | "every constructor of `T`" excludes the relocation constructor. | none |
| DER-8 | amended | required | required | A relocation constructor is not inherited. | none |
| UNI-3 | amended | required | required | A variant member with a non-trivial copy constructor or destructor deletes the union's relocation constructor. | none |
| STG-P4 | amended | principle | principle | The last sentence says that the block destroys each variable that is not relocated. | none |
| STG-21 | amended | undefined | undefined | The implicit destructor call does not happen for a relocated variable (X9:new-10). | none |
| KW-3 | amended | required | required | Names `relocate` among the words with a special meaning and points to X9:new-13. | none |
| `unary_expr` syntax (unary.md) | amended | syntax | syntax | Adds `"relocate" cast_expr`. | none |
| copy-move Syntax | amended | syntax | syntax | Adds `reloc_ctor: class_name "(" [cv] class_name [identifier] ")"`. | none |
| OBJ-7 | unchanged, state effect | required | required | The new object has its own address. The source's address does not move. | none |
| OBJ-10 | unchanged, state effect | undefined | undefined | A relocation into the storage of another object reuses that storage, as reuse does today. | none |
| STO-P1 | unchanged, state effect | principle | principle | Only automatic variables are tracked. Static and thread variables cannot be relocated by name. | none |
| DTOR-3 | unchanged, state effect | reject | reject | A variable of a class with a deleted destructor stays invalid, even if every path relocates it. | none |
| DTOR-5 | unchanged, state effect | required | required | "The variable constructed last is destroyed first" holds among the variables that are not relocated. | none |
| DTOR-6 | unchanged, state effect | required | required | The model for an end that keeps the storage. | none |
| CTOR-6 | unchanged, state effect | required | required | Gives the order of the parts that a relocation constructor relocates. | none |
| CPY-1 | unchanged, state effect | required | required | A relocation constructor is not a copy constructor. | none |
| CPY-2 | unchanged, state effect | required | required | A relocation constructor is not a move constructor. | none |
| CPY-4 | unchanged, state effect | required | required | A user-declared relocation constructor does not delete the implicit copy functions. | none |
| CPY-5 | unchanged, state effect | required | required | Its conditions also decide whether a class has an implicit relocation constructor. | none |
| CPY-7 | unchanged, state effect | required | required | The pattern for X9:new-4 and for "steps aside". | none |
| CPY-9 | unchanged, state effect | required | required | Chooses the constructor of the fallback. | none |
| CPY-10 | unchanged, state effect | required | required | The prvalue of `relocate e` initializes its target directly. | none |
| CPY-11 | unchanged, state effect | required | required | `return x;` still moves. Only `return relocate x;` relocates. | none |
| CPY-19 | unchanged, state effect | required | required | Relocation needs no byte copy. The incomplete list stays (X9-1). | none |
| CPY-27 | unchanged, state effect | required | required | The relocation constructor takes no part in triviality, here or in CTOR-23, DTOR-9 and STG-7. | none |
| CLS-19 | unchanged, state effect | required | required | Relocation does not depend on layout. The incomplete list stays (X9-1). | none |
| STG-2 | unchanged, state effect | required | required | A relocation by a function does not need consecutive bytes. | none |
| STG-6 | unchanged, state effect | required | required | A relocation into storage that holds an object ends that object without its destructor. | none |
| STG-7 | unchanged, state effect | required | required | A relocation constructor does not make a class an implicit-lifetime type. | none |
| STG-9 | unchanged, state effect | required | required | A new object placed in a relocated variable is reached by the old name, and the block does not destroy it (1.7). | none |
| STG-10 | unchanged, state effect | required | required | The uses that stay valid on a relocated variable, such as taking its address. | none |
| STG-12 | unchanged, state effect | required | required | Byte copies keep their present scope. | none |
| STG-20 | unchanged, state effect | undefined | undefined | A `const` variable cannot be relocated, so STG-20 never applies to a relocated variable. | none |
| STG-22 | unchanged, state effect | undefined | undefined | A member call on a relocated variable is undefined. | none |
| STG-25 | unchanged, state effect | impl-defined | impl-defined | A constructor relocates each base part, so shared tail bytes do not matter. | none |
| STG-28 | unchanged, state effect | required | required | The growable-array example relocates into elements that exist but are not built. | none |
| VIRT-6 | unchanged, state effect | required | required | During a relocation constructor of `C`, virtual calls reach the overriders of `C`. | none |
| VIRT-8 | unchanged, state effect | reject | reject | `relocate e` for an abstract class makes an object of that class, so VIRT-8 rejects it. | none |
| VIRT-12 | unchanged, state effect | undefined | undefined | The model for X9:new-11 (static and dynamic types that differ). | none |
| VIRT-13 | unchanged, state effect | recommended | recommended | The model for the third warning of X9:new-12. | none |
| VC-P4 | unchanged, state effect | principle | principle | The caller ends parameter objects, so a function cannot relocate its parameters. | none |
| VC-7 | unchanged, state effect | unspecified | unspecified | An extra copy of a trivially copyable argument is still allowed. A relocated argument is a prvalue like any other. | none |
| VC-10 | unchanged, state effect | impl-defined | impl-defined | Gives the reason that a callee cannot relocate its parameter. | none |
| LAM-4 | unchanged, state effect | required | required | A capture by copy of a relocated variable reads the variable outside its lifetime. An init-capture `[y = relocate x]` relocates. | none |
| SEQ-1 | unchanged, state effect | unspecified | unspecified | Relocations in two arguments happen in an unspecified order. X9:new-10 keeps unwinding correct for every order. | none |
| UNW-5 | unchanged, state effect | required | required | A throw out of a user-provided relocation constructor ends the program, because the constructor is non-throwing. | none |
| UNW-9 | unchanged, state effect | required | required | A parameter object that a relocation initialized ends during unwinding, as any parameter object does. | none |
| DCL-13 | unchanged, state effect | required | required | When `relocate` names a type, `T y(relocate x);` is still a function declaration. | none |
| STMT-6 | unchanged, state effect | required | required | With no declaration of `relocate`, `relocate x;` cannot be a declaration, so it is an expression statement. | none |
| KW-1 | unchanged, state effect | reject | reject | `relocate` is not added to the keywords. | none |
| NEW-5 | unchanged, state effect | undefined | undefined | `delete p` after `relocate *p` is a second end (DTOR-7). | none |
| JMP-6 | unchanged, state effect | reject | reject | A jump forward past a relocation does not change which declarations a jump can skip. | none |
| CE-P1 | unchanged, state effect | principle | principle | Constant evaluation tracks relocations and rejects a use after one. | none |
| OVL-17 | unchanged, state effect | required | required | The model for a constructor that is never a candidate. | none |
| TYP-P1 | unchanged, state effect | principle | principle | Every fundamental type relocates by step 1 of X9:new-7. That step rests on "scalar type", which Tidal does not define (X9-2). | none |

Counts: 90 changes. 13 new, 30 amended (two of them Syntax segments), and 47 unchanged but must state their effect. None withdrawn, none split. One existing test changes its outcome.

`classes/copy-move/copy-ctor-by-value.cpp` declares `S(S) {}`. Under P8 that declaration is a valid relocation constructor with no parts. So the test expects `accept` and moves to X9:new-1. CPY-15 needs a new reject test for `S(S, int = 0)`, which both compilers reject (probe p6). The other tests nearest the boundary keep their outcome:
- `classes/copy-move/template-ctor-by-value-own-class.cpp`: a constructor template is never a relocation constructor. Still accepted.
- `classes/constructors/double-destroy-sanitizer.cpp` and `double-destroy-constant.cpp`: an explicit destructor call is not a relocation. Still undefined.
- `foundations/storage/scope-end-other-type-sanitizer.cpp`: the variable is not relocated, so the block still calls `~Plain` for an `Other`. Still undefined.
- `foundations/storage/assignment-by-destroy-and-construct.cpp`: no relocation. Still runs.
- `foundations/objects/early-return-destroys.cpp`, `block-reverse-order.cpp`: no relocation. The order is the same.
- `foundations/keywords/contextual-names.cpp`: `relocate` is not a keyword. No test in `tests/` uses the name `relocate`.

The proposal needs these new tests under `tests/classes/copy-move/`:
- `relocation-ctor-defaulted.cpp` (run; `Buffer` of 1.1, with a counter that shows `~Buffer` runs once),
- `relocation-ctor-user-fixup.cpp` (run; `Line` of 1.4),
- `relocation-ctor-not-candidate.cpp` (run; `S b(a)` calls the copy constructor when `S(S)` exists),
- `relocation-ctor-implicit.cpp` (run; an aggregate with a member that can be relocated),
- `relocation-ctor-steps-aside.cpp` (run; a deleted implicit relocation falls back to a move and a destructor call),
- `relocation-ctor-deleted-used.cpp` (reject; X9:new-5),
- `relocation-ctor-two-params.cpp`, `relocation-ctor-default-arg.cpp`, `relocation-ctor-explicit.cpp` and `relocation-ctor-noexcept-false.cpp` (reject; X9:new-2),
- `relocation-union-nontrivial-member.cpp` (reject; X9:new-4 with UNI-3),
- `relocation-ctor-not-inherited.cpp` (reject; DER-8),
- `ctor-by-value-default-args.cpp` (reject; CPY-15),
- `ctor-by-value-class-template.cpp` (accept under P8. See X9-7 for the outcome today).

These tests go under `tests/expressions/unary/`:
- `relocate-return.cpp`, `relocate-conditional.cpp`, `relocate-loop-body.cpp` and `relocate-goto-back.cpp` (run; X9:new-10),
- `relocate-argument-unwinding.cpp` (run; one argument relocates a variable, another argument throws, and the variable is not destroyed again),
- `relocate-fallback-throws.cpp` (run; the move throws, and unwinding destroys the variable),
- `relocate-element.cpp` (run; the growable array of 1.6),
- `relocate-scalar.cpp` (run),
- `relocate-unevaluated.cpp` (accept; `decltype`, `sizeof`, `noexcept`),
- `relocate-name-declared.cpp` (run; probe p10), `relocate-type-declared.cpp` (accept; probe p4) and `relocate-call-form.cpp` (run; in a template, `relocate(t)` still calls a function that argument-dependent lookup finds),
- `relocate-parameter.cpp`, `relocate-static.cpp`, `relocate-const.cpp`, `relocate-reference.cpp`, `relocate-binding.cpp`, `relocate-data-member-name.cpp`, `relocate-array.cpp` and `relocate-abstract.cpp` (reject; X9:new-9),
- `relocate-twice-constant.cpp`, `relocate-then-read-constant.cpp`, `relocate-through-base-constant.cpp` and `relocate-then-delete-constant.cpp` (reject; constant evaluation of OBJ-2, X9:new-11 and DTOR-7),
- `relocate-use-after-warns.cpp` (warning; X9:new-12).

Under `tests/foundations/objects/`, the proposal needs `relocated-not-destroyed.cpp` and `relocated-order-of-others.cpp` (run). Under `tests/foundations/storage/`, it needs `relocated-storage-placement-not-destroyed.cpp` (run; 1.7).

## 3. Interactions

**Objects and lifetime (OBJ-P1, OBJ-P2, OBJ-P4, OBJ-1 to OBJ-3, OBJ-7, OBJ-10).** Relocation is a new event that ends a lifetime. Tidal has no single list of such events. So the proposal amends OBJ-P1 and OBJ-1 and checks STG-6, DTOR-6 and NEW-5 one at a time (X9-3). The reverse order of OBJ-P2 still holds among the variables that remain, and a relocated variable drops out of that order. OBJ-1 does not say which completion begins the lifetime when a constructor delegates. X9:new-6 needs that answer for a delegating relocation constructor (X9-6).

**Block exits (STO-1, JMP-P2, JMP-9, UNW-P1, UNW-1, DTOR-5).** Six places besides OBJ-P2 and OBJ-3 state that a block destroys its variables. Each one needs the same exception (X9-4). The exception is decided at run time, so the proposal needs no rules about flow of control. Tidal has no such rules to cite.

**Storage (STG-P4, STG-6, STG-9, STG-10, STG-21, STG-22, STG-25, STG-28).** The roadmap named STG-21 as the rule that the proposal must change. STG-21 stays `undefined` for its own cases, and its implicit destructor call no longer happens for a relocated variable. The defined skip is X9:new-10. DTOR-7 and STG-21 cover overlapping cases with different scopes, so the proposal amends both (X9-5). A relocation by a function builds base parts by construction. So neither STG-19's limit on replacing a base subobject nor STG-25's shared tail bytes applies. A relocation creates its object with a constructor and never creates one implicitly, so the design uses no part of STG-8.

**Dependence on provisional answers (Q-0304 to Q-0308).** The brief and the roadmap describe these questions as provisional answers about implicit objects. The questions actually cover byte steps, `char` arrays, `signed char`, constant evaluation of a replacement, and class size. Q-0308 is settled. The questions about implicit objects are Q-0323 and Q-0324, so the table includes them too.

| Question | Provisional answer | Where the design depends on it |
|---|---|---|
| Q-0304 | A byte pointer can step through the bytes of any object. | The design does not depend on it. A relocation by bytes would (X9-1). |
| Q-0305 | An array of `char` hosts no object. | A program that relocates into a `char` buffer ends the buffer (STG-6). The design's examples use storage from `::operator new`. A container that relocates into a `char` array depends on this answer. |
| Q-0306 | Access through `signed char` is undefined. | The design does not depend on it. |
| Q-0307 | Constant evaluation must reject the use of an old pointer after a replacement that STG-9 does not allow. | Indirectly. The design excludes `const` variables (X9:new-9) so that a later placement `new` never meets STG-20. If `const` sources were allowed, the result in constant evaluation would depend on Q-0307. GCC 16, Clang 22 and EDG read the old value there. |
| Q-0308 | Settled: an implementation rejects a class too large for `std::size_t`. | The design does not depend on it. |
| Q-0323 | `p + i` is valid before element `i` is built. | The growable array of 1.6 relocates into such elements. |
| Q-0324 | A cast of a byte array's address points to a byte. | A program that relocates into a byte array reaches the new object only through the result of placement `new` (STG-29). |
| Q-0054 | Constant evaluation rejects a second destruction. | `relocate-then-delete-constant.cpp` and the amended DTOR-7 rely on it. EDG 7.0 accepts a second destruction. |

**Copy and move (CPY-P1 to CPY-P4, CPY-1, CPY-2, CPY-4, CPY-5, CPY-7, CPY-9 to CPY-12, CPY-15).** The relocation constructor follows the patterns of CPY-P2 and CPY-P3. It is implicit when part-by-part work is right. It is deleted when a part cannot take part. It steps aside when the implementation deleted it. It takes the free space of CPY-15. The reason that CPY-P1 gives for CPY-15 does not hold for a prvalue argument (X9-8). CPY-11 and CPY-12 decide what `return x;` does, and the proposal leaves `return x;` unchanged.

**Constructors and destructors (CTOR-P1, CTOR-6, DTOR-3, DTOR-6, DTOR-7).** The relocation constructor is the only constructor that ends an object. Each part of its source ends either by its own relocation or by step 4 of 1.4. The destructor body of the class never ends a part. DTOR-3 still rejects a variable whose destructor is deleted.

**Virtual functions (VIRT-6, VIRT-8, VIRT-12, VIRT-13).** A relocation constructor cannot be virtual. So a relocation through a base cannot reach the relocation constructor of the derived class. X9:new-11 makes that case undefined, following VIRT-12, and X9:new-12 adds a warning in the style of VIRT-13. A tracked variable is a complete object, so its static and dynamic types agree. Tidal uses the term "dynamic type" without defining it, so X9:new-11 has no definition to cite (X9-2).

**Types (TYP-P1).** A scalar relocates by copying its value. That rule needs a list of scalar types. types.md lists only the fundamental types, and no page defines "scalar type" (X9-2).

**Values and parameters (VC-P4, VC-4, VC-7, VC-10, SEQ-1, UNW-9).** The result of `relocate e` is an ordinary prvalue. So it directly initializes a parameter, a return object, an object made by `new`, or a temporary. A parameter cannot be relocated, because the caller ends it.

**Overload resolution and deleted functions (OVL-P2, OVL-14, OVL-17, DEL-P2, DEL-4, EXS-4, CE-2).** The relocation constructor joins three lists that each name the special members in their own words: the functions that can be defaulted, the implicit functions that are non-throwing, and the implicit functions that count as `constexpr`. Tidal has no single list of special member functions that the proposal could amend once (X9-2).

**Keywords and statements (KW-P1, KW-1, KW-3, STMT-6, DCL-13).** X9:new-13 gives `relocate` its meaning only where lookup finds nothing and the program is invalid today. The rule uses lookup to decide syntax, as TARG-23 does for `<`. KW-3 states the positions for `override` and `final` only (X9-9).

**Lambdas (LAM-4).** An init-capture can relocate a variable. A capture by copy after a relocation reads the variable outside its lifetime.

**Unions (UNI-3, UNI-5).** A union can be relocated only when every variant member has a trivial copy constructor and a trivial destructor. Otherwise the implementation cannot know which member is active.

**Exception handlers.** A handler parameter `catch (T e)` is a variable of the handler block and not a function parameter, so X9:new-9 allows `relocate e`. The design does not touch the exception object.

**Coroutines.** OBJ-3 now says that it assumes a function body runs without suspension. X9:new-10 makes the same assumption.

**Not touched.** Copy and move assignment, aggregate initialization, the one-definition rule and templates keep their current rules. For a dependent type, the step of X9:new-7 is chosen at instantiation. `requires (T* p) { relocate *p; }` tests whether a type can be relocated, because the untracked form needs no variable.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- A constructor whose only parameter is its own class by value, with a body, `= default` or `= delete`. Both compilers reject it today (`copy-ctor-by-value.cpp`, probe p9). Both also reject it in a class template before any instantiation (probe p7).
- `relocate e` where lookup finds no declaration of `relocate`. Both compilers reject the forms of probes p1, p2, p3 and p5 today, with "not declared in this scope" or "use of undeclared identifier".

### 4.2 Undefined becomes defined

None. STG-21 and DTOR-7 keep their cases. The defined skip covers only variables that a relocation expression ends, and no valid program today has one. A program that writes the pattern by hand today (an explicit destructor call, then the end of the block) is still undefined.

### 4.3 Valid programs that change meaning

None. The implicit relocation constructor is never a candidate (OVL-14). It takes no part in triviality, in aggregates or in implicit-lifetime types. So no valid program can see whether the constructor exists. `return x;` keeps its meaning (CPY-11). Every meaning that a declaration gives `relocate` stays (X9:new-13).

### 4.4 Valid programs that become invalid

None. X9:new-2, X9:new-5 and X9:new-9 reject only forms that use a relocation constructor or a relocation expression. A full keyword would make probe p10 invalid (1.10).

### 4.5 Evidence

- Both compilers reject `S(S)`, `S(S) = default`, `explicit S(S)` and `S(S, int = 0)` (`copy-ctor-by-value.cpp`, probes p6, p8, p9). Clang calls each one a copy constructor that "must pass its first argument by reference". GCC says "invalid constructor".
- Both compilers reject `W(W)` in a class template at its definition (probe p7). GCC reports it under `-Wtemplate-body`.
- With no declaration of `relocate`, both compilers reject `relocate x` in an initializer, a `return`, an argument and a parenthesized initializer (probes p1 to p3, p5). Clang reports p5 as "unknown type name".
- With `struct relocate {}`, both compilers build `T y(relocate x);` as a function declaration (probe p4). With `int relocate`, both build and run `relocate * x.v` as a product (probe p10).
- Both compilers run probes `fb.cpp` and `p11.cpp` with exit status 0 under `-fsanitize=address`, and `fb.cpp` also under `undefined`. `fb.cpp` moves an element in allocated storage and then calls the old element's destructor explicitly. `p11.cpp` moves out of a union member and then calls that member's destructor. These are the hand-written forms of the fallback.
- During constant evaluation, both compilers reject a second destruction of a variable with a trivial destructor ("destroying 'p' outside its lifetime", probe dd.cpp). This result bears on X9-5.

## 5. Findings

**X9-1** (missing and wrong; classes/copy-move.md, classes/members.md, foundations/storage.md; CPY-19, CPY-27, CLS-19, STG-2, STG-7, STG-8, STG-12, STG-25; roadmap D21). CPY-19 and CLS-19 still end with "others that this page does not list yet". In three probes, the conditions that the rules do list disagree with both compilers. Probe tc.cpp asks the compilers' built-in queries:
- On both compilers, a class whose copy constructor is defaulted after its first declaration is not trivially copyable. CPY-19 says "implicit or defaulted", so it admits the class. CPY-27 counts such a function as provided, and CPY-19 does not cite CPY-27.
- On both compilers, a class with `~D() = delete;` and an `int` member is trivially copyable. CPY-19 asks for a destructor that is "implicit or defaulted", so it excludes the class.
- `struct Derived : Base { char c; }`, where `Base` has a data member, meets every listed condition of CLS-19. Both compilers say it is not standard-layout.

A relocation by bytes would need the following from these lists and the pages around them:
- a complete CPY-19, stated with the terms "trivial" (CPY-27, DTOR-9) and "eligible".
- a rule that a byte copy into fresh storage creates the object. STG-8 creates objects only for an allocation, the start of a byte array, and library functions. CPY-19 and STG-12 copy only into an object that already exists. A trivially copyable class need not be an implicit-lifetime type (STG-7). For example, a class whose only special member that is not deleted is a trivial copy assignment has no constructor that STG-7 accepts.
- a rule that the bytes are consecutive for every class that the design covers. STG-2 promises consecutive bytes only for trivially copyable or standard-layout classes, so a byte design cannot rely on STG-2 until CLS-19 is complete.
- a new class property for a class whose destructor is not trivial but whose bytes can move. That property would reuse both lists and would need a rule that the source ends without its destructor.
- STG-25's exclusion of base subobjects and `[[no_unique_address]]` members, and Q-0304 for the byte steps.

Suggestion: do roadmap-3 item 8 (complete both lists), with tests for the three probes above. A separate proposal can then add relocation by bytes on top of X9.

**X9-2** (missing; glossary, foundations/types.md, classes/virtual.md, declarations/functions/deleted.md; JMP-6, STG-7, STG-14, DEL-4, EXS-4). Rules use three terms that have no definition:
- "scalar type" appears in at least 14 places, for example JMP-6, STG-7, declarators.md, constant-evaluation.md and structured-bindings.md. types.md lists only the fundamental types. It does not mention enumerations, pointers or pointers to members.
- "special member function" appears in DEL-4, EXS-4, UNI-3, inline.md, and the glossary entries "eligible" and "trivial". No page lists the special members. P8 adds one and has no list to amend.
- "dynamic type" appears in STG-P7, STG-14 and member-pointers.md. X9:new-11 needs the term.

Suggestion: add three glossary entries. "Scalar type" lists the arithmetic, enumeration, pointer, pointer-to-member and `std::nullptr_t` types and cites TYP. "Special member function" lists the six functions. "Dynamic type" says that it is the type of the most derived object that a glvalue refers to.

**X9-3** (hard to find; foundations/objects.md, foundations/storage.md, classes/constructors.md; OBJ-P1, OBJ-1, OBJ-10, STG-6, DTOR-6, NEW-5). No single rule lists the events that end an object's lifetime. OBJ-1 names only the completion of the destructor. OBJ-P1 adds the release and reuse of storage. STG-6 and OBJ-10 cover reuse, DTOR-6 an explicit destructor call, and OBJ-P2 the end of a block and `delete`. A proposal that adds an event must check each of these rules by hand. Suggestion: a table in OBJ-P1 with one row per event and the rule for each event.

**X9-4** (hard to change; objects.md, storage-linkage.md, jumps.md, unwinding.md, constructors.md; OBJ-P2, OBJ-3, STO-1, JMP-P2, JMP-9, UNW-P1, UNW-1, DTOR-5). Eight principles and rules each say that leaving a block destroys its variables in reverse order. P8 must add the same exception to seven of them. A later proposal (P14, scope-exit blocks) will touch the same places. Suggestion: OBJ-3 holds the one statement. The other rules cite OBJ-3 for the order and add only what is specific to their kind of exit.

**X9-5** (unclear; foundations/storage.md, classes/constructors.md; STG-21, DTOR-7, Q-0054). DTOR-7 makes a second destruction at the end of a block undefined for any class. STG-21 makes it undefined for the block's implicit destructor call to find no object in the storage, but only for a class whose destructor is not trivial. Take a class `P` with a trivial destructor and the call `p.~P();`. DTOR-7 makes the end of the block undefined, and STG-21 says nothing about the case. Both compilers reject this case during constant evaluation (probe dd.cpp). Suggestion: STG-21 says that DTOR-7 covers the case where the storage still has the original type, for a destructor of any kind. STG-21 keeps its own case for storage that holds an object of another type.

**X9-6** (unclear; foundations/objects.md, exceptions/unwinding.md; OBJ-1, UNW-2, UNW-P2). OBJ-1 says that the lifetime "begins when its constructor completes". With a delegating constructor, two constructors complete. UNW-2 treats the object as complete when the target constructor completes. OBJ-1 does not say so. X9:new-6 must choose when the source ends under a delegating relocation constructor. Suggestion: OBJ-1 says that the completion of the target constructor begins the lifetime, and cites UNW-2.

**X9-7** (needs example; classes/copy-move.md; CPY-15). Both compilers reject a constructor `W(W)` that is not a template, in a class template, at the template's definition and before any instantiation (probe p7). GCC reports it under `-Wtemplate-body`. The tests of CPY-15 cover only a plain class and a constructor template. Suggestion: add a reject test for the class template case, and an Evidence note about GCC's warning group.

**X9-8** (wrong reason; classes/copy-move.md; CPY-P1, CPY-10, CPY-15). CPY-P1 says that a constructor that takes the class by value "cannot exist, because such a constructor would need itself to copy its argument". CPY-10 says that a prvalue argument initializes the parameter directly, with no copy. So the reason holds only for an lvalue or xvalue argument. Suggestion: CPY-P1 gives the reason as "an lvalue argument would need the constructor itself", or states the rule with no reason.

**X9-9** (missing; foundations/keywords.md, statements/blocks.md; KW-P1, KW-3, STMT-6, TARG-23). KW-P1 says that late words have a special meaning "only where a keyword could not appear". KW-3 gives the positions only for `override` and `final`. No rule says whether lookup can take part in recognizing such a word. TARG-23 uses lookup to decide what `<` means, so the language already has the pattern. Suggestion: KW-P1 states the two methods (position only, and position with lookup), with KW-3 and TARG-23 as the examples.

**X9-10** (missing; foundations/values.md, foundations/evaluation-order.md; VC-P4, VC-10, SEQ-1). VC-P4 says that the caller makes and ends each parameter object. No rule says what happens when the callee ends a parameter object itself, for example by an explicit destructor call. DTOR-7 suggests that the caller's end is then a second destruction. I found no test of the case. P8 had to exclude parameters for this reason. Suggestion: add a sentence to VC-10 saying that the caller's end of a parameter object counts as a destruction for DTOR-7, and add a constant-evaluation test.

**X9-11** (needs test; foundations/storage.md, foundations/objects.md; STG-9, STG-21, STG-P4). STG-9 says that the old name reaches a new object created in a variable's storage. STG-P4 says that the block then destroys that object as the declared type. `assignment-by-destroy-and-construct.cpp` uses this pattern with a class whose destructor is trivial, so the test cannot observe the block's destructor call. No test shows that the block destroys the new object exactly once when the destructor is not trivial. P8 changes this outcome when a relocation takes the place of the destructor call, and has no test to compare against. Suggestion: add a run test, `replace-variable-then-scope-end.cpp`, that counts destructor calls and cites STG-9 and STG-21.

**X9-12** (unclear; classes/copy-move.md; CPY-19, CPY-P6, STG-12). CPY-P6 says that copying the bytes of a trivially copyable object "copies the value". CPY-19 says that "the second object gets the same value". STG-12 adds that a copy through `char` needs a value in each byte (Q-0328). CPY-19 neither repeats nor cites that limit, and its test copies through `unsigned char`. Suggestion: CPY-19 cites STG-12 for the byte types and the limit on padding.

**X9-13** (needs example; classes/virtual.md; VIRT-1, VIRT-6, VIRT-12). No example or test shows a virtual call made inside a constructor through a reference to another complete object of the same class. VIRT-6 speaks of "the object" under construction. A relocation constructor calls through `src`, which is a different and complete object. VIRT-1 gives the answer for `src`, but a reader must combine VIRT-1 and VIRT-6 to find it. Suggestion: add an Examples note under VIRT-6.

**X9-14** (outside knowledge). These parts of the report come from general experience:
- the idea of a move that ends its source, and of a flag per variable that records whether the variable has been moved from, from languages that have both,
- the judgment that a growable array is the main use of relocation,
- the habit of limiting a user-provided relocation to fix-ups, so that the common case is defaulted.

The probes in X9-1 use built-in queries of GCC and Clang (`__is_trivially_copyable`, `__is_standard_layout`). Their names came from general knowledge of the compilers. The results are compiler behavior.

**X9-15** (outside the spec; roadmap-3 item 4, Q-0007). Three needs fall outside the spec:
- Implementation cost. A tracked variable needs a flag on some paths, and Tidal has no model in which to state that cost.
- Calling conventions. Relocating parameters would need the callee to end parameter objects. VC-10 leaves that choice to the implementation's convention.
- Version marking. P8 takes the space of `T(T)` and of the word `relocate` before an identifier. A designer cannot tell whether a later version uses that space.