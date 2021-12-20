use std::fs;

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<T>,
}

impl Node<usize> {
/*
    pub fn new(val: Option<usize>, l: Node<usize>, r: Node<usize>) -> Self {
        Node::<usize> {
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
            value: val,
        }
    }
*/
    pub fn new_blank() -> Self {
        Node::<usize> {
            left: None,
            right: None,
            value: None,
        }
    }

    pub fn add_expression(&mut self, char_iter: &mut std::iter::Peekable<std::str::Chars>) {
        if let Some(c) = char_iter.next() {

//            println!("Next char is '{}'", c);

            match c {
                '[' => {
//                    println!("First part starts with '{}'", c);
                    self.left = Some(Box::new(Node::new_blank()));
                    self.left.as_mut().unwrap().add_expression(char_iter);

                    if let Some(d) = char_iter.next() {
//                        println!("Char after first part of pair is '{}'", d);
                        if d != ',' {
                            panic!("Invalid expression - expected comma");
                        }
                    } else {
                        panic!("End of data with unbalanced pair");
                    }

//                    println!("Second part starts with '{}'", char_iter.peek().unwrap());
                    self.right = Some(Box::new(Node::new_blank()));
                    self.right.as_mut().unwrap().add_expression(char_iter);

                    if let Some(d) = char_iter.next() {
//                        println!("Char at end of pair is '{}'", d);
                        if d != ']' {
                            panic!("Pair doesn't end with ']'");
                        }
                    } else {
                        panic!("End of data without closing ']'");
                    }
                },

                ']' => {
                    println!("']' in main match - not sure I should get here");
                },

                ',' => {
                    println!("Comma in main case - not sure how we got here");
                    self.right = Some(Box::new(Node::new_blank()));
                    self.right.as_mut().unwrap().add_expression(char_iter);
                },

                '0'..='9' => {
                    let mut v = c.to_digit(10).unwrap();
                    loop {
                        if let Some(d) = char_iter.peek() {
                            if *d < '0' || *d > '9' { break; }
                        } else { break; }

                        let x = char_iter.next().unwrap();
                        v *= 10;
                        v += x.to_digit(10).unwrap();
                    }

                    self.value = Some(v as usize);
//                    println!("Storing value {}", v);
                },

                _ => panic!("Invalid character: '{}'", c),
            }
        }
    }

    pub fn print(&self) {
        /*
        if let Some(_) = &self.left {
            print!("L - link ");
        } else {
            print!("L - None ");
        }
        if let Some(_) = &self.right {
            print!("R - link ");
        } else {
            print!("R - None ");
        }
        if let Some(v) = &self.value {
            println!("V - {}", v);
        } else {
            println!("V - None");
        }
*/
        if let Some(l) = &self.left {
            print!("[");
            l.print();
            print!(",");
            if let Some(r) = &self.right {
                r.print();
                print!("]");
            } else {
                panic!("Left value, but no right value");
            }
        } else {
            print!("{}", self.value.as_ref().unwrap());
        }
    }

    fn send_right(&mut self, val: usize) {
        if let Some(_) = self.value {
            *self.value.as_mut().unwrap() += val;
            return;
        }

        self.left.as_mut().unwrap().send_right(val);
    }

    fn send_left(&mut self, val: usize) {
        if let Some(_) = self.value {
            *self.value.as_mut().unwrap() += val;
            return;
        }

        self.right.as_mut().unwrap().send_left(val);
    }

    // Start the explode process from a particular node - usually called from an Expression object
    // First traverse down the tree to find a node that meets the explode criteria
    // Then propagate the results back across the tree
    // Return values :-
    //      None - nothing has exploded
    //      Some(None, None) - Something did explode and values have been allocated (so nothing more to do)
    //      Some(Some, None), Some(None, Some), Some(Some, Some) - still have an explode value to allocate
    pub fn explode(&mut self, level: usize) -> Option<(Option<usize>, Option<usize>)> {
        // We shouldn't get down to an end node, but let's just check
        if let None = self.left { panic!("reached an end node in explode"); }

//        let mut kill_node = false;
        let mut return_value = None;

        {
            // Get the left and right pointers of the left and right nodes
            let left_node = self.left.as_mut().unwrap().value.clone();
            let right_node = self.right.as_ref().unwrap().value.clone();

            match (left_node, right_node) {
                (Some(l_val), Some(r_val)) => { // we have a numeric pair - [x,y], and we haven't already exploded
                    if level > 4 {  // Need to explode
                        return_value = Some((Some(l_val), Some(r_val)));
//                        kill_node = true;
                        self.left = None;
                        self.right = None;
                        self.value = Some(0);
                        //                        return (*left_val, *right_val);
                    }
                },

                (Some(_value), None) => { //  Have a value to the left and an expression to the right
                    let explode_result = self.right.as_mut().unwrap().explode(level + 1);
                    if let Some(explode_vals) = explode_result {
                        if let (Some(l_val), _) = explode_vals {
                            *self.left.as_mut().unwrap().value.as_mut().unwrap() += l_val;
                            return_value = Some((None, explode_vals.1));
                        } else {
                            return_value = explode_result;
                        }
                    } // return_value stays as default - None - nothing has exploded
                },

                (None, Some(_value)) => { // Have an expression to the left and a value to the right
                    let explode_result = self.left.as_mut().unwrap().explode(level + 1);
                    if let Some(explode_vals) = explode_result {
                        if let (_, Some(r_val)) = explode_vals {
                            *self.right.as_mut().unwrap().value.as_mut().unwrap() += r_val;
                            return_value = Some((explode_vals.0, None));
                        } else {
                            return_value = explode_result;
                        }
                    } // return_value stays as default - None - nothing has exploded
                },

                (None, None) => {  // Expressions to left and right
                    // Try and explode left first
                    let left_explode_result = self.left.as_mut().unwrap().explode(level + 1);
                    if let Some(explode_vals) = left_explode_result {
                        if let (_, Some(r_val)) = explode_vals {
                            self.right.as_mut().unwrap().send_right(r_val);
                            return Some((explode_vals.0, None));
                        } else {
                            return left_explode_result;
                        }
                    }

                    // Nothing exploded on left expression, so try right
                    let right_explode_result = self.right.as_mut().unwrap().explode(level + 1);
                    if let Some(explode_vals) = right_explode_result {
                        if let(Some(l_val), _) = explode_vals {
                            self.left.as_mut().unwrap().send_left(l_val);
                            return Some((None, explode_vals.1));
                        } else {
                            return right_explode_result;
                        }
                    }

                    // If we're here neither left nor right expression exploded, so return default - None
                }
            }
        }
/*
        if kill_node {
            self.left = None;
            self.right = None;
            self.value = Some(0);
        }
*/
        return_value
    }
}

struct Expression<T> {
    head: Node<T>
}

impl Expression<usize> {
    pub fn new() -> Self {
        Expression::<usize> { head: Node::new_blank() }
    }

    pub fn build_expression(&mut self, data: &str) {
        self.head.add_expression(&mut data.chars().peekable());
    }

    pub fn print(&self) {
        self.head.print();
        println!();
    }

    pub fn explode(&mut self) -> bool {
        if let Some(_) = self.head.explode(1) {
            true
        } else {
            false
        }
    }

//    pub fn get_head_ref(&self) -> Option<Box<Node<usize>> {
//        if let Some(h) = self.head {
//            Some(Box::new())
//        }
//    }

}


fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't read file");

    let mut expression = Expression::new();
    expression.build_expression(&data_string);

    expression.print();

    while expression.explode() {
        expression.print();
    }

}
