use std::collections::HashSet;
use std::hash::Hash;

trait AddIfNotExists<T> {
    fn add_if_not_exists(&mut self, num: T) -> bool;
}

impl<T: Eq + Hash> AddIfNotExists<T> for HashSet<T> {
    // Eq + Hash are bounds; aka constraints T must satisfy
    // Eq = can check equality (==)
    // Hash = can be hashed
    // so T: Eq + Hash = supports Eq and Hash
    fn add_if_not_exists(&mut self, val: T) -> bool {
        if self.contains(&val) {
            return true;
        }
        self.insert(val);
        false
    }
}

// partial eq is because nan != nan
// what goes between <> after impl?
// impl<T> = for all possible types T implementing the following
//

// fn my_func<T>
// impl<T> trait<T>??
// impl<T> = let T = (some T)
// impl<SomeTypeA> SomeTrait<SomeTypeA> for SomeTypeB
// impl<SomeTypeA> = "let SomeTypeA = A"
// SomeTrait<SomeTypeA> = "Define SomeTrait with access to SomeTypeA"
// SomeTrait for SomeTypeB = "SomeTypeB is the type that this trait is defined for"
// fn my_func<T> = "Define my_func with access to T"
