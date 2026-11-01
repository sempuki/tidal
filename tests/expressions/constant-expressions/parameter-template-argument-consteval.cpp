// tidal-rules: CE-30 CE-3
// tidal-expect: reject
// tidal-error-gcc: .n. is not a constant expression
// tidal-error-clang: non-type template argument is not a constant expression
// tidal-error-edg: parameter .n.
// tidal-note: The template argument starts its own constant evaluation, also inside a consteval function, so the parameter n is not usable there.
template <int N> struct Box { static constexpr int value = N; };
consteval int twice(int n) { return Box<n>::value * 2; }
int main() { return twice(1) - 2; }
