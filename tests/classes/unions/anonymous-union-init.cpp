// tidal-rules: UNI-7
// tidal-expect: run
struct Value {
  int kind;
  union { int i; float f; };
};
int main() { Value v{1, {5}}; return v.i == 5 ? 0 : 1; }
