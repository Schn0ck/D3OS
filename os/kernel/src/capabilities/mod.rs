pub trait Capability {
    fn can_access_date(&self) -> bool;
}

// Define a concrete capability
pub struct DateReadCapability;

impl Capability for DateReadCapability {
    fn can_access_date(&self) -> bool {
        true
    }
}