// tidal-rules: SW-1
// tidal-expect: run
enum class State { idle, busy };
int code(State s) {
  switch (s) {
    case State::idle: return 0;
    case State::busy: return 1;
  }
  return -1;  // a value with no enumerator reaches this line
}
int main() {
  if (code(State::busy) != 1) return 1;
  return code(static_cast<State>(7)) == -1 ? 0 : 2;
}
