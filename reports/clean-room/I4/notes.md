# I4 notes, Phase 1 (written before any test was read)

Inputs: specification/expressions/conditional.md, the glossary,
declarations/initialization/references.md, overloading/conversion-sequences.md,
overloading/overload-resolution.md (OVL-P4, OVL-7 to OVL-17),
classes/conversion-functions.md, classes/constructors.md (CTOR-9, CTOR-10,
CTOR-22), foundations/conversions.md (CONV-17, CONV-19), classes/access.md
(ACC-2, ACC-12), declarations/functions/deleted.md (DEL-1),
classes/derived.md (not needed in the end). Every page was read through
filter.py, which drops each "## Evidence" segment with its subsections and
each line that holds ":tests:".

## Filter slip

The first version of filter.py ended the Evidence skip at any heading, so
the "### Divergences" subsection under "## Evidence" would have printed.
I found the bug by listing the headings of conditional.md (headings only)
before reading any filtered page, and fixed it. No Evidence text or
:tests: line reached me. Each filtered page was checked with grep for
"tests:" and "Evidence" (0 hits).

## Steps the text did not give me

1. COND-3 rows 3 and 4 (conditional.md). "Two xvalues" and "An lvalue and
   an xvalue" do not say that the operands must have one type, or how
   cv-qualifiers combine. Taken literally, row 4 matches an `int` lvalue
   and a `long` xvalue. I read both rows as needing one type apart from
   cv-qualifiers. For row 3 I gave the larger set of cv-qualifiers, by
   analogy with row 1. For class operands I required identical types and
   sent the rest to COND-16, because COND-16 covers "different types".
   Kind: unclear.

2. COND-11 and COND-P2, "different types". COND-P2 says two lvalues of
   one class type with unrelated cv-qualifiers (`const A`, `volatile A`)
   are rejected. COND-11 starts overload resolution whenever "the
   operands have different types", at least one is a class, and neither
   converts. `const A` and `volatile A` meet all three, so a class `A`
   with `operator int()` seems to reach the `(int, int)` candidate. I
   followed COND-P2 and reject. Kind: unclear.

3. COND-16 steps 1 and 2, `explicit` functions. Step 3 asks for an
   implicit conversion sequence, which I read as a copy context (ICS-45:
   a parameter is copy-initialized; CTOR-9; CVF-11). Steps 1 and 2 say
   only "binds directly". I excluded `explicit` conversion functions in
   steps 1 and 2 too. Kind: outside knowledge.

4. COND-16 step 2 and a conversion function that returns a non-class
   value. COND-16 says a class prvalue never binds directly in steps 1
   and 2. It says nothing of an `operator int()` against an `int` xvalue.
   OVL-15 says an rvalue reference takes conversion functions that return
   a value. The glossary entry "bind directly" says no temporary is in
   between, so I excluded every prvalue result in steps 1 and 2. Kind:
   hard to find.

5. COND-16 step 1 and an inaccessible or ambiguous base. REF-1 says an
   lvalue reference binds directly to a class derived "through an
   accessible base". ICS-8 says the sequence forms in spite of an
   inaccessible base. COND-16 says access does not stop step 3, and says
   nothing about step 1. I let step 1 bind and then reject for the base
   (CONV-17, ACC-12). Step 3 would give the same rejection. Kind: unclear.

6. COND-16 "Otherwise" in step 3. When `Y` is an lvalue and step 1 does
   not bind, the text can mean that step 3 is still tried, or that step 1
   "applies" and decides "does not convert". I read it as: step 3 is
   tried. Kind: unclear.

7. COND-11 against OVL-11. OVL-11 says that after a built-in candidate
   wins, only the conversion function applies and then the operator's
   own rules apply. For `?:` that would let two classes that convert to
   `int&` give an `int` lvalue by row 1. COND-11 says each candidate
   gives a prvalue. I followed COND-11. Kind: unclear.

8. COND-11, the set of `T` for the `(T, T)` candidates. The set is
   infinite. The text does not say which pointer types to try, or how a
   pointer from one operand and a different pointer from the other meet
   (`int*` and `const int*`). I tried every pointer type that a
   conversion function or a non-class operand gives, plus the composite
   pointer type of each pair. Kind: missing.

9. COND-11, an operand with several conversion functions. I used ICS-7
   and ICS-37 to pick a function for each parameter, ICS-25 to compare
   two candidates (different functions tie), and treated a deleted or
   private function as viable that then fails (ICS-8, DEL-1, ACC-2).
   Kind: hard to find (spread over four rules on two pages).

10. COND-8, the xvalue case. COND-8 names only an lvalue and a prvalue of
    one class type. An lvalue and an xvalue of one class type give a
    prvalue (row 4), and the xvalue must be moved. I checked the move
    constructor for an xvalue (falling back to the copy constructor when
    the class has none) and the copy constructor for an lvalue. Kind:
    missing.

11. COND-16 step 3 copies. The "one class type" row makes a `const Box`
    prvalue from a `Box` lvalue, and the base row makes a `Base` prvalue
    from a `Derived` lvalue. The text does not say that these copies must
    be valid. I required the copy (or move) constructor of the target
    class. Kind: missing.

12. CONV-19 details for character types (`char16_t`, `char32_t`,
    `wchar_t`, `char8_t`), and the widths that decide signed against
    unsigned. I used an LP64 model. Kind: outside knowledge.

13. COND-3 rows 8 and 9 name "the literal 0" and a null pointer constant.
    The component takes a flag on the operand. Which other expressions
    are null pointer constants is not on the page. Kind: hard to find.

14. COND-16, ambiguous step 3 when the other operand also converts. The
    text says both "both convert: not valid (COND-5)" and "ambiguous
    sequence: not valid". I report COND-5 when both convert, else
    COND-16. Kind: unclear (which rule to cite).

15. CVF-4: a conversion function to its own class or a base is never
    used. COND-16 does not mention it. I applied it everywhere. Kind:
    hard to find.

16. How a constructor with a reference first parameter takes the
    operand. I used REF-1/REF-2/REF-3 for direct binding, a temporary for
    a `const&` or `&&` from a standard conversion, and no user-defined
    step before a constructor (ICS-10). Comparing a constructor against a
    conversion function uses the argument binding first (ICS-7), with
    ICS-28 and ICS-30. Kind: none (given), listed for the record.

## Model limits

- Constructors are single-argument (other parameters defaulted).
- Templates, `explicit(bool)`, abstract classes, and functions as
  operands are not modelled.
- Access is "public" or not, seen from outside the class.
