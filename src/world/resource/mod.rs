mod definitions;

pub use definitions::GameTime;

pub struct Resources {
    resources: ResourceTypes,
}

macro_rules! define_resources {
    ($($name:ident: $type:ty),* $(,)?) => {
        struct ResourceTypes {
            $($name: Option<$type>,)*
        }

        impl ResourceTypes {
            fn new() -> Self {
                Self {
                    $($name: None,)*
                }
            }
        }

        #[allow(dead_code)]
        impl Resources {
            pub fn new() -> Self {
                Self {
                    resources: ResourceTypes::new()
                }
            }

            $(
                pub fn $name(&self) -> Option<&$type> {
                    self.resources.$name.as_ref()
                }

                paste::paste! {
                    pub fn [<$name _mut>](&mut self) -> Option<&mut $type> {
                        self.resources.$name.as_mut()
                    }

                    pub fn [<expect_ $name>](&self) -> &$type {
                        self.$name().expect(concat!("Resource not found: ", stringify!($name)))
                    }

                    pub fn [<expect_ $name _mut>](&mut self) -> &mut $type {
                        self.[<$name _mut>]().expect(concat!("Resource not found: ", stringify!($name)))
                    }

                    pub fn [<insert_ $name>](&mut self, resource: $type) {
                        self.resources.$name = Some(resource);
                    }

                    pub fn [<remove_ $name>](&mut self) -> Option<$type> {
                        self.resources.$name.take()
                    }

                    pub fn [<contains_ $name>](&self) -> bool {
                        self.resources.$name.is_some()
                    }
                }
            )*
        }
    };
}

// Define all your resources here
define_resources! {
    game_time: GameTime,
}
