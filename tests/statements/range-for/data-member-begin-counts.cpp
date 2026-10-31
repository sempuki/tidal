// tidal-rules: RFOR-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0098. EDG uses the member form only for member functions. Clang and GCC count data members.
// tidal-note: Gap review 2026-10-31 (third).
namespace r {
struct Odd { int arr[3] = {1, 2, 3}; int begin = 0; int* end() { return arr + 3; } };
int* begin(Odd& o) { return o.arr; }
int* end(Odd& o) { return o.arr + 3; }
}
int f() { r::Odd o; int s = 0; for (int v : o) s += v; return s; }
