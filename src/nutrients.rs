use bevy::prelude::*;
pub enum Nutrient {
    Nitrogen,
    Phosphorus,
    Potassium,
    // Calcium,
    // Magnesium,
    // Sulfur,
}

#[derive(Component)]
pub struct NutrientSink(Nutrient);

#[derive(Component)]
pub struct NutrientSource(Nutrient);
