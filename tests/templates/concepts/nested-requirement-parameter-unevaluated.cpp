// tidal-rules: CPT-35
// tidal-expect: accept
struct Reading { int value; };
template <class T> concept Sized = requires (T t) {
  requires sizeof(t) > 1;
  requires sizeof(t.value) > 1;
};
static_assert(Sized<Reading>);
