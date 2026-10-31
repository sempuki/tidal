// tidal-rules: LNK-9
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
// tidal-expect-edg: reject
// tidal-error-edg: external/internal linkage conflict
// tidal-note: Q-0105. EDG rejects a block-scope extern that redeclares a hidden internal-linkage variable. Clang and GCC accept it.
static int shared = 5;
int get() { int shared = 1; (void)shared; { extern int shared; return shared; } }
int main() { return get() == 5 ? 0 : 1; }
