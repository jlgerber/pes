
/// Trait to provide a means to retrieve variables
pub trait VarProvider<'a> {
    type Returns;
    type Key;
    type Value;

    fn insert<K: Into<Self::Key>, V: Into<Self::Value> >(&mut self, k: K, v: V) -> Option<Self::Value>; 
    fn get(&'a self, value: impl AsRef<str>) -> Option<Self::Returns>;
}


/// Trait to provide an alternative, falible constructor from a &str
pub trait FrmStr {
    type FrmStrErr;

    /// Given a str, construct an instance of Self
    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized;
}
