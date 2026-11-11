// tidal-rules: LOOP-9 SW-1
// tidal-expect: accept
// tidal-note: G161. run(1) jumps to the case label inside the for body. The initializer step = 1 does not run. The rest of the body runs, then the iteration expression, then the condition, and the loop goes on: the value is 2 * 100 + 3. run(0) runs the initializer, so the body runs twice from its start.
constexpr int run(int k) {
  int step = 0;
  int sum = 0;
  switch (k) {
    case 0:
      for (step = 1; step < 3; ++step) {
        sum += 10;
        case 1:
        sum += 1;
      }
  }
  return sum * 100 + step;
}
static_assert(run(1) == 2303);
static_assert(run(0) == 2203);
