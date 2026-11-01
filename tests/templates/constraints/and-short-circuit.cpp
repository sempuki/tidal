// tidal-rules: CNS-5
// tidal-expect: accept
// tidal-note: The right operand would instantiate Trap<int>, or would name int::tag. Neither happens, because the left operand is false.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr bool value = true;
};
template <class T> concept Tagged = requires { typename T::tag; };
template <class T> concept SmallTag = Tagged<T> && (sizeof(typename T::tag) < 4);
template <class T> concept Guarded = (sizeof(T) > 100) && Trap<T>::value;
struct Item { using tag = char; };
static_assert(!Guarded<int>);
static_assert(SmallTag<Item> && !SmallTag<int>);
