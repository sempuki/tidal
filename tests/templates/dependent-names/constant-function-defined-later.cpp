// tidal-rules: DEP-31
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-error-clang: must be initialized by a constant expression
// tidal-note: limit is declared and not defined at the template's definition, and defined before the use. GCC and EDG accept the program, Clang rejects the constant at the definition. DEP-31 needs no diagnostic, so both results are allowed (Q-0250).
constexpr int limit();
template <class T> int size() { constexpr int n = limit(); return n; }
constexpr int limit() { return 3; }
int main() { return size<int>() == 3 ? 0 : 1; }
