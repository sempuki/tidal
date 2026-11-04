// tidal-rules: CTOR-12 OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: member call on object outside its lifetime
// tidal-note: Q-0374. The mem_init of the base A calls a member function of the member h, whose lifetime has not begun. Clang rejects the constant evaluation. GCC and EDG accept it, because the function reads nothing.
struct A { int a; constexpr A(int v) : a(v) {} };
struct Helper { constexpr int one() const { return 1; } };
struct D : A {
  Helper h;
  constexpr D() : A(h.one()) {}   // undefined: h is not yet initialized
};
constexpr D d;
