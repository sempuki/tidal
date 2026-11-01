// tidal-rules: CPT-26
// tidal-expect: accept
// tidal-note: At namespace scope the private member is not accessible, so the requirement does not hold.
template <class T> concept Peek = requires (T t) { t.secret; };
class Box { int secret = 0; };
static_assert(!Peek<Box>);
