// tidal-rules: CTAD-1 DEP-19
// tidal-expect: accept
// tidal-note: RV11-R-2. The type of a functional cast is typename T::Cell with no argument list, under a dependent qualifier. After substitution the name is a class template, and its arguments are deduced. Every compiler accepts the cast. The read of the RV10 repair diff reads the form as not valid, and pending.md row 4 records the read, as for the declaration form of Q-0243.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Kit { template <class U> struct Cell { U value; constexpr Cell(U v) : value(v) {} }; };
template <class T> constexpr auto make() { return typename T::Cell(3); }
static_assert(same<decltype(make<Kit>()), Kit::Cell<int>>);
static_assert(make<Kit>().value == 3);
