// G151: a bit-field width of class type whose class has one explicit
// conversion to int and one non-explicit conversion to long.
struct W {
  constexpr explicit operator int() const { return 3; }
  constexpr operator long() const { return 4; }
};
struct S { int f : W{}; };
static_assert(sizeof(S) > 0);
int main() {}
