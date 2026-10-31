// tidal-rules: TYN-6
// tidal-expect: run
class Engine;
class Car { public: Engine* e = nullptr; int wheels = 4; };
class Engine { public: int power = 100; };
int main() { Car c; Engine en; c.e = &en; return c.e->power == 100 ? 0 : 1; }
