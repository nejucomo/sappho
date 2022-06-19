#[cfg(test)]
#[rustfmt::skip]
mod gentests;

#[cfg(test)]
mod logic;

#[cfg(test)]
use self::logic::test_eval;

#[cfg(test)]
use self::logic::test_unparse;
