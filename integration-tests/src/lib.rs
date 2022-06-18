#[cfg(test)]
#[rustfmt::skip]
mod gentests;

#[cfg(test)]
mod testlogic;

#[cfg(test)]
use self::testlogic::test_eval;
