use kfn::kfn;

pub const BAN_TLD: &[u8] = b"banTld";
pub const HOST_ID: &[u8] = b"hostId";
pub const UID_ACCOUNT: &[u8] = b"{uid}account";

kfn!(clientUid);
