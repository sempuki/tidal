// tidal-rules: FN-11
// tidal-expect: run
// tidal-note: In a member function, the matrix implementations give the plain name of the function.
struct Widget {
  const char* name() const { return __func__; }
};
int main() {
  const char* n = Widget{}.name();
  return n[0] == 'n' && n[1] == 'a' && n[2] == 'm' && n[3] == 'e' && n[4] == 0 ? 0 : 1;
}
