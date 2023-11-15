// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut prefix_resource_split = prefix.split("/");
    let mut request_path_split = request_path.split("/");
    let mut current_resource: Option<&str> = prefix_resource_split.next();
    let mut next_resource: Option<&str> = prefix_resource_split.next();
    let mut current_request_resource: Option<&str> = request_path_split.next();

    while current_resource != None {
        println!("{current_resource:?}");
        println!("{current_request_resource:?}");
        if current_request_resource == None {
            return false;
        }

        if current_request_resource.unwrap() == String::from("") {
            current_request_resource = request_path_split.next();
        }

        match current_resource.unwrap() {
            "" => {
                current_resource = next_resource;
                next_resource = prefix_resource_split.next();
            }
            "*" => {
                if next_resource == None {
                    return true;
                }

                let left = &next_resource.unwrap();
                let right = &current_request_resource.unwrap();

                if left == right {
                    current_resource = next_resource;
                    next_resource = prefix_resource_split.next();
                    continue;
                }

                current_request_resource = request_path_split.next();
            }
            left => {
                let right: &str = &current_request_resource.unwrap();
                println!("{left}, {right}");
                if left != right {
                    return false;
                }

                current_resource = next_resource;
                next_resource = prefix_resource_split.next();
                current_request_resource = request_path_split.next();
            }
        }
    }
    return true;
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {
    println!("{}", prefix_matches("/v1/publishers", "/v1/publishers"));
    println!(
        "{}",
        prefix_matches("/v1/publishers", "/v1/publishersBooks")
    )
}
