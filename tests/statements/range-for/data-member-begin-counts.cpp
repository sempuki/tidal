// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: called object type .int. is not a function
// tidal-error-gcc: expression cannot be used as a function
// tidal-note: Q-0098. EDG uses the member form only for member functions. Clang and GCC count data members.
// tidal-note: Gap review 2026-10-31 (third).
namespace r {
struct Odd { int arr[3] = {1, 2, 3}; int begin = 0; int* end() { return arr + 3; } };
int* begin(Odd& o) { return o.arr; }
int* end(Odd& o) { return o.arr + 3; }
}
int f() { r::Odd o; int s = 0; for (int v : o) s += v; return s; }
