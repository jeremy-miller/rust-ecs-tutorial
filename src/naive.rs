//! Naive ECS implementation

struct Health(i32);
struct Name(&'static str);

struct World {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
}

impl World {
    fn new() -> Self {
        Self {
            health_components: Vec::new(),
            name_components: Vec::new(),
        }
    }

    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
    }
}

pub fn run() {
    let mut world = World::new();

    world.new_entity(Some(Health(-10)), Some(Name("Icarus")));
    world.new_entity(Some(Health(100)), Some(Name("Prometheus")));
    world.new_entity(None, Some(Name("Zeus")));

    let zip = world
        .health_components
        .iter()
        .zip(world.name_components.iter());

    let with_health_and_name =
        zip.filter_map(|(health, name): (&Option<Health>, &Option<Name>)| {
            Some((health.as_ref()?, name.as_ref()?))
        });

    for (health, name) in with_health_and_name {
        if health.0 < 0 {
            println!("{} has perished!", name.0);
        } else {
            println!("{} is still healthy", name.0);
        }
    }
}
