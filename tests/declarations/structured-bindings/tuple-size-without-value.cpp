// tidal-rules: SB-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: tuple_size<.S.>::value is not a valid integral constant-expression
// tidal-note: Gap review 2026-10-30. Q-0048. EDG rejects a complete std::tuple_size without value; Clang and GCC fall back to data members.
namespace std { template <class T> struct tuple_size; }
struct S { int a; int b; };
template <> struct std::tuple_size<S> {};
void f() { auto [x, y] = S{1, 2}; }
