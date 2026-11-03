// tidal-rules: LAM-13
// tidal-expect: accept
// tidal-note: The call operator of a static lambda is a static member function, so a call through the class name needs no object.
constexpr auto add = [](int a, int b) static { return a + b; };
using Add = decltype(add);
static_assert(Add::operator()(2, 3) == 5);
constexpr int (*p)(int, int) = &Add::operator();
static_assert(p(4, 5) == 9);
