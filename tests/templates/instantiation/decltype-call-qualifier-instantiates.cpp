// tidal-rules: INST-1 INST-2
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction decltype_qualifier_specialization_from_template_return.cpp (from the tries of units.cpp). The call is the whole operand of decltype, so the call alone does not instantiate Level<7> (INST-2). The qualified name that follows names a member of Level<7>, and that use instantiates the class (INST-1).
template <int N> struct Level { static constexpr int depth = N; };
template <int A, int B> constexpr Level<A + B> join(Level<A>, Level<B>) { return {}; }
static_assert(decltype(join(Level<2>(), Level<5>()))::depth == 7);
int main() { return decltype(join(Level<1>(), Level<1>()))::depth == 2 ? 0 : 1; }
