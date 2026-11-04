# X14: P20, packed classes

Date: 2026-11-03. Commit read: `573d849`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P20 has no description beyond its row in section 1.7 of `reports/clean-room/roadmap-4/designer.md`, so the marker and the rules are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- `reports/clean-room/X12/report.md`, as the model for this report's form, after a run of `tools/propose_check.py` (12 reports, 29 flagged rows; X12 has one flagged row, CE-31, which this report does not use).
- `reports/clean-room/roadmap-4/designer.md`: section 1.7 and item 2 of section 2.
- In full: `specification/declarations/alignment.md` and `specification/expressions/member-pointers.md`.
- `specification/classes/members.md`: the Syntax segment, the CLS-P list, CLS-10, CLS-11, CLS-19, CLS-29 to CLS-35.
- Other cited rules: storage (STG-1 to STG-4, STG-8, STG-13 to STG-18, STG-23, STG-28, STG-29, STG-P2, STG-P3, and the STG-13 and STG-17 Evidence), unions (the Syntax segment, BF-4, BF-9, UNI-10, UNI-15, UNI-16), casts (CAST-6, CAST-14, CAST-16, Examples), types (TYP-2), unary (UN-8, UN-9, UN-12, the `alignof` rules), postfix (PFX-1 to PFX-3), references (REF-1, REF-2, REF-8), conversion sequences (the ICS-P list, ICS-8, ICS-13), volatile (VOL-1 to VOL-13), attributes (ATTR-P1, ATTR-P2, ATTR-1, ATTR-2, ATTR-19), derived (`class_head` syntax), type-names (`forward_class` syntax), keywords (KW-3), structured bindings (SB-2, SB-3, SB-13), range-for (RFOR-1, RFOR-3), copy-move (CPY-28), the glossary entries for trivially copyable, implicit-lifetime type, standard-layout and fundamental alignment.
- Q-0069.
- Tests: `declarations/alignment/weaker-on-class-member.cpp`, `weaker-than-natural.cpp`, `member-raises-class.cpp`, `on-bitfield.cpp`, `class-forward-alignas.cpp`; `classes/members/standard-layout-reinterpret.cpp`, `layout-order-and-pointer.cpp`; `foundations/storage/object-bytes-cover-members.cpp`, `placement-misaligned-sanitizer.cpp`; `foundations/conversion-sequences/chosen-bit-field.cpp`; `expressions/member-pointers/form-bitfield.cpp`; `declarations/initialization/references/bit-field-const-ref.cpp`, `const-ref-binds-lvalue-directly.cpp`; `declarations/attributes/unknown-attribute-ignored.cpp`.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X14/p`. Probes of the marker's syntax and of the findings use `-std=c++23 -pedantic-errors`. Probes of the compilers' own packing extensions (`__attribute__((packed))`, `#pragma pack`, `[[gnu::packed]]`) drop `-pedantic-errors`, and some add `-Wall -Wextra`, `-O2 -S`, or `-fsanitize=undefined -fno-sanitize-recover=undefined`. The extension probes show what a design must handle. They do not define P20. My knowledge that these extensions exist is general knowledge (X14-9). Each probe flushes its output before any sanitizer stop. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 57 rule changes (8 new, 23 amended of which 3 are Syntax segments, 26 unchanged but must state their effect). No existing test changes its expected outcome. 10 findings, X14-1 to X14-10.

## 1. Design

### 1.1 The feature

Today every object sits at an address that is a multiple of its type's alignment (ALN-P1, ALN-4). A class gets padding between members so that each member stays aligned (CLS-34, CLS-35). A program that reads a file or network format with no padding must copy the bytes out one field at a time.

P20 adds a *packed class*. Each member of a packed class has alignment 1 in the class, so the members follow each other with no padding. The class has alignment 1. A member whose type needs a stricter alignment is an *under-aligned member*. A program can read, write and copy an under-aligned member. A program cannot form a pointer or a reference to an under-aligned member.

```cpp
packed struct Header {
  char kind;                     // offset 0
  unsigned length;               // offset 1, under-aligned
  unsigned short port;           // offset 5, under-aligned
};
static_assert(sizeof(Header) == 7 && alignof(Header) == 1);

void use(Header& h, unsigned char* wire) {
  unsigned n = h.length;         // valid: reads the value
  h.length += 4;                 // valid: built-in compound assignment
  const unsigned& c = h.length;  // valid: binds to a copy (X14:new-6)
  unsigned* p = &h.length;       // P20: rejected (X14:new-7)
  unsigned& r = h.length;        // P20: rejected (X14:new-7)
  Header* at = new (wire + 3) Header{};  // valid: any address is aligned for Header
}
```

### 1.2 The marker (new rules X14:new-1, X14:new-2; amended Syntax segments, KW-3)

The marker is the identifier `packed` immediately before the class key of a class definition or a forward declaration:

```cpp
packed struct Header { ... };
packed class Record;
packed union Word { unsigned u; unsigned char b[4]; };
template <class T> packed struct Box { char tag; T value; };
```

`packed` stays an identifier everywhere else. KW-3 gains this place. The three productions that start a class gain an optional `"packed"` before the class key: `class_head` (derived.md), `union_def` (unions.md) and `forward_class` (type-names.md).

The place before the class key is free today. An identifier directly followed by `class`, `struct` or `union` never forms a valid declaration. Both compilers reject `packed struct T { int a; };` when `packed` names a class (probe `m5.cpp`, "multiple types in one declaration", "cannot combine with previous 'type-name' declaration specifier"). Section 1.7 explains why the place after the name does not work.

X14:new-2 (reject) asks the declarations to agree, in the way ALN-3 does for `alignas`. If any declaration of a class has `packed`, the definition has `packed`. A declaration after a definition without `packed` must not have it. A forward declaration can leave the marker out. Across translation units, the definitions are compared token by token (ODR-10), so the marker is part of the match.

### 1.3 The layout (new rule X14:new-3; amended ALN-P1, ALN-1, ALN-2, CLS-31, CLS-34, BF-9)

X14:new-3 (required) gives the layout of a packed class:
- Each non-static data member and each base subobject has alignment 1 in the class.
- `alignas` on a member gives that member the stated alignment. In a packed class, the natural alignment of a member is 1, so an `alignas` smaller than the alignment of the member's type is valid there (ALN-2 is amended).
- The members and bases take the order of CLS-35 and of CLS-10. Each one starts at the first free byte that is a multiple of its alignment in the class.
- The alignment of the class follows ALN-1: the strictest of its own `alignas` and the alignments of its members in the class. With no `alignas`, it is 1.
- The size of the class is the end of its last part, rounded up to a multiple of its alignment (ALN-1). An empty packed class has the size of CLS-11.
- An anonymous union in a packed class is laid out as a packed union. All members of a packed union start at offset 0.

This rule is `required`, where today's layout rules are `implementation-defined` (CLS-31) or `unspecified` (CLS-34). A packed class exists to match a fixed byte layout, so the offsets must be the same on every implementation. CLS-31 and CLS-34 are amended to say that they do not apply to the members of a packed class.

Bit-fields stay implementation-defined (BF-9). The order of bits is already the implementation's choice, and P20 does not change that. BF-9 gains one item for a packed class: a bit-field takes the next free bits, also across a unit of `sizeof(T)` bytes. The extensions on both compilers do this today. In `struct { char c[3]; int b : 12; char z; }`, the packed form puts `b` at bit 24 and `z` at offset 5 (size 6). The plain form moves `b` to offset 4 and `z` to offset 6 (size 8) (probe `b1.cpp`).

```cpp
packed struct Mixed {
  char tag;                      // offset 0
  alignas(2) short count;        // offset 2: its own alignment 2 is kept
  alignas(2) int total;          // offset 4: alignment 2, still under-aligned
  double ratio;                  // offset 8, under-aligned
};
static_assert(alignof(Mixed) == 2 && sizeof(Mixed) == 16);
```

### 1.4 Limits on a packed class (new rules X14:new-4, X14:new-8)

X14:new-4 (reject) says a packed class must not have:
- a virtual function or a virtual base,
- a base class whose alignment is greater than 1.

A base of alignment 1 is an empty class or a packed class. The pointer for virtual calls is a hidden part with the alignment of a pointer (CLS-35). A base of greater alignment would sit at an offset that its own member functions cannot use through `this`. The extensions handle both cases differently. They keep a base of alignment 4 aligned and give the derived class alignment 4 (`PD`, size 12, probe `e1.cpp`). They pack the pointer for virtual calls (`PV`, size 13 and alignment 1). Section 1.7 lists the choices.

X14:new-8 (reject) limits the type of an under-aligned member:
- The type must be trivially copyable (CPY-28).
- A constructor that is not trivial must not initialize the member or a subobject of the member.

A constructor and a destructor have `this`. A trivially copyable class has a trivial destructor, so only a constructor can need `this`. The second item applies where the initialization happens, for example in the packed class's own default constructor. Aggregate initialization calls no constructor, so `Header h{1, 512, 80};` is valid. A member of class type whose alignment is 1, such as another packed class, is never under-aligned, so these limits do not apply to it.

### 1.5 Under-aligned glvalues (new rules X14:new-5, X14:new-6, X14:new-7)

X14:new-5 (required) defines the terms. A member of a packed class is *under-aligned* if its alignment in the class is less than the alignment of its type. The test does not look at the offset. In `packed struct S { int a; };`, `a` is under-aligned at offset 0, because an `S` can start at any address. An *under-aligned glvalue* is one of these:
- a member access (`.`, `->`, or an implicit `this->`) that names an under-aligned member,
- a member access, a `.*`, or a subscript, applied to an under-aligned glvalue, whose type has an alignment greater than 1,
- the name of a structured binding that refers to an under-aligned member or to a part of one (SB-3),
- the result of a built-in assignment, compound assignment, or prefix `++` or `--`, whose left operand is under-aligned,
- a glvalue conditional expression with an under-aligned operand, and a comma expression with an under-aligned right operand.

The property belongs to the expression, and the implementation knows it at translation. In a template, it is known after substitution, so `Box<int>::value` is under-aligned and `Box<char>::value` is not.

X14:new-6 (required) lists what a program can do with an under-aligned glvalue:
- read its value,
- write it with a built-in assignment, compound assignment, `++` or `--`,
- apply a member access, `.*`, or a subscript. A subscript of an under-aligned array does not convert the array to a pointer (PFX-1 is amended).
- initialize or assign an object of the same class type from it, if the constructor or assignment operator chosen is trivial. The operation copies the bytes and binds no reference.
- bind a `const` lvalue reference that is not `volatile`. The reference binds to a temporary that holds a copy, as for a bit-field (REF-2, BF-4).
- use it in an operand that is not evaluated (`sizeof`, `decltype`, `noexcept`).
- call a member function whose explicit object parameter is not a reference. The parameter is a copy.

X14:new-7 (reject) lists what a program must not do with an under-aligned glvalue:
- apply `&` (UN-9 is amended),
- bind any other reference: a reference that is not `const`, an rvalue reference, or a `const volatile` reference (REF-8 is amended),
- convert an array to a pointer,
- call a non-static member function with an implicit object parameter (ICS-13),
- capture it by reference, or use it as the range of a range-based `for` (RFOR-3 holds the range by reference),
- cast it to a reference type.

`&C::m` for an under-aligned member `m` is also not valid (MPTR-9 is amended). Like a bit-field, an under-aligned member has no pointer to member, so a pointer to member never names one. The result of `.*` is under-aligned only when its object operand is (MPTR-3 is amended).

A reference binding to an under-aligned glvalue forms its conversion sequence as usual and fails only for the function that overload resolution chooses (ICS-8 gains an item). The bit-field case works the same way. So `take(h.length)` with `take(unsigned&)` and `take(long)` chooses the first and is rejected. GCC's extension does exactly this (`T15` in probe `r1.cpp`).

### 1.6 What a pointer or reference to an under-aligned member means (amended ALN-4, STG-16, CLS-30, VOL-13)

The roadmap asks what a reference or pointer to a member that is not aligned means. In P20, no pointer of type `T*` and no reference of type `T&` refers to an under-aligned object of type `T`. ALN-P1 keeps its promise for every pointer and every reference. Only the half about where objects sit changes. The ways a program could try to form one have these results:

| Attempt | Result under P20 |
|---|---|
| `&h.length` | rejected (X14:new-7) |
| `unsigned& r = h.length;` | rejected (X14:new-7) |
| `const unsigned& c = h.length;` | valid; `c` refers to a temporary copy (X14:new-6) |
| `h.inner.f()`, with `inner` under-aligned | rejected (X14:new-7) |
| `&Header::length` | rejected (MPTR-9) |
| `h.list` decaying to `unsigned*` | rejected (X14:new-7) |
| `reinterpret_cast<unsigned*>(&h)`, where the first member is under-aligned | the result points to `h` and not to the member (STG-16 is amended) |
| a cast of a pointer to a byte of `h` to `unsigned*` | the result points to that byte (STG-16); a read through it is undefined (STG-23) |

ALN-4 is amended. An object has the alignment of its declaration or type, except an under-aligned member, whose alignment is its alignment in the packed class (X14:new-3). STG-16 is amended. An under-aligned member is not pointer-interconvertible with its class object. CLS-30 keeps "the object and that member start at one address", and its sentence on `reinterpret_cast` gains the same exception.

The bytes of an under-aligned member stay reachable through a byte pointer (STG-13). This path is the defined way to copy them without the member access:

```cpp
unsigned char* bytes = reinterpret_cast<unsigned char*>(&h);  // a byte of h
unsigned char first = bytes[1];                               // a byte of h.length
```

A volatile under-aligned member keeps VOL-1: each read and write is an observable effect. VOL-13 gains a row: the size of the access is implementation-defined. On x86-64, both compilers read a packed `volatile int` at offset 1 with one 4-byte load (`movl 1(%rdi), %eax`, probe `v1.cpp`).

Constant evaluation needs no change. A packed class can be a literal type, and its under-aligned members can be read and written during constant evaluation. Constant evaluation cannot use `reinterpret_cast` (CAST-8). Both extensions evaluate `p.a += 3` on a packed object during constant evaluation (probe `c1.cpp`).

### 1.7 Alternatives considered

**The marker.**
- An attribute, `[[packed]]`. ATTR-P1 says an implementation can skip an attribute, and ATTR-P2 says a standard attribute does not change what a valid program does. A packed class changes `sizeof`. A vendor attribute with this effect exists already: `[[gnu::packed]]` changes `sizeof` on both compilers with `-pedantic-errors` (probe `f3.cpp`, X14-7).
- `alignas(1)` on the class. ALN-2 rejects a weaker `alignas` today (Q-0069). GCC accepts `struct alignas(1) S { int x; };` and keeps alignment 4 (probe `f1.cpp`). So one text would have two meanings on two compilers. `alignas` on a class also says nothing about its members.
- `packed` after the class name, like `final`. `struct S packed {};` is a valid variable definition today when `S` is complete (probe `m2.cpp`). Both compilers already read `struct S final {};` as a class definition and reject it as a redefinition (`m1.cpp`). So a marker after the name changes the meaning of a valid program. P20 was meant to change none.
- `struct packed S { ... };`. This is a valid variable definition today when `packed` is a complete class.

**Pointers and references.**
- Let `&h.length` give an ordinary `unsigned*`, and make a read through it undefined. The extensions do this today, with a warning on both (`-Waddress-of-packed-member`, probe `r1.cpp` `T1`). Under UBSan, the read stops as "load of misaligned address" (probe `u2.cpp`). The rule would put an undefined case in every function that takes a pointer, far from the cause.
- A qualifier for an under-aligned type, such as `unsigned unaligned*`. The qualifier would reach conversions (CONV-9), overload resolution and templates. P20 keeps the type system as it is.
- Bind a `const` reference directly. Clang's extension does this, and GCC's binds a copy (probe `u1.cpp`: after `p.a = 7`, the reference reads 7 on Clang and 2 on GCC). A direct binding is a reference to an under-aligned object, which P20 excludes. The copy follows BF-4 and REF-2.
- Call a `const` member function on a copy of an under-aligned member. A function that returns a pointer into `*this` would then return a pointer into a temporary. P20 rejects the call. A program writes `auto(h.inner).f()` to make the copy visible. Both extensions accept the call and pass a misaligned `this`, which UBSan reports as "member call on misaligned address" (probe `u1.cpp`).

**Bases and virtual functions.**
- Pack a base of alignment 4, as for a member. Every derived-to-base pointer conversion would then give a misaligned pointer. The extensions avoid the problem by keeping the base aligned, which makes the derived class aligned too (`PD`). P20 rejects the base.
- Pack the pointer for virtual calls (the extensions do, `PV`). The implementation would load the pointer unaligned in every virtual call. A wire format has no use for a virtual function, so P20 rejects it.

**Member types.** Rejecting every member type of alignment greater than 1 except scalars would forbid a common case, a packed record with a small aggregate inside. X14:new-8 allows any trivially copyable type and checks constructors where they run.

## 2. Rule changes

| Rule ID | Kind of change | Old class | New class | Change | Tests affected |
|---|---|---|---|---|---|
| X14:new-1 | new | none | required | `packed` immediately before the class key of a class definition or forward declaration makes a packed class (1.2). | none |
| X14:new-2 | new | none | reject | If any declaration of a class has `packed`, the definition has it. A declaration after a definition without `packed` does not have it (1.2). | none |
| X14:new-3 | new | none | required | Layout of a packed class: alignment 1 for each member and base unless `alignas` says more, the first free byte in CLS-35 order, the class alignment and size from ALN-1, packed anonymous unions (1.3). | none |
| X14:new-4 | new | none | reject | A packed class has no virtual function, no virtual base, and no base of alignment greater than 1 (1.4). | none |
| X14:new-5 | new | none | required | Defines an under-aligned member and an under-aligned glvalue (1.5). | none |
| X14:new-6 | new | none | required | A program can read, write, access members of, subscript, trivially copy, and bind a `const` non-`volatile` reference to an under-aligned glvalue (a copy) (1.5). | none |
| X14:new-7 | new | none | reject | A program must not take the address of an under-aligned glvalue, bind another reference to it, convert it from array to pointer, call a member function with an implicit object parameter on it, capture it by reference, or use it as a `for` range (1.5). | none |
| X14:new-8 | new | none | reject | An under-aligned member has a trivially copyable type, and no constructor that is not trivial initializes it or its subobjects (1.4). | none |
| `class_head` syntax (derived.md) | amended | syntax | syntax | An optional `"packed"` before `class` or `struct`. | none |
| `union_def` syntax (unions.md) | amended | syntax | syntax | An optional `"packed"` before `union` (X14-3 also asks for an `alignment_specifier`). | none |
| `forward_class` syntax (type-names.md) | amended | syntax | syntax | An optional `"packed"` before the class key (X14-3). | none |
| KW-3 | amended | required | required | `packed` is an identifier with a special meaning only immediately before a class key. | none |
| ALN-P1 | amended | principle | principle | Every object sits at a multiple of its alignment, except an under-aligned member of a packed class. Every pointer and reference stays aligned. | none |
| ALN-1 | amended | required | required | The alignment of a member in a packed class is 1 or its `alignas`. The class alignment uses those values. | none |
| ALN-2 | amended | reject | reject | In a packed class, the natural alignment of a member is 1, so a weaker `alignas` on a member is valid there (Q-0069). | none |
| ALN-4 | amended | required | required | The exception for an under-aligned member (1.6). | none |
| CLS-30 | amended | required | required | The `reinterpret_cast` sentence does not apply to an under-aligned first member. | none |
| CLS-31 | amended | impl-defined | impl-defined | The list does not apply to the members and bases of a packed class, except for bit-fields. | none |
| CLS-34 | amended | unspecified | unspecified | "The padding between members" does not apply to a packed class (X14:new-3). | none |
| BF-9 | amended | impl-defined | impl-defined | In a packed class, a bit-field takes the next free bits, also across a unit (1.3). | none |
| STG-16 | amended | required | required | An under-aligned member is not pointer-interconvertible with its class object. | none |
| REF-1 | amended | required | required | "an lvalue that is not a bit-field" gains "and not under-aligned". | none |
| REF-2 | amended | required | required | A `const` non-`volatile` reference binds to a copy of an under-aligned glvalue. | none |
| REF-8 | amended | reject | reject | A reference that is not `const` must not bind to an under-aligned glvalue. | none |
| ICS-8 | amended | reject | reject | Gains the item "an under-aligned glvalue bound to a reference that X14:new-7 forbids". | none |
| PFX-1 | amended | required | required | The `*(a + i)` equivalence does not form a pointer for an under-aligned array. | none |
| UN-9 | amended | reject | reject | The operand of `&` must not be an under-aligned glvalue. | none |
| MPTR-3 | amended | required | required | The result of `.*` is under-aligned if its object operand is under-aligned and the member type's alignment is greater than 1. | none |
| MPTR-9 | amended | reject | reject | Gains the item "an under-aligned member", next to "a bit-field". | none |
| VOL-13 | amended | impl-defined | impl-defined | Gains a row for a volatile under-aligned scalar: one access of the scalar's size on x86-64 (probe `v1.cpp`). | none |
| ALN-3 | unchanged, state effect | reject | reject | `alignas` agreement is unchanged. X14:new-2 is its counterpart for the marker. | none |
| ALN-5 | unchanged, state effect | impl-defined | impl-defined | The fundamental alignments are unchanged and still decide which members are under-aligned. | none |
| ALN-6 | unchanged, state effect | no-diagnostic | no-diagnostic | Applies to `alignas` only. ODR-10 covers the marker. | none |
| CLS-10 | unchanged, state effect | required | required | The members of a packed class still have increasing addresses. | none |
| CLS-11 | unchanged, state effect | impl-defined | impl-defined | An empty packed class has the same size as other empty classes. | none |
| CLS-19 | unchanged, state effect | required | required | A packed class can be standard-layout. Packing changes none of its conditions. | none |
| CLS-32 | unchanged, state effect | required | required | The objects at the start of a packed class are the same. | none |
| CLS-33 | unchanged, state effect | required | required | An empty standard-layout base of a packed class still takes no bytes. | none |
| CLS-35 | unchanged, state effect | unspecified | unspecified | Its order row applies to a packed class. Its padding row does not. | none |
| STG-2 | unchanged, state effect | required | required | A packed object occupies `sizeof` consecutive bytes, with no padding between members. | none |
| STG-8 | unchanged, state effect | required | required | Implicit creation can create a packed object at any address. | none |
| STG-13 | unchanged, state effect | required | required | The defined way to reach the bytes of an under-aligned member. | none |
| STG-14 | unchanged, state effect | required | required | Reads of an under-aligned member go through its own glvalue, which STG-14 lists. | none |
| STG-17 | unchanged, state effect | undefined | undefined | Placement `new` of a packed class is aligned at every address. | none |
| STG-23 | unchanged, state effect | undefined | undefined | A read through a `T*` cast from a byte of a packed object is undefined (1.6, X14-1). | none |
| BF-4 | unchanged, state effect | reject | reject | The model for X14:new-6 and X14:new-7 (X14-6). | none |
| UNI-15 | unchanged, state effect | required | required | "The same alignment requirement" uses the member's alignment in its class, so a packed and a plain class with an `int` member have no common initial sequence there. | none |
| UNI-16 | unchanged, state effect | required | required | Follows from UNI-15. | none |
| CPY-28 | unchanged, state effect | required | required | A packed class with trivially copyable members is trivially copyable. X14:new-8 uses the term. | none |
| SB-3 | unchanged, state effect | required | required | A name that refers to an under-aligned member is an under-aligned glvalue (X14:new-5). | none |
| RFOR-3 | unchanged, state effect | required | required | Holds the range by reference, so X14:new-7 rejects an under-aligned range. | none |
| ICS-13 | unchanged, state effect | required | required | The implicit object parameter is a reference, so X14:new-7 rejects the call. | none |
| MPTR-4 | unchanged, state effect | required | required | No pointer to member names an under-aligned member, so conversions need no change. | none |
| ATTR-1 | unchanged, state effect | required | required | The reason the marker is not an attribute (1.7, X14-7). | none |
| ATTR-P2 | unchanged, state effect | principle | principle | The same reason. | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | Two definitions that differ in the marker do not match. | none |

Counts: 57 changes. 8 new, 23 amended (3 of them Syntax segments), 26 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

Every program that P20 affects declares a packed class, and no valid program does that today. The tests nearest the boundary keep their outcome:
- `declarations/alignment/weaker-than-natural.cpp` and `weaker-on-class-member.cpp`: neither class is packed, so ALN-2 still rejects.
- `classes/members/standard-layout-reinterpret.cpp`: `S` is not packed, so CLS-30 still gives a usable pointer.
- `classes/members/layout-order-and-pointer.cpp`: no class is packed. The `Padded` assertions hold.
- `declarations/initialization/references/bit-field-const-ref.cpp`, `foundations/conversion-sequences/chosen-bit-field.cpp`, `expressions/member-pointers/form-bitfield.cpp`: bit-fields keep their rules.
- `foundations/keywords/contextual-names.cpp`: `packed` is not in it, and P20 keeps every use of `packed` as a name valid.

The proposal needs these new tests.

Under `tests/declarations/alignment/`:
- `packed-layout.cpp` (accept; `Header` and `Mixed` of 1.1 and 1.3, with `sizeof` and `alignof` assertions),
- `packed-alignas-weaker-member.cpp` (accept; `alignas(2) int` in a packed class),
- `packed-alignas-class.cpp` (accept; `packed struct alignas(4) S { char c; int a; };` has alignment 4 and size 8),
- `packed-union.cpp` (accept; alignment 1, size of the largest member),
- `packed-anonymous-union.cpp` (accept),
- `packed-name-unchanged.cpp` (accept; `struct S {}; struct S packed {};` stays a variable, and a variable named `packed`),
- `packed-declarations-disagree.cpp` (reject; X14:new-2),
- `packed-virtual.cpp` and `packed-aligned-base.cpp` (reject; X14:new-4),
- `packed-base-packed.cpp` (accept; a packed base and an empty base),
- `packed-member-nontrivial-copy.cpp` and `packed-member-constructor.cpp` (reject; X14:new-8),
- `packed-member-packed-class-constructor.cpp` (accept; a member of packed class type with a user-provided constructor),
- `packed-template.cpp` (accept for `Box<char>`, with `&b.value`) and `packed-template-int.cpp` (reject; `&b.value` for `Box<int>`).

Under `tests/expressions/` and `tests/declarations/initialization/references/`:
- `packed-read-write.cpp` (run; read, `=`, `+=`, `++`, a subscript of an under-aligned array, a trivial copy of a class member),
- `packed-address.cpp`, `packed-nonconst-ref.cpp`, `packed-rvalue-ref.cpp`, `packed-const-volatile-ref.cpp`, `packed-array-decay.cpp`, `packed-member-call.cpp`, `packed-range-for.cpp`, `packed-capture-ref.cpp` (reject; one item of X14:new-7 each),
- `packed-const-ref-copy.cpp` (run; the reference keeps the old value after a write, as in `bit-field-const-ref.cpp`),
- `packed-chosen-ref.cpp` (reject; ICS-8, the `take` pair of 1.5),
- `packed-explicit-object-by-value.cpp` (run),
- `packed-member-pointer.cpp` (reject; MPTR-9),
- `packed-member-pointer-into-member.cpp` (run; `h.inner.*pm` reads a value),
- `packed-structured-binding.cpp` (run with `auto&`, reject when a name's address is taken),
- `packed-constant.cpp` (accept; constant evaluation reads and writes a member),
- `packed-placement-any-address.cpp` (run; placement `new` at offset 3 of a byte array),
- `packed-cast-byte-read-sanitizer.cpp` (run-fail; a `T*` cast from a byte of a packed object, read under UBSan; STG-23).

## 3. Interactions

**Alignment (ALN-P1, ALN-1 to ALN-6).** P20 is the first rule that lets an object sit below its type's alignment. ALN-P1 promises alignment for objects in every kind of storage. P20 narrows the promise to pointers and references, which keep it. ALN-2's ban on a weaker `alignas` stays for everything outside a packed class. Inside one, the weaker value has a meaning, which is a third answer to Q-0069 that neither of its options names. ALN-1 already lacks bases in the class alignment (X14-2), and X14:new-3 has to name bases itself.

**Class layout (CLS-10, CLS-11, CLS-19, CLS-30 to CLS-35).** P20 replaces two grades of freedom with one fixed layout, for one kind of class. The matrix layout of CLS-35 gives the order, and P20 removes only its padding. Standard-layout status does not change, but CLS-30's usable-pointer sentence needs an exception for the first member.

**Bit-fields (BF-4, BF-9).** Bit-fields are the model. An under-aligned member has no address, no pointer to member, no non-`const` reference, and a `const` reference binds to a copy. BF-4's wording of that last point is wider than REF-2 (X14-6). BF-9 gains an item for packed allocation.

**Storage (STG-2, STG-8, STG-13, STG-14, STG-16, STG-17, STG-23).** STG-16 loses the case of an under-aligned first member. The other rules hold as written. A cast to `T*` of a byte inside a packed object points to the byte, and STG-23 makes a read undefined. P20 relies on that inference, which Tidal does not state for any misaligned pointer (X14-1).

**References and overload resolution (REF-1, REF-2, REF-8, ICS-8, ICS-13).** P20 adds the under-aligned case next to each bit-field case. The implicit object parameter of a member function is a reference, so the call is the same fault as a reference binding.

**Pointers to members (MPTR-2, MPTR-3, MPTR-4, MPTR-9).** Forming the pointer is rejected, as for a bit-field. A pointer to a member of an ordinary class still works on an under-aligned object of that class, and the result is under-aligned. The result's alignment depends on the object operand, so no conversion between pointers to members needs a new rule.

**Expressions (UN-9, PFX-1, PFX-3, RFOR-3, SB-3).** The subscript of an array member must skip the array-to-pointer conversion that PFX-1 describes. Range-based `for` and reference captures fail, and a program copies the array first.

**Unions (UNI-15, UNI-16).** The "same alignment requirement" condition of UNI-15 already separates a packed class from a plain one with the same members. Two packed classes with the same members are layout-compatible.

**Volatile (VOL-1, VOL-13).** An under-aligned volatile read is still one observable effect. The size of the access joins VOL-13.

**Attributes (ATTR-1, ATTR-P1, ATTR-P2).** These rules rule out an attribute as the marker (1.7).

**Not touched.** Conversions, `alignof` of a type, `new` and `delete` (a packed class has alignment 1 and uses the default allocation functions), and constant evaluation keep today's rules.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- `packed struct S { ... };`, `packed class C;`, `packed union U { ... };`. Both compilers reject an identifier directly before a class key today (probe `m5.cpp`).
- `alignas` weaker than the natural alignment on a member of a packed class. In the extensions, Clang rejects it, and GCC accepts it and uses the weaker value (`PW`: alignment 2, `a` at offset 2, probe `b1.cpp`). Outside a packed class, ALN-2 still rejects.

### 4.2 Undefined becomes defined

None for an existing program. A program that changes a class to a packed class and copies fields with a byte loop can read the fields directly instead. No existing undefined case becomes defined.

### 4.3 Valid programs that change meaning

None. `packed` keeps every meaning it has as a name. `struct S packed {};` stays a variable definition (probe `m2.cpp`), and `struct S packed;` stays a variable declaration (`m3.cpp`).

A test of validity can ask whether `&x.m` or a reference binding is valid, for example in a requires expression. The answer changes only for members of packed classes, which no existing program has.

Changing an existing class to a packed class changes `sizeof`, the offsets of its members, and which uses are valid. That is a change to the program, and its purpose.

### 4.4 Valid programs that become invalid

None. X14:new-2, X14:new-4, X14:new-7 and X14:new-8 apply only to packed classes.

### 4.5 Evidence

The extension probes show what today's compilers do with their own packing. They are not P20.
- `m1.cpp` to `m5.cpp`: `struct S final {};` after a definition of `S` is a redefinition on both. `struct S packed {};` and `struct S packed;` are valid variable declarations on both. `packed struct T { int a; };` is rejected on both when `packed` names a class.
- `e1.cpp` (extensions): `P { char c; int a; double d; Q q; int arr[3]; }` packed has size 29 and alignment 1, with offsets 1, 5, 13 and 17. A packed class with an `alignas(8)` member has alignment 8 and the member at offset 8. An empty packed class has size 1. A packed class with a base of alignment 4 has alignment 4 and size 12. A packed polymorphic class has size 13 and alignment 1. `#pragma pack(1)` gives the same layout as the attribute. Both compilers agree on every value.
- `r1.cpp` (extensions, `-Wall -Wextra`): both warn on `&p.a` (`-Waddress-of-packed-member`). GCC rejects a non-`const` reference to a packed member ("cannot bind packed field"), also through `auto&&`, a range-based `for`, and an overload that chooses `int&` over `long`. Clang accepts all of these. Both accept a member call on a packed member of class type, a `const` reference, a subscript, a copy, structured bindings with `auto&`, and `p.*m` for `int P::* m = &P::a`.
- `u1.cpp` (extensions, UBSan): a `const int&` to a packed member binds to a copy on GCC and directly on Clang. Both report "member call on misaligned address" for the call on a packed member of class type.
- `u2.cpp` (extensions, UBSan): forming a misaligned `int*` passes on both. A read through `&p.a`, or through a cast from `char*`, stops as "load of misaligned address" on both.
- `u3.cpp` (no extension, UBSan): a read through a misaligned `int*` cast from a `char` array stops as "load of misaligned address" on both (X14-1).
- `b1.cpp` (extensions): the bit-field layout of 1.3, and the weaker `alignas` of 4.1.
- `c1.cpp` (extensions): constant evaluation reads and writes a packed member on both.
- `v1.cpp` (extensions, `-O2 -S`): a packed `volatile int` read is one `movl` at offset 1 on both.
- `f1.cpp` to `f5.cpp`, `f5b.cpp`: support findings X14-2 to X14-7.

## 5. Findings

**X14-1** (hard to find; foundations/storage.md, declarations/alignment.md; ALN-4, STG-16, STG-17, STG-18, STG-23). No rule says that a read or write through a pointer whose address is not a multiple of the type's alignment is undefined. STG-17 names misalignment only for placement `new`. A reader gets the answer by inference: ALN-4 puts every object at an aligned address, so a cast pointer points to a byte or to the original object (STG-16), and STG-23 or STG-18 makes the access undefined. Both compilers' UBSan stop such a read as "load of misaligned address" (probe `u3.cpp`: `int* cp = reinterpret_cast<int*>(buf.b + 1); *cp`). No test covers it. Suggestion: an Evidence item under STG-23, or a sentence in ALN-4, that names the misaligned case and the rules that make it undefined, and a `-sanitizer` run-fail test like `u3.cpp`.

**X14-2** (missing; declarations/alignment.md, classes/members.md; ALN-1, ALN-P1, CLS-35). ALN-1 says "The alignment of a class is the strictest of its own `alignas` and of its members." Bases are not named, and neither is the pointer for virtual calls. CLS-35's Padding row says "the largest alignment of its pointer, bases, and members". A base raises the alignment on both compilers: `struct B { alignas(16) char c; }; struct D : B { char x; };` gives `alignof(D) == 16` and `sizeof(D) == 32` (probes `f5.cpp`, `f5b.cpp`). The ALN-1 tests cover a member (`member-raises-class.cpp`) and no base. Suggestion: ALN-1 says "of its own `alignas`, its bases, and its members", with an accept test for the base case.

**X14-3** (wrong; classes/unions.md, declarations/type-names.md, declarations/alignment.md; Syntax segments, ALN-1). ALN-1 lets `alignas` apply to "a class declaration, including a forward declaration", and alignment.md's Syntax segment says it comes "after the class key of a class definition". `class_head` (derived.md) has the slot. `union_def` (unions.md) has `{attribute_spec}` and no `alignment_specifier`. `forward_class` (type-names.md) is `class_key identifier ";"` with neither. Both compilers accept `union alignas(8) U { char c; };` and `union alignas(16) V;` (probe `f4.cpp`), and `class-forward-alignas.cpp` expects accept for a forward declaration with `alignas`. Suggestion: add `{attribute_spec | alignment_specifier}` after the key in `union_def` and `forward_class`, and a union test under ALN-1.

**X14-4** (missing; declarations/alignment.md, Q-0069; ALN-2). ALN-2 forbids an `alignas` weaker than the natural alignment "of the entity". Its tests cover a variable and a data member. No test covers a class. GCC accepts `struct alignas(1) S { int x; };` and keeps alignment 4. Clang rejects it ("requested alignment is less than minimum alignment of 4 for type 'S'", probe `f1.cpp`). Q-0069's Affects line names only the two existing tests. Suggestion: a reject test `weaker-on-class.cpp` with a GCC accept expectation, and the test added to Q-0069.

**X14-5** (unclear; declarations/alignment.md, glossary; page introduction, ALN-P1, ALN-2). The page opens with "The alignment of a type is the number of bytes that its address is a multiple of." A type has no address. "Natural alignment" appears in ALN-P1, ALN-2 and the Evidence, and nothing defines it. The glossary has "fundamental alignment" and "default new alignment", and no entry for "alignment". P20 needs both terms to define an under-aligned member. Suggestion: glossary entries. The alignment of a type is the number that the address of every object of the type is a multiple of. The natural alignment of an entity is `alignof` of its type, before any `alignas`.

**X14-6** (unclear; classes/unions.md, declarations/initialization/references.md, declarations/volatile.md; BF-4, REF-2, VOL-10). BF-4 says "A `const` reference binds to a temporary that holds a copy of the value." REF-2 limits this to a `const` reference "that is not `volatile`", and VOL-10 says a `const volatile` reference does not bind to a temporary. Both compilers reject `const volatile int& r = b.v;` for a bit-field `v` (probe `f2.cpp`; Clang says "non-const reference cannot bind to bit-field"). No test covers it. Suggestion: BF-4 says "A `const` reference that is not `volatile`", and a reject test citing BF-4 and VOL-10.

**X14-7** (missing; declarations/attributes.md; ATTR-1, ATTR-P1, ATTR-P2). ATTR-1 says an implementation ignores an attribute that it does not recognize. ATTR-P2 covers only standard attributes. No rule covers a non-standard attribute that the implementation recognizes. `struct [[gnu::packed]] P { char c; int a; };` passes `static_assert(sizeof(P) == 5)` on both compilers under `-pedantic-errors` (probe `f3.cpp`). So a valid program's layout depends on an attribute that another implementation would ignore. `unknown-attribute-ignored.cpp` uses only unknown names. Suggestion: an `impl-defined` rule, or an ATTR-1 Evidence item, that says a recognized non-standard attribute has an implementation-defined effect, with the `gnu::packed` case as its test.

**X14-8** (hard to find; foundations/storage.md; STG-17, TYP-2, ALN-1). STG-17 says "not aligned for the type ({rule}`TYP-2`)". TYP-2 gives the alignments of fundamental types only. The test `placement-misaligned-sanitizer.cpp` uses `long`, so it fits. For a class type the alignment comes from ALN-1, and for an `alignas` variable from ALN-4. Suggestion: cite ALN-1 next to TYP-2 in STG-17.

**X14-9** (outside knowledge). These come from general experience:
- the motivation, that file and network formats have no padding and that programs copy their fields one at a time,
- the knowledge that GCC and Clang offer `__attribute__((packed))`, `[[gnu::packed]]` and `#pragma pack`, and the names of the warning and UBSan checks to look for,
- the knowledge that some targets trap on a misaligned load, which is why P20 never gives a program an ordinary pointer to an under-aligned member,
- the qualifier alternative in 1.7, which resembles a qualifier that one other compiler offers.

**X14-10** (outside the spec; Q-0007, roadmap-3 section 1.1). Two needs fall outside the spec:
- Version marking. P20 takes the place before a class key. A designer cannot tell from Tidal whether a later version gives that place a meaning.
- Implementation cost. Every read of an under-aligned member may need an unaligned load or a sequence of byte loads on a strict target. Tidal has no model to state this cost in.
