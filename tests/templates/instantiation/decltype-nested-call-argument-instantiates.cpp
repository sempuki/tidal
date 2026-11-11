// tidal-rules: INST-2
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction decltype_nested_operator_template_calls.cpp (from the tries of units.cpp). The inner call is an argument of the outer call, so the inner call instantiates its result class (INST-2). The whole operand of decltype is the outer call, whose result type is Level<7>.
template <int N> struct Level { static constexpr int depth = N; };
template <int A, int B> constexpr Level<A + B> operator+(Level<A>, Level<B>) { return {}; }
template <class X, class Y> inline constexpr bool same_type = false;
template <class X> inline constexpr bool same_type<X, X> = true;
static_assert(same_type<decltype(Level<1>() + (Level<2>() + Level<4>())), Level<7>>);
int main() { return same_type<decltype((Level<3>() + Level<3>()) + Level<0>()), Level<6>> ? 0 : 1; }
