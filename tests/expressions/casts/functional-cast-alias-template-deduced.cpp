// tidal-rules: CTAD-1 CTAD-14
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-note: RV11-R-2. The type of a functional cast is an alias template with no argument list, with parentheses and with braces. The arguments of the class template are deduced through the alias. Clang 18 has no deduction through an alias template (Q-0240).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Cell { T value; constexpr Cell(T v) : value(v) {} };
template <class T> using Slot = Cell<T>;
constexpr auto small = Slot(3);
constexpr auto wide = Slot{2.5};
static_assert(same<decltype(small), const Cell<int>>);
static_assert(same<decltype(wide), const Cell<double>>);
static_assert(small.value == 3);
