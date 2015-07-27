use behaviours;
use mach::collisions::SimpleCollisions;

assert_collisions_behaviour!(|| SimpleCollisions::new() );
