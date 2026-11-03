# Predict batches: classes, reader A

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-constructors (166 programs), R-initializer-lists (31), R-access (63)

All three answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R-constructors/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-initializer-lists/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-access/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ:
- constructors: p50, p55, p58, p60, p112, p117, p140, p141, p157, p159
- initializer lists: p27
- access: p02, p56

## RCA-1

- Batch and program: R-constructors p38
- Rule IDs: CTOR-3, CTOR-20, DTOR-3
- What I needed: The program does `new S`, and S has a member whose destructor is deleted. DTOR-3 says a `new` expression for such a class is valid. CTOR-3 says a member with a deleted destructor deletes the class's default constructor, which would make `new S` invalid. I answered reject because of CTOR-3. The two rules seem to pull in opposite directions, and only a careful reader sees that CTOR-3 decides this case.
- Suggestion: In DTOR-3, add a sentence or example. It should say that the `new` exemption covers the class's own destructor, and that a member's deleted destructor still deletes the default constructor that `new S` needs.

## RCA-2

- Batch and program: R-constructors p45 and p77
- Rule IDs: DTOR-3
- What I needed: Each program has a `new` expression for a class whose destructor is deleted. In p45 the initializer is a function call that returns that class by value. In p77 the initializer is a braced list of the member values. DTOR-3 says the `new` expression is valid. It also says any prvalue of the class is invalid, "also where the prvalue initializes an object directly". I could not tell which part wins for the initializer of a `new`. I answered reject for p45 and compiles for p77.
- Suggestion: State whether a prvalue of the class inside a `new` initializer is allowed, with one example of each form.

## RCA-3

- Batch and program: R-constructors p41, p97, p122, p148; also p112
- Rule IDs: CE-9, CTOR-12, DTOR-7
- What I needed: CE-9 points to a table of the undefined rules and the compilers that reject each one during constant evaluation. The table is a generated directive, and it does not render in the copy of the specification. So I could not confirm that every compiler rejects these during constant evaluation:
  - a read of a member that has no value yet,
  - a member call made after the destructor has run,
  - a member call made from a base initializer.

  The DTOR-7 Evidence mentions only EDG 7.0 and only for a second destruction.
- Suggestion: Render the undefined-rule table in the exercise copy. Or add one line to the Evidence of CTOR-12 and DTOR-7 that says how constant evaluation treats each case.

## RCA-4

- Batch and program: R-constructors p105, also p36
- Rule IDs: CTOR-13
- What I needed: The build uses `-Wall -Werror`, so the outcome depends on whether the compilers warn when the initializer list order differs from the declaration order. CTOR-13 is only a recommendation, and the Evidence does not say which compilers warn under which flags. I guessed that all of them warn, so p105 is rejected.
- Suggestion: Record in the Evidence which compilers give that warning under `-Wall` (or by default).

## RCA-5

- Batch and program: R-constructors p50, p140, p141, p157
- Rule IDs: CTOR-24, CTOR-4
- What I needed: The CTOR-24 table ends with "Each row follows the majority." The table has four compiler columns, the baseline has eight compilers, and some rows split two against two (p140). I could not tell whether "majority" means the test expects one outcome or expects disagreement. For p157 I also could not tell whether the rule text itself rejects the case. I answered `depends` for all four.
- Suggestion: For each row, give the outcome the tests expect. Say how a tie is counted.

## RCA-6

- Batch and program: R-constructors p32
- Rule IDs: CTOR-5
- What I needed: The member initializer `v(v + 1)` uses a parameter and a member with the same name. I needed to know that the name inside the parentheses finds the parameter, while the name before the parentheses finds the member. The constructors page does not say how names are looked up in a member initializer.
- Suggestion: Add a sentence to CTOR-5 on name lookup in a `mem_init`, and point to the names page.

## RCA-7

- Batch and program: R-constructors p96
- Rule IDs: NAM-31, CTOR-25
- What I needed: A call whose name finds a class by ordinary lookup, while argument-dependent lookup could find a function of that name. I found NAM-31 on the names page, which says argument-dependent lookup does not run when ordinary lookup finds a type. The constructors page does not link to that rule, and a reader of that page would expect a functional cast to look like any other call.
- Suggestion: Under CTOR-25, add a cross-reference to NAM-31 for a call whose name is a class.

## RCA-8

- Batch and program: R-constructors p126
- Rule IDs: CTOR-25, CTOR-10, ILC-3
- What I needed: A single program holds seven initializations, among them a braced list nested inside braces and a class that converts to the target type. CTOR-25's Evidence says all of them are accepted in general terms. I could not trace why the nested braced list picks the one-int constructor over the implicit copy constructor. That needs the rule about user conversions for a single nested braced element, and no page I read gives it.
- Suggestion: Add an example to ILC-3 or CTOR-25 with a nested braced list of one element, naming the constructor that is chosen and the rule that decides it.

## RCA-9

- Batch and program: R-initializer-lists p03
- Rule IDs: ILC-3, ILC-4
- What I needed: The initializer-list constructor takes int and the list holds 1.5. ILC-4 says narrowing does not move the selection to another constructor. ILC-3 does not say whether a narrowing element still makes the initializer-list constructor viable in the first phase. If it does not, the double constructor would win. I answered reject.
- Suggestion: In ILC-3 or ILC-4, state that a narrowing conversion still counts as viable when the phase chooses its constructor.

## RCA-10

- Batch and program: R-initializer-lists p11
- Rule IDs: ILC-8, ICS-19, ICS-8
- What I needed: Overload resolution between two functions, where a braced argument converts to two classes and one of the two constructors is explicit. The second argument breaks the tie toward the function whose class has the explicit constructor. ILC-8 cites Q-0132, but the Evidence records no compiler split for that question. I answered reject and was unsure whether the compilers agree.
- Suggestion: Say in the Evidence whether Q-0132 has a compiler split, or drop the citation if it has none.

## RCA-11

- Batch and program: R-access p40
- Rule IDs: ACC-12, NAM-30
- What I needed: A function is found through the namespace of a private base, and the call then needs a conversion to that base. The access page does not mention this case. The names page covers it in one sentence at the end of NAM-30.
- Suggestion: Add a cross-reference from ACC-12 to that sentence of NAM-30.

## RCA-12

- Batch and program: R-access p07 and p58
- Rule IDs: ACC-4
- What I needed: `&D::x` names a member that is declared in B. I needed to know that the result still converts to a pointer to a member of D, so the function's return type `int D::*` is valid. ACC-4 gives the access rule but not the type of the result.
- Suggestion: Add one example to ACC-4 that shows the type of `&D::x` and the conversion to the return type.
