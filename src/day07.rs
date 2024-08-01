use nom::character::complete::{alpha1, char, newline, space0};
use nom::error::Error;
use nom::{bytes::complete::take_until, IResult};
//use std::collections::HashMap;

pub fn day07(input_lines: &str) -> (String, String) {
    // Split into known, contiguous locations so commands move at most one level at a time. Skip 1 because it will be either empty or starting from an unknown location in the tree. We could get this info back in later.
    let info_by_root_node = input_lines.split("$ cd /").filter(|buf| !buf.is_empty());
    let roots: Vec<(&str, Node)> = info_by_root_node
        .map(|buf| Node::parse_node_from_buffer("/".to_string(), buf).unwrap())
        .collect();
    // We could have multiple slices through the directory tree or not start at root. Could reconcile those here. For now we know we don't so just take the one we have.
    let (_remaining_buf, last) = roots.last().unwrap();
    let answer1 = filter_tree(last, |node| node.deep_size < 100000)
        .iter()
        .map(|node| node.deep_size)
        .sum::<i32>();

    let answer2 = filter_tree(last, |node: &Node| {
        node.deep_size >= 30000000 - (70000000 - last.deep_size)
    })
    .iter()
    .map(|node| node.deep_size)
    .min()
    .unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

fn filter_tree(node: &Node, filter_condition: impl Fn(&Node) -> bool) -> Vec<&Node> {
    let mut matching_nodes = vec![];
    let mut testing_nodes = vec![node];
    while !testing_nodes.is_empty() {
        matching_nodes.extend(testing_nodes.iter().filter(|node| filter_condition(node)));
        testing_nodes = testing_nodes
            .iter()
            .flat_map(|node| &node.children)
            .collect();
    }
    matching_nodes
}

enum SupportedCommands {
    Cd,
    Ls,
}

// Could replace the regex and not just take every character before newline as cmd argument
// pub fn parse_filename(buf: &str) -> IResult<&str, &str> {
//     let (buf, _) = space0(buf)?;
//     Ok((buf, filename))
// }
pub struct Command<'a> {
    name: SupportedCommands,
    arg: &'a str,
    output: &'a str,
}

impl Command<'_> {
    fn parse_from_buf(buf: &str) -> IResult<&str, Command> {
        let (buf, _) = char('$')(buf)?;
        let (buf, _) = space0(buf)?;
        let (buf, name) = alpha1(buf)?;
        let (buf, _) = space0(buf)?;
        let (buf, arg) = take_until("\n")(buf)?;
        let (buf, _) = space0(buf)?;
        let (buf, _) = newline(buf)?;
        // The output is assumed to be everything up to the next command. If there's no other commands then assume it's everything left.
        let output = take_until::<&str, &str, Error<&str>>("$")(buf);
        let (buf, output) = match output {
            Ok((buf, output)) => (buf, output),
            Err(_) => ("", buf),
        };
        Ok((
            buf,
            Command {
                name: match name {
                    "cd" => SupportedCommands::Cd,
                    "ls" => SupportedCommands::Ls,
                    _ => panic!("Unsupported command"),
                },
                arg: arg.trim(),
                output,
            },
        ))
    }

    // fn parse_argument(buf: &str) -> IResult<&str, String> {
    //     let (buf, _) = space1(buf)?;
    //     let (buf, arg) = alphanumeric0(buf)?;
    //     Ok((buf, arg.to_string()))
    // }
}

pub struct Node {
    name: String,
    child_names: Vec<String>,
    children: Vec<Node>,
    shallow_size: i32,
    deep_size: i32,
}

impl Node {
    fn new_empty(name: &str) -> Self {
        Node {
            name: name.to_string(),
            child_names: vec![],
            children: vec![],
            shallow_size: 0,
            deep_size: 0,
        }
    }

    fn parse_node_from_buffer(name: String, buf: &str) -> IResult<&str, Self> {
        // We call this when we've just cd'd into a new directory. cd has no output, so let's ignore everything until we get to a command
        let mut node = Node::new_empty(name.as_str());
        let (buf, _) = take_until("$")(buf)?;
        let mut remaining_buf = buf;
        while !remaining_buf.is_empty() {
            let (buf, cmd) = Command::parse_from_buf(remaining_buf)?;
            remaining_buf = match cmd.name {
                SupportedCommands::Ls => {
                    node.add_info_from_ls_output(cmd.output);
                    buf
                }
                SupportedCommands::Cd => match cmd.arg {
                    ".." => {
                        return Ok((buf, node));
                    }
                    _ => {
                        // Doesn't deal with us visiting the same directory multiple times
                        let (buf, child) =
                            Node::parse_node_from_buffer(cmd.arg.to_string(), buf).unwrap();
                        node.deep_size += child.deep_size;
                        node.children.push(child);
                        buf
                    }
                },
            };
        }
        Ok((remaining_buf, node))
    }

    fn add_info_from_ls_output(&mut self, ls_output: &str) {
        // Should be a parser here too, which would have better filename handling and prevent us compiling this all the time, but fine for now
        let file_regex = regex::Regex::new(r"(\d+)\s(.*)").unwrap();
        let directory_regex = regex::Regex::new(r"dir\s(.*)").unwrap();
        // Shallow size is constant for fixed input file, so rewrite it here
        self.shallow_size = 0;
        self.shallow_size += file_regex
            .captures_iter(ls_output)
            .map(|file| file[1].parse::<i32>().unwrap())
            .sum::<i32>();

        self.child_names.extend(
            directory_regex
                .captures_iter(ls_output)
                .map(|dir| dir[1].trim().to_string())
                .collect::<Vec<String>>(),
        );
        // ls could be run before or after finding all the child directories so can't zero here. Doesn't handle running ls multiple times
        self.deep_size += self.shallow_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(
            day07(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            )
            .0,
            "95437".to_string()
        )
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(
            day07(
                "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"
            )
            .1,
            "24933642".to_string()
        )
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(
            day07(
                "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"
            ),
            ("95437".to_string(), "24933642".to_string())
        )
    }
}
