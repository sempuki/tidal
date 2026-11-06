// OVL-33: two rewritten candidates from one operator<=>; the written order wins.
struct Order {
  friend int operator<(Order, int) { return 1; }
  friend int operator<(int, Order) { return 2; }
};
struct Item {};
Order operator<=>(const Item&, const Item&) { return {}; }
Item a, b;
int main() { return (a < b) == 1 ? 0 : 1; }
