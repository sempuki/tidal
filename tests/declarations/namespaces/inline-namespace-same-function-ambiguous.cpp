// tidal-rules: NS-12
// tidal-expect: reject
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: f(int) is declared in the inline namespace v1 and again in lib. The two are different functions with the same parameters, so overload resolution cannot choose.
namespace lib {
  inline namespace v1 { int f(int) { return 1; } }
  int f(int);
}
int main() { return lib::f(1); }
