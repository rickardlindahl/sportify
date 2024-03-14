use prost::Message;
use std::io::Cursor;

// Include the `teams` module, which is generated from teams.proto.
pub mod protos {
    pub mod teams {
        include!(concat!(env!("OUT_DIR"), "/protobuf.teams.rs"));
    }
}

use protos::teams;

pub fn serialize_add_team(team: &teams::CreateTeam) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(team.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    team.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_add_team(buf: &[u8]) -> Result<teams::CreateTeam, prost::DecodeError> {
    teams::CreateTeam::decode(&mut Cursor::new(buf))
}
