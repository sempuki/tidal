// tidal-rules: COND-8
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: The prvalue comes second and the lvalue third. The copy constructor is private, so the copy from the lvalue is not valid.
class S {
  S(const S&);
public:
  S() = default;
  S(S&&) = default;
};
S make();
S s;
void f(bool c) { auto&& r = c ? make() : s; }
