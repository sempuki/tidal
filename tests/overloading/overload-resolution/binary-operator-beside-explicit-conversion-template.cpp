// tidal-rules: OVL-9 DED-31
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction binary_operator_beside_conversion_function_template.cpp (from the tries of units.cpp). Money has an explicit conversion function template. A built-in candidate of - needs an implicit conversion of each operand, and an explicit conversion function gives none (OVL-9), so the user operator- is the only viable function.
struct Money {
  long long cents{};
  template <class U = long long>
  constexpr explicit operator U() const { return static_cast<U>(cents); }
};
constexpr Money operator-(Money a, Money b) { return Money{a.cents - b.cents}; }
static_assert((Money{500} - Money{120}).cents == 380);
int main() {
  Money m = Money{90} - Money{40};
  return static_cast<long long>(m) == 50 ? 0 : 1;
}
