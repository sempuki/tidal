// tidal-rules: ACC-4
// tidal-expect: reject
// tidal-error-clang: is a protected member
// tidal-error-gcc: protected within this context
// tidal-error-edg: inaccessible|not accessible through
class B { protected: int x = 1; };
class D : public B {
public:
  static int B::* ptr() { return &B::x; }
};
