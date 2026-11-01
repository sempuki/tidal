// tidal-rules: PACK-14
// tidal-expect: accept
// tidal-note: The bases come in the order of the pack, so they are constructed in that order. Each base constructor adds its digit to the log.
struct One { constexpr One(int& log) { log = log * 10 + 1; } };
struct Two { constexpr Two(int& log) { log = log * 10 + 2; } };
template <class... Bs> struct Mix : Bs... { constexpr Mix(int& log) : Bs(log)... {} };
template <class... Bs> constexpr int order() { int log = 0; Mix<Bs...> m(log); (void)m; return log; }
static_assert(order<One, Two>() == 12);
static_assert(order<Two, One>() == 21);
