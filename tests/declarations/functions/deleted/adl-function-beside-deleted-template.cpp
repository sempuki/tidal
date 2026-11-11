// tidal-rules: NAM-5 OVL-27 DEL-1
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction adl_function_beside_deleted_template.cpp (from algo_second_try.cpp). Ordinary lookup finds the deleted template, and argument-dependent lookup adds shapes::area (NAM-5). Both take the argument by an exact match, and the function that is not a template specialization wins (OVL-27). The deleted template is not chosen, so the call is valid (DEL-1).
namespace shapes {
struct Square { int side = 3; };
int area(const Square& s) { return s.side * s.side; }
}
template <class T> int area(const T&) = delete;
int main() {
  shapes::Square s;
  return area(s) == 9 ? 0 : 1;
}
