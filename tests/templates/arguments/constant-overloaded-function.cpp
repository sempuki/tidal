// tidal-rules: TARG-12
// tidal-expect: run
// tidal-note: The overloaded name pick selects the overload whose type is the parameter's type.
template <int (*F)(int)> struct ByInt { static int run() { return F(0); } };
template <int (*F)(double)> struct ByDouble { static int run() { return F(0); } };
template <int (&F)(int)> struct ByRef { static int run() { return F(0); } };
int pick(int) { return 1; }
int pick(double) { return 2; }
int main() { return ByInt<pick>::run() == 1 && ByDouble<pick>::run() == 2 && ByRef<pick>::run() == 1 ? 0 : 1; }
