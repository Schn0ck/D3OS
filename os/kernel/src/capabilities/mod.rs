#[derive(Clone,Copy,Debug)]
pub struct Capability {
    pub read: bool,
    pub write: bool,
    pub map_user_heap: bool,
    pub process_execute_binary: bool,
    pub process_id: bool,
    pub process_exit: bool,
    pub thread_create: bool,
    pub thread_id: bool,
    pub thread_switch: bool,
    pub thread_sleep: bool,
    pub thread_join: bool,
    pub thread_exit: bool,
    pub get_system_time: bool,
    pub get_date: bool,
    pub set_date: bool,
}

impl Capability {
    /// Returns a new Capability with default values.
    pub const fn default() -> Self {
        Self::all() //TODO find a default
    }

    /// Returns a new Capability with all values set to false.
    pub const fn new() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the read value set to true.
    pub const fn read() -> Self {
        Self {
            read: true,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the write value set to true.
    pub const fn write() -> Self {
        Self {
            read: false,
            write: true,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the map_user_heap value set to true.
    pub const fn map_user_heap() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: true,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the process_execute_binary value set to true.
    pub const fn process_execute_binary() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: true,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the process_id value set to true.
    pub const fn process_id() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: true,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the process_exit value set to true.
    pub const fn process_exit() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: true,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the thread_create value set to true.
    pub const fn thread_create() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: true,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the thread_id value set to true.
    pub const fn thread_id() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: true,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the thread_switch value set to true.
    pub const fn thread_switch() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: true,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the thread_sleep value set to true.
    pub const fn thread_sleep() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: true,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the thread_join value set to true.
    pub const fn thread_join() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: true,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the thread_exit value set to true.
    pub const fn thread_exit() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: true,
            get_system_time: false,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the get_system_time value set to true.
    pub const fn get_system_time() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: true,
            get_date: false,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the get_date value set to true.
    pub const fn get_date() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: true,
            set_date: false,
        }
    }

    /// Returns a new Capability with only the set_date value set to true.
    pub const fn set_date() -> Self {
        Self {
            read: false,
            write: false,
            map_user_heap: false,
            process_execute_binary: false,
            process_id: false,
            process_exit: false,
            thread_create: false,
            thread_id: false,
            thread_switch: false,
            thread_sleep: false,
            thread_join: false,
            thread_exit: false,
            get_system_time: false,
            get_date: false,
            set_date: true,
        }
    }

    /// Returns a new Capability with all values set to true.
    pub const fn all() -> Self {
        Self {
            read: true,
            write: true,
            map_user_heap: true,
            process_execute_binary: true,
            process_id: true,
            process_exit: true,
            thread_create: true,
            thread_id: true,
            thread_switch: true,
            thread_sleep: true,
            thread_join: true,
            thread_exit: true,
            get_system_time: true,
            get_date: true,
            set_date: true,
        }
    }

    /// Returns a new Capability with all values set to false.
    /// Equal to Capability::new().
    pub const fn none() -> Self {
        Self::new()
    }

    /// Returns if the inputted Capabilites are allowed by this Capability.
    pub const fn is_allowed(&self, capability: Capability) -> bool {
        self.read >= capability.read &&
        self.write >= capability.write &&
        self.map_user_heap >= capability.map_user_heap &&
        self.process_execute_binary >= capability.process_execute_binary &&
        self.process_id >= capability.process_id &&
        self.process_exit >= capability.process_exit &&
        self.thread_create >= capability.thread_create &&
        self.thread_id >= capability.thread_id &&
        self.thread_switch >= capability.thread_switch &&
        self.thread_sleep >= capability.thread_sleep &&
        self.thread_join >= capability.thread_join &&
        self.thread_exit >= capability.thread_exit &&
        self.get_system_time >= capability.get_system_time &&
        self.get_date >= capability.get_date &&
        self.set_date >= capability.set_date
    }

    /// Adds the inputted Capabilities to this Capability.
    pub const fn allow(&mut self, capability: Capability) {
        self.read |= capability.read;
        self.write |= capability.write;
        self.map_user_heap |= capability.map_user_heap;
        self.process_execute_binary |= capability.process_execute_binary;
        self.process_id |= capability.process_id;
        self.process_exit |= capability.process_exit;
        self.thread_create |= capability.thread_create;
        self.thread_id |= capability.thread_id;
        self.thread_switch |= capability.thread_switch;
        self.thread_sleep |= capability.thread_sleep;
        self.thread_join |= capability.thread_join;
        self.thread_exit |= capability.thread_exit;
        self.get_system_time |= capability.get_system_time;
        self.get_date |= capability.get_date;
        self.set_date |= capability.set_date;
    }

    /// Removes the inputted Capabilities from this Capability.
    pub const fn deny(&mut self, capability: Capability) {
        self.read &= !capability.read;
        self.write &= !capability.write;
        self.map_user_heap &= !capability.map_user_heap;
        self.process_execute_binary &= !capability.process_execute_binary;
        self.process_id &= !capability.process_id;
        self.process_exit &= !capability.process_exit;
        self.thread_create &= !capability.thread_create;
        self.thread_id &= !capability.thread_id;
        self.thread_switch &= !capability.thread_switch;
        self.thread_sleep &= !capability.thread_sleep;
        self.thread_join &= !capability.thread_join;
        self.thread_exit &= !capability.thread_exit;
        self.get_system_time &= !capability.get_system_time;
        self.get_date &= !capability.get_date;
        self.set_date &= !capability.set_date;
    }
}
