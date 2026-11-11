struct M {
  int a[2][3]{};
  friend constexpr bool operator==(const M&, const M&) = default;
};
static_assert(M{} == M{});
int main() { return M{} == M{} ? 0 : 1; }
