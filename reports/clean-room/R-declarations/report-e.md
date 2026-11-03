# Predict batches: declarations, reader E

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-linkage-specifications (32 programs), R-namespaces (43 programs), R-storage-linkage (57 programs).

## RDE-1

- Batch and program: R-linkage-specifications p03
- Rule IDs: LSP-6
- What I needed: The program assigns a C function to a plain function pointer. LSP-6 says that a pointer to a C function does not convert to a pointer to a C++ function where the type records linkage. That reads as a reject on EDG. The Evidence lists EDG rejects only in the other direction (a C++ function passed where a C function type is expected). I could not tell whether EDG rejects this direction too, so I answered `depends` as a guess.
- Suggestion: State in LSP-6 or the Evidence what each compiler does when a C function initializes a pointer to a C++ function type.

## RDE-2

- Batch and program: R-linkage-specifications p04
- Rule IDs: LSP-6, LSP-1
- What I needed: The program declares two overloads that differ only in the linkage of their parameter's function type. On compilers that do not record the linkage, I had to work out that the second declaration redeclares the first and is still valid.
- Suggestion: Add a sentence to LSP-6 that says what happens where the types are the same: the second declaration names the same function.

## RDE-3

- Batch and program: R-linkage-specifications p29
- Rule IDs: LSP-1
- What I needed: The page names `"C++"` as a valid language string. It does not say directly that `extern "C++"` at namespace scope matches a declaration with no linkage specification.
- Suggestion: State in LSP-1 that `extern "C++"` gives the default linkage, so it changes nothing outside an `extern "C"` block.

## RDE-4

- Batch and program: R-storage-linkage p27
- Rule IDs: LNK-3, LNK-1
- What I needed: Both files include `shared-limit.h`, and that header is not in the batch directory. As given, the include would fail and the program would be rejected. I guessed `runs`, assuming the header declares `one_limit` with external linkage, `each_limit` as a plain `const`, and `far_limit` as `extern const`.
- Suggestion: Copy helper headers into the exercise directory, or list them in the README.

## RDE-5

- Batch and program: R-storage-linkage p48
- Rule IDs: STO-11, UNI-9
- What I needed: The storage-linkage Evidence lists only the LNK-9 divergence, but its count says two tests disagree. I found the second one, Clang accepting an anonymous union without `static` in an unnamed namespace, only in the Evidence of the unions page.
- Suggestion: Name the Q-0045 divergence in the Evidence of the storage-linkage page too, since STO-11 cites Q-0045.

## RDE-6

- Batch and program: R-storage-linkage p31
- Rule IDs: STO-9
- What I needed: The initializer of a block `static` calls its own function again. STO-9 makes this undefined. The page does not say what Clang and GCC do in practice, for example whether the run stops with an error or loops.
- Suggestion: Add a line to the Evidence on what the baseline compilers do with a recursive block-static initialization at `-O0`.

## RDE-7

- Batch and program: R-namespaces p18, R-namespaces p37, R-storage-linkage p14
- Rule IDs: NS-6, LNK-10, LNK-8
- What I needed: The namespaces page sends the reader to LNK-10 for storage classes on qualified definitions. To decide that `static` on a qualified definition is valid only when the first declaration is also `static`, I had to read the storage-linkage page.
- Suggestion: Add to NS-6 a short example of a valid and an invalid `static` qualified definition.

## RDE-8

- Batch and program: R-namespaces p40
- Rule IDs: NS-8, NS-17
- What I needed: A using directive inside a function, with a global variable of the same name. NS-8 says lookup finds the members at the level of the innermost namespace that contains both the directive and the named namespace. For a directive in a function, that namespace is the global namespace, so the use is ambiguous. I needed to work this out myself, because the page's example uses a directive at global scope.
- Suggestion: Add an example with a directive in a function body that conflicts with a global name.
