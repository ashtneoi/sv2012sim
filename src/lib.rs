use std::collections::VecDeque;
use std::default::Default;
use std::ops::Index;
use std::slice::SliceIndex;

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

    fn execute_region(&mut self, region: Region) {
        // TODO: Initial capacity?
        let mut buf = VecDeque::new();
        buf.append(&mut self.regions[region as usize]);
        for ev in buf {
            ev.execute(self);
        }
    }

    fn region_set_is_empty<R>(&self, range: R) -> bool
    where
        R: SliceIndex<[VecDeque<Event>], Output=[VecDeque<Event>]>,
        // ^ That shouldn't be necessary, but it is. :(
    {
        self.regions.index(range).iter().all(|x| x.is_empty())
    }

    fn execute(&mut self) {
        use Region::*;

        let mut buf = VecDeque::new();

        self.execute_region(Preponed);

        self.execute_region(PreActive);

        while !self.region_set_is_empty(
            Active as usize..=PrePostponed as usize
        ) {
            while !self.region_set_is_empty(
                Active as usize..=PostObserved as usize
            ) {
                self.execute_region(Active);

                let first_nonempty = self
                    .regions[Active as usize..=PostObserved as usize]
                    .iter()
                    .position(|x| !x.is_empty());
                if let Some(first_nonempty) = first_nonempty {
                    buf.append(&mut self.regions[first_nonempty]);
                    self.regions[Active as usize].append(&mut buf)
                }
            }

            while !self.region_set_is_empty(
                Reactive as usize..=PostReNba as usize
            ) {
                self.execute_region(Reactive);

                let first_nonempty = self
                    .regions[Reactive as usize..=PostReNba as usize]
                    .iter()
                    .position(|x| !x.is_empty());
                if let Some(first_nonempty) = first_nonempty {
                    buf.append(&mut self.regions[first_nonempty]);
                    self.regions[Reactive as usize].append(&mut buf)
                }
            }

            if self.region_set_is_empty(
                Active as usize..PostReNba as usize
            ) {
                self.execute_region(PrePostponed);
            }
        }

        self.execute_region(Postponed);
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
