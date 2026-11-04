# Predict batches: foundations, reader C

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-storage (83 programs), R-language-support (58)

Both answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R-storage/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-language-support/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ:
- storage: p08, p19, p43, p44, p45
- language support: p03, p06, p16, p24, p29, p36, p39, p49, p52, p53, p58

The storage Evidence says the compilers disagree on 6 of 83 tests, and I found only 5. The language-support Evidence says 13 of 58, and I found 11 (p38 differs in exit code only, and both codes are nonzero).

## RPC-1

- Pages and rules: storage, STG-21, STG-22, STG-19
- Kind: unclear
- What happened: The `vptr` builds are p08, p32, p45, p49, p67 and p78. The Evidence says "the vptr sanitizer of GCC reports the destructor call at the end of the block and the cast to a derived class. The Clang sanitizer reports neither. Both report a member call on storage with no object." For p08 and p45 I answered `depends` (GCC run-fail, Clang exit 0). The Evidence does not say that the Clang run then exits 0. In p32 the member call goes through the name of an ended Derived whose storage now holds a Base. That storage holds an object, so the sentence about "storage with no object" does not plainly cover it. I answered run-fail.
- Suggestion: Name the test file for each sentence in this Evidence item, and give the outcome on each family for each one.

## RPC-2

- Pages and rules: storage, STG-12, STG-13, STG-14; volatile, VOL-6; copy-move, CPY-27
- Kind: unclear
- What happened: In p58, a struct with a `volatile int` member is copied byte by byte through `unsigned char`. CPY-27 keeps the copy functions trivial, so the type is trivially copyable, and STG-12 allows the byte copy. VOL-6 says any access to a volatile object through a glvalue that is not volatile is undefined. A byte read of the volatile member looks like such an access. I answered runs.
- Suggestion: In STG-12 or VOL-6, say whether a read of a volatile object's bytes through a byte glvalue that is not volatile counts as an access under VOL-6.

## RPC-3

- Pages and rules: storage, STG-11, STG-25, STG-12
- Kind: unclear
- What happened: In p50, the bytes of a whole `Base` are copied into the base subobject of a `Derived`, and a member of `Derived` lies in the tail padding of the base. The source's padding byte has no value (STG-11), so after the copy the member `extra` holds a byte with no value, and the program then reads `extra` as `char`. The page does not say whether that read is defined. I answered runs, because the test seems to check only that the member changed.
- Suggestion: Add a sentence to STG-25 that gives the value of a derived member after a byte copy writes a padding byte over it.

## RPC-4

- Pages and rules: storage, STG-1; declarators, DCL-19
- Kind: hard to find
- What happened: p06 declares an array whose bound fits in `std::size_t` but whose total size in bytes does not. STG-1 says it covers arrays, and DCL-19 checks only the bound. The STG-1 Evidence reports compiler results for classes only, and shows the compilers split on classes. I guessed that every compiler rejects the array.
- Suggestion: Add a line to the STG-1 Evidence on an array whose total size overflows, with the result on each compiler.

## RPC-5

- Pages and rules: storage, STG-1
- Kind: unclear
- What happened: For p43 (`-m32`) the Evidence says Clang "accept[s] a class larger than std::size_t holds". The program also takes `sizeof` of that class in a `constexpr` variable. On x86-64, GCC accepts the class and rejects `sizeof`, so I could not tell whether Clang under `-m32` accepts the `sizeof` too. I answered `depends` for p43 and p44.
- Suggestion: For each target, say what each compiler does with the class definition and with `sizeof` of it, as two separate results.

## RPC-6

- Pages and rules: unions, UNI-10, UNI-15
- Kind: unclear
- What happened: In p52, the program reads `s.span.range.second` while `line` is active. Here `range` is a member in the common initial sequence, and `second` is a member of `range`. UNI-10 allows a read of "a member of the other one" that lies in the common initial sequence, and it does not say whether a member of such a member counts. I answered runs.
- Suggestion: Add an example to UNI-10 with nested structs, and say whether a member of a member in the common initial sequence can be read.

## RPC-7

- Pages and rules: storage, STG-8, STG-29, STG-P3
- Kind: needs an example
- What happened: p65 casts a byte array's address to `int*`, then writes and reads through the result. STG-8 says the start of a byte array's lifetime creates objects implicitly, and the STG-P3 table puts that next to the allocation call. The page's example shows that a `static_cast` of an allocation result reaches the created objects. Only STG-29 says a cast of the array's address does not reach them. As a new reader, I expected the two cases to behave alike. I answered undefined because of STG-29.
- Suggestion: In STG-P3, add an example that shows the two cases side by side. A cast of `::operator new`'s result reaches the implicit object, and a cast of a byte array's address does not.

## RPC-8

- Pages and rules: language support, LSUP-8; new and delete, NEW-35, NEW-36
- Kind: hard to find
- What happened: The language-support page has programs with an erroneous array bound (p06, p36, p38, p39, p58). It says only that "Clang throws std::bad_alloc for an erroneous array bound". I could predict the exit codes only from the NEW-35 table on the new-and-delete page. That table gives what each family does for each erroneous case: whether the allocation function is called, and with which size. For p38, both families fail but with different codes, so I answered run-fail.
- Suggestion: In the LSUP-8 Evidence, point to the NEW-35 table and the NEW-36 Evidence item.

## RPC-9

- Pages and rules: language support, LSUP-6; new and delete, NEW-37
- Kind: hard to find
- What happened: p24 uses `new (std::nothrow)` during constant evaluation. The language-support page does not say whether the nothrow forms are valid there. NEW-37 lists them as valid, and its Evidence says GCC 14 and EDG reject them. I answered `depends`.
- Suggestion: Add a sentence to LSUP-6 that links to NEW-37 and Q-0303.

## RPC-10

- Pages and rules: language support, LSUP-2
- Kind: unclear
- What happened: In p26, `strong_ordering` lacks the member `equal`, and the program evaluates only `1 <=> 2`, so the result is `less`. LSUP-2 requires all four members. The Evidence does not say whether the compilers check members that the use does not need. I answered reject.
- Suggestion: Add an Evidence line on a missing member that the operation does not yield.

## RPC-11

- Pages and rules: language support, LSUP-2
- Kind: missing
- What happened: In p27, the static members hold the values -7, 5, 6 and 9, and the program checks that `<=>` gives those values, also at run time. The page does not say whether the implementation copies the program's static members or builds the result from fixed values such as -1, 0 and 1. I guessed that it copies the static members, and answered runs.
- Suggestion: State in LSUP-2 that the result of `<=>` on built-in operands is a copy of the matching static member.
