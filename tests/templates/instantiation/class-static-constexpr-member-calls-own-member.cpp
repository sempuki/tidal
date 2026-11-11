// tidal-rules: INST-5
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction static_constexpr_member_from_member_function.cpp and probe static_constexpr_member_from_member_function_run.cpp (from the tries of fsm.cpp). The instantiation of Limits<T> does not make the initializer of the static constexpr member max (INST-5). The first use of max makes the initializer, and then the class is complete and compute has its definition. The class form that is not a template is rejected (static-constexpr-member-calls-own-member.cpp).
template <class T> struct Limits {
  static constexpr int compute() { return static_cast<int>(sizeof(T)) * 2; }
  static constexpr int max = compute();
};
static_assert(Limits<short>::max == static_cast<int>(sizeof(short)) * 2);
int main() { return Limits<char>::max == 2 ? 0 : 1; }
