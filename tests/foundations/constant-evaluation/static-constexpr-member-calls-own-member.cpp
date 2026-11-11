// tidal-rules: CE-16 CLS-2
// tidal-expect: reject
// tidal-error-clang: undefined function .compute. cannot be used in a constant expression
// tidal-error-gcc: called in a constant expression before its definition is complete
// tidal-error-edg: constexpr function .Limits::compute. .* is not defined
// tidal-note: Roadmap round 14, programmer reduction static_constexpr_member_from_member_function_plain_class.cpp (from the tries of fsm.cpp). Limits is a class that is not a template. The initializer of a static data member is not among the places where the class is complete (CLS-2), so at the call in the initializer of max, the static member function compute has only its declaration. The call is not a constant expression (CE-16).
struct Limits {
  static constexpr int compute() { return 4; }
  static constexpr int max = compute();
};
int main() { return Limits::max - 4; }
