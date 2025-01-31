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

pub struct RadixTree {
    root_node: RadixNode,
}

impl Default for RadixTree {
    fn default() -> Self {
        Self {
            root_node: RadixNode {
                prefix: String::new(),
                node_type: RadixNodeType::Exact,
                endpoint_id: None,
                children: Vec::new(),
            },
        }
    }
}

impl RadixTree {
    pub fn new() -> Self {
        Self {
            root_node: RadixNode {
                prefix: String::new(),
                node_type: RadixNodeType::Exact,
                endpoint_id: None,
                children: Vec::new(),
            },
        }
    }

    pub fn insert(&mut self, path: &str, endpoint_id: String) {
        self.root_node.insert(path, endpoint_id);
    }

    pub fn find(&self, path: &str) -> Option<(String, HashMap<String, String>)> {
        if let Some(match_result) = self.root_node.find(path) {
            Some((match_result.endpoint_id, match_result.parameters))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct RadixNode {
    prefix: String,
    node_type: RadixNodeType,
    endpoint_id: Option<String>,
    children: Vec<RadixNode>,
}

struct RadixNodeBuilder {
    prefix: String,
    node_type: RadixNodeType,
    endpoint_id: Option<String>,
    children: Vec<RadixNode>,
}

impl RadixNodeBuilder {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            node_type: RadixNodeType::Exact,
            endpoint_id: None,
            children: Vec::new(),
        }
    }

    pub fn prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn node_type(mut self, node_type: RadixNodeType) -> Self {
        self.node_type = node_type;
        self
    }

    pub fn endpoint_id(mut self, endpoint_id: Option<String>) -> Self {
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
            prefix: self.prefix,
            node_type: self.node_type,
            endpoint_id: self.endpoint_id,
            children: self.children,
        }
    }
}

impl RadixNode {
    // temporary solution to use endpoint_id as Optional arg
    pub fn new(prefix: String, node_type: RadixNodeType, endpoint_id: Option<String>) -> Self {
        RadixNode {
            prefix,
            node_type,
            endpoint_id,
            children: Vec::new(),
        }
    }

    fn common_prefix_length(parent: &str, word: &str) -> usize {
        word.chars()
            .zip(parent.chars())
            .take_while(|(p, w)| p == w)
            .count()
    }

    fn insert(&mut self, path: &str, endpoint_id: String) {
        let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            self.endpoint_id = Some(endpoint_id);

            // Every time we are at the end of the path
            // We reorder the saved children by priority
            // Exact - 3, PathParam - 2, WildCards - 3(not yet implemented).
            // Reason is when we match incoming path we iterate children from Exact matches through
            // PathParam and to WildCards
            self.children.sort_by(|a, b| a.node_type.cmp(&b.node_type));
            return;
        }

        let segment = segments.remove(0);

        for i in 0..self.children.len() {
            let common_prefix = Self::common_prefix_length(&self.children[i].prefix, segment);
            if common_prefix > 0 {
                // this would happend only if the node is dynamic already, e.g. starts with *abc
                // new would be *ability. i dont like the idea using common prefix on dynamic
                // nodes
                let builder = RadixNodeBuilder::new();
                if let Some(path) = segment.to_string().strip_prefix('*') {
                    self.children.push(
                        builder
                            .prefix(path.to_string())
                            .node_type(RadixNodeType::WildCard)
                            .build(),
                    );
                    self.children[i].insert(&segments.join("/"), endpoint_id);
                    return;
                // this would happend only if the node is dynamic already, e.g. starts with :abc
                // new would be :ability. i dont like the idea using common prefix on dynamic
                // nodes
                } else if let Some(path) = segment.to_string().strip_prefix(':') {
                    self.children.push(
                        builder
                            .prefix(path.to_string())
                            .node_type(RadixNodeType::PathArgument)
                            .build(),
                    );
                    self.children[i].insert(&segments.join("/"), endpoint_id);
                    return;
                }

                let child = &mut self.children[i];

                // the nodes are the same continue the iteration
                if child.prefix.len() == common_prefix && segment.len() == child.prefix.len() {
                    child.insert(&segments.join("/"), endpoint_id);
                    return;

                // new segment have common_prefix same size as existing node but is longer that
                // that node soo we need to split just a segment and keep the existing node
                } else if child.prefix.len() == common_prefix {
                    let new_node_suffix = segment[common_prefix..].to_string();
                    // checking if the node does exist already
                    if let Some(existing_child_idx) = child
                        .children
                        .iter()
                        .position(|child| child.prefix == new_node_suffix)
                    {
                        child.children[existing_child_idx]
                            .insert(&segments.join("/"), endpoint_id.clone());
                    } else {
                        let mut new_child_node = builder
                            .prefix(new_node_suffix)
                            .node_type(RadixNodeType::Exact)
                            .build();
                        new_child_node.insert(&segments.join("/"), endpoint_id.clone());
                        child.children.push(new_child_node);
                    }
                    return;
                } else if child.prefix.len() > common_prefix {
                    // split the current node at common prefix
                    let old_node_prefix = child.prefix[..common_prefix].to_string();
                    let old_node_suffix = child.prefix[common_prefix..].to_string();
                    // create new child from segment after common prefix
                    let new_node_suffix = segment[common_prefix..].to_string();

                    // TODO: we need to check if the new_node_suffix is not empty can happend when
                    // we adding new "us" segment to existing "user" node so we split at "us" and "" will
                    // be the new_node_suffix remainder which we need to check for and handle
                    if !new_node_suffix.is_empty() {
                        let mut new_child_node = RadixNodeBuilder::new()
                            .prefix(new_node_suffix)
                            .node_type(RadixNodeType::Exact)
                            .build();
                        // create the new parent with common prefix
                        // and we need to inherit the node_type
                        // let mut new_parent_node =
                        // RadixNode::new(old_node_prefix, child.node_type, None);
                        // create new child with remainder of the previous node
                        // and we need to inherit the node_type
                        let mut suffix_node = RadixNodeBuilder::new()
                            .prefix(old_node_suffix)
                            .node_type(child.node_type)
                            .endpoint_id(child.endpoint_id.clone())
                            .build();

                        new_child_node.insert(&segments.join("/"), endpoint_id);

                        // insert the new node and remainder of the old node into children of the
                        // parent node created from slicing the old
                        // new_parent_node.children = child.children;
                        let old_child = std::mem::replace(
                            child,
                            RadixNodeBuilder::new()
                                .prefix(old_node_prefix)
                                .node_type(child.node_type)
                                .build(),
                        );
                        suffix_node.children = old_child.children;
                        child.children.push(new_child_node);
                        child.children.push(suffix_node);

                        return;
                    }
                    let mut suffix_node = RadixNodeBuilder::new()
                        .prefix(old_node_suffix)
                        .node_type(child.node_type)
                        .endpoint_id(child.endpoint_id.clone())
                        .build();

                    let old_child = std::mem::replace(
                        child,
                        RadixNodeBuilder::new()
                            .prefix(old_node_prefix)
                            .node_type(child.node_type)
                            .build(),
                    );

                    suffix_node.children = old_child.children;
                    child.children.push(suffix_node);
                    child.insert(&segments.join("/"), endpoint_id);

                    return;
                }
            }
        }

        // completely new node
        let builder = RadixNodeBuilder::new();
        let mut new_node = if let Some(path) = segment.to_string().strip_prefix('*') {
            builder
                .prefix(path.to_string())
                .node_type(RadixNodeType::WildCard)
                .build()
        } else if let Some(path) = segment.to_string().strip_prefix(':') {
            builder
                .prefix(path.to_string())
                .node_type(RadixNodeType::PathArgument)
                .build()
        } else {
            builder
                .prefix(segment.to_string())
                .node_type(RadixNodeType::Exact)
                .build()
        };

        new_node.insert(&segments.join("/"), endpoint_id);
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
                                Self::common_prefix_length(&child.prefix, current_path_segment);
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
                            new_params.insert(child.prefix.clone(), remaining_path[0].to_owned());
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
                                .insert(child.prefix[1..].to_owned(), remaining_path.join("/"));
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
    endpoint_id: String,
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
        new_tree.insert("/user/:user_id", 0.to_string());
        new_tree.insert("/useless/:useless_id", 1.to_string());
        new_tree.insert("/", 2.to_string());
        new_tree.insert("/user", 2.to_string());
        new_tree.insert("/use", 3.to_string());
        new_tree.insert("/us", 4.to_string());
        new_tree.insert("/dead/*end", 10.to_string());
        assert_eq!(0.to_string(), new_tree.find("user/9").expect("is there").0);
        assert_eq!(
            1.to_string(),
            new_tree.find("/useless/9").expect("is there").0
        );
        assert_eq!(2.to_string(), new_tree.find("/").expect("is there").0);
        assert_eq!(2.to_string(), new_tree.find("/user").expect("is there").0);
        assert_eq!(3.to_string(), new_tree.find("/use").expect("is there").0);
        assert_eq!(4.to_string(), new_tree.find("/us").expect("is there").0);
        assert_eq!(
            10.to_string(),
            new_tree.find("/dead/all/over").expect("is there").0
        );
    }
}
