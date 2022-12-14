use std::collections::HashMap;

#[derive(Default)]
pub struct Directory {
    size: u32,
    children: HashMap<String, Self>,
}

impl Directory {
    pub fn size(&self) -> u32 {
        self.size + self.children.values().map(Self::size).sum::<u32>()
    }

    pub fn all_directories(&self) -> Vec<&Self> {
        let mut directories = self.children.values().collect::<Vec<_>>();
        directories.extend(self.children.values().flat_map(Self::all_directories));

        directories
    }

    pub fn parse_lines(lines: Vec<String>) -> Self {
        let mut filesystem = Self::default();
        let mut current_path = Vec::<String>::new();

        for line in lines {
            let mut current_directory = &mut filesystem;
            for dir_name in &current_path {
                current_directory = current_directory.children.get_mut(dir_name).unwrap();
            }

            let mut chars = line.chars();

            let first_word = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();

            match first_word.as_str() {
                "$" => {
                    match chars
                        .by_ref()
                        .take_while(|c| !c.is_ascii_whitespace())
                        .collect::<String>()
                        .as_str()
                    {
                        "cd" => {
                            let dir_name = chars.by_ref().collect::<String>();
                            match dir_name.as_str() {
                                "/" => {
                                    current_path.clear();
                                }
                                ".." => {
                                    current_path.pop();
                                }
                                _ => {
                                    current_path.push(dir_name);
                                }
                            }
                        }
                        "ls" => (),
                        _ => unreachable!(),
                    }
                }
                "dir" => {
                    let dir_name = chars.by_ref().collect::<String>();

                    if !current_directory.children.contains_key(&dir_name) {
                        current_directory
                            .children
                            .insert(dir_name.clone(), Self::default());
                    }
                }
                _ => {
                    let size = first_word.parse::<u32>().unwrap();
                    current_directory.size += size;
                }
            }
        }

        filesystem
    }
}
