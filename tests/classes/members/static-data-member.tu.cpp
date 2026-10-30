struct Counter { static int count; };
void bump() { Counter::count += 1; }
