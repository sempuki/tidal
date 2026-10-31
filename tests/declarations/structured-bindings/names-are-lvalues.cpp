// tidal-rules: SB-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using A = int[2];
int main() { auto&& [a, b] = A{1, 2}; a = 5; static_assert(same<decltype((a)), int&>); return a == 5 ? 0 : 1; }
