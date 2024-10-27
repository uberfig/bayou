#[derive(Debug, Clone, Copy)]
pub enum PermissionLevel {
    /// intended for the main admin account(s) of the server, will be
    /// featured and considered the pont of contact for the instance,
    /// can be set to be auto followed by new users
    AdminOne,
    /// intended for anyone who has admin access to the server
    AdminTwo,
    /// intended for mods who can take vito actoin in an emergency
    ModOne,
    /// intended for mods who need to open a proposal for mod changes
    ModTwo,
    /// intended for public registration servers to limit things to only
    /// known users for example if they wish to have only known users or
    /// higher able to vote on proposals so that a malicious actor can't
    /// start making accounts to influence a decision. When manual approval
    /// is used, all approved users will be trusted and pending users will
    /// be untrusted. this would allow for a switching to manual approval
    /// in the event of an emergency still allowing trusted users to
    /// continue unnaffected and untrusted accounts would be preserved and
    /// prompted to send an application for approval when they log in next
    TrustedUser,
    /// the default, what they can do is up to server policy, used for
    /// accounts pending approval in a manual approval setup
    UntrustedUser,
}

impl From<i16> for PermissionLevel {
    fn from(value: i16) -> Self {
        match value {
            1 => PermissionLevel::AdminOne,
            2 => PermissionLevel::AdminTwo,
            3 => PermissionLevel::ModOne,
            4 => PermissionLevel::ModTwo,
            5 => PermissionLevel::TrustedUser,
            6 => PermissionLevel::UntrustedUser,
            _ => PermissionLevel::UntrustedUser,
        }
    }
}
impl From<PermissionLevel> for i16 {
    fn from(val: PermissionLevel) -> Self {
        match val {
            PermissionLevel::AdminOne => 1,
            PermissionLevel::AdminTwo => 2,
            PermissionLevel::ModOne => 3,
            PermissionLevel::ModTwo => 4,
            PermissionLevel::TrustedUser => 5,
            PermissionLevel::UntrustedUser => 6,
        }
    }
}
