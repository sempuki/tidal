// tidal-rules: CTAD-10
// tidal-expect: accept
// tidal-note: The bound 3 does not depend on a parameter, so the braces of the array are elided, also for a string literal. Each element of the list initializes one element of the array, and "ab" gives T = const char* (Q-0277).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Three { T v[3]; };
Three numbers{1, 2, 3};
Three words{"ab"};
template <class T> struct Mixed { T v[3]; int n; };
Mixed m{1, 2, 3, 4};
static_assert(same<decltype(numbers), Three<int>> && same<decltype(words), Three<const char*>>);
static_assert(same<decltype(m), Mixed<int>>);
