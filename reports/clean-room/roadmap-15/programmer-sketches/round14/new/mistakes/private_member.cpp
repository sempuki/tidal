class Account { int balance = 0; public: void add(int v) { balance += v; } };
int main() { Account a; a.add(3); return a.balance - 3; }
