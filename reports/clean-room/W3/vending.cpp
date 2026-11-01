// Vending machine state machine.
// Rules used: ENUM-P2 (scoped enum), SW-1/SW-3 (switch on scoped enum,
// case values State::X), JMP-1 (break), RFOR-1 (loop over event array),
// STMT-2 (declarations in blocks), FN-7 (return on every path),
// CX-3 (constexpr function used by static_assert and at run time).

enum class State { Idle, HasCredit, Dispensing, OutOfStock };
enum class Kind { Coin, Select, Cancel, Take };

struct Event {
  Kind kind;
  int value;  // coin amount, or item index
};

struct Machine {
  State state = State::Idle;
  int credit = 0;
  int stock[3] = {2, 0, 1};
  int price[3] = {50, 75, 100};
  int change = 0;      // total change returned
  int dispensed = 0;   // number of items handed out

  constexpr void step(Event e) {
    switch (state) {
      case State::Idle:
        if (e.kind == Kind::Coin) {
          credit = e.value;
          state = State::HasCredit;
        }
        break;
      case State::HasCredit:
        switch (e.kind) {
          case Kind::Coin:
            credit += e.value;
            break;
          case Kind::Select: {  // braces: SW-5 forbids skipping item's initialization
            int item = e.value;
            if (item < 0 || item >= 3) break;
            if (stock[item] == 0) {
              state = State::OutOfStock;
              break;
            }
            if (credit < price[item]) break;
            credit -= price[item];
            stock[item] -= 1;
            state = State::Dispensing;
            break;
          }
          case Kind::Cancel:
            change += credit;
            credit = 0;
            state = State::Idle;
            break;
          case Kind::Take:
            break;
        }
        break;
      case State::Dispensing:
        if (e.kind == Kind::Take) {
          dispensed += 1;
          change += credit;
          credit = 0;
          state = State::Idle;
        }
        break;
      case State::OutOfStock:
        // any event returns to HasCredit so the user can pick again
        state = State::HasCredit;
        break;
    }
  }
};

// Display code for each state. Every case returns.
constexpr int code(State s) {
  switch (s) {
    case State::Idle: return 0;
    case State::HasCredit: return 1;
    case State::Dispensing: return 2;
    case State::OutOfStock: return 3;
  }
  return -1;  // ENUM-P3: a scoped enum can hold any int, so FN-7 applies
}

constexpr int run(const Event* events, int n) {
  Machine m;
  for (int i = 0; i < n; ++i) m.step(events[i]);
  return m.dispensed * 1000 + m.change;
}

constexpr Event script[] = {
    {Kind::Coin, 100},  {Kind::Select, 1},  // item 1 is out of stock
    {Kind::Cancel, 0},                      // leaves OutOfStock
    {Kind::Select, 0},  {Kind::Take, 0},    // 50, change 50
    {Kind::Coin, 25},   {Kind::Cancel, 0},  // change 25
};

constexpr int script_len = sizeof(script) / sizeof(script[0]);

static_assert(run(script, script_len) == 1075);

int main() {
  Machine m;
  for (Event e : script) m.step(e);
  if (m.dispensed != 1) return 1;
  if (m.change != 75) return 2;
  if (m.state != State::Idle) return 3;
  if (m.stock[0] != 1) return 4;
  if (code(m.state) != 0) return 5;
  return 0;
}
