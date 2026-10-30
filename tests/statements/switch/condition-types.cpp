// tidal-rules: SW-2
// tidal-expect: accept
enum E { A, B };
enum class F { C, D };
struct Wrap { operator int() const { return 1; } };
void f(char c, E e, F g, bool b, long long l, Wrap w) {
  switch (c) { case 'a': break; }
  switch (e) { case A: break; case B: break; }
  switch (g) { case F::C: break; }
  switch (b) { case true: break; }
  switch (l) { case 1LL << 40: break; }
  switch (w) { case 1: break; }
}
