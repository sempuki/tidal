enum class State { Idle, HasCredit };
int code(State s) {
  switch (s) {
    case State::Idle: return 0;
    case State::HasCredit: return 1;
  }
}
int main() { return code(static_cast<State>(7)); }
