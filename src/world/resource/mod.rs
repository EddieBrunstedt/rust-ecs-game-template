mod definitions;

pub use definitions::*;

pub struct Resources {
    resources: ResourceTypes,
}

/// Generates a complete resource management system from a list of resource types.
///
/// This macro creates:
/// - A ResourceTypes struct containing `Option<T>` fields for each resource
/// - A complete API for managing each resource type including:
///   - Getters (immutable and mutable)
///   - Expectation methods that panic if resource is missing
///   - Insert/remove/contains methods
///
/// # Example
/// ```rust
/// define_resources! {
///     game_time: GameTime,
///     physics: PhysicsState,
/// }
/// ```
macro_rules! define_resources {
    ($($name:ident: $type:ty),* $(,)?) => {
        /// Internal storage for all registered resource types
        struct ResourceTypes {
            $($name: Option<$type>,)*
        }

        impl ResourceTypes {
            /// Creates a new ResourceTypes with all fields set to None
            fn new() -> Self {
                Self {
                    $($name: None,)*
                }
            }
        }

        #[allow(dead_code)]
        impl Resources {
            /// Creates a new empty Resources container
            pub fn new() -> Self {
                Self {
                    resources: ResourceTypes::new()
                }
            }

            $(
                /// Gets an immutable reference to the resource if it exists
                pub fn $name(&self) -> Option<&$type> {
                    self.resources.$name.as_ref()
                }

                paste::paste! {
                    /// Gets a mutable reference to the resource if it exists
                    pub fn [<$name _mut>](&mut self) -> Option<&mut $type> {
                        self.resources.$name.as_mut()
                    }

                    /// Gets an immutable reference to the resource, panicking if it doesn't exist
                    pub fn [<expect_ $name>](&self) -> &$type {
                        self.$name().expect(concat!("Resource not found: ", stringify!($name)))
                    }

                    /// Gets a mutable reference to the resource, panicking if it doesn't exist
                    pub fn [<expect_ $name _mut>](&mut self) -> &mut $type {
                        self.[<$name _mut>]().expect(concat!("Resource not found: ", stringify!($name)))
                    }

                    /// Inserts a new resource, replacing any existing one
                    pub fn [<insert_ $name>](&mut self, resource: $type) {
                        self.resources.$name = Some(resource);
                    }

                    /// Removes and returns the resource if it exists
                    pub fn [<remove_ $name>](&mut self) -> Option<$type> {
                        self.resources.$name.take()
                    }

                    /// Checks if the resource exists
                    pub fn [<contains_ $name>](&self) -> bool {
                        self.resources.$name.is_some()
                    }
                }
            )*
        }
    };
}

// Generate the resources.
define_resources! {
    game_time: GameTimeResource,
}
