// W10: a transaction that rolls back on error.
// Go habit: defer tx.Rollback() and an explicit tx.Commit().
// Here a guard object restores the saved state in its destructor
// unless commit() ran (OBJ-P2, UNW-1).

namespace std {
class exception {               // LSUP-8: shape from the page
public:
  exception() noexcept {}
  virtual ~exception();         // the library defines it
};
}

struct TxError : std::exception {
  int code;
  explicit TxError(int c) noexcept : code(c) {}
};

struct InsufficientFunds : TxError {
  int account;
  InsufficientFunds(int a) noexcept : TxError(1), account(a) {}
};

struct Ledger {
  int balance[4];
};

class Transaction {
  Ledger& live;
  Ledger saved;
  bool committed = false;
public:
  constexpr explicit Transaction(Ledger& l) : live(l), saved(l) {}
  Transaction(const Transaction&) = delete;
  Transaction& operator=(const Transaction&) = delete;
  constexpr void commit() noexcept { committed = true; }
  // No specifier: noexcept by DTOR-4, so it is safe during unwinding.
  constexpr ~Transaction() {
    if (!committed) live = saved;
  }
};

// A throw on a path the evaluation does not take is fine (THR-10, CE-8).
constexpr void transfer(Ledger& l, int from, int to, int amount) {
  Transaction tx(l);
  l.balance[to] += amount;      // change first, check after
  l.balance[from] -= amount;
  if (l.balance[from] < 0) throw InsufficientFunds(from);
  tx.commit();
}

// Early return also rolls back: usable in constant evaluation.
constexpr bool try_transfer(Ledger& l, int from, int to, int amount) {
  Transaction tx(l);
  l.balance[to] += amount;
  l.balance[from] -= amount;
  if (l.balance[from] < 0) return false;
  tx.commit();
  return true;
}

constexpr bool compile_time_check() {
  Ledger l{{10, 0, 0, 0}};
  bool a = try_transfer(l, 0, 1, 4);   // ok
  bool b = try_transfer(l, 0, 2, 100); // rolled back
  transfer(l, 1, 3, 1);                // the throw is not reached
  return a && !b && l.balance[0] == 6 && l.balance[1] == 3 &&
         l.balance[2] == 0 && l.balance[3] == 1;
}
static_assert(compile_time_check());
static_assert(!noexcept(transfer(*static_cast<Ledger*>(nullptr), 0, 1, 1)));

// A batch of steps: a handler rolls the whole batch back and rethrows
// the same exception object (TRY-8, THR-6).
void batch(Ledger& l) {
  Ledger before = l;
  try {
    transfer(l, 0, 1, 5);
    transfer(l, 1, 2, 50);              // throws
    transfer(l, 2, 3, 1);               // never runs (UNW-1)
  } catch (...) {
    l = before;
    throw;
  }
}

int main() {
  Ledger l{{10, 0, 0, 0}};

  // 1. One failed step: the destructor restores the ledger.
  try {
    transfer(l, 0, 1, 11);
    return 1;
  } catch (const InsufficientFunds& e) {  // derived handler first (TRY-4)
    if (e.account != 0 || e.code != 1) return 2;
  } catch (const TxError&) {
    return 3;
  }
  if (l.balance[0] != 10 || l.balance[1] != 0) return 4;

  // 2. A failed batch: earlier committed steps are undone too.
  try {
    batch(l);
    return 5;
  } catch (const std::exception& e) {     // base handler catches derived (TRY-2)
    (void)e;
  }
  if (l.balance[0] != 10 || l.balance[1] != 0 || l.balance[2] != 0) return 6;

  // 3. A good step commits.
  transfer(l, 0, 3, 7);
  if (l.balance[0] != 3 || l.balance[3] != 7) return 7;
  return 0;
}
