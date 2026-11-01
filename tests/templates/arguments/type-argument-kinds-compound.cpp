// tidal-rules: TARG-25
// tidal-expect: accept
// tidal-note: A function type, an array of unknown bound, a reference, a const type, and a closure type are type arguments.
template <class T> struct Name { static constexpr int value = 1; };
auto lambda = [] {};
static_assert(Name<int()>::value == 1);
static_assert(Name<int[]>::value == 1);
static_assert(Name<int&>::value == 1);
static_assert(Name<const int>::value == 1);
static_assert(Name<decltype(lambda)>::value == 1);
