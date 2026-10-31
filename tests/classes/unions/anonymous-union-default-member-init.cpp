// tidal-rules: UNI-7
// tidal-expect: run
struct Value {
  union { int i = 8; float f; };
};
int main() { Value v; return v.i == 8 ? 0 : 1; }
