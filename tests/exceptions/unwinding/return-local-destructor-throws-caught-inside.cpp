// tidal-rules: UNW-1
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-note: Q-0365. The first return statement builds the result, and then the destructor of t throws. A handler in the same function catches the exception, so the first result is destroyed before that handler runs. The handler's return statement builds a second result, which the caller receives. GCC destroys the first result before the handler runs. Clang never destroys it, and the program returns 10.
// tidal-note: Under NRVO, r can be the result object itself, so the test compares the objects ended with the objects made when the handler starts.
int made = 0;
int ended = 0;
int ended_in_handler = -1;
int made_in_handler = -2;
struct R {
  int tag;
  R(int t) : tag(t) { ++made; }
  R(const R& o) : tag(o.tag) { ++made; }
  ~R() { ++ended; }
};
struct T { ~T() noexcept(false) { throw 1; } };
R named() {
  try {
    T t;
    R r(1);
    return r;
  } catch (int) {
    ended_in_handler = ended;
    made_in_handler = made;
    return R(2);
  }
}
R fresh() {
  try {
    T t;
    return R(1);
  } catch (int) {
    ended_in_handler = ended;
    made_in_handler = made;
    return R(2);
  }
}
int main() {
  bool all_ended_named, all_ended_fresh;
  int tag_named, tag_fresh;
  {
    R x = named();
    tag_named = x.tag;
    all_ended_named = ended_in_handler == made_in_handler;
  }
  {
    R y = fresh();
    tag_fresh = y.tag;
    all_ended_fresh = ended_in_handler == made_in_handler;
  }
  if (tag_named != 2 || tag_fresh != 2) return 1;
  if (!all_ended_named || !all_ended_fresh) return 10;
  return made == ended ? 0 : 2;
}
