struct Q {
  long long value{};
  template <class U = long long>
  constexpr explicit operator U() const { return value; }
};
constexpr Q operator/(Q a, Q b) { return Q{a.value / b.value}; }
static_assert((Q{10} / Q{2}).value == 5);
int main() { return 0; }
