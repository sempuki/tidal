// tidal-rules: CE-30 CE-8
// tidal-expect: reject
// tidal-error-gcc: call to consteval function .*is not a constant expression
// tidal-error-clang: call to consteval function .* is not a constant expression
// tidal-note: The consteval call starts its own evaluation. The run-time try block around it is not part of that evaluation.
consteval int check(int v) { if (v != 0) throw v; return 0; }
int main() {
  try { return check(1); }
  catch (...) { return 2; }
}
