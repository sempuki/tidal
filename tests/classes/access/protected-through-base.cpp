// tidal-rules: ACC-4
// tidal-expect: reject
// tidal-error-clang: is a protected member
// tidal-error-gcc: protected within this context
// tidal-error-edg: inaccessible|not accessible through
class B { protected: int x = 1; };
class D : public B {
public:
  int get(const B& b) const { return b.x; }
};
