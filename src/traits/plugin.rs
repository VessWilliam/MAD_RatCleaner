use crate::core::App;

pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub trait PluginGroup {
    fn build_group(self, app: &mut App);
}

impl<P: Plugin> PluginGroup for P {
    fn build_group(self, app: &mut App) {
        self.build(app);
    }
}

macro_rules! impl_plugin_group_tuple {
    ($($name:ident),+) => {
        impl<$($name:Plugin), +> PluginGroup for ($($name,)+) {
           #[allow(non_snake_case)]
           fn build_group(self, app:&mut App){
               let ($($name,)+) = self;
               $( $name.build(app);)+
           }
        }
    };
}

impl_plugin_group_tuple!(A);
impl_plugin_group_tuple!(A, B);
impl_plugin_group_tuple!(A, B, C);
impl_plugin_group_tuple!(A, B, C, D);
impl_plugin_group_tuple!(A, B, C, D, E);
