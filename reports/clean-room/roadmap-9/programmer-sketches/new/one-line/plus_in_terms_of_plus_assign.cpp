struct Money { long cents = 0; Money(long c) : cents(c) {} Money& operator+=(const Money& o) { cents += o.cents; return *this; } friend Money operator+(Money a, const Money& b) { a += b; return a; } };
int main() { Money m(150); Money t = m + 50; t += 25; Money u = 5 + t; return u.cents == 230 ? 0 : 1; }
