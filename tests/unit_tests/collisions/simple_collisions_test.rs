use behaviours;
use mach::collisions::SimpleCollisions;

assert_collision_space_behaviour!(|| SimpleCollisions::new() );
