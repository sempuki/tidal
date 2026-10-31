# Q-0105: Does a block-scope extern declaration redeclare a hidden internal-linkage variable?

**Status:** provisional
**Affects:** LNK-9, `tests/declarations/storage-linkage/extern-in-block-hidden.cpp`

## Context

A namespace-scope `static` variable is hidden by a local variable of the same name. An inner block declares the name `extern`. Clang and GCC accept the declaration and read the namespace-scope variable. EDG 7.0 rejects it.

## Options

1. The declaration redeclares the namespace-scope entity and takes its internal linkage.

2. The declaration is not valid when the entity is hidden.

## Provisional answer

Option 1. Clang and GCC agree.

## Opinions
