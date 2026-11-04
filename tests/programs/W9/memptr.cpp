// tidal-rules: MPTR-1 MPTR-2 MPTR-3 MPTR-4 MPTR-5 MPTR-10 VIRT-1
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Write suite W9, program memptr.cpp (reports/clean-room/W9/report.md). The rules are the ones in the report's construct-to-rule notes that the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// tidal-note: Q-0353. During constant evaluation EDG 7.0 loses the member that table[0], converted to a pointer to member of Loud, names.
// W9: dispatch through pointers to members.
// Rules used: MPTR-1, MPTR-2, MPTR-3, MPTR-4, MPTR-5, MPTR-10, VIRT-1.

struct Machine {
  int acc = 0;
  int steps = 0;
  constexpr void inc() { ++acc; }
  constexpr void dbl() { acc *= 2; }
  constexpr void neg() { acc = -acc; }
  constexpr virtual int peek() const { return acc; }
  virtual ~Machine() = default;
};

struct Loud : Machine {
  int resets = 0;
  constexpr int peek() const override { return acc + 1000; }
  constexpr void reset() { acc = 0; ++resets; }
};

using Op = void (Machine::*)();             // MPTR-1: a type alias
using Peek = int (Machine::*)() const;
using Field = int Machine::*;

constexpr Op table[] = {&Machine::inc, &Machine::dbl, &Machine::neg};
constexpr Field fields[] = {&Machine::acc, &Machine::steps};

// MPTR-3: (m.*op)() calls the member, with the parentheses MPTR-10 needs.
constexpr void run(Machine& m, const int* prog, int n) {
  for (int i = 0; i < n; ++i) {
    (m.*table[prog[i]])();
    ++(m.*fields[1]);
  }
}

constexpr int check() {
  Machine m;
  const int prog[] = {0, 0, 1, 1, 2};       // inc inc dbl dbl neg
  run(m, prog, 5);
  if (m.acc != -8) return 1;
  if (m.*fields[1] != 5) return 2;

  // MPTR-3: a pointer to a virtual function dispatches on the object.
  Loud l;
  Peek pk = &Machine::peek;
  Machine* mp = &l;
  if ((mp->*pk)() != 1000) return 3;
  if ((m.*pk)() != -8) return 4;

  // MPTR-4: a pointer to a member of a base converts to one of a derived class.
  void (Loud::* lop)() = table[0];
  (l.*lop)();
  if (l.acc != 1) return 5;

  // MPTR-4: static_cast goes the other way. Apply it only to a Loud.
  Op r = static_cast<Op>(&Loud::reset);
  (mp->*r)();
  if (l.acc != 0 || l.resets != 1) return 6;

  // MPTR-5: equality of data member pointers.
  if (fields[0] != &Machine::acc) return 9;
  if (fields[0] == fields[1]) return 10;
  return 0;
}

// MPTR-5: equality of member function pointers. GCC with
// -fsanitize=undefined refuses this comparison in a constant
// expression, so the check runs only at run time.
bool compare_ops(Op chosen) {
  if (table[0] == table[1]) return false;
  if (chosen == nullptr) return false;
  return chosen == table[2];
}

static_assert(check() == 0);

Op unset;                                   // MPTR-4: static storage, null

int main() {
  if (check() != 0) return 1;
  if (unset != nullptr) return 2;
  if (!compare_ops(table[2])) return 3;
  return 0;
}
