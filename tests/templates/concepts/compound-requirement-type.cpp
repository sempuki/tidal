// tidal-rules: CPT-33
// tidal-expect: accept
// tidal-note: The concept after -> gets the type and value category of the expression, as decltype of the parenthesized expression gives them.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class U> concept Is = same<T, U>;
template <class T> concept Big = sizeof(T) > 1;
struct Thing { int member; char letter(); double size(); };
template <class T> concept Shapes = requires (T t, T& ref) {
  { t.member } -> Is<int&>;
  { static_cast<T&&>(t).member } -> Is<int&&>;
  { t.member + 1 } -> Is<int>;
  { ref } -> Is<T&>;
  { t.size() } -> Big;
};
template <class T> concept BigLetter = requires (T t) { { t.letter() } -> Big; };
static_assert(Shapes<Thing> && !Shapes<int> && !BigLetter<Thing>);
