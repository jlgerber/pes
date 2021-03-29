use std::{
    cell::RefCell,
    rc::Rc,
};

use nom::{
    branch::alt,
    combinator::all_consuming
};

use crate::{
    env::{BasicVarProvider, PathMode},
    parser_atoms::ws,
    PNResult,
};

use super::*;


/// Given an Rc wrapped provider, return a parser which parses the paths from a string
///
/// # Example
///
/// ```
/// # use pes_core::parser::{BasicVarProvider, parse_all_paths_with_provider};
/// # use pes_core::traits::VarProvider;
/// # use pes_core::env::PathMode;
/// # use std::rc::Rc;
/// # use std::collections::VecDeque;
/// # use std::cell::RefCell;
/// #
/// # fn main()  {
/// let mut provider = BasicVarProvider::new();
/// provider.insert("root", "foobar");
/// provider.insert("name", "fred");
/// let provider = std::rc::Rc::new(RefCell::new(provider));
/// let result = parse_all_paths_with_provider(Rc::clone(&provider))("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1, PathMode::Exact(VecDeque::from(vec![
///     "/packages/foobar/stuff/fred".to_string(),
///     "/foo/bar/bla".to_string()
/// ])));
/// # }
// todo: make these generic over VarProvider
pub fn parse_all_paths_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) 
    -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> 
{
    //let provider = provider.clone();
    move |s: &'a str| {
        alt((
            parse_append_paths_with_provider(Rc::clone(&provider)), 
            parse_prepend_paths_with_provider(Rc::clone(&provider)),
            parse_exact_paths_with_provider(Rc::clone(&provider))
        ))(s)
    }
}


/// Given an Rc wrapped BasicVarProvider and a path str, parse the path str, returning a PathMode or error
///
/// # Example
/// ```
/// # use pes_core::parser::{BasicVarProvider, parse_consuming_all_paths_with_provider};
/// # use pes_core::traits::VarProvider;
/// # use pes_core::env::PathMode;
/// # use std::cell::RefCell;
/// # use std::rc::Rc;
/// # use std::collections::VecDeque;
/// # fn main()  {
/// let mut provider = BasicVarProvider::new();
/// provider.insert("root", "foobar");
/// provider.insert("name", "fred");
///
/// let provider = Rc::new(RefCell::new(provider));
///
/// let result = parse_consuming_all_paths_with_provider(
///                     Rc::clone(&provider), 
///                     " /packages/{root}/stuff/{name}:/foo/bar/bla "
///              ).unwrap();
///
/// assert_eq!(result, PathMode::Exact(VecDeque::from(vec![
///     "/packages/foobar/stuff/fred".to_string(),
///     "/foo/bar/bla".to_string()
/// ])));
/// # }
pub fn parse_consuming_all_paths_with_provider(provider: Rc<RefCell<BasicVarProvider>>, s: &str) 
    //-> PNResult<&'a str, PathMode> 
    -> PNCompleteResult<&str, PathMode>
{
    let (_, result) = all_consuming(
        ws( // drop surrounding whitespace
            parse_all_paths_with_provider(provider)
        )
    )(s)?;
    Ok(result)

}
