#include "limits.h"

// The earlier `extern` declaration in the header keeps this definition
// external (LNK-3). `extern` with an initializer is a definition (STO-7).
// Since the header already declared it extern, `const int` alone would
// also do; the `extern` here makes it plain to a reader.
extern const int limits::build_number = 2026;
