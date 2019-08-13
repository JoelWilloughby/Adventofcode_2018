#![deny(unsafe_code)]

#[derive(Debug)]
pub struct LinkedList {
    contents : Vec<Node>,
    generation : usize,
    next_null : Option<usize>,
    head : Option<usize>,
    num_occupied : usize,
}

pub struct Game {
    list : LinkedList,
    current : Index,
    round : usize,
}

#[derive(Debug)]
pub enum Node {
    Null{
        next_null : Option<usize>
    },
    Used(UsedNode),
}

#[derive(Debug)]
pub struct UsedNode {
    data : usize,
    generation : usize,
    // There is always a prev and next. It may be self
    prev : usize,
    next : usize,
}

#[derive(Debug)]
pub struct Index {
    index : usize,
    generation : usize,
}

impl Index {
    fn new(index : usize, generation: usize) -> Self {
        Self {index, generation}
    }
}

impl Game {
    fn new() -> Self {
        let mut list = LinkedList::new();
        let first = list.push_front(0);
        Game {
            list,
            current: first,
            round: 1,
        }
    }

    // Takes a turn and returns a score if any
    fn take_turn(&mut self) -> usize {
        let mut score = 0;
        if self.round % 23 == 0 {
            score += self.round;
            score += self.list.remove_after_index(&self.current, -7).unwrap();
            self.current = self.list.find_actual_index(&self.current, -6).unwrap();
        } else {
            // Insert the marble between the 1 and 2 guys after
            self.current = self.list.insert_after_index(&self.current, self.round, 1).unwrap();
        }

        self.round += 1;

        score
    }
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            contents : vec![],
            generation : 0,
            next_null : None,
            head : None,
            num_occupied : 0,
        }
    }

    fn len(&self) -> usize {
        self.num_occupied
    }

    fn find_index(&self, start: usize, offset: isize) -> usize {
        let mut index = start;
        let dir = offset > 0;
        let jumps = offset.abs();

        // Find the insertion point
        for _ in 0..jumps {
            match &self.contents[index] {
                Node::Used(node) => {
                     index = if dir {
                         node.next
                     } else {
                         node.prev
                     };
                },
                Node::Null { .. } => panic!("Bad List!"),
            }
        };

        index
    }

    fn find_actual_index(&self, index: &Index, offset: isize) -> Option<Index> {
        let index = match self.contents.get(index.index)? {
            Node::Used(node) if node.generation == index.generation => index.index,
            _ => return None,
        };

        let index = self.find_index(index, offset);
        match &self.contents[index] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => Some(Index::new(index, node.generation)),
        }
    }

    /// Inserts a node offset nodes after a given index. Returns the new index of the inserted
    /// node if successful. Returns None otherwise
    fn insert_after_index(&mut self, index: &Index, elem: usize, offset: isize) -> Option<Index> {
        let index = match self.contents.get(index.index)? {
            Node::Used(node) if node.generation == index.generation => index.index,
            _ => return None,
        };

        let insert_index = self.find_index(index, offset + 1);
        let (insert_prev, insert_next) = match &self.contents[insert_index] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => (node.prev, node.next),
        };

        let next_position = self.next_position();
        self.contents[next_position] = Node::Used(UsedNode{
            data: elem,
            generation: self.generation,
            next: insert_index,
            prev: insert_prev,
        });

        // Fix up the insert's previous
        match &mut self.contents[insert_index] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => node.prev = next_position,
        }

        // Fix up the old insert_prev's next
        match &mut self.contents[insert_prev] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => node.next = next_position,
        }

        self.num_occupied += 1;
        Some(Index::new(
            next_position,
            self.generation,
        ))
    }

    ///
    fn remove_after_index(&mut self, index: &Index, offset: isize) -> Option<usize> {
        let index = match self.contents.get(index.index)? {
            Node::Used(node) if node.generation == index.generation => index.index,
            _ => return None,
        };

        // Grab the index to remove
        let remove_index = self.find_index(index, offset);
        let (remove_prev, remove_next, remove_data) = match &self.contents[remove_index] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => (node.prev, node.next, node.data),
        };

        match &mut self.contents[remove_prev] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => node.next = remove_next,
        }

        match &mut self.contents[remove_next] {
            Node::Null { .. } => panic!("Bad List!"),
            Node::Used(node) => node.prev = remove_prev,
        }

        // Fix up or next null list
        self.contents[remove_index] = Node::Null {next_null: self.next_null};
        self.next_null = Some(remove_index);

        if remove_index == self.head.unwrap() {
            // We need to move the head to something else.
            if self.num_occupied == 1 {
                // We are the last node, sad
                self.head = None;
            } else {
                self.head = Some(remove_next);
            }
        };


        self.num_occupied -= 1;
        self.generation += 1;
        // Return the removed data for the world to use
        Some(remove_data)
    }

    /// Helper function to find the next position we can isert stuff into
    fn next_position(&mut self) -> usize {
        if let Some(position) = self.next_null {
            match self.contents[position] {
                Node::Null { next_null } => self.next_null = next_null,
                Node::Used(_) => panic!("Bad List!"),
            }

            position
        } else {
            let position = self.contents.len();
            self.contents.push(Node::Null{next_null: None});
            position
        }
    }

    /// Standard getter
    fn get(&self, index: &Index) -> Option<&usize> {
        match self.contents.get(index.index)? {
            Node::Used(node) if node.generation == index.generation => Some(&node.data),
            _ => None,
        }
    }

    /// Inserts an element at the front of the list. Returns the new index of the inserted element
    fn push_front(&mut self, val : usize) -> Index {
        let next_position = self.next_position();

        if let Some(head_index) = self.head {
            // Head is not none, cool beans, just push the thing in front of head
            let head_prev = match &self.contents[head_index] {
                Node::Null { .. } => panic!("Bad List!"),
                Node::Used(node) => node.prev,
            };

            self.contents[next_position] = Node::Used(UsedNode {
                data: val,
                generation: self.generation,
                next: head_index,
                prev: head_prev,
            });

            // Fix up the head prev
            match &mut self.contents[head_index] {
                Node::Null { .. } => panic!("Bad List!"),
                Node::Used(node) => node.prev = next_position,
            }

            // Fix up the head prev's next
            match &mut self.contents[head_prev] {
                Node::Null { .. } => panic!("Bad List!"),
                Node::Used(node) => node.next = next_position,
            }

        } else {
            // Just insert the thing, this is easy
            self.contents[next_position] = Node::Used(UsedNode {
                data : val,
                generation : self.generation,
                next : next_position,
                prev : next_position,
            });
        }

        self.head = Some(next_position);
        self.num_occupied += 1;

        Index::new(next_position, self.generation)
    }
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "()");
        } else {
            write!(f, "(");
            let mut index = self.head.unwrap();
            match &self.contents[index] {
                Node::Used(node) => {
                    write!(f, "{}", node.data);
                    index = node.next;
                },
                _ => panic!("Bad List!"),
            }
            while index != self.head.unwrap() {
                write!(f, "-> ");
                match &self.contents[index] {
                    Node::Used(node) => {
                        write!(f, "{}", node.data);
                        index = node.next;
                    },
                    _ => panic!("Bad List!"),
                }
            }
            write!(f, ")");
        };

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;
    use crate::Game;

    #[test]
    fn new_list() {
        let list = LinkedList::new();
        assert_eq!(list.len(), 0)
    }

    #[test]
    fn insert_stuff() {
        let mut list = LinkedList::new();
        let a = list.push_front(1);
        let d = list.insert_after_index(&a, 4, 1);
        let c = list.push_front(3);
        println!("{:?}", list);
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(&a), Some(&1));
        assert_eq!(list.get(&c), Some(&3));
        assert!(d.is_some());
        assert_eq!(list.get(&(d.unwrap())), Some(&4));
    }

    #[test]
    fn remove() {
        let mut list = LinkedList::new();
        let a = list.push_front(1);
        let b = list.push_front(200);
        let c = list.push_front(3);
        let d = list.push_front(4);

        assert_eq!(list.remove_after_index(&b, 0), Some(200));
        assert_eq!(list.get(&b), None);
        println!("{:?}", list);
    }

    #[test]
    fn remove_and_insert() {
        let mut list = LinkedList::new();
        let a = list.push_front(1);
        let b = list.push_front(200);
        let c = list.push_front(3);
        let d = list.push_front(4);

        list.remove_after_index(&b, 0);
        let e = list.push_front(150);
        println!("{:?}", list);
        assert_eq!(e.index, b.index);
    }

    #[test]
    fn run_game() {
        let num_rounds = 7186400;

        let mut scores: [usize; 400] = [0; 400];

        let mut game = Game::new();

        let mut curr: usize = 0;

        while game.round <= num_rounds {
            scores[curr] += game.take_turn();

            //println!("Round {} {}", game.round, game.list.get(&game.current).unwrap());
            //println!("List: {}", game.list);
            curr += 1;
            curr = curr % scores.len();
        }


        let mut max = 0;
        let mut max_player = 0;
        for (i, player) in scores.iter().enumerate() {
            if *player > max {
                max = *player;
                max_player = i;
            }
            println!("Player {}, Score {}", i, player);
        }

        println!("Max is {}", max)
    }
}
