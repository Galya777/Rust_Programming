use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Direction {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "North" => Ok(Direction::North),
            "South" => Ok(Direction::South),
            "East"  => Ok(Direction::East),
            "West"  => Ok(Direction::West),
            _       => Err(Errors::DirectionParseError(s.to_owned())),
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East  => Self::West,
            Self::West  => Self::East,
        }
    }
}

#[derive(Default)]
pub struct Room {
    pub name: String,
    links: HashMap<Direction, String>,
}

impl Room {
    fn new(name: &str) -> Self {
        Room { name: name.to_owned(), ..Self::default() }
    }
}

pub struct Dungeon {
    rooms: HashMap<String, Room>,
}

impl Dungeon {
    pub fn new() -> Self {
        Dungeon { rooms: HashMap::new() }
    }

    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        let mut dungeon = Dungeon::new();
        let mut iterator = reader.lines().enumerate();
        let mut line_number = 0;

        let (_, rooms_line) = iterator.next().ok_or_else(|| Errors::LineParseError { line_number: 0 })?;
        let rooms_line = rooms_line.map_err(Errors::IoError)?;
        if rooms_line.trim() != "## Rooms" {
            return Err(Errors::LineParseError { line_number: 1 });
        }

        while let Some((index, line)) = iterator.next() {
            line_number = index + 1;
            let line = line.map_err(Errors::IoError)?;

            if line.trim().len() == 0 {
                break;
            }

            let room_name = match_prefix("- ", &line).ok_or_else(|| Errors::LineParseError { line_number })?;
            dungeon.add_room(room_name)?;
        }

        let (index, links_line) = iterator.next().ok_or_else(|| Errors::LineParseError { line_number })?;
        let links_line = links_line.map_err(Errors::IoError)?;
        if links_line.trim() != "## Links" {
            return Err(Errors::LineParseError { line_number: index + 1 });
        }

        while let Some((index, line)) = iterator.next() {
            let line_number = index + 1;
            let line = line.map_err(Errors::IoError)?;
            let link_description = match_prefix("- ", &line).
                ok_or_else(|| Errors::LineParseError { line_number })?;

            let parts: Vec<&str> = link_description.split(" -> ").collect();
            if parts.len() != 3 {
                return Err(Errors::LineParseError { line_number });
            }

            dungeon.set_link(parts[0], parts[1].parse()?, parts[2])?;
        }

        Ok(dungeon)
    }

    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        if self.rooms.contains_key(name) {
            return Err(Errors::DuplicateRoom(name.to_owned()))
        }

        self.rooms.insert(name.to_owned(), Room::new(name));
        Ok(())
    }

    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        let room = self.rooms.get_mut(room_name).
            ok_or_else(|| Errors::UnknownRoom(room_name.to_owned()))?;
        room.links.insert(direction, other_room_name.to_owned());

        let other_room = self.rooms.get_mut(other_room_name).
            ok_or_else(|| Errors::UnknownRoom(other_room_name.to_owned()))?;
        other_room.links.insert(direction.opposite(), room_name.to_owned());

        Ok(())
    }

    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        self.rooms.get(room_name).
            ok_or_else(|| Errors::UnknownRoom(room_name.to_owned()))
    }

    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        let room = self.rooms.get(room_name).
            ok_or_else(|| Errors::UnknownRoom(room_name.to_owned()))?;

        if let Some(other_room_name) = room.links.get(&direction) {
            self.rooms.get(other_room_name).
                ok_or_else(|| Errors::UnknownRoom(other_room_name.to_owned())).
                map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {

        let start_room = self.get_room(start_room_name)?;
        let end_room = self.get_room(end_room_name)?;

        if start_room_name == end_room_name {
            return Ok(Some(vec![end_room]));
        }

        let mut room_queue = VecDeque::new();
        let mut parents = HashMap::<&str, &str>::new();

        let mut seen = HashSet::new();
        seen.insert(&start_room.name);
        room_queue.push_front(start_room);

        while let Some(current_room) = room_queue.pop_back() {
            if current_room.name == end_room_name {
                break;
            }

            for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
                if let Some(next_room) = self.get_next_room(&current_room.name, direction)? {
                    if !seen.contains(&next_room.name) {
                        room_queue.push_front(next_room);
                        seen.insert(&next_room.name);
                        parents.insert(&next_room.name, &current_room.name);
                    }
                }
            }
        }

        if parents.get(end_room.name.as_str()).is_none() {
            return Ok(None);
        }

        let mut path = vec![end_room];
        let mut current_room_name = end_room.name.as_str();

        while let Some(parent_name) = parents.get(&current_room_name) {
            path.push(self.get_room(parent_name)?);
            current_room_name = parent_name;
        }

        path.reverse();
        Ok(Some(path))
    }
}

pub fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    if input.starts_with(prefix) {
        Some(&input[prefix.len()..])
    } else {
        None
    }
}
