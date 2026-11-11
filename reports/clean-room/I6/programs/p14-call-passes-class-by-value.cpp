// INST-1, INST-P1 (notes 20): a call that passes a class specialization by value.
template <class T> struct Box { static_assert(sizeof(T) == 0); };
void take(Box<int>);
void use(Box<int>* p) { take(*p); }
int main() { return 0; }
