use std::collections::VecDeque;
use std::default::Default;

#[derive(Clone)]
struct Timeline {
    time_slots: VecDeque<TimeSlot>,
}

enum Region {
    Preponed,
    PreActive,
    Active,
    Inactive,
    PreNba,
    Nba,
    PostNba,
    PreObserved,
    Observed,
    PostObserved,
    Reactive,
    ReInactive,
    PreReNba,
    ReNba,
    PostReNba,
    PrePostponed,
    Postponed,
}

#[derive(Clone)]
struct TimeSlot {
    time: (), // TODO

    regions: Vec<VecDeque<Event>>,
}

impl TimeSlot {
    fn new() -> Self {
        Self {
            time: (),
            regions: vec![Default::default(); Region::Postponed as usize + 1],
        }
    }

    fn execute_region<I: IntoIterator<Item=Event>>(&mut self, region: I) {
        for ev in region {
            ev.execute(self)
        }
    }

    fn execute(&mut self) {
        use Region::*;

        let mut buf = VecDeque::new();

        buf.append(&mut self.regions[Preponed as usize]);
        self.execute_region(buf.drain(..));

        buf.append(&mut self.regions[PreActive as usize]);
        self.execute_region(buf.drain(..));

        while self.regions[Active as usize..=PrePostponed as usize].iter().any(
            |x| !x.is_empty()
        ) {
            while let Some(first_nonempty) = self
                    .regions[Active as usize..=PostObserved as usize]
                    .iter()
                    .position(|x| !x.is_empty()) {
                buf.append(&mut self.regions[Active as usize]);
                self.execute_region(buf.drain(..));

                if first_nonempty != Active as usize {
                    buf.append(&mut self.regions[first_nonempty]);
                    self.regions[Active as usize].append(&mut buf)
                }
            }

            while let Some(first_nonempty) = self
                    .regions[Reactive as usize..=PostReNba as usize]
                    .iter()
                    .position(|x| !x.is_empty()) {
            }
        }
    }
}

#[derive(Clone)]
enum Event {
    Update,
    Evaluation,
}

impl Event {
    fn execute(self, current_time_slot: &mut TimeSlot) {
    }
}

struct Process {
    sensitive_to: (), // TODO
}
