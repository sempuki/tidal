// tidal-rules: DTOR-3
// tidal-expect: accept
class S {
  ~S() {}
public:
  static void make_and_drop() { S s; }
};
