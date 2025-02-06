use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Clone, Copy)]
enum RadixNodeType {
    Exact,
    PathArgument,
    WildCard,
}

impl Ord for RadixNodeType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        type Type = RadixNodeType;
        let self_priority = match self {
            Type::Exact => 3,
            Type::PathArgument => 2,
            Type::WildCard => 1,
        };
        let other_priority = match other {
            Type::Exact => 3,
            Type::PathArgument => 2,
            Type::WildCard => 1,
        };
        self_priority.cmp(&other_priority)
    }
}

impl PartialOrd for RadixNodeType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct RadixTree(RadixNode);

impl Default for RadixTree {
    fn default() -> Self {
        Self(RadixNode {
            constant: String::new(),
            node_type: RadixNodeType::Exact,
            endpoint_id: None,
            children: Vec::new(),
        })
    }
}

impl RadixTree {
    pub fn new() -> Self {
        Self(RadixNode {
            constant: String::new(),
            node_type: RadixNodeType::Exact,
            endpoint_id: None,
            children: Vec::new(),
        })
    }

    pub fn insert(&mut self, path: &str, endpoint_id: u64) {
        let mut path_segments = path.split('/').filter(|s| !s.is_empty()).collect();
        self.0.insert(&mut path_segments, endpoint_id);
    }

    pub fn find(&self, path: &str) -> Option<(u64, HashMap<String, String>)> {
        if let Some(match_result) = self.0.find(path) {
            Some((match_result.endpoint_id, match_result.parameters))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct RadixNode {
    constant: String,
    node_type: RadixNodeType,
    endpoint_id: Option<u64>,
    children: Vec<RadixNode>,
}

struct RadixNodeBuilder {
    constant: String,
    node_type: RadixNodeType,
    endpoint_id: Option<u64>,
    children: Vec<RadixNode>,
}

impl RadixNodeBuilder {
    pub fn new() -> Self {
        Self {
            constant: String::new(),
            node_type: RadixNodeType::Exact,
            endpoint_id: None,
            children: Vec::new(),
        }
    }

    pub fn constant(mut self, constant: String) -> Self {
        self.constant = constant;
        self
    }

    pub fn node_type(mut self, node_type: RadixNodeType) -> Self {
        self.node_type = node_type;
        self
    }

    pub fn endpoint_id(mut self, endpoint_id: Option<u64>) -> Self {
        self.endpoint_id = endpoint_id;
        self
    }

    pub fn child(mut self, child: RadixNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<RadixNode>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> RadixNode {
        RadixNode {
            constant: self.constant,
            node_type: self.node_type,
            endpoint_id: self.endpoint_id,
            children: self.children,
        }
    }
}

impl RadixNode {
    fn common_prefix_length(parent: &str, word: &str) -> usize {
        word.chars()
            .zip(parent.chars())
            .take_while(|(p, w)| p == w)
            .count()
    }

    fn insert(&mut self, segments: &mut Vec<&str>, endpoint_id: u64) {
        if segments.is_empty() {
            self.endpoint_id = Some(endpoint_id);
            // Every time we are at the end of the path
            // We reorder the saved children by priority
            // Exact - 3, PathParam - 2, WildCards - 3(not yet implemented).
            // Reason is when we match incoming path we iterate children from Exact matches through
            // PathParam and to WildCards makes find more simple
            self.children.sort_by(|a, b| a.node_type.cmp(&b.node_type));
            return;
        }

        let segment = segments.remove(0);

        // need to handle case when " " is path it will be parsed as / root
        if segment == " " {
            self.insert(&mut vec![], endpoint_id);
            return;
        }

        for i in 0..self.children.len() {
            let common_prefix = Self::common_prefix_length(&self.children[i].constant, segment);
            if common_prefix > 0 {
                // this would happend only if the node is dynamic already,
                // e.g. starts with *abc or :abc
                // new would be *ability or :ability
                // i dont like the idea using common prefix on dynamic nodes
                if segment.starts_with("*") || segment.starts_with(":") {
                    // checking if the node does exist already
                    if let Some(existing_child_idx) = self
                        .children
                        .iter()
                        .position(|child| child.constant == segment)
                    {
                        self.children[existing_child_idx].insert(segments, endpoint_id);
                    } else {
                        // if let Some(path) = segment.strip_prefix('*') {
                        //     builder = builder
                        //         .constant(path.to_string())
                        //         .node_type(RadixNodeType::WildCard);
                        // } else if let Some(path) = segment.strip_prefix(':') {
                        // builder = builder
                        let radix_node = RadixNodeBuilder::new()
                            .constant(segment.to_string())
                            .node_type(RadixNodeType::PathArgument)
                            .build();
                        // }
                        self.children.push(radix_node);
                        self.children[i].insert(segments, endpoint_id);
                        return;
                    }
                }

                let child = &mut self.children[i];

                // the nodes are the same continue the inserting
                if child.constant.len() == common_prefix && segment.len() == child.constant.len() {
                    child.insert(segments, endpoint_id);
                    return;

                // new segment have common_prefix same size as existing node but is longer that
                // that node soo we need to split just a segment and keep the existing node
                // existing node -> use ... new segment useless so we just split useless
                } else if child.constant.len() == common_prefix {
                    let new_node_suffix = segment[common_prefix..].to_string();

                    // checking if the node does exist already
                    if let Some(existing_child_idx) = child
                        .children
                        .iter()
                        .position(|child| child.constant == new_node_suffix)
                    {
                        child.children[existing_child_idx].insert(segments, endpoint_id);
                    } else {
                        let mut new_child_node = RadixNodeBuilder::new()
                            .constant(new_node_suffix)
                            .node_type(RadixNodeType::Exact)
                            .build();
                        new_child_node.insert(segments, endpoint_id);
                        child.children.push(new_child_node);
                    }
                    return;
                } else if child.constant.len() > common_prefix {
                    /*
                     * Child -> prefix node & suffix node
                     * Segment -> suffix node
                     *
                     * lets call child node that we split GrandParent
                     * lets call splitted child's prefix Parent
                     * lets call splitted child's suffix OlderSibling
                     * lets call splitted segment's suffix YoungerSibling
                     * */

                    // split the current node at common prefix
                    let parent = child.constant[..common_prefix].to_string();
                    let older_sibling = child.constant[common_prefix..].to_string();
                    // create new child from segment after common prefix
                    let younger_sibling = segment[common_prefix..].to_string();

                    let parent_node = RadixNodeBuilder::new()
                        .constant(parent)
                        .node_type(child.node_type)
                        .build();

                    let mut older_sibling_node = RadixNodeBuilder::new()
                        .constant(older_sibling)
                        .node_type(child.node_type)
                        .endpoint_id(child.endpoint_id)
                        .build();

                    // we just swap these nodes and return is the previous unsplitted child node
                    let grand_parent = std::mem::replace(child, parent_node);
                    // we copy the old children of the node into new node
                    older_sibling_node.children = grand_parent.children;

                    if !younger_sibling.is_empty() {
                        let mut younger_sibling_node = RadixNodeBuilder::new()
                            .constant(younger_sibling)
                            .node_type(RadixNodeType::Exact)
                            .build();
                        younger_sibling_node.insert(segments, endpoint_id);
                        child.children.push(younger_sibling_node);
                    }
                    child.children.push(older_sibling_node);
                    child.insert(segments, endpoint_id);

                    return;
                }
            }
        }

        // completely new node
        let builder = RadixNodeBuilder::new();
        let mut new_node = if segment.starts_with('*') {
            builder
                .constant(segment.to_string())
                .node_type(RadixNodeType::WildCard)
                .build()
        } else if segment.starts_with(':') {
            builder
                .constant(segment.to_string())
                .node_type(RadixNodeType::PathArgument)
                .build()
        } else {
            builder
                .constant(segment.to_string())
                .node_type(RadixNodeType::Exact)
                .build()
        };

        new_node.insert(segments, endpoint_id);
        self.children.push(new_node);
    }

    fn find(&self, path: &str) -> Option<MatchResult> {
        /*
            when we have multiple registered paths like this:
                GET a/b/c
                GET a/b/:id
                GET a/b/:activity_id
                GET a/b/:some_string

                GET /order/{order_id}
                GET /order/{id}

                THIS SHOULD NOT WORK THIS TWO PATH WILL CAUSE CONFLIC WHEN ROUTING
                GET /order/{id}/activity/{activity_id}
                GET /order/{order_id}/activity/{activity_id}

            how do we distinguish that incoming request to path a/b/c
            should be handled by handler registered under a/b/c
            it could be easily :id, :activity_id or :some_string path parameter.

            We could first try the normal path node 'c' and then the other path params nodes

            There should be priority for Exact matches first
            path params second
            wildcards as last

        */

        let mut queue = VecDeque::new();

        queue.push_back(SearchState {
            curr_node: self,
            remaining_path: path.split('/').filter(|s| !s.is_empty()).collect(),
            parameters: HashMap::new(),
            priority: MatchPriority::Exact,
        });

        let mut curr_match: Option<MatchResult> = None;

        while let Some(SearchState {
            curr_node,
            remaining_path,
            parameters,
            priority,
        }) = queue.pop_front()
        {
            if remaining_path.is_empty()
                && curr_node.endpoint_id.is_some()
                && (curr_match.is_none()
                    || priority > curr_match.as_ref().expect("Search State").priority)
            {
                curr_match = Some(MatchResult {
                    endpoint_id: curr_node
                        .endpoint_id
                        .as_ref()
                        .expect("Search State")
                        .clone(),
                    parameters,
                    priority,
                })
            } else if !remaining_path.is_empty() {
                for child in &curr_node.children {
                    match child.node_type {
                        /*
                         *   My idea is the first when we insert this paths into tree
                         *   we first sort children by priority
                         *   Exact hightest to WildCard lowest
                         *   and here we just iterate the children from Exact to WildCard
                         * */
                        // first try match exact
                        RadixNodeType::Exact => {
                            let current_path_segment = remaining_path[0];
                            let mut remaining_path_segments = remaining_path[1..].to_vec();
                            // always start match from beginning
                            // it can't happend that children will be less or loss because they
                            // would get splitted on l during inserting of path
                            // prefix can be partial word or just a character
                            let common_prefix_length =
                                Self::common_prefix_length(&child.constant, current_path_segment);
                            if common_prefix_length > 0 {
                                let remainder = &current_path_segment[common_prefix_length..];
                                if !remainder.is_empty() {
                                    remaining_path_segments.insert(0, remainder);
                                }
                                queue.push_back(SearchState {
                                    curr_node: child,
                                    remaining_path: remaining_path_segments,
                                    parameters: parameters.clone(),
                                    priority,
                                });
                            }
                        }
                        // second try match parameter
                        RadixNodeType::PathArgument => {
                            let mut new_params = parameters.clone();
                            new_params.insert(child.constant.clone(), remaining_path[0].to_owned());
                            queue.push_back(SearchState {
                                curr_node: child,
                                remaining_path: remaining_path[1..].to_vec(),
                                parameters: new_params,
                                // idk if this is correct to do let me think about it
                                priority: std::cmp::min(priority, MatchPriority::Parameter),
                            });
                        }
                        // third try match wildCard
                        RadixNodeType::WildCard => {
                            let mut new_params = parameters.clone();
                            new_params
                                .insert(child.constant[1..].to_owned(), remaining_path.join("/"));
                            queue.push_back(SearchState {
                                curr_node: child,
                                remaining_path: vec![],
                                parameters: new_params,
                                priority: MatchPriority::WildCard,
                            });
                        }
                    }
                }
            }
        }
        curr_match
    }
}

struct SearchState<'a> {
    curr_node: &'a RadixNode,
    remaining_path: Vec<&'a str>,
    parameters: HashMap<String, String>,
    priority: MatchPriority,
}

struct MatchResult {
    endpoint_id: u64,
    parameters: HashMap<String, String>,
    priority: MatchPriority,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum MatchPriority {
    Exact = 3,
    Parameter = 2,
    WildCard = 1,
}

#[cfg(test)]
mod tests {
    use crate::from_request::PathParam;

    use super::*;

    #[test]
    fn test() {
        async fn handle_get(PathParam(order_id): PathParam<usize>) -> String {
            println!("Handling GET request");
            order_id.to_string()
        }
        let mut new_tree = RadixTree::new();
        new_tree.insert(" ", 1_u64);
        new_tree.insert("", 69_u64);
        new_tree.insert("/user/:user_id", 0_u64);
        new_tree.insert("/useless/:useless_id", 1_u64);
        new_tree.insert("/", 2_u64);
        new_tree.insert("/user", 2_u64);
        new_tree.insert("/use", 3_u64);
        new_tree.insert("/us", 4_u64);
        new_tree.insert("/dead/*end", 10_u64);
        assert_eq!(0_u64, new_tree.find("user/9").expect("is there").0);
        assert_eq!(1_u64, new_tree.find("/useless/9").expect("is there").0);
        assert_eq!(2_u64, new_tree.find("/").expect("is there").0);
        assert_eq!(2_u64, new_tree.find("/user").expect("is there").0);
        assert_eq!(3_u64, new_tree.find("/use").expect("is there").0);
        assert_eq!(4_u64, new_tree.find("/us").expect("is there").0);
        assert_eq!(10_u64, new_tree.find("/dead/all/over").expect("is there").0);
    }
}
