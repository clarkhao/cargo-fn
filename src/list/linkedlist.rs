#[derive(PartialEq, Clone, Debug, Default)]
pub struct LinkedList<T>
where
    T: PartialEq,
{
    pub value: T,
    pub next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T>
where
    T: PartialEq + Default,
{
    pub fn new_node(value: T) -> LinkedList<T> {
        LinkedList { value, next: None }
    }
    pub fn add_node(&mut self, value: T) -> &mut LinkedList<T> {
        let new_node = LinkedList::new_node(value);
        self.next = Some(Box::new(new_node));
        self.next.as_deref_mut().unwrap()
    }
}

impl<T> Iterator for LinkedList<T>
where
    T: Clone + PartialEq + Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = std::mem::take(self);
        if let Some(next) = current.next {
            *self = *next;
        }
        if current.value == Default::default() {
            None
        } else {
            Some(current.value)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new() {
        let text_node = LinkedList::new_node(String::from("Hello"));
        assert_eq!(text_node.value, "Hello");
        assert_eq!(text_node.next, None);
    }

    #[test]
    fn test_add_node() {
        let mut head = LinkedList::new_node("Hello");
        head.add_node("World").add_node("I").add_node("Am");
        assert_eq!(head.value, "Hello");
        let next = head.next;
        match next {
            Some(node) => {
                assert_eq!(node.value, "World");
                let next = node.next;
                match next {
                    Some(node) => {
                        assert_eq!(node.value, "I")
                    }
                    None => (),
                }
            }
            None => (),
        }
    }

    #[test]
    fn test_iterator() {
        let mut head = LinkedList::new_node("Hello");
        head.add_node("World").add_node("I").add_node("Am");
        let mut iterator = head.into_iter();
        assert!(iterator.next() == Some("Hello"));
        assert!(iterator.next() == Some("World"));
        assert!(iterator.next() == Some("I"));
        assert!(iterator.next() == Some("Am"));
    }
}
