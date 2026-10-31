// tidal-rules: UNI-7
// tidal-expect: run
struct Value {
  int kind;
  union { int i; float f; };
};
int main() { Value v; v.kind = 0; v.i = 7; return v.i == 7 ? 0 : 1; }
