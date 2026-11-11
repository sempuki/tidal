// CTAD through a deduction guide, where the class also has a constrained
// constructor template whose implicit guide fails its constraint.
template <class... Ts> struct Tuple {
  template <class... Us> requires (sizeof...(Us) == sizeof...(Ts) && sizeof...(Ts) > 0)
  constexpr Tuple(Us...) {}
};
template <class... Us> Tuple(Us...) -> Tuple<Us...>;
int main() {
  auto t = Tuple(1, 2);
  (void)t;
  return 0;
}
