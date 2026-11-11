// Planted mistake: a private member read from outside the class.
class Account { long balance = 0; public: long get() const { return balance; } };
int main() { Account a; return static_cast<int>(a.balance); }
