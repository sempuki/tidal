// tidal-rules: CTAD-9 CTAD-27
// tidal-expect: accept
// tidal-note: The base list F... is a trailing pack, so it takes every element of the list, one base per element.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct ByInt { constexpr int operator()(int) const { return 1; } };
struct ByDouble { constexpr int operator()(double) const { return 2; } };
template <class... F> struct Overloads : F... { using F::operator()...; };
constexpr Overloads all{ByInt{}, ByDouble{}};
static_assert(same<decltype(all), const Overloads<ByInt, ByDouble>>);
static_assert(all(1) == 1 && all(1.0) == 2);
