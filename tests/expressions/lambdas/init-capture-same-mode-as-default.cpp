// tidal-rules: LAM-6 LAM-7
// tidal-expect: run
// tidal-note: Decision 0059, item RV5, point RV5-B-14. An init capture is not a simple capture, so it can use the same mode as the capture default: [=, x = 1] and [&, &r = v] are valid.
int main() {
  int y = 2;
  int v = 3;
  auto by_copy = [=, x = 1] { return x + y; };
  auto by_reference = [&, &r = v] { return r + y; };
  return by_copy() == 3 && by_reference() == 5 ? 0 : 1;
}
