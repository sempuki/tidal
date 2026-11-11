template <bool B> struct Q {
  int value = 7;
  constexpr explicit operator int() const requires B { return value; }
  constexpr explicit operator int() const requires (!B) { return 0; }
};
int main() { Q<true> q; return static_cast<int>(q) - 7; }
