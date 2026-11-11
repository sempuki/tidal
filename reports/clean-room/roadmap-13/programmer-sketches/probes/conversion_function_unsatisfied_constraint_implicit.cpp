template <bool B> struct Q {
  int value = 0;
  constexpr operator int() const requires B { return value; }
};
int main() { Q<false> q; int x = q; return x; }
