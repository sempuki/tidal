// tidal-rules: REF-1 REF-2 REF-3 REF-8 REF-9 REF-10 REF-7 REF-4 REF-5 CTOR-17 CE-23 CLS-3
// tidal-expect: run
// tidal-note: Write suite W6, program ref_members.cpp (reports/clean-room/W6/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Reference members and the life of temporaries.
// Built from Tidal: REF-1, REF-2, REF-3, REF-4, REF-5, REF-7, CTOR-5,
// CTOR-17, CE-23, CLS-3.

// A type that counts its live objects and logs each destruction.
struct Note {
  static inline int live = 0;
  static inline int log[16] = {};
  static inline int logged = 0;
  int id;
  explicit Note(int i) : id(i) { ++live; }
  Note(const Note& other) : id(other.id) { ++live; }
  ~Note() {
    --live;
    log[logged++] = id;
  }
};

Note make(int id) { return Note(id); }
const Note& pass(const Note& n) { return n; }

// An aggregate with a reference member.
struct View {
  const Note& note;
  int extra;
};

// A class whose constructor binds a reference member to an lvalue.
class Account {
  int& balance;
public:
  explicit Account(int& b) : balance(b) {}
  void deposit(int amount) { balance += amount; }
};

struct Box {
  int v;
};

// The same rules at translation time, with a constexpr destructor.
struct Tracker {
  int* live;
  constexpr explicit Tracker(int* l) : live(l) { ++*live; }
  Tracker(const Tracker&) = delete;
  constexpr ~Tracker() { --*live; }
};
struct Holder {
  const Tracker& t;
};

constexpr int during(const Tracker& t) { return *t.live; }

constexpr int lifetimes() {
  int live = 0;
  int seen_ref = 0;
  int seen_brace = 0;
  int seen_paren = 0;
  {
    const Tracker& t = Tracker(&live);   // extended (REF-4)
    seen_ref = live;
  }
  int after_ref = live;
  {
    Holder h{Tracker(&live)};            // braces: extended (REF-4)
    seen_brace = live;
    static_cast<void>(h);
  }
  // Parentheses do not extend (REF-5). Clang warns about such a holder
  // (-Wdangling), so the case is in probe/paren_holder.cpp. The count
  // stays 0 here.
  int in_call = during(Tracker(&live));  // lives to the end of the call
  return seen_ref * 10000 + after_ref * 1000 + seen_brace * 100 +
         seen_paren * 10 + in_call;
}
static_assert(lifetimes() == 10101);

int main() {
  int result = 0;

  // REF-4: two extended temporaries end with their scope, in reverse
  // order of their references.
  {
    const Note& a = make(1);
    Note&& b = make(2);
    if (Note::live != 2 || a.id != 1 || b.id != 2) result |= 1;
  }
  if (Note::live != 0 || Note::logged != 2 || Note::log[0] != 2 ||
      Note::log[1] != 1) {
    result |= 2;
  }

  // REF-4: a reference member of an aggregate initialized with braces.
  {
    View v{make(3), 7};
    if (Note::live != 1 || v.note.id != 3 || v.extra != 7) result |= 4;
  }
  if (Note::live != 0) result |= 8;

  // REF-4: the selected operand of ?:.
  bool pick_first = Note::logged > 0;
  {
    const Note& c = pick_first ? make(4) : make(5);
    if (Note::live != 1 || c.id != 4) result |= 16;
  }

  // REF-5: a parameter does not extend; use the result inside the full
  // expression only.
  int id = pass(make(6)).id;
  if (id != 6 || Note::live != 0) result |= 32;

  // REF-4: binding to a member of a temporary keeps the whole object.
  const int& kept = Box{8}.v;
  if (kept != 8) result |= 64;

  // REF-2, REF-7: a const reference to another type binds a temporary
  // that holds the converted value.
  int source = 1;
  const long& copy = source;
  source = 2;
  if (copy != 1) result |= 128;

  // REF-1: a reference member bound to an lvalue changes that lvalue.
  int balance = 10;
  Account account(balance);
  account.deposit(5);
  if (balance != 15) result |= 256;

  // REF-3: an rvalue reference binds a temporary of the converted value.
  double d = 2.5;
  int&& whole = static_cast<int>(d);
  whole += 1;
  if (whole != 3) result |= 512;

  return result;
}
