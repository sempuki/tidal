// accounts.cpp: a small bank account hierarchy.
// Spec rules used: CLS-4, CLS-5 (const member), ACC-1, ACC-8 (public base),
// CTOR-5, CTOR-6 (member initializer list), VIRT-1, VIRT-3 (override),
// VIRT-7 (virtual destructor), THR-1, THR-4, TRY-1, TRY-2 (throw / catch),
// NEW-1, NEW-9 (new / delete), TR-2 (exit status).

struct OverdraftError {
  const char* owner;
  long long requested;
  long long available;
};

class Account {
public:
  Account(const char* owner, long long opening)
      : owner_(owner), balance_(opening) {}
  virtual ~Account() = default;

  const char* owner() const { return owner_; }
  long long balance() const { return balance_; }

  void deposit(long long amount) { balance_ += amount; }

  // The base account cannot go below zero.
  virtual void withdraw(long long amount) {
    if (amount > balance_) throw OverdraftError{owner_, amount, balance_};
    balance_ -= amount;
  }

protected:
  void set_balance(long long value) { balance_ = value; }

private:
  // Members are built in declaration order (CTOR-6), so this order must
  // match the member initializer list above, or -Wall warns (CTOR-13).
  const char* owner_;
  long long balance_;
};

// A checking account can go below zero, down to its limit.
class Checking final : public Account {
public:
  Checking(const char* owner, long long opening, long long limit)
      : Account(owner, opening), limit_(limit) {}

  void withdraw(long long amount) override {
    if (amount > balance() + limit_)
      throw OverdraftError{owner(), amount, balance() + limit_};
    set_balance(balance() - amount);
  }

private:
  long long limit_;
};

// A savings account charges a fee of 1 for each withdrawal.
class Savings final : public Account {
public:
  Savings(const char* owner, long long opening) : Account(owner, opening) {}

  void withdraw(long long amount) override { Account::withdraw(amount + 1); }
};

// Takes pointers to const, so it can only call const members (CLS-5).
// Account*[] converts to const Account* const* (CV-7).
long long total(const Account* const* list, int count) {
  long long sum = 0;
  for (int i = 0; i < count; ++i) sum += list[i]->balance();
  return sum;
}

int main() {
  Account* accounts[3] = {
      new Account("ann", 100),
      new Checking("bob", 50, 200),
      new Savings("cy", 30),
  };

  int failures = 0;
  int overdrafts = 0;

  long long amounts[3] = {150, 150, 40};
  for (int i = 0; i < 3; ++i) {
    try {
      accounts[i]->withdraw(amounts[i]);   // virtual call (VIRT-1)
    } catch (const OverdraftError& e) {    // by reference (TRY-2)
      ++overdrafts;
      if (e.requested != amounts[i] && e.requested != amounts[i] + 1)
        ++failures;
    }
  }

  // ann: 150 > 100, overdraft. bob: 150 <= 250, balance -100.
  // cy: 41 > 30, overdraft.
  if (overdrafts != 2) ++failures;
  if (accounts[0]->balance() != 100) ++failures;
  if (accounts[1]->balance() != -100) ++failures;
  if (accounts[2]->balance() != 30) ++failures;
  if (total(accounts, 3) != 30) ++failures;

  const Checking audit("dee", 10, 0);
  if (audit.balance() != 10) ++failures;   // const accessor on const object

  for (int i = 0; i < 3; ++i) delete accounts[i];   // VIRT-7
  return failures;
}
