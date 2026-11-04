// tidal-rules: OBJ-12
// tidal-expect: accept
// tidal-note: watcher is constructed before the thread_local in tally, so tally's object ends first when the thread ends. The destructor of watcher then calls tally, and control passes through the declaration of the ended object. No tool in the matrix stops the run, so the test checks only that the program compiles.
struct Count { int n = 0; ~Count() {} };
Count& tally() { thread_local Count c; return c; }
struct Watcher { ~Watcher() { ++tally().n; } };
thread_local Watcher watcher;
int main() {
  static_cast<void>(&watcher);
  return tally().n;
}
