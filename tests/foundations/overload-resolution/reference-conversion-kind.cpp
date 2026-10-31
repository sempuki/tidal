// tidal-rules: OVL-15
// tidal-expect: accept
// tidal-note: Only the conversion that returns int& serves the lvalue reference, and only the one that returns int&& serves the rvalue reference, so each binding has one candidate.
int a = 1;
int b = 2;
struct Source {
  constexpr operator int&() const { return a; }
  constexpr operator int&&() const { return static_cast<int&&>(b); }
};
int& left = Source{};
int&& right = Source{};
int check[&left == &a && &right == &b ? 1 : -1];
