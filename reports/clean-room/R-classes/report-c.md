# Predict batches: classes, reader C

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-operators (98 programs), R-comparisons (74), R-conversion-functions (64)

I wrote all three answer files:

- /home/ryanm/Code/tidal/build/exercises/R-operators/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-comparisons/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-conversion-functions/answers.json

Each file parses as JSON and has one entry per program.

My `depends` answers match the disagreements that the Evidence sections record:

- R-operators: p75.
- R-comparisons: p13, p21, p35, p37, p68.
- R-conversion-functions: p16.

## RCC-1

- Batch and program: R-operators p73
- Rules: OVL-10, OVL-9, OPR-15
- What I needed: The program has an unscoped enumeration with a non-member `operator==(const E&, const E&)`. OVL-10 removes a built-in candidate when a non-member has "the same parameter types", but it does not say if `const E&` counts the same as `E`. If the built-in `(E, E)` stays, the two candidates seem to tie. I guessed reject (ambiguous).
- Suggestion: Say in OVL-10 if reference parameters count as the same type as the built-in parameters. Add an example with an enumeration.

## RCC-2

- Batch and program: R-operators p26
- Rules: OPR-10
- What I needed: OPR-10 says a literal operator "can be declared" at namespace scope or as a friend. It does not say that a declaration at block scope is invalid. I guessed reject.
- Suggestion: Write the rule as a limit. A literal operator must be declared at namespace scope or as a friend, and a block-scope declaration is not valid.

## RCC-3

- Batch and programs: R-operators p21 and p92
- Rules: OPR-6, NAM-27, NAM-31
- What I needed: In both programs, a nearer declaration of `operator+` hides the global one, and argument-dependent lookup does not find it. In p21 the nearer declaration is at block scope, and in p92 it is in an inner namespace. OPR-6 says only "found by ordinary and argument-dependent lookup". I had to find the table in NAM-P7 to learn that a block-scope declaration turns off argument-dependent lookup.
- Suggestion: Add a sentence or an example to OPR-6. A nearer operator function hides an outer one for an operator expression, the same as for a call by name.

## RCC-4

- Batch and program: R-comparisons p61
- Rules: DCMP-4, DCMP-5, DCMP-11
- What I needed: DCMP-3 counts `!=` as a secondary operator. DCMP-4 says a defaulted secondary operator calls `<=>`, and DCMP-5 deletes it when the class has no usable `<=>`. Read literally, a defaulted `!=` in a class with only a defaulted `==` is deleted, so I answered reject. My general knowledge says a defaulted `!=` uses `==`.
- Suggestion: Say what a defaulted `!=` uses, and when it is deleted, apart from the relational operators.

## RCC-5

- Batch and programs: R-comparisons p38 and p53
- Rules: DCMP-7, OVL-32, OVL-36
- What I needed: DCMP-7 gives "an `==` that is not `const`" as the example of an ambiguous reversed candidate. The rule does not explain why that case ties and the `const` case does not. I worked out that each candidate is better for one argument, then guessed that p38 warns and p53 does not.
- Suggestion: Add a short example that shows the two calls side by side and which one draws the warning. Say if the warning needs `-Wall`.

## RCC-6

- Batch and program: R-comparisons p44
- Rules: DCMP-13
- What I needed: DCMP-13 says the search for a matching `!=` of a member `==` is "member lookup in the class of the object operand". It does not say if a `!=` declared in a base class counts. I guessed that it does, so I answered reject.
- Suggestion: State if a `!=` inherited from a base matches an `==` declared in the derived class, and add a test note.

## RCC-7

- Batch and programs: R-comparisons p23 and p29
- Rules: DCMP-5, DCMP-3, DCMP-11
- What I needed: A defaulted `<=>` declared to return `int` is "deleted" under DCMP-5. DCMP-3 lists return-type limits that cause a reject. I could not tell at first if the declaration alone is an error. I answered that p23 (declared, not used) compiles and that p29 (used) is rejected.
- Suggestion: Say plainly that a non-category return type on a defaulted `<=>` is valid to declare and only a use is an error.

## RCC-8

- Batch and program: R-conversion-functions p61
- Rules: CVF-3
- What I needed: CVF-3 lists `T(e)`, `T{e}`, and `static_cast` as direct initialization. It does not say if a declaration with braces, such as a variable of type `int` initialized from a class object in braces, counts. I guessed that it does, so the program compiles.
- Suggestion: Name direct-list-initialization of a variable in CVF-3. Add the brace form to the examples.

## RCC-9

- Batch and program: R-conversion-functions p49
- Rules: CVF-1, REF-5
- What I needed: The example on the page says the reference dangles after the temporary dies. The program then casts the reference to `void`. Nothing on the page says if that cast reads the object. I guessed that it does not read, so the program runs.
- Suggestion: Add a note or a cross-reference that says if naming a dangling reference without reading its value is undefined.
