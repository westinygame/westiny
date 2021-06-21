#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NodeState {
    Uninit,
    Success,
    Failure,
    Running,
}

impl Default for NodeState {
    fn default() -> Self {
        NodeState::Uninit
    }
}

pub trait Node<T> {
    fn state(&self) -> NodeState;
    fn execute(&mut self, a: &mut T) -> NodeState;
    fn reset(&mut self);
}

pub struct SequenceNode<T> {
    state: NodeState,
    children: Vec<Box<dyn Node<T>>>,
}

impl<T> SequenceNode<T> {
    pub fn new(children: Vec<Box<dyn Node<T>>>) -> Self {
        SequenceNode {
            state: NodeState::default(),
            children
        }
    }
}

impl<T> Node<T> for SequenceNode<T> {
    fn state(&self) -> NodeState {
        self.state
    }

    fn execute(&mut self, args: &mut T) -> NodeState {
        for idx in 0..self.children.len() {
            let child = &mut self.children[idx];
            if child.state() == NodeState::Running || child.state() == NodeState::Uninit {
                self.state = child.execute(args);

                // If node returned success, root is still running
                if self.state == NodeState::Success {
                    self.state = NodeState::Running;
                }
                return self.state

            } else if child.state() == NodeState::Failure {
                self.state = NodeState::Failure;
                return self.state
            }
        }

        self.state = NodeState::Success;
        self.state
    }

    fn reset(&mut self) {
        self.state = NodeState::Uninit;
        self.children.iter_mut()
            .for_each(|child| child.reset() );
    }
}

pub struct LeafNode<F> {
    state: NodeState,
    action: F,
}

impl<F> LeafNode<F> {
    pub fn new(action: F) -> Self {
        LeafNode {
            state: NodeState::default(),
            action
        }
    }
}

impl<F, T> Node<T>for LeafNode<F>
    where F: Fn(&mut T) -> NodeState + Sync
{
    fn state(&self) -> NodeState {
        self.state
    }

    fn execute(&mut self, args: &mut T) -> NodeState {
        self.state = (self.action)(args);
        self.state
    }

    fn reset(&mut self) {
        self.state = NodeState::Uninit;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn behavior_tree_sequence_1_leaf() {
        let clos = |arg : &mut u8| {
            *arg += 1;
            NodeState::Running
        };

        let nodes: Vec<Box<dyn Node<u8>>> = vec![
            Box::new(LeafNode::new(clos))
        ];
        let mut root = SequenceNode::new(nodes);
        let mut arg = 3;
        root.execute(&mut arg);
        assert_eq!(4, arg);
        assert_eq!(NodeState::Running, root.state);
    }

    #[test]
    pub fn behavior_tree_sequence_2_leaf_success() {
        let nodes: Vec<Box<dyn Node<u8>>> = vec![
            Box::new(LeafNode::new(|arg: &mut u8| {
                *arg += 1;
                NodeState::Success
            })),
            Box::new(LeafNode::new(|arg: &mut u8| {
                *arg += 20;
                if *arg < 40 { NodeState::Running } else { NodeState::Success }
            })),
        ];

        let mut root = SequenceNode::new(nodes);
        let mut arg = 0u8;
        root.execute(&mut arg);
        assert_eq!(1, arg, "Execute first");
        assert_eq!(NodeState::Running, root.state);

        root.execute(&mut arg);
        assert_eq!(21, arg, "Execute second");
        assert_eq!(NodeState::Running, root.state);

        root.execute(&mut arg);
        assert_eq!(41, arg, "Execute second again");
        assert_eq!(NodeState::Running, root.state);

        root.execute(&mut arg);
        assert_eq!(41, arg, "Execute none");
        assert_eq!(NodeState::Success, root.state);
    }

    #[test]
    pub fn behavior_tree_2_leaf_failure() {
        let nodes : Vec<Box<dyn Node<u8>>> = vec![
            Box::new(LeafNode::new(|arg: &mut u8| {
                *arg += 1;
                NodeState::Failure
            })),
            Box::new(LeafNode::new(|arg: &mut u8| {
                *arg += 10;
                NodeState::Success
            }))
        ];

        let mut root = SequenceNode::new(nodes);
        let mut arg = 0u8;
        root.execute(&mut arg);
        assert_eq!(1, arg);
        assert_eq!(NodeState::Failure, root.state());
        assert_eq!(NodeState::Uninit, root.children[1].state());

        root.execute(&mut arg);
        assert_eq!(1, arg);
        assert_eq!(NodeState::Failure, root.state());
        assert_eq!(NodeState::Uninit, root.children[1].state())
    }

    #[test]
    pub fn sequence_node_reset()
    {
        let mut dummy = 0u8;

        // Create leaf nodes & exec them to modify their state
        let mut action1 = LeafNode::new(|arg: &mut u8| { NodeState::Failure });
        action1.execute(&mut dummy);
        assert_eq!(NodeState::Failure, action1.state);

        let mut action2 = LeafNode::new(|arg: &mut u8| { NodeState::Failure });
        action2.execute(&mut dummy);
        assert_eq!(NodeState::Failure, action2.state);

        let nodes: Vec<Box<dyn Node<u8>>> = vec![ Box::new(action1), Box::new(action2) ];

        let mut seq = SequenceNode::new(nodes);

        seq.execute(&mut dummy);
        assert_eq!(NodeState::Failure, seq.state());

        seq.reset();
        assert_eq!(NodeState::Uninit, seq.state());
        assert_eq!(NodeState::Uninit, seq.children[0].state());
        assert_eq!(NodeState::Uninit, seq.children[1].state());
    }
}