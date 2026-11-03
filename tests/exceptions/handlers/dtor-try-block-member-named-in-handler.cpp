// tidal-rules: TRY-11
// tidal-expect: accept
// tidal-note: Compiles on every compiler. The handler of a destructor's function try block reads a member. The use is undefined, because the member is destroyed before the handler runs.
struct S {
  int m = 1;
  ~S() try {
    throw 1;
  } catch (int) {
    int v = m;
    (void)v;
  }
};
