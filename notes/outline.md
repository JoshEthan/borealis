# Borealis Outline

## High Level

* ECS at its core (but only where needed)
* Simple API backend by flexible systems
    * ex: PBR renderer built on a render graph system
* Flexbox UI
    * Simple, standard, good implementations exist
* 100% Rust (except for the few cases where this is impossible)
* Batteries included
    * 2D/3D Rendering, UI, Phyisics, Networking, etc.
* Editor: also a "game"
    * Dogfood components
* Fast app compile time (< 5 seconds)

## Dependencies

* Legion ECS
* wgfx-rs
* nalgebra
* nphysics/ncollide
* error-log
* pollster
* winit

## Outline

* Core
    * Shared
        * Types
            * enum PropertyValue
                * DATATYPE_WRAPPERS_HERE
                * Analog: godot's Variant
            * struct Property
            * struct Texture
        * Components
            <!-- Hierarchy -->
            * Parent
                * Children ```Vec<EntityId>```
            <!-- Properties -->
            * Properties
                * ```HashMap<string, Property>```
            <!-- Rendering -->
            * Mesh
            * Armature
            * Material
        * Systems
            <!-- Rendering -->
            * UpdateArmatureTransforms
            * SyncPropertiesToMaterialUniforms
    * 3D
        * Components
            <!-- Position -->
            * Transform
            * GlobalTransform
            <!-- Physics -->
            * PhysicsBody
            * CollisionShape
            * RigidBody
        * Systems
            <!-- Position -->
            * CalculateGlobalTransform
                * Dep: Child, GlobalTransform, Transform
            <!-- Physics -->
            * UpdateCollision/NCollide
                * Dep: CollisionShape, PhysicsBody, GlobalTransform
            * UpdateRigidBodies/NCollide
                * Dep: PhysicsBody, RigidBody, GlobalTransform
    * 2D
        * Components
            <!-- Position -->
            * Transform2d
            * GlobalTransform2d
            <!-- UI -->
            * Element
            <!-- Physics -->
            * PhysicsBody2d
            * CollisionShape2d
            * RigidBody2d
        * Systems
            <!-- Position -->
            * CalculateGlobalTransform2d
                * Dep: Child, GlobalTransform2d, Transform2d
            <!-- Physics -->
            * UpdateCollisions2d/NCollide
                * Dep: CollisioShape2d, PhysicsBody2d, GlobalTransform2d
            * UpdateRigidBodies2d/NCollide
                * Dep: PhysicsBody2d, RigidBody2d, GlobalTransform2d