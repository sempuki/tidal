// tidal-rules: DCL-15
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-error-gcc: cannot convert .int \(S::\*\)\(\) &&. to .int \(S::\*\)\(\).
// tidal-note: Q-0348. The function type has the ref-qualifier &&. A call through the pointer to member uses an rvalue object. GCC drops the ref-qualifier from the type of the pointer to member and rejects the initialization. Clang and EDG accept.
using G = int() &&;
struct S {
  G take;
  int v = 4;
};
int S::take() && { return v; }
G S::* pm = &S::take;
int main() { return (S{}.*pm)() == 4 ? 0 : 1; }
