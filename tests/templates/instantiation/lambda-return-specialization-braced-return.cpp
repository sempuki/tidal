// tidal-rules: INST-1 JMP-3
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction lambda_return_specialization_braced_return.cpp (from parse_second_try.cpp). No use before the lambda makes Outcome<int>. The definition of the lambda's call operator returns the class, so the definition instantiates it (INST-1), and the braced list in the return statement initializes the result (JMP-3).
template <class V> struct Outcome { bool found; V payload; };
int main() {
  auto lookup = [](int key) -> Outcome<int> { return {key > 0, key * 2}; };
  Outcome<int> r = lookup(21);
  return r.found && r.payload == 42 ? 0 : 1;
}
