using System;
using System.Collections.Generic;

// A turnstile state machine driven by a queue of events. Each state is
// an object that handles an event and returns the next state. A
// using block marks the machine closed when it leaves, also on early exit.
interface IState { IState On(string ev, Machine m); string Name { get; } }

sealed class Locked : IState
{
    public string Name => "locked";
    public IState On(string ev, Machine m)
    {
        if (ev == "coin") { m.Coins++; return new Unlocked(); }
        if (ev == "push") m.Alarms++;
        return this;
    }
}

sealed class Unlocked : IState
{
    public string Name => "unlocked";
    public IState On(string ev, Machine m)
    {
        if (ev == "push") { m.Passes++; return new Locked(); }
        if (ev == "coin") m.Refunds++;
        return this;
    }
}

sealed class Machine : IDisposable
{
    public int Coins, Passes, Alarms, Refunds;
    public IState State = new Locked();
    public static bool Closed = false;
    public void Run(Queue<string> events)
    {
        while (events.Count > 0)
        {
            var ev = events.Dequeue();
            if (ev == "stop") return;
            State = State.On(ev, this);
        }
    }
    public void Dispose() { Closed = true; }
}

static class Program
{
    static int Main()
    {
        int failures = 0;
        var q = new Queue<string>(new[] { "push", "coin", "coin", "push", "push", "coin", "stop", "push" });
        Machine m;
        using (m = new Machine()) { m.Run(q); }
        if (m.Coins != 2 || m.Refunds != 1 || m.Passes != 1 || m.Alarms != 2) failures++;
        if (m.State.Name != "unlocked") failures++;
        if (!Machine.Closed) failures++;
        if (q.Count != 1) failures++;
        return failures;
    }
}
