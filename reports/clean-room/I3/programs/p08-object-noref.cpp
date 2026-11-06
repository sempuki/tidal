// ICS-40: no ICS-28 for an implicit object parameter without a ref-qualifier.
struct Item { int operator+(int) const { return 1; } };
int operator+(const Item&&, long) { return 2; }
struct Fixed { int operator+(int) const & { return 1; } };
int operator+(const Fixed&&, int) { return 2; }
int main() { return (Item{} + 0) == 1 && (Fixed{} + 0) == 2 ? 0 : 1; }
