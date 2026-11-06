// A lambda with a template head passed to a function template at
// namespace scope. A lambda with an `auto` parameter in its place works.
// g++ and clang++ accept, and the program exits 0.
// tidal-fe: unsupported at line 7: check: constant initializer: call
// needing template deduction or unknown arguments.
template <class F> constexpr int g(F) { return 1; }
constexpr int v = g([]<class T>() {});
int main() { return v - 1; }
