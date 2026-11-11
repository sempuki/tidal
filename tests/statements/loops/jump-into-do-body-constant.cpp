// tidal-rules: LOOP-9 SW-1
// tidal-expect: accept
// tidal-note: G161. run(1) jumps to the case label inside the do body. The rest of the body runs, then the condition, and the loop goes on as after a normal pass: the value is 2 * 100 + 3.
constexpr int run(int k) {
  int count = 0;
  int sum = 0;
  switch (k) {
    case 0:
      do {
        sum += 10;
        case 1:
        sum += 1;
      } while (++count < 3);
  }
  return sum * 100 + count;
}
static_assert(run(1) == 2303);
static_assert(run(0) == 3303);
