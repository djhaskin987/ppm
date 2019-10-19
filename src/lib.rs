use std::cmp::Ordering;

#[derive(Eq)]
struct Version(String);

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        //let first_part = consumeNonDigits(self);

//        let mut chars_a = self.0.chars();
//        let mut chars_b = other.0.chars();
        self.0.cmp(&other.0)
    }
}


enum AlternativeStatus {
    Absent,
    Present
}

enum VersionRelation {
    LessThan,
    LessEqual,
    Equal,
    NotEqual,
    GreaterEqual,
    Greater,
    Matches,
    InRange,
    PessimGreater
}

struct VersionSpecTerm {
    relation: VersionRelation,
    version: String
}

struct Alternative {
    status: AlternativeStatus,
    name: String,
    // This is a *DNF* of version terms
    spec: Vec<Vec<VersionSpecTerm>>
}

struct Package {
    name: String,
    version: String,
    // This is a *CNF* of requirements
    requirements: Vec<Vec<Alternative>>
}


//pub fn install_package(location: &str) -> Result<(), Box<dyn std::error::Error>> {
//
//
//    return Ok(());
//}
