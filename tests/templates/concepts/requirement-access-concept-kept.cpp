// tidal-rules: CNS-10 CPT-26
// tidal-expect: accept
// tidal-note: Peek<Box> is first checked inside a member of Box, where the private member is accessible, so the result is true. At namespace scope the same concept-id would be false. The result changes, so the program is not valid. CNS-10 needs no diagnostic, and every compiler accepts and keeps the first result (Q-0288).
template <class T> concept Peek = requires (T t) { t.secret; };
class Box {
  int secret = 0;
public:
  static constexpr bool viaConcept() { return Peek<Box>; }
};
static_assert(Box::viaConcept());
constexpr bool again = Peek<Box>;
