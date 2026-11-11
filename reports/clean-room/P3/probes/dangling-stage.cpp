// Probe: a C#-style stage that holds its source by reference. The source
// Nat{} is a temporary that ends with the full expression (OBJ-9).
// tidal-fe: undefined [STG-22, OBJ-2] at line 6. g++ and clang++ with
// -fsanitize=address report stack-use-after-scope.
struct Nat { int i = 0; bool next(int& o) { o = ++i; return true; } };
template <class Src> struct Odd { Src& src; bool next(int& o) { while (src.next(o)) if (o % 2) return true; return false; } };
template <class Src> Odd<Src> odd(Src&& s) { return Odd<Src>{s}; }
int main() { auto s = odd(Nat{}); int x = 0; s.next(x); s.next(x); return x - 3; }
