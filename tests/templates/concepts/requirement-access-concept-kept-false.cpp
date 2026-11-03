// tidal-rules: CNS-10 CPT-26
// tidal-expect: accept
// tidal-note: Peek<Box> is first checked at namespace scope, where the private member is not accessible, so the result is false. Inside the member defined later, the same concept-id would be true. The result changes, so the program is not valid. CNS-10 needs no diagnostic, and every compiler accepts and keeps the first result (Q-0288).
template <class T> concept Peek = requires (T t) { t.secret; };
class Box {
  int secret = 0;
public:
  static constexpr bool viaConcept();
};
static_assert(!Peek<Box>);
constexpr bool Box::viaConcept() { return Peek<Box>; }
constexpr bool again = Box::viaConcept();
