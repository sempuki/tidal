// tidal-rules: CTAD-14
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-note: The copy candidate, a user guide, and the aggregate candidate of the class template all reach the alias (Q-0240).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { template <class U> constexpr Box(U) {} };
template <class T> Box(T*) -> Box<T>;
template <class T> using Alias = Box<T>;
int n = 0;
Alias a(&n);
static_assert(same<decltype(a), Box<int>>);
constexpr Box<char> one(1);
Alias b(one);
static_assert(same<decltype(b), Box<char>>);
template <class T, class U> struct Agg { T first; U second; };
template <class U> using IntAgg = Agg<int, U>;
IntAgg c{1, 'c'};
static_assert(same<decltype(c), Agg<int, char>>);
