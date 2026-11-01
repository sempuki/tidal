// tidal-rules: UN-18
// tidal-expect: accept
struct Key {
  int v;
  bool operator==(const Key& other) const { return v == other.v; }
};
Key left{1}, right{2};
static_assert(!noexcept(left != right));
static_assert(!noexcept(left == right));
int main() {}
