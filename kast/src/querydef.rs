// use chumsky::Parser as _;
use derive_new::new;
use sappho_effect::QueryEffect;
// use sappho_keyword::Keyword::Query as KwQuery;
// use sappho_parsable::{ParsableWith, Parser};
// use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, KastProvider};

#[derive(Clone, Debug, PartialEq, new)]
pub struct QueryDef<K>(#[new(into)] BoxWise<K, QueryEffect>)
where
    K: KastProvider;

// impl<K> ParsableWith<ProcWiseParser<'_, K>> for QueryDef<K>
// where
//     K: KastProvider,
// {
//     fn make_parser_with(proc: ProcWiseParser<'_, K>) -> impl Parser<Self> {
//         KwQuery
//             .parse()
//             .then_space()
//             .ignore_then(BoxWise::parser_with(proc))
//             .map(Self)
//     }
// }

// impl<K> Unparse for QueryDef<K>
// where
//     K: KastProvider,
// {
//     fn unparse_into(&self, s: &mut Stream) {
//         s.write(&KwQuery);
//         s.write(" ");
//         s.write(&self.0);
//     }
// }
