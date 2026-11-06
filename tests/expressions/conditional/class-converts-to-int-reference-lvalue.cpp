// tidal-rules: COND-16 OVL-15 ICS-11
// tidal-expect: run
// tidal-note: Roadmap round 9 (I4-4), I4 reduction r1-conv-to-int-ref-lvalue.cpp. A class lvalue with a conversion function to int& meets an int lvalue. Step 1 binds int& directly to the result of the conversion function, so the result is an int lvalue, and an assignment through it changes the referenced object.
int store = 0;
struct Handle {
  operator int&() { return store; }
};
int main() {
  Handle h;
  int other = 0;
  (other == 0 ? h : other) = 7;
  return store == 7 && other == 0 ? 0 : 1;
}
