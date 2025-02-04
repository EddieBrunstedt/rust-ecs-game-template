#[macro_export]
macro_rules! define_components {
    ($(($name:ident, $type:ty)),* $(,)?) => {
        // Generate the component storages
        define_component_storages! {
            $($name: $type),*
        }

        impl_component_storage! {
            $($name: $type),*
        }
    };
}

#[macro_export]
macro_rules! define_component_storages {
    ($($name:ident: $type:ty),*) => {
        /// Internal storage containing all registered component types
        pub struct ComponentStorages {
            $(
                $name: ComponentMap<$type>,
            )*
        }

        impl ComponentStorages {
            /// Creates a new ComponentStorages with empty maps for all components
            fn new() -> Self {
                Self {
                    $(
                        $name: ComponentMap::new(),
                    )*
                }
            }
        }

        #[allow(dead_code)]
        impl Components {
            /// Creates a new empty Components container
            pub fn new() -> Self {
                Self {
                    storages: ComponentStorages::new()
                }
            }

            /// Removes all components associated with the given entity
            pub fn remove_all_components_with_entity(&mut self, entity: Entity) {
                $(
                    self.storages.$name.entries.remove(&entity);
                )*
            }

            $(
                /// Gets an immutable reference to the component if it exists on the entity
                pub fn $name(&self, entity: Entity) -> Option<&$type> {
                    self.storages.$name.get(entity)
                }

                paste::paste! {
                    /// Gets a mutable reference to the component if it exists on the entity
                    pub fn [<$name _mut>](&mut self, entity: Entity) -> Option<&mut $type> {
                        self.storages.$name.get_mut(entity)
                    }

                    /// Gets an immutable reference to the component, panicking if it doesn't exist
                    pub fn [<expect_ $name>](&self, entity: Entity) -> &$type {
                        self.$name(entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    /// Gets a mutable reference to the component, panicking if it doesn't exist
                    pub fn [<expect_ $name _mut>](&mut self, entity: Entity) -> &mut $type {
                        self.[<$name _mut>](entity)
                            .unwrap_or_else(|| panic!("Entity {} is missing required component: {}", entity, stringify!($type)))
                    }

                    /// Checks if the entity has this component
                    pub fn [<has_ $name>](&self, entity: Entity) -> bool {
                        self.$name(entity).is_some()
                    }

                    /// Adds a component to an entity, replacing any existing one
                    fn [<add_ $name>](&mut self, entity: Entity, component: $type) {
                        self.storages.$name.insert(entity, component);
                    }

                    /// Removes this component from the entity if it exists
                    fn [<remove_ $name>](&mut self, entity: Entity) {
                        self.storages.$name.entries.remove(&entity);
                    }
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! impl_component_storage {
    ($($name:ident: $type:ty),*) => {
        $(
            impl ComponentStorage<$type> for Components {
                fn add_component(&mut self, entity: Entity, component: $type) {
                    paste::paste! {
                        self.[<add_ $name>](entity, component);
                    }
                }
            }
        )*
    }
}
