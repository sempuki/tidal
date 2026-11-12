struct Squares {
  static constexpr int operator[](int i) { return i * i; }
};
static_assert(Squares{}[5] == 25);
int main() { Squares s; return s[3] - 9; }
