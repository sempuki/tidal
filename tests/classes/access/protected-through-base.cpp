// tidal-rules: ACC-4
// tidal-expect: reject
class B { protected: int x = 1; };
class D : public B {
public:
  int get(const B& b) const { return b.x; }
};
