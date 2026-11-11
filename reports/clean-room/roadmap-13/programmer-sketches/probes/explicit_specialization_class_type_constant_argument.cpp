struct D {
  int a = 0;
  constexpr bool operator==(const D&) const = default;
};
template <D V> struct Q {};
template <class X> struct is_one { static constexpr bool value = false; };
template <> struct is_one<Q<D{1}>> { static constexpr bool value = true; };
int main() { return (is_one<Q<D{1}>>::value ? 0 : 1) + (is_one<Q<D{2}>>::value ? 2 : 0); }
