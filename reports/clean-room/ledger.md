# Ledger of exercise findings

Each finding (R10-1, W18-2, X22-6, and so on) that a report under
`reports/clean-room/` names has one row here (decision 0058). The
front-end findings (G1, G2, ...) have their own ledger in
`rust-frontend/ledger.md`. The State column says how the clean-room
writer answered the finding:

- `commit`: a commit acted on it, or named it as done.
- `question`: a question holds it.
- `text`: the specification already decides it, or it needs no change.
- `out of scope`: the topic is outside Tidal's scope.
- `planned`: an item of a phase decision takes it; the Record names the item.
- `open`: no answer yet.

For a `commit` row from before this ledger, the Record column names
the first commits after the report that cite the finding, alone or in
a range such as "RC-1 to RC-25". `tools/check_specification.py` fails
when a report names a finding that has no row here, when a `commit`
row names no commit, when an `open` or `planned` row whose Record names
the front end names no test, and when a `planned` row names an item that the
roadmap marks done or moved (decision 0059).

A gap review under `reports/review/` gives each point that its repair
commit leaves an ID such as `RV5-A-3`, and the point gets a row here.
The rows RV4-1 to RV4-7 give IDs to the doubts that the RV4 repair
commit (7cf4574b) left for the next review. The RV5 reviews gave their
own IDs (RV5-A-n and RV5-B-n), and so do the RV6, RV7, and RV8 reviews
(RV6-A-n, RV7-B-n, and so on). The read of a repair diff gives IDs of
the form RV8-R-n (the read of RV7's repair diff) or RV9-R-n (the read of
RV8's). The rows RM12-1 to RM12-3 give IDs to three findings
that only a commit message or a probe README held until the round 12
editor report named them (`roadmap-12/editor.md`, "Records that
drift").

| Finding | Report | State | Record |
|---|---|---|---|
| DC-1 | `D-classes/report.md` | commit | 008bc71c |
| DC-2 | `D-classes/report.md` | commit | 008bc71c |
| DC-3 | `D-classes/report.md` | commit | 008bc71c |
| DC-4 | `D-classes/report.md` | commit | 008bc71c |
| DC-5 | `D-classes/report.md` | commit | 008bc71c |
| DC-6 | `D-classes/report.md` | commit | 008bc71c |
| DC-7 | `D-classes/report.md` | commit | 008bc71c |
| DC-8 | `D-classes/report.md` | commit | 008bc71c |
| DC-9 | `D-classes/report.md` | commit | 008bc71c |
| DC-10 | `D-classes/report.md` | commit | 008bc71c |
| DC-11 | `D-classes/report.md` | commit | 008bc71c |
| DC-12 | `D-classes/report.md` | commit | 008bc71c |
| DC-13 | `D-classes/report.md` | commit | 008bc71c |
| DC-14 | `D-classes/report.md` | commit | 008bc71c |
| DC-15 | `D-classes/report.md` | commit | 008bc71c |
| DC-16 | `D-classes/report.md` | commit | 008bc71c |
| DC-17 | `D-classes/report.md` | commit | 008bc71c |
| I1-1 | `I1/report.md` | commit | 076aa546 |
| I1-2 | `I1/report.md` | commit | 076aa546 |
| I1-3 | `I1/report.md` | commit | 076aa546 |
| I1-4 | `I1/report.md` | commit | 076aa546 |
| I1-5 | `I1/report.md` | commit | 076aa546 |
| I1-6 | `I1/report.md` | commit | 076aa546 |
| I1-7 | `I1/report.md` | commit | 076aa546 |
| I1-8 | `I1/report.md` | commit | 076aa546 |
| I1-9 | `I1/report.md` | commit | 076aa546 |
| I1-10 | `I1/report.md` | commit | 076aa546 |
| I1-11 | `I1/report.md` | commit | 076aa546 |
| I1-12 | `I1/report.md` | commit | 076aa546 |
| I1-13 | `I1/report.md` | commit | 076aa546 |
| I2-1 | `I2/report.md` | commit | d9c18bcc |
| I2-2 | `I2/report.md` | commit | d9c18bcc |
| I2-3 | `I2/report.md` | commit | d9c18bcc |
| I2-4 | `I2/report.md` | commit | d9c18bcc |
| I2-5 | `I2/report.md` | commit | d9c18bcc |
| I2-6 | `I2/report.md` | commit | d9c18bcc |
| I2-7 | `I2/report.md` | commit | d9c18bcc |
| I2-8 | `I2/report.md` | commit | d9c18bcc |
| I2-9 | `I2/report.md` | commit | d9c18bcc |
| I2-10 | `I2/report.md` | commit | d9c18bcc |
| I2-11 | `I2/report.md` | commit | d9c18bcc |
| I2-12 | `I2/report.md` | commit | d9c18bcc |
| I2-13 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-14 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-15 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-16 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-17 | `I2/report.md` | commit | d9c18bcc |
| I3-1 | `I3/report.md` | commit | 3b83788d |
| I3-2 | `I3/report.md` | commit | 4a193ea4 |
| I3-3 | `I3/report.md` | commit | 4a193ea4 |
| I3-4 | `I3/report.md` | text | a component error; the text decides it |
| I3-5 | `I3/report.md` | commit | 3b83788d |
| I3-6 | `I3/report.md` | text | a component error; the text decides it |
| I3-7 | `I3/report.md` | commit | 4a193ea4 |
| I3-8 | `I3/report.md` | commit | 4a193ea4 |
| I3-9 | `I3/report.md` | out of scope | extended floating-point types stay out of scope |
| I3-10 | `I3/report.md` | commit | 4a193ea4 |
| I3-11 | `I3/report.md` | commit | 4a193ea4 |
| I3-12 | `I3/report.md` | text | CONV-1 gives the promotion of every character type |
| I3-13 | `I3/report.md` | commit | 3b83788d |
| I3-14 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-15 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-16 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-17 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-18 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| PU1-1 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-2 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-3 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-4 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-5 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-6 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-7 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-8 | `PU1/report.md` | commit | b7e0adf5 |
| PU2-1 | `PU2/report.md` | commit | 32c89f7b |
| PU2-2 | `PU2/report.md` | commit | 32c89f7b |
| PU2-3 | `PU2/report.md` | commit | 32c89f7b |
| PU2-4 | `PU2/report.md` | commit | 32c89f7b |
| PU2-5 | `PU2/report.md` | commit | 32c89f7b |
| PU2-6 | `PU2/report.md` | commit | 32c89f7b |
| PU3-1 | `PU3/report.md` | question | Q-0395 |
| PU3-2 | `PU3/report.md` | question | Q-0376 |
| PU3-3 | `PU3/report.md` | commit | 47f58739 |
| PU3-4 | `PU3/report.md` | commit | 47f58739 |
| R10-1 | `R10/report.md` | commit | 47f58739 |
| R10-2 | `R10/report.md` | commit | 47f58739 |
| R10-3 | `R10/report.md` | commit | 47f58739 |
| R10-4 | `R10/report.md` | commit | 3b83788d |
| R10-5 | `R10/report.md` | commit | 47f58739 |
| R10-6 | `R10/report.md` | commit | 3b83788d |
| R10-7 | `R10/report.md` | commit | 47f58739 |
| R10-8 | `R10/report.md` | text | no change needed; the finding says so |
| R10-9 | `R10/report.md` | commit | 47f58739 |
| R8A-1 | `R8/report-a.md` | commit | 60072525 |
| R8A-2 | `R8/report-a.md` | commit | 60072525 |
| R8A-3 | `R8/report-a.md` | commit | 60072525 |
| R8A-4 | `R8/report-a.md` | commit | 60072525 |
| R8A-5 | `R8/report-a.md` | commit | 60072525 |
| R8A-6 | `R8/report-a.md` | commit | 60072525 |
| R8A-7 | `R8/report-a.md` | commit | 60072525 |
| R8A-8 | `R8/report-a.md` | commit | 60072525 |
| R8A-9 | `R8/report-a.md` | commit | 60072525 |
| R8A-10 | `R8/report-a.md` | commit | 60072525 |
| R8B-1 | `R8/report-b.md` | commit | 60072525 |
| R8B-2 | `R8/report-b.md` | commit | 60072525 |
| R8B-3 | `R8/report-b.md` | commit | 60072525 |
| R8B-4 | `R8/report-b.md` | commit | 60072525 |
| R8B-5 | `R8/report-b.md` | commit | 60072525 |
| R8B-6 | `R8/report-b.md` | commit | 60072525 |
| R8B-7 | `R8/report-b.md` | commit | 60072525 |
| R8B-8 | `R8/report-b.md` | commit | 60072525 |
| R8B-9 | `R8/report-b.md` | commit | 60072525 |
| R8B-10 | `R8/report-b.md` | commit | 60072525 |
| R8C-1 | `R8/report-c.md` | commit | 60072525 |
| R8C-2 | `R8/report-c.md` | commit | 60072525 |
| R8C-3 | `R8/report-c.md` | commit | 60072525 |
| R8C-4 | `R8/report-c.md` | commit | 60072525 |
| R8C-5 | `R8/report-c.md` | commit | 60072525 |
| R8C-6 | `R8/report-c.md` | commit | 60072525 |
| R8C-7 | `R8/report-c.md` | commit | 60072525 |
| R8C-8 | `R8/report-c.md` | commit | 60072525 |
| R8C-9 | `R8/report-c.md` | commit | 60072525 |
| R8C-10 | `R8/report-c.md` | commit | 60072525 |
| R8C-11 | `R8/report-c.md` | commit | 60072525 |
| R8D-1 | `R8/report-d.md` | commit | 60072525 |
| R8D-2 | `R8/report-d.md` | commit | 60072525 |
| R8D-3 | `R8/report-d.md` | commit | 60072525 |
| R8D-4 | `R8/report-d.md` | commit | 60072525 |
| R8D-5 | `R8/report-d.md` | commit | 60072525 |
| R8D-6 | `R8/report-d.md` | commit | 60072525 |
| R8D-7 | `R8/report-d.md` | commit | 60072525 |
| R8D-8 | `R8/report-d.md` | commit | 60072525 |
| R9A-1 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-2 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-3 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-4 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-5 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-6 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-7 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-8 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-9 | `R9/report-a.md` | commit | 32c89f7b |
| R9B-1 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-2 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-3 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-4 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-5 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-6 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-7 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-8 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-9 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-10 | `R9/report-b.md` | commit | 32c89f7b |
| R9C-1 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-2 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-3 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-4 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-5 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-6 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-7 | `R9/report-c.md` | commit | 32c89f7b |
| R9D-1 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-2 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-3 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-4 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-5 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-6 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-7 | `R9/report-d.md` | commit | 32c89f7b |
| RA-1 | `R-templates/report-a.md` | commit | b456a30d |
| RA-2 | `R-templates/report-a.md` | commit | b456a30d |
| RA-3 | `R-templates/report-a.md` | commit | b456a30d |
| RA-4 | `R-templates/report-a.md` | commit | b456a30d |
| RA-5 | `R-templates/report-a.md` | commit | b456a30d |
| RA-6 | `R-templates/report-a.md` | commit | b456a30d |
| RA-7 | `R-templates/report-a.md` | commit | b456a30d |
| RA-8 | `R-templates/report-a.md` | commit | b456a30d |
| RA-9 | `R-templates/report-a.md` | commit | b456a30d |
| RA-10 | `R-templates/report-a.md` | commit | b456a30d |
| RA-11 | `R-templates/report-a.md` | commit | b456a30d |
| RA-12 | `R-templates/report-a.md` | commit | b456a30d |
| RA-13 | `R-templates/report-a.md` | commit | b456a30d, 2c57d445 |
| RA-14 | `R-templates/report-a.md` | commit | b456a30d |
| RB-1 | `R-templates/report-b.md` | commit | b456a30d |
| RB-2 | `R-templates/report-b.md` | commit | b456a30d |
| RB-3 | `R-templates/report-b.md` | commit | b456a30d |
| RB-4 | `R-templates/report-b.md` | commit | b456a30d |
| RB-5 | `R-templates/report-b.md` | commit | b456a30d |
| RB-6 | `R-templates/report-b.md` | commit | b456a30d |
| RB-7 | `R-templates/report-b.md` | commit | b456a30d |
| RB-8 | `R-templates/report-b.md` | commit | b456a30d |
| RB-9 | `R-templates/report-b.md` | commit | b456a30d |
| RB-10 | `R-templates/report-b.md` | commit | b456a30d |
| RB-11 | `R-templates/report-b.md` | commit | b456a30d |
| RB-12 | `R-templates/report-b.md` | commit | b456a30d |
| RB-13 | `R-templates/report-b.md` | commit | b456a30d |
| RB-14 | `R-templates/report-b.md` | commit | b456a30d |
| RB-15 | `R-templates/report-b.md` | commit | b456a30d |
| RB-16 | `R-templates/report-b.md` | commit | b456a30d |
| RB-17 | `R-templates/report-b.md` | commit | b456a30d |
| RC-1 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-2 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-3 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-4 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-5 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-6 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-7 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-8 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-9 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-10 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-11 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-12 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-13 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-14 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-15 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-16 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-17 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-18 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-19 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-20 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-21 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-22 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-23 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-24 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-25 | `R-templates/report-c.md` | commit | 3c569c2b |
| RCA-1 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-2 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-3 | `R-classes/report-a.md` | commit | 2d92b5e3, d4705c79 |
| RCA-4 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-5 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-6 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-7 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-8 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-9 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-10 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-11 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-12 | `R-classes/report-a.md` | commit | d4705c79 |
| RCB-1 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-2 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-3 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-4 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-5 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-6 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-7 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-8 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-9 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-10 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-11 | `R-classes/report-b.md` | commit | d4705c79 |
| RCC-1 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-2 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-3 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-4 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-5 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-6 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-7 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-8 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-9 | `R-classes/report-c.md` | commit | d4705c79 |
| RCD-1 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-2 | `R-classes/report-d.md` | commit | a025c999, d4705c79 |
| RCD-3 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-4 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-5 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-6 | `R-classes/report-d.md` | commit | d4705c79 |
| RDA-1 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-2 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-3 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-4 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-5 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-6 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-7 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-8 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-9 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-10 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-11 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDB-1 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-2 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-3 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-4 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-5 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-6 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-7 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-8 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-9 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-10 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDC-1 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-2 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-3 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-4 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-5 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-6 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-7 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-8 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-9 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-10 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDD-1 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-2 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-3 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-4 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-5 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-6 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-7 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-8 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-9 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDE-1 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-2 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-3 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-4 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-5 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-6 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-7 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-8 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| REA-1 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-2 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-3 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-4 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-5 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-6 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-7 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-8 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-9 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-10 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-11 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-12 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-13 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REB-1 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-2 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-3 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-4 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-5 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-6 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-7 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-8 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-9 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-10 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-11 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-12 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-13 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-14 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REC-1 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-2 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-3 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-4 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-5 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-6 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-7 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-8 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-9 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-10 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-11 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-12 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-13 | `R-expressions/report-c.md` | commit | 1b47525e |
| RED-1 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-2 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-3 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-4 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-5 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-6 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-7 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-8 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-9 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-10 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-11 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-12 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-13 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-14 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-15 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-16 | `R-expressions/report-d.md` | commit | 1b47525e |
| RPA-1 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-2 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-3 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-4 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-5 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-6 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-7 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-8 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-9 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-10 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPB-1 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-2 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-3 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-4 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-5 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-6 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-7 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-8 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPC-1 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPC-2 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPC-3 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPC-4 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-5 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-6 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-7 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-8 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-9 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-10 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-11 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPD-1 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-2 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-3 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-4 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-5 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-6 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-7 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-8 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-9 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-10 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-11 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPE-1 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-2 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-3 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-4 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-5 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-6 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RS-1 | `R-statements/report.md` | commit | 1b47525e |
| RS-2 | `R-statements/report.md` | commit | 1b47525e |
| RS-3 | `R-statements/report.md` | commit | 1b47525e |
| RS-4 | `R-statements/report.md` | commit | 1b47525e |
| RS-5 | `R-statements/report.md` | commit | 1b47525e |
| RS-6 | `R-statements/report.md` | commit | 1b47525e |
| RS-7 | `R-statements/report.md` | commit | 1b47525e |
| RS-8 | `R-statements/report.md` | commit | 1b47525e |
| RS-9 | `R-statements/report.md` | commit | 1b47525e |
| RS-10 | `R-statements/report.md` | commit | 1b47525e |
| RS-11 | `R-statements/report.md` | commit | 1b47525e |
| RS-12 | `R-statements/report.md` | commit | 1b47525e |
| RS-13 | `R-statements/report.md` | commit | 1b47525e |
| RS-14 | `R-statements/report.md` | commit | 1b47525e |
| RS-15 | `R-statements/report.md` | commit | 1b47525e |
| RS-16 | `R-statements/report.md` | commit | 1b47525e |
| W1-1 | `W1/report.md` | commit | ea47eaf8, 0bf1a546 |
| W1-2 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-3 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-4 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-5 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-6 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-7 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-8 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-9 | `W1/report.md` | commit | 0bf1a546 |
| W10-1 | `W10/report.md` | commit | d58799c6 |
| W10-2 | `W10/report.md` | commit | d58799c6 |
| W10-3 | `W10/report.md` | commit | d58799c6 |
| W10-4 | `W10/report.md` | commit | d58799c6 |
| W10-5 | `W10/report.md` | commit | d58799c6 |
| W10-6 | `W10/report.md` | commit | d58799c6 |
| W11-1 | `W11/report.md` | commit | d58799c6 |
| W11-2 | `W11/report.md` | commit | d58799c6 |
| W11-3 | `W11/report.md` | commit | d58799c6 |
| W11-4 | `W11/report.md` | commit | d58799c6 |
| W11-5 | `W11/report.md` | commit | d58799c6 |
| W11-6 | `W11/report.md` | commit | d58799c6 |
| W11-7 | `W11/report.md` | commit | d58799c6 |
| W12-1 | `W12/report.md` | commit | d58799c6 |
| W12-2 | `W12/report.md` | commit | d58799c6 |
| W12-3 | `W12/report.md` | commit | d58799c6 |
| W12-4 | `W12/report.md` | commit | d58799c6 |
| W12-5 | `W12/report.md` | commit | d58799c6 |
| W12-6 | `W12/report.md` | commit | d58799c6 |
| W12-7 | `W12/report.md` | commit | d58799c6 |
| W13-1 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-2 | `W13/report.md` | commit | 0bf1a546 |
| W13-3 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-4 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-5 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-6 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-7 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-8 | `W13/report.md` | commit | 0bf1a546 |
| W13-9 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-1 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-2 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-3 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-4 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-5 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-6 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-7 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-8 | `W14/report.md` | text | TR-P2 links the units into one program; SINIT-P2 gives the accessor idiom; the linker command is a tool matter |
| W15-1 | `W15/report.md` | commit | ef3408dd |
| W15-2 | `W15/report.md` | commit | ef3408dd |
| W15-3 | `W15/report.md` | commit | ef3408dd, b456a30d |
| W15-4 | `W15/report.md` | commit | ef3408dd, b456a30d |
| W15-5 | `W15/report.md` | commit | ef3408dd |
| W15-6 | `W15/report.md` | commit | ef3408dd |
| W15-7 | `W15/report.md` | commit | ef3408dd |
| W15-8 | `W15/report.md` | commit | ef3408dd |
| W15-9 | `W15/report.md` | commit | ef3408dd |
| W16-1 | `W16/report.md` | commit | 26f05fca |
| W16-2 | `W16/report.md` | commit | 26f05fca |
| W16-3 | `W16/report.md` | commit | 26f05fca |
| W16-4 | `W16/report.md` | commit | 26f05fca |
| W16-5 | `W16/report.md` | commit | 26f05fca |
| W16-6 | `W16/report.md` | commit | 26f05fca |
| W16-7 | `W16/report.md` | commit | 26f05fca |
| W17-1 | `W17/report.md` | commit | 26f05fca |
| W17-2 | `W17/report.md` | commit | 26f05fca |
| W17-3 | `W17/report.md` | commit | 26f05fca |
| W17-4 | `W17/report.md` | commit | 26f05fca |
| W17-5 | `W17/report.md` | commit | 26f05fca |
| W17-6 | `W17/report.md` | commit | 26f05fca |
| W17-7 | `W17/report.md` | commit | 26f05fca |
| W18-1 | `W18/report.md` | commit | eff1a372 |
| W18-2 | `W18/report.md` | commit | 4a193ea4 |
| W18-3 | `W18/report.md` | commit | eff1a372 |
| W18-4 | `W18/report.md` | commit | eff1a372 |
| W18-5 | `W18/report.md` | commit | eff1a372 |
| W18-6 | `W18/report.md` | commit | eff1a372 |
| W18-7 | `W18/report.md` | commit | eff1a372 |
| W18-8 | `W18/report.md` | commit | eff1a372 |
| W18-9 | `W18/report.md` | commit | 4a193ea4 |
| W18-10 | `W18/report.md` | commit | eff1a372 |
| W18-11 | `W18/report.md` | commit | eff1a372 |
| W18-12 | `W18/report.md` | commit | 4a193ea4 |
| W18-13 | `W18/report.md` | commit | 4a193ea4 |
| W18-14 | `W18/report.md` | text | UNI-7 gives the designator form; the -Wextra warning is outside Tidal |
| W18-15 | `W18/report.md` | commit | 4a193ea4 |
| W19-1 | `W19/report.md` | commit | b1d89e77 |
| W19-2 | `W19/report.md` | commit | b1d89e77 |
| W19-3 | `W19/report.md` | commit | eff1a372 |
| W19-4 | `W19/report.md` | commit | eff1a372 |
| W19-5 | `W19/report.md` | commit | b1d89e77 |
| W19-6 | `W19/report.md` | text | needs no change: domain knowledge and idioms that TSP-10 and the declarators page allow |
| W2-1 | `W2/report.md` | commit | d58799c6 |
| W2-2 | `W2/report.md` | commit | d58799c6 |
| W2-3 | `W2/report.md` | commit | d58799c6 |
| W2-4 | `W2/report.md` | commit | d58799c6 |
| W2-5 | `W2/report.md` | commit | d58799c6 |
| W2-6 | `W2/report.md` | commit | d58799c6 |
| W3-1 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-2 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-3 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-4 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-5 | `W3/report.md` | commit | 0bf1a546 |
| W3-6 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-1 | `W4/report.md` | commit | 0bf1a546 |
| W4-2 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-3 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-4 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-5 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-6 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-7 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-8 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-9 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-10 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W5-1 | `W5/report.md` | commit | e5cb0b27, d58799c6 |
| W5-2 | `W5/report.md` | commit | d58799c6 |
| W5-3 | `W5/report.md` | commit | d58799c6 |
| W5-4 | `W5/report.md` | commit | d58799c6 |
| W5-5 | `W5/report.md` | commit | d58799c6 |
| W5-6 | `W5/report.md` | commit | d58799c6 |
| W5-7 | `W5/report.md` | commit | d58799c6 |
| W6-1 | `W6/report.md` | commit | d58799c6 |
| W6-2 | `W6/report.md` | commit | d58799c6 |
| W6-3 | `W6/report.md` | commit | d58799c6 |
| W6-4 | `W6/report.md` | commit | d58799c6 |
| W6-5 | `W6/report.md` | commit | d58799c6 |
| W6-6 | `W6/report.md` | commit | d58799c6 |
| W6-7 | `W6/report.md` | commit | d58799c6 |
| W7-1 | `W7/report.md` | commit | d58799c6 |
| W7-2 | `W7/report.md` | commit | d58799c6 |
| W7-3 | `W7/report.md` | commit | d58799c6 |
| W7-4 | `W7/report.md` | commit | d58799c6 |
| W7-5 | `W7/report.md` | commit | d58799c6 |
| W7-6 | `W7/report.md` | commit | d58799c6 |
| W7-7 | `W7/report.md` | commit | d58799c6 |
| W8-1 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-2 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-3 | `W8/report.md` | commit | 0bf1a546 |
| W8-4 | `W8/report.md` | commit | 0bf1a546 |
| W8-5 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-6 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-7 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-8 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-9 | `W8/report.md` | text | needs no change: the preference for noexcept moves belongs to the library |
| W9-1 | `W9/report.md` | commit | d58799c6 |
| W9-2 | `W9/report.md` | commit | d58799c6 |
| W9-3 | `W9/report.md` | commit | d58799c6 |
| W9-4 | `W9/report.md` | commit | d58799c6 |
| W9-5 | `W9/report.md` | commit | d58799c6 |
| W9-6 | `W9/report.md` | commit | d58799c6 |
| W9-7 | `W9/report.md` | commit | d58799c6 |
| W9-8 | `W9/report.md` | commit | d58799c6 |
| W9-9 | `W9/report.md` | commit | d58799c6 |
| X1-1 | `X1/report.md` | commit | dd7372af |
| X1-2 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-3 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-4 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-5 | `X1/report.md` | commit | 1fd0bc54, 5325b7ee |
| X1-6 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-7 | `X1/report.md` | commit | 1fd0bc54, dd7372af, 2263667c |
| X1-8 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-9 | `X1/report.md` | commit | dd7372af, 2263667c |
| X1-10 | `X1/report.md` | commit | dd7372af, 2263667c |
| X1-11 | `X1/report.md` | commit | 5325b7ee |
| X1-12 | `X1/report.md` | commit | dd7372af |
| X1-13 | `X1/report.md` | commit | dd7372af, 2263667c |
| X1-14 | `X1/report.md` | commit | dd7372af, b95c0b71 |
| X1-15 | `X1/report.md` | commit | 963caf5b, dd7372af |
| X1-16 | `X1/report.md` | commit | dd7372af |
| X1-17 | `X1/report.md` | commit | dd7372af |
| X10-1 | `X10/report.md` | commit | bddd2a08 |
| X10-2 | `X10/report.md` | commit | bddd2a08 |
| X10-3 | `X10/report.md` | commit | bddd2a08 |
| X10-4 | `X10/report.md` | commit | bddd2a08 |
| X10-5 | `X10/report.md` | commit | bddd2a08 |
| X10-6 | `X10/report.md` | commit | bddd2a08 |
| X10-7 | `X10/report.md` | commit | bddd2a08 |
| X10-8 | `X10/report.md` | commit | bddd2a08 |
| X10-9 | `X10/report.md` | commit | bddd2a08 |
| X10-10 | `X10/report.md` | commit | bddd2a08 |
| X10-11 | `X10/report.md` | commit | bddd2a08 |
| X10-12 | `X10/report.md` | commit | bddd2a08 |
| X11-1 | `X11/report.md` | commit | bddd2a08 |
| X11-2 | `X11/report.md` | commit | bddd2a08 |
| X11-3 | `X11/report.md` | commit | bddd2a08 |
| X11-4 | `X11/report.md` | commit | bddd2a08 |
| X11-5 | `X11/report.md` | commit | bddd2a08 |
| X11-6 | `X11/report.md` | commit | bddd2a08 |
| X11-7 | `X11/report.md` | commit | bddd2a08 |
| X11-8 | `X11/report.md` | commit | bddd2a08 |
| X11-9 | `X11/report.md` | commit | bddd2a08 |
| X12-1 | `X12/report.md` | commit | bddd2a08 |
| X12-2 | `X12/report.md` | commit | bddd2a08 |
| X12-3 | `X12/report.md` | commit | bddd2a08 |
| X12-4 | `X12/report.md` | commit | bddd2a08 |
| X12-5 | `X12/report.md` | commit | bddd2a08 |
| X12-6 | `X12/report.md` | commit | bddd2a08 |
| X12-7 | `X12/report.md` | commit | bddd2a08 |
| X12-8 | `X12/report.md` | commit | bddd2a08 |
| X12-9 | `X12/report.md` | commit | bddd2a08 |
| X12-10 | `X12/report.md` | commit | bddd2a08 |
| X12-11 | `X12/report.md` | commit | bddd2a08 |
| X12-12 | `X12/report.md` | commit | bddd2a08 |
| X12-13 | `X12/report.md` | commit | bddd2a08 |
| X13-1 | `X13/report.md` | commit | 966aa1b2 |
| X13-2 | `X13/report.md` | commit | 966aa1b2 |
| X13-3 | `X13/report.md` | commit | 966aa1b2 |
| X13-4 | `X13/report.md` | commit | 966aa1b2 |
| X13-5 | `X13/report.md` | commit | 966aa1b2 |
| X13-6 | `X13/report.md` | commit | 966aa1b2 |
| X13-7 | `X13/report.md` | commit | 966aa1b2 |
| X13-8 | `X13/report.md` | commit | 966aa1b2 |
| X13-9 | `X13/report.md` | commit | 966aa1b2 |
| X13-10 | `X13/report.md` | commit | 966aa1b2 |
| X13-11 | `X13/report.md` | commit | 966aa1b2 |
| X13-12 | `X13/report.md` | commit | 966aa1b2 |
| X13-13 | `X13/report.md` | commit | 966aa1b2 |
| X14-1 | `X14/report.md` | commit | 966aa1b2 |
| X14-2 | `X14/report.md` | commit | 966aa1b2 |
| X14-3 | `X14/report.md` | commit | 966aa1b2 |
| X14-4 | `X14/report.md` | commit | 966aa1b2 |
| X14-5 | `X14/report.md` | commit | 966aa1b2 |
| X14-6 | `X14/report.md` | commit | 966aa1b2 |
| X14-7 | `X14/report.md` | commit | 966aa1b2 |
| X14-8 | `X14/report.md` | commit | 966aa1b2 |
| X14-9 | `X14/report.md` | commit | 966aa1b2 |
| X14-10 | `X14/report.md` | commit | 966aa1b2 |
| X15-1 | `X15/report.md` | commit | 966aa1b2 |
| X15-2 | `X15/report.md` | commit | 966aa1b2 |
| X15-3 | `X15/report.md` | commit | 966aa1b2 |
| X15-4 | `X15/report.md` | commit | 966aa1b2 |
| X15-5 | `X15/report.md` | commit | 966aa1b2 |
| X15-6 | `X15/report.md` | commit | 966aa1b2 |
| X15-7 | `X15/report.md` | commit | 966aa1b2 |
| X15-8 | `X15/report.md` | commit | 966aa1b2 |
| X15-9 | `X15/report.md` | commit | 966aa1b2 |
| X16-1 | `X16/report.md` | commit | 966aa1b2 |
| X16-2 | `X16/report.md` | commit | 966aa1b2 |
| X16-3 | `X16/report.md` | commit | 966aa1b2 |
| X16-4 | `X16/report.md` | commit | 966aa1b2 |
| X16-5 | `X16/report.md` | commit | 966aa1b2 |
| X16-6 | `X16/report.md` | commit | 966aa1b2 |
| X16-7 | `X16/report.md` | commit | 966aa1b2 |
| X17-1 | `X17/report.md` | commit | 076aa546 |
| X17-2 | `X17/report.md` | commit | 076aa546 |
| X17-3 | `X17/report.md` | commit | 076aa546 |
| X17-4 | `X17/report.md` | commit | 076aa546 |
| X17-5 | `X17/report.md` | commit | 076aa546 |
| X17-6 | `X17/report.md` | commit | 076aa546 |
| X17-7 | `X17/report.md` | commit | 076aa546 |
| X17-8 | `X17/report.md` | commit | 076aa546 |
| X17-9 | `X17/report.md` | commit | 076aa546 |
| X17-10 | `X17/report.md` | commit | 076aa546 |
| X17-11 | `X17/report.md` | commit | 076aa546 |
| X18-1 | `X18/report.md` | commit | 076aa546 |
| X18-2 | `X18/report.md` | commit | 076aa546 |
| X18-3 | `X18/report.md` | commit | 076aa546 |
| X18-4 | `X18/report.md` | commit | 076aa546 |
| X18-5 | `X18/report.md` | commit | 076aa546 |
| X18-6 | `X18/report.md` | commit | 076aa546 |
| X18-7 | `X18/report.md` | commit | 076aa546 |
| X18-8 | `X18/report.md` | commit | 076aa546 |
| X18-9 | `X18/report.md` | commit | 076aa546 |
| X18-10 | `X18/report.md` | commit | 076aa546 |
| X18-11 | `X18/report.md` | commit | 076aa546 |
| X19-1 | `X19/report.md` | commit | 076aa546 |
| X19-2 | `X19/report.md` | commit | 076aa546 |
| X19-3 | `X19/report.md` | commit | 076aa546 |
| X19-4 | `X19/report.md` | commit | 076aa546 |
| X19-5 | `X19/report.md` | commit | 076aa546 |
| X19-6 | `X19/report.md` | commit | 076aa546 |
| X19-7 | `X19/report.md` | commit | 076aa546 |
| X19-8 | `X19/report.md` | commit | 076aa546 |
| X19-9 | `X19/report.md` | commit | 076aa546 |
| X19-10 | `X19/report.md` | commit | 076aa546 |
| X19-11 | `X19/report.md` | commit | 076aa546 |
| X20-1 | `X20/report.md` | commit | 076aa546 |
| X20-2 | `X20/report.md` | commit | 076aa546 |
| X20-3 | `X20/report.md` | commit | 076aa546 |
| X20-4 | `X20/report.md` | commit | 076aa546 |
| X20-5 | `X20/report.md` | commit | 076aa546 |
| X20-6 | `X20/report.md` | commit | 076aa546 |
| X20-7 | `X20/report.md` | commit | 076aa546, 3019417d |
| X20-8 | `X20/report.md` | commit | 076aa546 |
| X20-9 | `X20/report.md` | commit | 076aa546 |
| X20-10 | `X20/report.md` | commit | 076aa546 |
| X20-11 | `X20/report.md` | commit | 076aa546 |
| X21-1 | `X21/report.md` | commit | d9c18bcc |
| X21-2 | `X21/report.md` | commit | d9c18bcc |
| X21-3 | `X21/report.md` | commit | d9c18bcc |
| X21-4 | `X21/report.md` | commit | d9c18bcc |
| X21-5 | `X21/report.md` | commit | d9c18bcc |
| X21-6 | `X21/report.md` | commit | d9c18bcc |
| X21-7 | `X21/report.md` | commit | d9c18bcc |
| X21-8 | `X21/report.md` | commit | d9c18bcc |
| X21-9 | `X21/report.md` | commit | d9c18bcc |
| X21-10 | `X21/report.md` | commit | d9c18bcc |
| X21-11 | `X21/report.md` | commit | d9c18bcc |
| X22-1 | `X22/report.md` | commit | 1b05738b |
| X22-2 | `X22/report.md` | commit | 1b05738b |
| X22-3 | `X22/report.md` | commit | 1b05738b |
| X22-4 | `X22/report.md` | commit | 1b05738b |
| X22-5 | `X22/report.md` | commit | 1b05738b |
| X22-6 | `X22/report.md` | question | Q-0412 |
| X22-7 | `X22/report.md` | commit | 1b05738b |
| X22-8 | `X22/report.md` | out of scope | needs outside the specification (versions, cost) |
| X22-9 | `X22/report.md` | commit | 1b05738b |
| X3-1 | `X3/report.md` | commit | e9eacc80, dd7372af |
| X3-2 | `X3/report.md` | commit | 8d312937, dd7372af |
| X3-3 | `X3/report.md` | commit | dd7372af, 2263667c |
| X3-4 | `X3/report.md` | commit | dd7372af |
| X3-5 | `X3/report.md` | commit | e6961598, dd7372af |
| X3-6 | `X3/report.md` | commit | dd7372af |
| X3-7 | `X3/report.md` | commit | dd7372af |
| X3-8 | `X3/report.md` | commit | 963caf5b, dd7372af |
| X3-9 | `X3/report.md` | commit | dd7372af, b95c0b71 |
| X3-10 | `X3/report.md` | commit | dd7372af, b95c0b71 |
| X3-11 | `X3/report.md` | commit | dd7372af |
| X3-12 | `X3/report.md` | commit | dd7372af |
| X3-13 | `X3/report.md` | commit | dd7372af |
| X4-1 | `X4/report.md` | commit | 5325b7ee |
| X4-2 | `X4/report.md` | commit | e6961598 |
| X4-3 | `X4/report.md` | commit | 5325b7ee |
| X4-4 | `X4/report.md` | commit | 5325b7ee, 2263667c |
| X4-5 | `X4/report.md` | commit | 5325b7ee |
| X4-6 | `X4/report.md` | commit | e6961598 |
| X4-7 | `X4/report.md` | commit | e6961598 |
| X4-8 | `X4/report.md` | commit | e6961598 |
| X4-9 | `X4/report.md` | commit | 5325b7ee |
| X4-10 | `X4/report.md` | commit | 963caf5b, 5325b7ee |
| X4-11 | `X4/report.md` | commit | 5325b7ee |
| X4-12 | `X4/report.md` | commit | 5325b7ee, b95c0b71 |
| X4-13 | `X4/report.md` | commit | 5325b7ee, b95c0b71 |
| X4-14 | `X4/report.md` | commit | 5325b7ee, b95c0b71 |
| X4-15 | `X4/report.md` | commit | 5325b7ee |
| X4-16 | `X4/report.md` | commit | 5325b7ee |
| X5-1 | `X5/report.md` | commit | 3c569c2b |
| X5-2 | `X5/report.md` | commit | 3c569c2b |
| X5-3 | `X5/report.md` | commit | 3c569c2b |
| X5-4 | `X5/report.md` | commit | 3c569c2b |
| X5-5 | `X5/report.md` | commit | 3c569c2b |
| X5-6 | `X5/report.md` | commit | 3c569c2b |
| X5-7 | `X5/report.md` | commit | 3c569c2b |
| X5-8 | `X5/report.md` | commit | 3c569c2b |
| X5-9 | `X5/report.md` | commit | 3c569c2b |
| X5-10 | `X5/report.md` | commit | 3c569c2b |
| X5-11 | `X5/report.md` | commit | 3c569c2b |
| X5-12 | `X5/report.md` | commit | 3c569c2b |
| X5-13 | `X5/report.md` | commit | 3c569c2b |
| X5-14 | `X5/report.md` | commit | 3c569c2b |
| X5-15 | `X5/report.md` | commit | 3c569c2b |
| X5-16 | `X5/report.md` | commit | 3c569c2b |
| X5-17 | `X5/report.md` | commit | 3c569c2b |
| X5-18 | `X5/report.md` | text | needs no change: a judgment about usage, with no language rule involved |
| X5-19 | `X5/report.md` | question | Q-0007 |
| X5-20 | `X5/report.md` | commit | 3c569c2b |
| X6-1 | `X6/report.md` | commit | 3c569c2b |
| X6-2 | `X6/report.md` | commit | 3c569c2b (matched by diff hunk) |
| X6-3 | `X6/report.md` | commit | 9b60e6e4, 3c569c2b |
| X6-4 | `X6/report.md` | commit | 3c569c2b |
| X6-5 | `X6/report.md` | commit | 3c569c2b |
| X6-6 | `X6/report.md` | commit | 3c569c2b |
| X6-7 | `X6/report.md` | commit | 3c569c2b |
| X6-8 | `X6/report.md` | commit | 3c569c2b |
| X6-9 | `X6/report.md` | commit | 3c569c2b |
| X6-10 | `X6/report.md` | commit | 3c569c2b |
| X6-11 | `X6/report.md` | commit | 3c569c2b |
| X6-12 | `X6/report.md` | commit | 3c569c2b |
| X6-13 | `X6/report.md` | commit | 3c569c2b |
| X6-14 | `X6/report.md` | text | needs no change: a judgment about usage, with no language rule involved |
| X6-15 | `X6/report.md` | question | Q-0007 |
| X7-1 | `X7/report.md` | commit | 97a31fbb |
| X7-2 | `X7/report.md` | commit | 97a31fbb |
| X7-3 | `X7/report.md` | commit | 97a31fbb |
| X7-4 | `X7/report.md` | commit | 97a31fbb |
| X7-5 | `X7/report.md` | commit | 97a31fbb |
| X7-6 | `X7/report.md` | commit | 97a31fbb |
| X7-7 | `X7/report.md` | commit | 97a31fbb |
| X7-8 | `X7/report.md` | commit | 97a31fbb |
| X7-9 | `X7/report.md` | commit | 97a31fbb |
| X7-10 | `X7/report.md` | commit | 97a31fbb |
| X7-11 | `X7/report.md` | commit | 97a31fbb |
| X7-12 | `X7/report.md` | commit | 97a31fbb |
| X8-1 | `X8/report.md` | commit | 97a31fbb |
| X8-2 | `X8/report.md` | commit | 97a31fbb |
| X8-3 | `X8/report.md` | commit | 97a31fbb |
| X8-4 | `X8/report.md` | commit | 97a31fbb |
| X8-5 | `X8/report.md` | commit | 97a31fbb |
| X8-6 | `X8/report.md` | commit | 97a31fbb |
| X8-7 | `X8/report.md` | commit | 97a31fbb |
| X8-8 | `X8/report.md` | commit | 97a31fbb |
| X8-9 | `X8/report.md` | commit | 97a31fbb |
| X8-10 | `X8/report.md` | commit | 97a31fbb |
| X8-11 | `X8/report.md` | commit | 97a31fbb |
| X8-12 | `X8/report.md` | commit | 97a31fbb |
| X8-13 | `X8/report.md` | commit | 97a31fbb |
| X8-14 | `X8/report.md` | commit | 97a31fbb |
| X8-15 | `X8/report.md` | commit | 97a31fbb |
| X8-16 | `X8/report.md` | commit | 97a31fbb |
| X8-17 | `X8/report.md` | commit | 97a31fbb |
| X9-1 | `X9/report.md` | commit | bddd2a08 |
| X9-2 | `X9/report.md` | commit | bddd2a08 |
| X9-3 | `X9/report.md` | commit | bddd2a08 |
| X9-4 | `X9/report.md` | commit | bddd2a08 |
| X9-5 | `X9/report.md` | commit | bddd2a08 |
| X9-6 | `X9/report.md` | commit | bddd2a08 |
| X9-7 | `X9/report.md` | commit | bddd2a08 |
| X9-8 | `X9/report.md` | commit | bddd2a08 |
| X9-9 | `X9/report.md` | commit | bddd2a08 |
| X9-10 | `X9/report.md` | commit | bddd2a08 |
| X9-11 | `X9/report.md` | commit | bddd2a08 |
| X9-12 | `X9/report.md` | commit | bddd2a08 |
| X9-13 | `X9/report.md` | commit | bddd2a08 |
| X9-14 | `X9/report.md` | commit | bddd2a08 |
| X9-15 | `X9/report.md` | commit | bddd2a08 |
| P1-1 | `P1/report.md` | commit | 1017b19c (front-end round 23); test declarations/declarators/unknown-bound-converted-class-elements.cpp |
| P1-2 | `P1/report.md` | commit | 1017b19c (front-end round 23); test declarations/initialization/references/const-ref-through-converting-constructor.cpp and overloading/overload-resolution/const-ref-parameter-through-converting-constructor.cpp |
| P1-3 | `P1/report.md` | commit | 0f3f522f |
| P1-4 | `P1/report.md` | out of scope | the specification does not list what the language lacks |
| P1-5 | `P1/report.md` | commit | 0f3f522f |
| P1-6 | `P1/report.md` | text | DCL-19 and ADD-8 decide it; the finding asks for no change |
| P1-7 | `P1/report.md` | commit | 0f3f522f |
| P1-8 | `P1/report.md` | commit | 0f3f522f |
| P1-9 | `P1/report.md` | commit | 0f3f522f |
| P2-1 | `P2/report.md` | commit | 1130ceb8, decision 0062, item F4: DCMP-P1 says that only the comparison operators can be defaulted and a hash is written by the class; an example of an owning array with deleted copy functions on the new-delete page |
| P2-2 | `P2/report.md` | commit | 1130ceb8, decision 0062, item F4: an example of a counted shared handle on the new-delete page (CPY-P2, NEW-4, OPR-5) |
| P2-3 | `P2/report.md` | commit | 1130ceb8, decision 0062, item F4: an example after PRIM-9 that compares two const char* at run time, with no constant evaluation |
| P2-4 | `P2/report.md` | commit | 1130ceb8, decision 0062, item F4: an example on the multiplicative page: a wrapping hash in unsigned, converted back with CONV-2 |
| P2-5 | `P2/report.md` | text | NEW-5 points to OBJ-2, and the front end stops the literal port with OBJ-2 as the text says; probe reductions/removeif-literal-port.cpp |
| P2-6 | `P2/report.md` | question | Q-0431; decision 0062, item F4: LSUP-1 gives the order, decltype(sizeof 0) for the count, and any access for the constructor; test initializer-list-count-before-pointer.cpp |
| P2-7 | `P2/report.md` | text | decision 0062: the report names no change to the text; outside knowledge: a pointer or nullptr for a missing value (CMP-6, CMP-15, NEW-4) |
| P2-8 | `P2/report.md` | commit | 1130ceb8, decision 0062, item F4: DCL-19 links CE-3 and CV-8 for the variables that a bound can read |
| P2-9 | `P2/report.md` | commit | b035efd4 (front-end round 25, decision 0062, item FE): the test passes; front end: "unspecified" is the first word when every explored order gives one result (SEQ-1); the text needs no change, and the front end's first line changes; test programs/P2/ledger.cpp |
| P3-1 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: RFOR-P1 says that a sequence computed on demand is a class with its own state, because coroutines are out of scope (KW-1); an example under RFOR-2 with an iterator and a separate end type |
| P3-2 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: an example on the lambdas page of a callable class template that owns its closure, with a virtual destructor and deleted copies (VIRT-P3, NEW-P2, VC-P2) |
| P3-3 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: an example on the decltype page that names a call's result type through a declared-only function template (ODR-3, INST-7) |
| P3-4 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: an example on the comparison page that == on two const char* compares addresses (CMP-15, PRIM-9) |
| P3-5 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: an example under OBJ-9 of a class that keeps a reference to a temporary argument |
| P3-6 | `P3/report.md` | question | Q-0441; decision 0063, item F5: INST-5, INST-1, and INST-P2 say that a member function's parameter and return types need to be complete only when the definition is made; tests member-parameter-type-waits-for-definition.cpp, member-deduced-return-waits-for-definition.cpp, static-member-incomplete-argument-declared.cpp, static-inline-member-incomplete-argument.cpp |
| P3-7 | `P3/report.md` | commit | a91ccc77 (front-end round 26, decision 0063, item FE): the test passes; front end: an unused member definition in a base over the derived class is checked early (NAM-9, INST-5); test templates/instantiation/member-return-type-waits-for-definition.cpp |
| P3-8 | `P3/report.md` | commit | a91ccc77 (front-end round 26, decision 0063, item FE): the test passes; front end: a constructor body checked while the derived class is incomplete (TYN-7, INST-5); test templates/instantiation/member-constructor-made-at-call.cpp |
| P3-9 | `P3/report.md` | planned | decision 0064, item FE: front-end round 26 (a91ccc77): the test passes its check and stops in the run (report-26 section 9); front end: range-based for with begin and end from a base over the derived class (RFOR-2); test statements/range-for/begin-end-from-base-over-derived.cpp |
| P3-10 | `P3/report.md` | planned | decision 0064, item FE: front-end round 26 (a91ccc77): the test passes its check and stops in the run (report-26 section 9); front end: a user operator= with a class parameter stops as class assignment (OVL-7); test overloading/overload-resolution/assignment-operator-class-parameter.cpp |
| P3-11 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: VC-P2 says that copy initialization through a converting constructor initializes the object directly; new test foundations/values/copy-init-converting-constructor-no-copy.cpp |
| P3-12 | `P3/report.md` | commit | 232ac618, decision 0063, item F5: glossary entries for closure type (LAM-2) and range (RFOR-2) |
| P4-1 | `P4/report.md` | planned | decision 0064, item FE: FN-12, CLS-2, INST-1: FN-12 read as rejecting a member function defined in its class whose type is a specialization over the class; the P4 commit adds the FN-12 sentence from the matrix; front end: tests declarations/functions/definitions/member-in-class-specialization-over-class.cpp (false reject) and member-in-class-type-completed-after-class.cpp (missed reject) |
| P4-2 | `P4/report.md` | planned | decision 0064, item FE: LSUP-1, OPR-6: front end: an operator function whose parameter is std::initializer_list<E> stops; test classes/operators/initializer-list-parameter.cpp |
| P4-3 | `P4/report.md` | planned | decision 0064, item F6: RFOR-P1, RFOR-2, CPT-P3: no example of an adaptor over any sequence |
| P4-4 | `P4/report.md` | planned | decision 0064, item F6: CPT-P3, CPT-28: no example that names a member call's result type without the library (outside knowledge) |
| P4-5 | `P4/report.md` | planned | decision 0064, item F6: STG-P2 Maybe example, CPY-5, CPY-8: the example shows no move constructor, and the sentence after it cites CPY-8 where CPY-5 applies |
| P4-6 | `P4/report.md` | text | DEL-1: the specification needs no change; the front-end message names a deleted move constructor where the deleted copy constructor is selected, and no test can observe a message's wording (frontend_check.py reads the verdict and the rule ID, which are right) |
| P4-7 | `P4/report.md` | planned | decision 0064, item F6: OPR-8, CONV-4: OPR-8 does not say which value the numeric literal form receives |
| I4-1 | `I4/report.md` | commit | ee6085aa: COND-16 base row |
| I4-2 | `I4/report.md` | commit | ee6085aa: COND-16 step 1, Q-0418 |
| I4-3 | `I4/report.md` | commit | ee6085aa: COND-P2 and COND-11 |
| I4-4 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-converts-to-int-reference-lvalue.cpp |
| I4-5 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-explicit-conversion-only.cpp |
| I4-6 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-ambiguous-base-operand.cpp |
| I4-7 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-private-base-operand.cpp |
| I4-8 | `I4/report.md` | commit | 03e578c4 |
| I4-9 | `I4/report.md` | commit | 03e578c4 |
| I4-10 | `I4/report.md` | commit | 03e578c4 |
| I4-11 | `I4/report.md` | commit | 03e578c4 |
| I4-12 | `I4/report.md` | commit | 03e578c4 |
| I4-13 | `I4/report.md` | commit | 03e578c4 |
| I4-14 | `I4/report.md` | commit | 03e578c4 |
| X23-1 | `X23/report.md` | commit | ee6085aa: ALN-4 and NEW-P5 |
| X23-2 | `X23/report.md` | commit | ee6085aa: NEW-29, Q-0417 |
| X23-3 | `X23/report.md` | commit | 0f3f522f |
| X23-4 | `X23/report.md` | commit | ee6085aa: NEW-21 |
| X23-5 | `X23/report.md` | commit | ee6085aa: STG-5 and NEW-29 |
| X23-6 | `X23/report.md` | commit | 0f3f522f |
| X23-7 | `X23/report.md` | commit | 0f3f522f |
| X25-1 | `X25/report.md` | planned | decision 0064, item F6: LSUP-10, UNI-14: LSUP-10's two places for std::construct_at in constant evaluation do not cover a union member that is not active, which UNI-14 accepts |
| X25-2 | `X25/report.md` | planned | decision 0064, item F6: OBJ-1, STG-28: no row for when an array's own lifetime begins |
| X25-3 | `X25/report.md` | planned | decision 0064, item F6: glossary lifetime, OBJ-1: the entry ends a class object's lifetime when its destructor completes, and OBJ-1 when the call starts; the scalar start differs too |
| X25-4 | `X25/report.md` | planned | decision 0064, item F6: UNI-14, UNI-6: at run time, whether the member name reaches an element built inside an inactive array member, and whether a non-overlapping active member stays active |
| X25-5 | `X25/report.md` | planned | decision 0064, item F6: CE-22, CE-P3: whether a union's trivial copy reads the parts of its active member |
| X25-6 | `X25/report.md` | planned | decision 0064, item F6: STG-P2: the storage-form table sits at the end of the storage.md Examples, and STG-P2 does not cite it |
| X25-7 | `X25/report.md` | planned | decision 0064, item F6: CE-21: no example of an element that an explicit destructor call ended |
| X25-8 | `X25/report.md` | commit | bcf30b72, decision 0063, item E: the portfolio entry of P27 says no only for a new declaration form, and the table of exercises.md section 4 gains X25 |
| X25-9 | `X25/report.md` | planned | decision 0064, item F6: outside knowledge: fixed-capacity vectors are common; no rule rests on it |
| RV4-1 | `../review/2026-11-05-gap-review-phase-10-expressions-templates.md` | commit | 3bed9cb5: LAM-6 says simple capture (RV5-B-14) |
| RV4-2 | `../review/2026-11-05-gap-review-phase-10-expressions-templates.md` | text | LAM-29 states the right result (RV5-B-15) |
| RV4-3 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 3bed9cb5: SW-P1 (RV5-A-18) |
| RV4-4 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 6c6f69f8: the limits of JMP-10 in AUTO-P3 and RET-6 |
| RV4-5 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 3b83788d: SINIT-6 points at OBJ-11 |
| RV4-6 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 6c6f69f8: TR-17 for two definitions of main and the class of the main rule |
| RV4-7 | `../review/2026-11-05-gap-review-phase-10-expressions-templates.md` | commit | 6c6f69f8: DED-29 for an address with a target type, Q-0420 |
| RV5-A-1 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: CLS-26 and trailing requires clauses |
| RV5-A-2 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: ALN-4 cites STG-23 alone |
| RV5-A-3 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: the decltype(auto) row of AUTO-P3 |
| RV5-A-4 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: RET-6 |
| RV5-A-5 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: REF-1, every implicit pointer conversion |
| RV5-A-6 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: SINIT-6 |
| RV5-A-7 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: UNW-3 points at NEW-22 for a class |
| RV5-A-8 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: CONV-1 and BF-6, bit-fields of enumeration type |
| RV5-A-9 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | text | LSUP-11 keeps the matrix's answer; pending.md row 4 holds the split |
| RV5-A-10 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: NAM-21's friend sentence |
| RV5-A-11 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | question | Q-0416 |
| RV5-A-12 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: STG-5, a class's allocation function hides placement |
| RV5-A-13 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: STG-P3 |
| RV5-A-14 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: an Evidence note on TR-17 |
| RV5-A-15 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: TR-P1 |
| RV5-A-16 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: JMP-10, with Q-0422 for structured bindings |
| RV5-A-17 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | question | Q-0421 |
| RV5-A-18 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: SW-P1 |
| RV5-B-1 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | question | Q-0418 |
| RV5-B-2 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on COND-16 and COND-3 |
| RV5-B-3 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on COND-16 and COND-3 |
| RV5-B-4 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on NEW-P5, pending.md row 4 |
| RV5-B-5 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on NEW-22 |
| RV5-B-6 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: ICS-11 |
| RV5-B-7 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on ICS-17 |
| RV5-B-8 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | question | Q-0419 |
| RV5-B-9 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on ICS-45, pending.md row 4 |
| RV5-B-10 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | text | DED-29 is a fair reading; the doubt is in the reference's note |
| RV5-B-11 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | text | DEP-P6 follows the intent of the C++23 change; the doubt is in the reference |
| RV5-B-12 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: ORD-13 names the conditions it replaces |
| RV5-B-13 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | question | Q-0417 |
| RV5-B-14 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: LAM-5 to LAM-7, simple and init captures |
| RV5-B-15 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | text | LAM-29 states the right result; the reference is silent |
| RV6-A-1 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: REF-1's last bullet names the referenced type; REF-1 links OVL-15 for the lvalue result of a conversion function |
| RV6-A-2 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: RET-6 gives a parameter of object type, which ends where VC-10 puts its destruction |
| RV6-A-3 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | question | Q-0416 |
| RV6-A-4 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | text | UNW-3 allows both global forms after a throw; pending.md row 4 holds the case and names this review |
| RV6-A-5 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: CONV-1 lists the long types for the wide character types |
| RV6-A-6 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: NAM-21 lists a trailing requires clause, with a test; an Evidence note on a friend parameter type in an inline namespace |
| RV6-A-7 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | text | LSUP-7's Evidence note and decision 0041 record the program's own placement forms as a Tidal choice |
| RV6-A-8 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: an Evidence note on TR-P1: no undefined rule covers a form at translation, and the declarations in namespace std are a Tidal choice |
| RV6-A-9 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: JMP-10 and RET-6 name a thread_local local, with a test; JMP-P3 names automatic storage and the structured binding (Q-0422) |
| RV6-A-10 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: CV-6 names decltype, with a test |
| RV6-A-11 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: DTOR-6 and DTOR-8 say a decltype that gives a reference type does not denote the class, with a reject test |
| RV6-A-12 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | text | LSUP-11 keeps the matrix's answer; pending.md row 4 holds the split and names this review |
| RV6-A-13 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: UNI-14 limits the rejection to constant evaluation; an Evidence note on the construct_at reading and the two run-time outcomes (Q-0425) |
| RV6-A-14 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: UNI-6 and UNI-14 make the run-time construction valid; the run test expects a run (Q-0425) |
| RV6-A-15 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: UNI-P1 and UNI-14 name a trivial copy of the whole union, with a test |
| RV6-A-16 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: the STG-P2 table row for a union with an array member names the initializer and construct_at |
| RV6-A-17 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: UNI-9 names an anonymous union and allows a static assertion, with two tests; UNI-P3 says an anonymous union declares no type (Q-0424) |
| RV6-A-18 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | commit | 9c009448, decision 0061, item RV6: CPY-15 covers a cv-qualified class by value, with a test; the CNS-11 clause keeps the compilers' answer in pending.md row 4 (Q-0426) |
| RV6-A-19 | `../review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md` | text | TR-17's Evidence note records the open reading |
| RV6-B-1 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: Q-0426 joins pending.md row 4 with the CNS-11 and W20-3 cases; the CNS-11 and CPY-15 Divergences lines name the reviews |
| RV6-B-2 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: an Evidence note on DEP-15; pending.md row 4 gains the DEP-15 and DEP-P4 item |
| RV6-B-3 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: DED-24 accepts an enumeration qualifier and names members of the wrong kind, with two tests |
| RV6-B-4 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: REF-2 and the glossary call the non-class prvalue binding direct; an Evidence note on ICS-46 |
| RV6-B-5 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: an Evidence note on LAM-6 from the writer probe |
| RV6-B-6 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | text | the correction changes no Tidal text |
| RV6-B-7 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | text | the doubt is in the reference; DED-20 states the compilers' result |
| RV6-B-8 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: DED-8 and VC-2 keep the cv-qualifiers of an array prvalue, with a test |
| RV6-B-9 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: DEP-24 and the DEP-P5 row name a base that is not dependent and the :: case of a dependent base, with two tests (Q-0427) |
| RV6-B-10 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 7dd6d7ca, decision 0061, item RV6: Q-0428 settles on INST-P1 |
| RV6-B-11 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | question | Q-0418 |
| RV6-B-12 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | text | the COND-16 and COND-3 Evidence note answers RV5-B-2 and RV5-B-3 |
| RV6-B-13 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | commit | 9c009448, decision 0061, item RV6: NEW-22 gives the sized aligned form after delete, with a test; the throw case stays in pending.md row 4 (UNW-3) |
| RV6-B-14 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | text | the ICS-17 Evidence probe supports the compilers' condition; the doubt is in the reference |
| RV6-B-15 | `../review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md` | text | DED-29 is a fair reading; the RV5-B-10 doubt is in the reference |
| RV7-A-1 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: DTOR-8 says a qualified base-class name after ~ names that base's destructor, with a test |
| RV7-A-2 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | text | CPY-15 keeps the compilers' answer on the CNS-11 skip; the Q-0426 divergence line and the pending.md row 4 item name this review |
| RV7-A-3 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: UNI-P3 says the anonymous union is itself an unnamed type, no declaration inside it introduces another type, and it can hold static assertions (Q-0424) |
| RV7-A-4 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: UNI-14 names the trivial move constructor and move assignment, with a test |
| RV7-A-5 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: REF-2 lists the direct cases (an xvalue of a type REF-1 admits, the result of a conversion function, a prvalue); the glossary entry names the conversion function result; two tests; an Evidence note on the pointer form |
| RV7-A-6 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | question | Q-0410, Q-0415 |
| RV7-A-7 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: LOOP-9 lists a condition declaration and the variables of a range-based for, with two tests |
| RV7-A-8 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: LSUP-6 gives the program's declaration of std::nothrow as extern with no initializer |
| RV7-A-9 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | question | Q-0431 |
| RV7-A-10 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | b5d925e0, decision 0062, item RV7: Q-0430 joins pending.md row 4 with two cases; a storage Note records both reviews and the path they read as defined; STG-P2 cites Q-0430 |
| RV7-A-11 | `../review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` | commit | 4dc57e81, decision 0062, item RV7: Q-0422 settles on JMP-P3; Q-0424 waits for a review and a use of UNI-P3 |
| RV7-B-1 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | question | Q-0226 |
| RV7-B-2 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: CNS-13, DEP-30, the DEP-P6 row, and the CNS principle row give a template-id in a template never instantiated to DEP-30; its two tests cite DEP-30 |
| RV7-B-3 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: ADD-14 Note and divergence lines; pending.md row 4 gains Q-0430 and the item ADD-14, ADD-P6 for Q-0049's one-past move |
| RV7-B-4 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | text | NEW-22 and UNW-3 keep the compilers' answer after a throw; the NEW-22 Evidence note and the UNW-3 row 4 item name this review |
| RV7-B-5 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: DEP-6 leaves a qualified name into the current instantiation and a member access through this to DEP-17; DEP-30 and DEP-P6 list them as need not diagnose |
| RV7-B-6 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: INST-1, INST-2, and INST-P1 limit the decltype exemption to a call that is the whole operand, with three tests |
| RV7-B-7 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: DEP-40 limits lookup at the definition to a namespace or an independent class qualifier, with a test |
| RV7-B-8 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | text | the doubt is small; DED-24 states the wrong-kind case after template, and no probe separates the other position |
| RV7-B-9 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | question | Q-0427 |
| RV7-B-10 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: the DEP-P5 sentence on the middle column allows the "not valid" rows (Q-0225) |
| RV7-B-11 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | question | Q-0434 |
| RV7-B-12 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: DEP-4 names typeid; an Evidence note says no test checks it (Q-0432) |
| RV7-B-13 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: the DEP-P1 lead sentence names a dependent call name and points to DEP-8 for the cases |
| RV7-B-14 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | question | Q-0433 |
| RV7-B-15 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: the INST-33 reject example says not valid, no diagnostic required (INST-32), and that every compiler rejects |
| RV7-B-16 | `../review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md` | commit | b5d925e0, decision 0062, item RV7: DED-20 states that a pack of a member template that expands only the enclosing class's pack is ordinary parameters, and the two tests cite DED-20 (Q-0429) |
| RV8-R-1 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: DTOR-8 says that a base class name with no qualifier does not denote the object's type, with a test; Q-0444 and its row 4 item hold GCC's answer and the read's lean |
| RV8-R-2 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: REF-2 leaves a bit-field rvalue out of the direct cases; the doubt on a binding through a converting constructor stays |
| RV8-R-3 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | text | LOOP-9 already makes the jump not valid; the loop variable and the initializer statement of a range-based for change no result, and the doubt stays |
| RV8-R-4 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: INST-2 names a call to a consteval function, with a test; every compiler leaves the class not instantiated, so an INST Note records the read and pending.md row 4 gains the item INST-1, INST-2 |
| RV8-R-5 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | text | INST-2 and UNEV-1 give the results that the read finds right for a class template and for a plain incomplete class; the doubt on the missing reason stays |
| RV8-R-6 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: DEP-40 sends a qualifier that names the current instantiation to DEP-17 and DEP-18 |
| RV8-R-7 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: the DEP-P1 lead sentence names an operator expression with a type-dependent operand |
| RV8-R-8 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | text | the member access through a dereferenced this or a local object of the class's own type has no test; the doubt stays |
| RV8-R-9 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: the row 4 item for ADD-14 now holds the matching signed or unsigned integer case under Q-0442, with a test; the Q-0049 line and the ADD-14 Note say that the reference agrees for a base class subobject |
| RV8-R-10 | `../review/2026-11-11-read-of-rv7-repair-diff.md` | commit | d6fd9cea, decision 0063, item RV8: a storage Note records the read's lean toward part A, with RV8-A-1 |
| RV8-A-1 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | commit | d6fd9cea, decision 0063, item RV8: STG-28 states the cast to element k through a byte pointer as an exception to STG-16, and STG-16 names it; a test reads a class at element 1 through the cast pointer; a storage Note and the row 4 STG-28 item record the review (Q-0430) |
| RV8-A-2 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | commit | d6fd9cea, decision 0063, item RV8: the STG-28 example names a class with a user-provided destructor; the class of allocated-storage-byte-pointer-class-placement.cpp gains a user-provided copy constructor, and its note cites STG-7 |
| RV8-A-3 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | commit | d6fd9cea, decision 0063, item RV8: OPR-8, OPR-10, OPR-16, and OPR-P3 give the literal operator template with one class-type parameter for string literals, with six tests; Q-0446 holds the splits |
| RV8-A-4 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | commit | d6fd9cea, decision 0063, item RV8: OPR-8 gives the numeric form of each kind of literal; OPR-16 says that the const char* form and the template are valid declarations, with a test whose GCC override names Q-0435 |
| RV8-A-5 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | text | Q-0436 is supported; the doubt on extended integer types is recorded in Q-0440 |
| RV8-A-6 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | question | Q-0444 |
| RV8-A-7 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | commit | d6fd9cea, decision 0063, item RV8: UNI-P3 says that an anonymous union needs static at namespace scope outside an unnamed namespace, and cites UNI-9 |
| RV8-A-8 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | text | every test meets the condition that the union object is alive; the doubts of RV6-A-13 and on the live union stay |
| RV8-A-9 | `../review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md` | commit | d6fd9cea, decision 0063, item RV8: REF-2 leaves a bit-field rvalue out of the direct cases, with RV8-R-2; the doubt on a binding through a converting constructor stays |
| RV8-B-1 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | text | the tests make no specialization and every compiler rejects at the definition, so no test separates the readings; the doubt stays |
| RV8-B-2 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | question | Q-0225, Q-0226 |
| RV8-B-3 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | text | the left-out value-dependent forms change no test; the doubt stays |
| RV8-B-4 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | commit | d6fd9cea, decision 0063, item RV8: a test holds the inline static member apart; the CTAD divergence line and a Note record the review; pending.md row 4 gains the item CTAD-2, Q-0439 |
| RV8-B-5 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | commit | d6fd9cea, decision 0063, item RV8: a CTAD-17 Note and the Q-0020 line record the review's reading; pending.md row 4 gains the rule item CTAD-17 |
| RV8-B-6 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | commit | d6fd9cea, decision 0063, item RV8: a test separates the readings, and every compiler rejects it; a CTAD-16 Note records the review; pending.md row 4 gains the rule item CTAD-16 |
| RV8-B-7 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | text | the guides from implicit constructors give the types of the copy candidate, and no probe separates the readings; the doubt stays |
| RV8-B-8 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | commit | d6fd9cea, decision 0063, item RV8: CTAD-18 counts a first parameter that is a reference to the list and a guide whose parameters all have defaults, with two tests (Q-0438, Q-0443) |
| RV8-B-9 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | commit | d6fd9cea, decision 0063, item RV8: INST-5 says that a call that passes or returns the class by value also needs the types complete; the doubt of Q-0441 on the inline member stays |
| RV8-B-10 | `../review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md` | text | PRIM-2 agrees; the review reads GCC's warning as a diagnostic, recorded in Q-0440 |
| RM12-1 | `roadmap-12/editor.md` | commit | d6fd9cea, decision 0063, item RV8: ADD-11 says that a byte pointer into an object and a pointer into the storage that holds it have no difference, with an accept test whose interpreter expectation is run-fail |
| RM12-2 | `roadmap-12/editor.md` | commit | d6fd9cea, decision 0063, item RV8: elaborated_type takes attributes after the class key, TYN-10 rejects them outside a forward declaration, and TYN-6 allows them in one, with two tests; Q-0445 holds GCC's acceptance |
| RM12-3 | `roadmap-12/editor.md` | question | Q-0444 |
| RV9-R-1 | `../review/2026-11-11-read-of-rv8-repair-diff.md` | commit | bdc27d44, decision 0063, item RV8: Q-0444 records the read's support and waits for a principle (CTOR-P1); its provisional answer, the constructors divergence line, and the note of dtor-call-unqualified-base.cpp drop the withdrawn lean of RV8-R-1 and RV8-A-6; pending.md row 4 drops the item DTOR-8, Q-0444 |
| RV9-R-2 | `../review/2026-11-11-read-of-rv8-repair-diff.md` | commit | bdc27d44, decision 0063, item RV8: the INST Note and the note of class-decltype-consteval-call-not-instantiated.cpp say that a call in the operand of decltype is no immediate invocation, so INST-2 agrees; pending.md row 4 drops the item INST-1, INST-2 |
| RV9-R-3 | `../review/2026-11-11-read-of-rv8-repair-diff.md` | commit | bdc27d44, decision 0063, item RV8: Q-0442's option 2 makes every step, a step of zero included, and the difference of two such pointers undefined; the note of integer-counterpart-one-past.cpp names the difference; the expectation stays, and Q-0442 holds the case in pending.md row 4 |
| RV9-R-4 | `../review/2026-11-11-read-of-rv8-repair-diff.md` | question | Q-0445 |
| RV9-R-5 | `../review/2026-11-11-read-of-rv8-repair-diff.md` | text | OPR-8 does not name a literal operator template parameter whose class type is deduced from the string literal; every compiler accepts the form, the reference is silent, and the doubt stays (recorded in Q-0446) |
| W20-1 | `W20/report.md` | commit | 0ae4cdb5, decision 0061, item F3: STG-P2 and the FixedVec example say that a pointer to the object in one row does not reach the object in the next row (ADD-10); test foundations/storage/byte-rows-pointer-does-not-reach-next-row.cpp |
| W20-2 | `W20/report.md` | commit | 53577f27 (front-end round 24, decision 0061, item FE): the test passes; F3 states in UNI-14 that a construction inside a member that is not active does not make it active, and that pointers to the elements of such an array member can be formed, stepped, and compared (STG-10); UNI-6 makes the construction undefined (Q-0425); front end: a subscript of an array member with no active member is unsupported; tests classes/unions/array-member-element-address-no-active-member.cpp, array-member-element-steps-no-active-member-constant.cpp, construct-at-array-member-element-constant.cpp, chain-assign-array-element-nontrivial-constant.cpp |
| W20-3 | `W20/report.md` | commit | 53577f27 (front-end round 24, decision 0061, item FE): the test passes; F3 states in CNS-11 that the constraints of the template candidates are not checked when a candidate that is not a template takes every argument with no conversion, and in CPY-15 that a by-value constructor template is checked before it is dropped otherwise (Q-0426); front end: CNS-35 false reject; tests templates/constraints/by-value-constructor-template-constraint-copies-class.cpp, exact-non-template-skips-constraint-check.cpp, added-const-non-template-checks-constraint.cpp, classes/copy-move/copy-constructor-exact-skips-template-constraint.cpp, by-value-constructor-template-constraint-checked.cpp |
| W20-4 | `W20/report.md` | commit | 0ae4cdb5, decision 0061, item F3: a CMP-15 Evidence note on GCC's rejection of tag address comparisons in a static_assert under -fsanitize=undefined or -fsanitize=null |
| W20-5 | `W20/report.md` | commit | 0ae4cdb5, decision 0061, item F3: CAST-1 states the void* round trip, links STG-16, and names CE-4 for constant evaluation; tests expressions/casts/static-cast-void-pointer-round-trip-class.cpp, static-cast-void-pointer-round-trip-constant.cpp |
| W20-6 | `W20/report.md` | commit | 0ae4cdb5, decision 0061, item F3: a table beside the FixedVec example of five storage forms: inside the object, constant evaluation, one array of T, and a default constructor |
| W20-7 | `W20/report.md` | text | outside knowledge on assignment idioms and sorting; no language rule involved |
| I5-1 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-30 points to DED-29 for the body of a specialization whose result type the match needs (Q-0420) |
| I5-2 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-29's first list and the DED-P5 table name the body of another specialization that the function type needs |
| I5-3 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-7 says a cv-qualifier on a function type has no effect, and CV-6 covers a template parameter; tests templates/deduction/function-type-const-dropped.cpp, declarations/declarators/cv-on-function-template-parameter.cpp |
| I5-4 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-15 and DED-P4 say deduction takes the base relation alone, and OVL-37, CONV-17, and ACC-12 check the conversion after the choice; tests templates/deduction/private-base-with-fallback.cpp, protected-base-with-fallback.cpp |
| I5-5 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-24 names an incomplete class, and INST-1 names a qualified name that names a member; test templates/instantiation/class-qualified-name.cpp |
| I5-6 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-8 links VC-2 on the cv-qualifiers of a prvalue; tests templates/deduction/forwarding-const-scalar-prvalue.cpp, forwarding-const-class-prvalue.cpp |
| I5-7 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: an example for DED-20 with two explicit pack elements and a deduced parameter after the pack; test templates/deduction/pack-not-last-explicit-then-deduced.cpp |
| I5-8 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-14 and a DED-P4 table say that the pair fails for const T** and an int** argument; tests templates/deduction/difference-qualification-not-reached.cpp, difference-qualification-not-reached-no-fallback.cpp |
| I5-9 | `I5/report.md` | commit | 53577f27 (front-end round 24, decision 0061, item FE): the test passes; F3 answered I5-5 in DED-24; front end: T::type with an incomplete T stops as unsupported; test templates/deduction/qualified-name-in-incomplete-class.cpp |
| I5-10 | `I5/report.md` | commit | 53577f27 (front-end round 24, decision 0061, item FE): the test passes; F3 states the base relation in REF-1, COND-16, and DED-15 (G153); front end: a reference binds a private or ambiguous base; tests declarations/initialization/references/lvalue-ref-to-private-base.cpp, lvalue-ref-to-ambiguous-base.cpp, templates/deduction/two-equal-bases-with-fallback.cpp, private-base-with-fallback.cpp |
| I5-11 | `I5/report.md` | commit | 0ae4cdb5, decision 0061, item F3: DED-24 names arrays of functions and of void, and DED-11 names the size type as decltype(sizeof 0); test templates/deduction/substitution-failure-array-of-function-or-void.cpp |
| I6-1 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-5 says that an unqualified call with no dependent argument uses ordinary and argument-dependent lookup, both at the definition, and the DEP-P1 table gains a row for it; independent-call-early.cpp decides it |
| I6-2 | `I6/report.md` | text | decision 0062: the report names no change to the text; a mistake in the component (DEP-34 with no argument list); no text change |
| I6-3 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: a DEP-P5 row for typename T::template box with no list, a type whose arguments are deduced (DEP-26, Q-0226) |
| I6-4 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-P5 says that the parser's reading at the definition reports no error, and that the right column is checked at the instantiation |
| I6-5 | `I6/report.md` | question | Q-0434; decision 0062, item F4: DEP-2 makes a member access through this or an object of the current instantiation type-dependent only for a dependent member type or a dependent base; test this-member-independent-type-bound-early.cpp |
| I6-6 | `I6/report.md` | question | Q-0432; decision 0062, item F4: DEP-4's last bullet leaves out the operands of sizeof, alignof, and noexcept, and DEP-P2 says so; tests sizeof-value-dependent-operand-call-bound-early.cpp, sizeof-value-dependent-operand-no-typename.cpp |
| I6-7 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-6 and DEP-12 apply also when no specialization is made, DEP-30 covers only a name that lookup finds, and DEP-P6 gains two rejects rows; two tests move from DEP-30 to DEP-6 and DEP-12 |
| I6-8 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: the DEP-P6 row for CNS-13 says rejects, as CNS-13 does |
| I6-9 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: new rule DEP-40: a qualified call, and an unqualified call whose name finds a member, look up at the definition with no argument-dependent lookup; DEP-P1 rows; tests qualified-call-dependent-argument-bound-early.cpp, member-found-call-no-argument-lookup.cpp |
| I6-10 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-9 says a point of instantiation (INST-31) and that INST-32 applies where the points differ; an Evidence note gives the GCC split |
| I6-11 | `I6/report.md` | commit | 53577f27 (front-end round 24, decision 0061, item FE): the test passes; front end: this->template with a dependent base stops; test templates/dependent-names/template-keyword-this-dependent-base.cpp |
| I6-12 | `I6/report.md` | commit | 53577f27 (front-end round 24, decision 0061, item FE): the test passes; front end: operator lookup leaves out the namespaces of a base; test overloading/overload-resolution/operator-base-class-namespace.cpp |
| I6-13 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: INST-1 lists a call that passes the class by value and a call outside decltype, INST-2 and the INST-P1 table name a call inside decltype; tests class-call-passes-by-value.cpp, class-decltype-of-call-not-instantiated.cpp |
| I6-14 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: a reject example under DED-29 with a fallback pick(...) |
| I6-15 | `I6/report.md` | question | Q-0433; decision 0062, item F4: INST-33 no longer makes a static constexpr initializer with the class (Clang 22, GCC 16), INST-5 lists constexpr members and the static const exception, INST-P2 gains rows; five tests |
| I6-16 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: INST-5 and the INST-P2 table: a friend body waits for a call, and INST-7 makes it; tests friend-body-waits-for-use.cpp, friend-body-made-at-call.cpp |
| I6-17 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-37 links OPR-2 for the number of parameters of each operator function |
| I6-18 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: the example "The keywords" gives DEP-6 for the product with the undeclared q, and a line C::value_type count; with DEP-29 |
| I6-19 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-20 names the return type of a member function |
| I6-20 | `I6/report.md` | commit | 1130ceb8, decision 0062, item F4: DEP-2 says that this in a local class of a function template is not type-dependent (Q-0434; test this-in-local-class-not-type-dependent.cpp); DEP-9 links NAM-P1 and OVL-P1, and DEP-37 links OPR-2 |
| I7-1 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-10 says that a member or base whose type is not dependent follows LIST-4; an example with Plain and int v[2]; new test aggregate-elision-fixed-int-array.cpp, and aggregate-string-fixed-array.cpp cites CTAD-10 |
| I7-2 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-25 and CTAD-P1 say that step 5 chooses a constructor of the deduced type again, which can differ from the one that gave the guide; a reject example with the tie of result-then-initialize |
| I7-3 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-16 notes that the inherited copy candidate can give a type whose initialization fails (DER-8, CTAD-26); inherited-copy-candidate-user-guide.cpp cites CTAD-26 and its note drops the aggregate candidate |
| I7-4 | `I7/report.md` | text | a mistake in the component (CTAD-12, a template guide declared twice) |
| I7-5 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-5 says that only the constructors that the class declares give guides; an Evidence note says that no probe separates the readings |
| I7-6 | `I7/report.md` | question | Q-0438; decision 0063, item F5: CTAD-18 defines the initializer-list guide; tests initializer-list-guide-with-default.cpp and initializer-list-guide-empty-list.cpp |
| I7-7 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: a worked example of the CTAD-14 match with IntPair; an Evidence note says that no probe separates dropping a guide that cannot match from failing it under CTAD-28 |
| I7-8 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-14 says that the steps run once per alias, inner first, and that each alias's constraints apply; new test alias-of-alias-constraint.cpp |
| I7-9 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-16 lists the guides that a derived class takes and says that the guide with no parameters and the aggregate candidate stay with the base; new tests inherited-no-guide-without-parameters.cpp and inherited-no-aggregate-candidate.cpp |
| I7-10 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-P1 says that a user guide with no template head is a function and OVL-27 prefers it; CTAD-P3 says that CTAD-22 is modeled on OVL-27 |
| I7-11 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-17 says that the limit of OVL-14 and ICS-10 does not apply to the guides; new test copy-init-guide-user-conversion.cpp (Clang 18 override, Q-0020) |
| I7-12 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-18 says "unlike ILC-3 for constructors" and cites Q-0081 |
| I7-13 | `I7/report.md` | question | Q-0439; decision 0063, item F5: CTAD-1 lists a new expression with or without an initializer, CTAD-2 lists a static data member with no initializer; tests site-new-no-initializer.cpp and site-static-member-no-initializer.cpp |
| I7-14 | `I7/report.md` | question | Q-0437; decision 0063, item F5: CTAD-17 says that each guide's condition is evaluated for its own deduced arguments before the choice; test conditional-explicit-dropped-before-choice.cpp |
| I7-15 | `I7/report.md` | commit | 232ac618, decision 0063, item F5: CTAD-9 links ICS-18 for the sequence of a braced element to an aggregate member or base |
| I7-16 | `I7/report.md` | commit | b035efd4 (front-end round 25, decision 0062, item FE): the test passes; front end: an aggregate candidate with designators stops; test templates/class-template-deduction/aggregate-candidate.cpp |
| I7-17 | `I7/report.md` | commit | b035efd4 (front-end round 25, decision 0062, item FE): the test passes; front end: a braced copy of a class whose list constructor takes another type stops; test classes/initializer-lists/braced-copy-list-ctor-other-element-type.cpp |
