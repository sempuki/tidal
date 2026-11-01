// tidal-rules: ACC-3
// tidal-expect: accept
// tidal-note: The same selection inside a member and inside a friend is valid, because the check uses the scope of the expression.
class Meter {
  static void f(int);
public:
  static void f(double);
  static void (*from_member())(int) { return &Meter::f; }
  friend void (*from_friend())(int);
};
void (*from_friend())(int) { return &Meter::f; }
