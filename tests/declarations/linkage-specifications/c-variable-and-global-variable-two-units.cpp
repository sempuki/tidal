// tidal-rules: LSP-8
// tidal-expect: run
// tidal-sources: c-variable-and-global-variable-two-units.tu.cpp
// tidal-note: This unit declares shared as a C variable, and the other unit defines a global variable shared with C++ language linkage. The two declarations name one entity and clash in language linkage. Neither unit sees the other, so no diagnostic is required. Clang and GCC link the two as one variable.
extern "C" int shared;
int main() { return shared == 4 ? 0 : 1; }
