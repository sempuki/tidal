// tidal-rules: CPT-28
// tidal-expect: accept
// tidal-note: No object of a parameter's type is created, so a type whose constructors and destructor are deleted works.
struct Fixed {
  Fixed() = delete;
  Fixed(const Fixed&) = delete;
  ~Fixed() = delete;
  int value;
};
template <class T> concept HasValue = requires (T t) { t.value; };
static_assert(HasValue<Fixed>);
template <class T> concept Adds = requires (T a, decltype(a) b) { a + b; };
static_assert(Adds<int>);
