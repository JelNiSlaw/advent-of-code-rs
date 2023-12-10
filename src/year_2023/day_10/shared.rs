#[derive(Clone, Copy)]
pub enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
}

impl Pipe {
    fn parse(byte: u8) -> Option<Self> {
        match byte {
            b'|' => Some(Self::NS),
            b'-' => Some(Self::EW),
            b'L' => Some(Self::NE),
            b'J' => Some(Self::NW),
            b'7' => Some(Self::SW),
            b'F' => Some(Self::SE),
            b'.' => None,
            b'S' => Some(Self::Start),
            _ => unreachable!(),
        }
    }

    const fn connections(self) -> [(i32, i32); 2] {
        match self {
            Self::NS => [(0, -1), (0, 1)],
            Self::EW => [(1, 0), (-1, 0)],
            Self::NE => [(0, -1), (1, 0)],
            Self::NW => [(0, -1), (-1, 0)],
            Self::SW => [(0, 1), (-1, 0)],
            Self::SE => [(0, 1), (1, 0)],
            Self::Start => [(0, 0), (0, 0)],
        }
    }

    pub const fn is_north_facing(self) -> bool {
        matches!(self, Self::NS | Self::NE | Self::NW)
    }
}

pub struct Maze {
    pipes: Vec<Vec<Option<Pipe>>>,
}

impl Maze {
    pub fn parse(lines: Vec<String>) -> Self {
        let pipes = lines
            .into_iter()
            .map(|line| line.bytes().map(Pipe::parse).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { pipes }
    }

    fn start_location(&self) -> (usize, usize) {
        self.pipes
            .iter()
            .zip(0..)
            .find_map(|(row, y)| {
                row.iter()
                    .zip(0..)
                    .find_map(|(pipe, x)| matches!(pipe, Some(Pipe::Start)).then_some(x))
                    .map(|x| (x, y))
            })
            .unwrap()
    }

    pub fn pipe_loop_locations(&self) -> Vec<Vec<Option<Pipe>>> {
        let mut locations = self
            .pipes
            .iter()
            .map(|row| row.iter().map(|_| None).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let start_location = self.start_location();
        let mut last_location = start_location;
        locations[start_location.1][start_location.0] = Some(Pipe::Start);

        let mut current_location = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .find_map(|offset: (i32, i32)| {
            let x = i32::try_from(start_location.0).unwrap() + offset.0;
            let y = i32::try_from(start_location.1).unwrap() + offset.1;

            if x < 0 || y < 0 {
                return None;
            }

            let x = usize::try_from(x).unwrap();
            let y = usize::try_from(y).unwrap();

            if y > self.pipes.len() || x > self.pipes[y].len() {
                return None;
            }

            let Some(pipe) = self.pipes[y][x] else {
                return None;
            };

            pipe.connections()
                .into_iter()
                .map(|offset| {
                    (
                        usize::try_from(i32::try_from(x).unwrap() + offset.0).unwrap(),
                        usize::try_from(i32::try_from(y).unwrap() + offset.1).unwrap(),
                    )
                })
                .any(|location| location == start_location)
                .then_some((x, y))
        })
        .unwrap();

        loop {
            let Some(pipe) = self.pipes[current_location.1][current_location.0] else {
                unreachable!()
            };

            if matches!(pipe, Pipe::Start) {
                break;
            }

            locations[current_location.1][current_location.0] = Some(pipe);

            for offset in pipe.connections() {
                let new_location = (
                    usize::try_from(i32::try_from(current_location.0).unwrap() + offset.0).unwrap(),
                    usize::try_from(i32::try_from(current_location.1).unwrap() + offset.1).unwrap(),
                );

                if new_location != last_location {
                    last_location = current_location;
                    current_location = new_location;

                    break;
                }
            }
        }

        locations
    }
}
