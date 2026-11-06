// tidal-rules: OBJ-11 SINIT-6
// tidal-expect: run
// tidal-note: Decision 0059, item F2 (R10-6), with the round 9 writer probe r10-6-static-thread-local-member.cpp. A thread_local static data member and a thread_local variable at namespace scope, in one unit, both with constructors. The first use of the member constructs both, if the construction was deferred. Construction before main is also allowed.
int trace = 0;
struct Mark {
  int id;
  Mark(int i) : id(i) { trace = trace * 10 + i; }
};
thread_local Mark plain(1);
struct Holder { static thread_local Mark member; };
thread_local Mark Holder::member(2);
int main() {
  int before = trace;
  int id = Holder::member.id;
  int after = trace;
  bool both = after == 12 || after == 21;
  return (before == 0 || before == after) && id == 2 && both ? 0 : 1;
}
