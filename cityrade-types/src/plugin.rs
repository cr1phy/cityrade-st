trait Plugin {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn description(&self) -> String;
    fn author(&self) -> String;
    fn license(&self) -> String;
    fn run(&self);
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }
}