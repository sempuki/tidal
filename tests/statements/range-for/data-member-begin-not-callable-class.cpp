// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: does not provide a call operator
// tidal-error-gcc: no match for call to
// tidal-note: Q-0098. The data member begin has a class type with no operator(), so the member form makes o.begin() a call that is not valid. The free functions are not used. EDG uses the member form only for member functions and takes the free functions.
namespace r {
struct Tag {};
struct Odd { int arr[3] = {1, 2, 3}; Tag begin; int* end() { return arr + 3; } };
int* begin(Odd& o) { return o.arr; }
int* end(Odd& o) { return o.arr + 3; }
}
int f() { r::Odd o; int s = 0; for (int v : o) s += v; return s; }
