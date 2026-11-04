// tidal-rules: LSP-8
// tidal-expect: run
// tidal-sources: global-variable-and-c-variable-two-units.tu.cpp
// tidal-note: This unit declares a global variable level with C++ language linkage, and the other unit defines level as a C variable. The clash is across units, so no diagnostic is required. Clang and GCC link the two as one variable.
extern int level;
int main() { return level == 6 ? 0 : 1; }
