// tidal-rules: SINIT-3
// tidal-expect: run
// tidal-sources: across-units-order.tu.cpp
int other_value();
int seen = other_value();
int main() { return seen == 0 || seen == 7 ? 0 : 1; }
