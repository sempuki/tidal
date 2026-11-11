// tidal-rules: NAM-5 OVL-27 DEL-1 CNS-7
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer probe adl_function_beside_deleted_template_in_requires.cpp. In the requires expression, the call finds the deleted template by ordinary lookup and shapes::area by argument-dependent lookup (NAM-5). For shapes::Square the plain function wins (OVL-27), so the concept holds. For int only the deleted template is a candidate, the call refers to a deleted function (DEL-1), and the concept fails (CNS-7).
template <class T> int area(const T&) = delete;
namespace shapes {
struct Square { int side = 3; };
int area(const Square& s) { return s.side * s.side; }
}
template <class T> concept HasArea = requires(const T& t) { area(t); };
int main() {
  if (!HasArea<shapes::Square>) return 1;
  return HasArea<int> ? 2 : 0;
}
