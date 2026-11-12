enum class Suit { clubs, hearts };
struct Card {
  using enum Suit;
  Suit s = hearts;
};
static_assert(Card{}.s == Card::hearts && Card::clubs == Suit::clubs);
int main() { return Card{}.s == Suit::hearts ? 0 : 1; }
