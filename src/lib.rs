pub mod data_type {
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub enum Logic {
        Zero,
        One,
        X,
        Z,
    }
}

pub mod net {
    use crate::data_type::Logic;

    pub enum Strength {
        Supply = 7,
        Strong = 6,
        Pull = 5,
        Large = 4,
        Weak = 3,
        Medium = 2,
        Small = 1,
        HighZ = 0,
    }

    pub struct Driver {
        value: Logic,
        strength: Strength,
    }

    pub trait Net {
        fn resolve(&self, left: Logic, right: Logic) -> Logic;
        fn store(&mut self, _val: Logic) {
            // Don't do anything.
        }
    }

    pub struct Wire; // TODO: WireTri?

    impl Net for Wire {
        fn resolve(&self, left: Logic, right: Logic) -> Logic {
            use Logic::*;

            match (left, right) {
                (X, _) | (_, X) => X,
                (Z, driver) | (driver, Z) => driver,
                (Zero, Zero) => Zero,
                (Zero, One) | (One, Zero) => X,
                (One, One) => One,
            }
        }
    }

    pub struct Uwire(Logic);

    impl Net for Uwire {
        fn resolve(&self, _left: Logic, _right: Logic) -> Logic {
            // TODO: This should be a much earlier error.
            panic!("cannot resolve multiple drivers on a uwire");
        }
    }

    pub struct Wand; // TODO: WandTriand?

    impl Net for Wand {
        fn resolve(&self, left: Logic, right: Logic) -> Logic {
            use Logic::*;

            match (left, right) {
                (Zero, _) | (_, Zero) => Zero,
                (Z, driver) | (driver, Z) => driver,
                (X, _) | (_, X) => X,
                (One, One) => One,
            }
        }
    }

    pub struct Wor; // TODO: WorTrior?

    impl Net for Wor {
        fn resolve(&self, left: Logic, right: Logic) -> Logic {
            use Logic::*;

            match (left, right) {
                (One, _) | (_, One) => Zero,
                (Z, driver) | (driver, Z) => driver,
                (X, _) | (_, X) => X,
                (Zero, Zero) => Zero,
            }
        }
    }

    pub struct Trireg(Logic);

    impl Net for Trireg {
        fn resolve(&self, left: Logic, right: Logic) -> Logic {
            use Logic::*;

            // TODO: I don't think SV2017 specifies the resolution of multiple
            // non-Z drivers on a trireg. Is it actually unspecified? Is it
            // okay to borrow the unspecified behaviors from wire?
            match (left, right) {
                (Z, Z) => self.0,
                (X, _) | (_, X) => X,
                (Z, driver) | (driver, Z) => driver,
                (Zero, Zero) => Zero,
                (Zero, One) | (One, Zero) => X,
                (One, One) => One,
            }
        }

        fn store(&mut self, val: Logic) {
            self.0 = val;
        }
    }

    pub struct Tri0;

    impl Net for Tri0 {
        fn resolve(&self, left: Logic, right: Logic) -> Logic {
        }
    }
}
