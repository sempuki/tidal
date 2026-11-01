// tidal-rules: CPT-26
// tidal-expect: accept
// tidal-note: The concept-id is first checked inside a member of Box, where the private member is accessible.
template <class T> concept Peek = requires (T t) { t.secret; };
class Box {
  int secret = 0;
public:
  static constexpr bool viaConcept() { return Peek<Box>; }
};
static_assert(Box::viaConcept());
