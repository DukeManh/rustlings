// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

trait RemoveBar {
    fn remove_all_bars(self) -> Self;
}


impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

impl RemoveBar for Vec<String> {
    fn remove_all_bars(mut self) -> Self {
        let mut i = 0;
        while i < self.len() {
            if self[i] == "Bar" {
                self.remove(i);
            } else {
                i += 1;
            }
        }

        self
    }
}

//TODO: Add your code here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo: Vec<String> = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));

        foo = vec!["Bar".into(), "Foo".into(), "Bar".into(), "Bar".into(), "Bar".into(), "Foo".into(), "Foo".into()];
        foo = foo.remove_all_bars();
        assert_eq!(foo, ["Foo", "Foo", "Foo"]);
    }
}
