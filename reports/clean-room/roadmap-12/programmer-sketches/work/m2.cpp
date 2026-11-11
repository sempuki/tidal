struct M {
  int a[2][3]{};
  bool operator==(const M&) const = default;
};
static_assert(M{} == M{});
int main() { return 0; }
