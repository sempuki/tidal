// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-note: Q-0086. Clang accepts a call to a function that returns an abstract class. GCC and EDG reject it.
struct A { virtual void f() = 0; };
A make();
void g() { make(); }
