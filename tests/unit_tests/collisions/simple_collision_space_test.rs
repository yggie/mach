use behaviours;
use mach::collisions::SimpleCollisionSpace;

assert_collision_space_behaviour!(|| SimpleCollisionSpace::new() );
