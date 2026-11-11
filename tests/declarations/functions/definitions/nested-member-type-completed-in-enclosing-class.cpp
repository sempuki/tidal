// tidal-rules: FN-12 CLS-2
// tidal-expect: run
// tidal-note: RV9-1. Inner defines make in its class with the return type Later, which Outer completes after Inner. FN-12 checks the types where the outermost enclosing class ends, and Later is complete there. GCC, Clang, and EDG accept.
struct Outer {
  struct Later;
  struct Inner {
    Later make() const { return Later{4}; }
  };
  struct Later { int k; };
};
int main() {
  return Outer::Inner{}.make().k - 4;
}
