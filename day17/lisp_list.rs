enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct ListIterator<'a, T> {
    curr_list: &'a List<T>
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_list {
            List::Cons(car, cdr) =>{
                self.curr_list = cdr;
                Some(&car)
            },
            List::Nil => None,
        }
    }
}

impl<T> List<T> {
    // Create an empty list
    fn new() -> List<T> {
        // `Nil` has type `List`
        List::Nil
    }

    fn iter(&self) -> ListIterator<T> {
        ListIterator { curr_list: self }
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: T) -> List<T> {
        // `Cons` also has type List
        List::Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> usize {
        match self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            List::Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String where T: Display {
        match self {
            List::Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}