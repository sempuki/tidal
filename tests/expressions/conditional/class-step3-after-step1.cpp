// tidal-rules: COND-16 COND-3
// tidal-expect: run
// tidal-note: Decision 0059, item F2 (I4-13), I4 program p11-step3-after-step1.cpp. Both operands are lvalues, and the conversion function of Maker returns a Point by value, so step 1 gives no conversion. Step 3 converts the Maker operand to a Point prvalue.
struct Point { int x; };
struct Maker {
  operator Point() { return Point{9}; }
};
int main(int argc, char**) {
  Maker m;
  Point p{1};
  Point r = argc > 0 ? m : p;
  return r.x == 9 ? 0 : 1;
}
