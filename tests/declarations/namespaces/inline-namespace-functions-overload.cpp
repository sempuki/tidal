// tidal-rules: NS-12 NS-3
// tidal-expect: run
// tidal-note: f(int) is declared in the inline namespace v1 and f(double) in its enclosing namespace lib. Both form one overload set, so the qualified and the unqualified calls choose by the argument.
namespace lib {
  inline namespace v1 { int f(int) { return 1; } }
  int f(double) { return 2; }
  int call() { return f(1); }
}
int main() { return lib::f(1) == 1 && lib::f(1.0) == 2 && lib::call() == 1 ? 0 : 1; }
