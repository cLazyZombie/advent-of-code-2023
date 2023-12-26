fn main() {}

struct Maps {
    src_name: String,
    dest_name: String,
    elements: Vec<MapElement>,
}
impl Maps {
    pub fn init(input: &str) -> Self {
        let idx = input.trim().rfind(" ").unwrap();
        let parsed = input[..idx].split('-').collect::<Vec<_>>();
        let src_name = parsed[0].to_string();
        let dest_name = parsed[2].to_string();
        Maps {
            src_name,
            dest_name,
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, input: &str) {
        let parsed = input.trim().split(' ').collect::<Vec<_>>();
        let src = parsed[0].trim().parse::<u64>().unwrap();
        let dest = parsed[1].trim().parse::<u64>().unwrap();
        let range = parsed[2].trim().parse::<u64>().unwrap();
        self.elements.push(MapElement { dest, src, range });
    }

    pub fn get_dest(&self, src: u64) -> u64 {
        for element in &self.elements {
            if let Some(dest) = element.get_dest(src) {
                return dest;
            }
        }

        src
    }
}

#[derive(Debug, PartialEq)]
struct MapElement {
    dest: u64,
    src: u64,
    range: u64,
}
impl MapElement {
    pub fn check_src_range(&self, v: u64) -> bool {
        v >= self.src && v < self.src + self.range
    }

    pub fn get_dest(&self, v: u64) -> Option<u64> {
        if self.check_src_range(v) {
            Some(self.dest + v - self.src)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_maps {
    use super::*;

    #[test]
    fn test_init() {
        let maps = Maps::init("seed-to-soil maps: \n");
        assert_eq!(maps.src_name, "seed");
        assert_eq!(maps.dest_name, "soil");
    }

    #[test]
    fn test_add_element() {
        let mut maps = Maps::init("seed-to-soil maps: \n");
        maps.add_element("1 2 3\n");
        maps.add_element("4 5 6");

        assert_eq!(maps.elements.len(), 2);
        assert_eq!(
            maps.elements[0],
            MapElement {
                dest: 2,
                src: 1,
                range: 3,
            }
        );
    }

    #[test]
    fn test_get_dest() {
        let maps = Maps {
            src_name: "src".to_string(),
            dest_name: "dest".to_string(),
            elements: vec![
                MapElement {
                    dest: 1,
                    src: 0,
                    range: 10,
                },
                MapElement {
                    dest: 11,
                    src: 20,
                    range: 10,
                },
                MapElement {
                    dest: 40,
                    src: 30,
                    range: 1,
                },
            ],
        };

        assert_eq!(maps.get_dest(0), 1);
        assert_eq!(maps.get_dest(9), 10);
        assert_eq!(maps.get_dest(10), 10);
        assert_eq!(maps.get_dest(20), 11);
        assert_eq!(maps.get_dest(30), 40);
        assert_eq!(maps.get_dest(31), 31);
    }
}
