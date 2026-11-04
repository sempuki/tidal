// Probe: INST-27. In a unit with an explicit instantiation declaration,
// a constexpr constructor is still instantiated, so a constant can use it.
template <int F> struct Fx {
  long long raw;
  explicit constexpr Fx(int w) : raw(static_cast<long long>(w) << F) {}
  int floor() const;
};
template <int F> int Fx<F>::floor() const { return static_cast<int>(raw >> F); }
extern template struct Fx<16>;
static_assert(Fx<16>(3).raw == 3 * 65536);
int main() { return 0; }
