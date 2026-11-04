// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: inline-alias-differs-across-units.tu.cpp
// tidal-note: Propose exercise X20. The inline function has the same tokens in both units, but the alias Count names int in one unit and long long in the other. The definitions do not match, and no compiler or linker reports it. The test accepts either value.
using Count = int;
inline int width() { return sizeof(Count); }
int width_elsewhere();
int main() {
  int a = width(), b = width_elsewhere();
  return (a == sizeof(int) || a == sizeof(long long)) && (b == sizeof(int) || b == sizeof(long long)) ? 0 : 1;
}
