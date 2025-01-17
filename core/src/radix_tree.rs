#[derive(Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum RadixNodeType {
    #[default]
    Top,
    Normal,
    PathArgument,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct RadixNode {
    prefix: String,
    node_type: RadixNodeType,
    query_arg: String,
    endpoint_id: Option<String>,
    children: Vec<RadixNode>,
}

pub struct RadixTree {
    root_node: RadixNode,
}

// TODO: make builder pattern for building RadixNode
impl RadixNode {
    // temporary solution to use endpoint_id as Optional arg
    pub fn new(prefix: String, node_type: RadixNodeType, endpoint_id: Option<String>) -> Self {
        RadixNode {
            prefix,
            node_type,
            query_arg: String::new(),
            endpoint_id,
            children: Vec::new(),
        }
    }

    pub fn add_query_arguments(&mut self, q: String) {
        self.query_arg = q
    }

    fn common_prefix_length(parent: &str, word: &str) -> usize {
        word.chars()
            .zip(parent.chars())
            .take_while(|(p, w)| p == w)
            .count()
    }

    pub fn insert(&mut self, path: &str, endpoint_id: String) {
        let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            self.endpoint_id = Some(endpoint_id);
            return;
        }

        let segment = segments.remove(0);

        // check if the node already exist e.g. user -> useless = use - r and -less now i want to
        // add user again but when we split at the common prefix - r node already exists 

        for i in 0..self.children.len() {
            let common_prefix = Self::common_prefix_length(&self.children[i].prefix, segment);

            if common_prefix > 0 {
                // this would happend only if the node is dynamic already, e.g. starts with :abc
                // new would be :ability. i dont like the idea using common prefix on dynamic
                // nodes
                if let Some(path) = segment.to_string().strip_prefix(':') {
                    self.children.push(RadixNode::new(
                        path.to_string(),
                        RadixNodeType::PathArgument,
                        None,
                    ));
                    self.children[i].insert(&segments.join("/"), endpoint_id);
                    return;
                };

                let child = &mut self.children[i];
                if child.prefix.len() == common_prefix && segment.len() == child.prefix.len() {
                    // the nodes are the same continue the iteration
                    child.insert(&segments.join("/"), endpoint_id);
                    return;
                } else if child.prefix.len() == common_prefix {
                    // new segment have common_prefix same size as existing node but is longer that
                    // that node soo we need to split just a segment and keep the existing node
                    let new_node_suffix = segment[common_prefix..].to_string();
                    let mut new_child_node =
                        RadixNode::new(new_node_suffix, RadixNodeType::Normal, None);
                    // cant return need to process rest of the path
                    if !segments.is_empty() {
                        new_child_node.insert(&segments.join("/"), endpoint_id.clone());
                    }
                    child.children.push(new_child_node);
                    return;


                } else if child.prefix.len() >= common_prefix {
                    // split the current node at common prefix
                    // create new child from segment after common prefix
                    let old_node_prefix = child.prefix[..common_prefix].to_string();
                    let old_node_suffix = child.prefix[common_prefix..].to_string();
                    let new_node_suffix = segment[common_prefix..].to_string();

                    let mut new_child_node =
                        RadixNode::new(new_node_suffix, RadixNodeType::Normal, None);
                    // create the new parent with common prefix
                    // and we need to inherit the node_type
                    // let mut new_parent_node =
                    // RadixNode::new(old_node_prefix, child.node_type, None);
                    // create new child with remainder of the previous node
                    // and we need to inherit the node_type
                    let mut suffix_node =
                        RadixNode::new(old_node_suffix, child.node_type, child.endpoint_id.clone());

                    // cant return need to process rest of the path
                    if !segments.is_empty() {
                        new_child_node.insert(&segments.join("/"), endpoint_id);
                    }

                    // insert the new node and remainder of the old node into children of the
                    // parent node created from slicing the old
                    // new_parent_node.children = child.children;
                    let old_child = std::mem::replace(
                        child,
                        RadixNode::new(old_node_prefix, child.node_type, None),
                    );
                    suffix_node.children = old_child.children;
                    child.children.push(new_child_node);
                    child.children.push(suffix_node);

                    return;
                }
            }
        }
        // completely new node
        let mut new_node = if let Some(path) = segment.to_string().strip_prefix(':') {
            RadixNode::new(path.to_string(), RadixNodeType::PathArgument, None)
        } else {
            RadixNode::new(segment.to_string(), RadixNodeType::Normal, None)
        };
        new_node.insert(&segments.join("/"), endpoint_id);
        self.children.push(new_node);
    }
}

impl RadixTree {
    pub fn new() -> RadixTree {
        RadixTree {
            root_node: RadixNode {
                prefix: String::new(),
                node_type: RadixNodeType::default(),
                query_arg: String::new(),
                endpoint_id: None,
                children: Vec::new(),
            },
        }
    }

    pub fn insert(&mut self, path: &str, endpoint_id: String) {
        self.root_node.insert(path, endpoint_id);
    }

    // pub fn find(&self, path: &str) -> Option<String> {
    //     let mut current = &self.root_node;
    //     let mut whole_path = vec![];
    //
    //     /*
    //         when we have multiple registered paths like this:
    //             GET a/b/c
    //             GET a/b/:id
    //             GET a/b/:activity_id
    //             GET a/b/:some_string
    //
    //             GET /order/{order_id}
    //             GET /order/{id}
    //
    //             THIS SHOULD NOT WORK THIS TWO PATH WILL CAUSE CONFLIC WHEN ROUTING
    //             GET /order/{id}/activity/{activity_id}
    //             GET /order/{order_id}/activity/{activity_id}
    //
    //         how do we distinguisth that incoming request to path a/b/c
    //         should be handled by handler registered under a/b/c
    //         it could be easily :id, :activity_id or :some_string path parameter.
    //
    //         We could first try the normal path node 'c' and then the other path params nodes
    //
    //     */
    //
    //     for segment in path.split('/').filter(|s| !s.is_empty()) {
    //         // segment have query params
    //         if let Some((segment, _)) = segment.split_once('?') {
    //             if let Some(child) = current.children.get(segment) {
    //                 current = child;
    //                 whole_path.push(current.prefix.clone());
    //             } else {
    //                 let s = current.children.iter().map_while(|(_, c)| {
    //                     if c.node_type == RadixNodeType::PathArgument {
    //                         //
    //                         Some(c)
    //                     } else {
    //                         None
    //                     }
    //                 });
    //                 // } else if let Some((_, node)) = current
    //                 //     .children
    //                 //     .iter()
    //                 //     .find(|(_, c)| c.node_type == RadixNodeType::PathArgument)
    //                 // {
    //                 //     // in real scenario registered handler will be executed if the path params will
    //                 //     whole_path.push(node.prefix.clone())
    //                 // } else {
    //                 //     return None;
    //                 // }
    //             }
    //         } else if let Some(child) = current.children.get(segment) {
    //             current = child;
    //             whole_path.push(current.prefix.clone());
    //         } else {
    //             // else if let Some((_, node)) = current
    //             //     .children
    //             //     .iter()
    //             //     .find(|(_, c)| c.node_type == RadixNodeType::PathArgument)
    //             // {
    //             //     whole_path.push(node.prefix.clone())
    //             // } else {
    //             //     return None;
    //             // }
    //         }
    //     }
    //
    //     let final_node = current;
    //
    //     Some(whole_path.join("/"))
    // }
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
        let s = "/user/:user_id";
        let mut new_tree = RadixTree::new();
        new_tree.insert(s, 0.to_string());
        new_tree.insert("useless/:useless_id", 1.to_string());
        new_tree.insert("/", 2.to_string());
        new_tree.insert("/user", 2.to_string());
        new_tree.insert("/use", 3.to_string());
        // let i = "user/9?key=value";
        // let res = new_tree.find(i);
        // assert_eq!(s, res.expect("is there"));
    }
}
