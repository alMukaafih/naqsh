/// Kind of target to Build.
pub enum TargetKind {
    /// A target which has the characteristics of a vibrant color which is light in luminance.
    LightVibrant,
    /// A target which has the characteristics of a vibrant color which is neither light or dark.
    Vibrant,
    /// A target which has the characteristics of a vibrant color which is dark in luminance.
    DarkVibrant,
    /// A target which has the characteristics of a muted color which is light in luminance.
    LightMuted,
    /// A target which has the characteristics of a muted color which is neither light or dark.
    Muted,
    /// A target which has the characteristics of a muted color which is dark in luminance.
    DarkMuted
}

/// A struct which allows custom selection of colors in a [Palette](super::Palette)'s generation. Instances
/// can be created via the [TargetBuilder] class.
///
/// To use the target, use the [add_target](super::PaletteBuilder::add_target) API when building a
/// Palette.
pub struct Target {
    m_saturation_targets: [f32;3],
    m_lightness_targets: [f32;3],
    m_weights: [f32;3],
    m_is_exclusive: bool,
}

impl Default for Target {
    fn default() -> Self {
        let mut target = Self {
            m_saturation_targets: [0.0;3],
            m_lightness_targets: [0.0;3],
            m_weights: [0.0;3],
            m_is_exclusive: true
        };
        target.set_target_default_values();
        target.set_default_weights();
        target
    }
}

impl Target {
    const TARGET_DARK_LUMA: f32 = 0.26;
    const MAX_DARK_LUMA: f32 = 0.45;

    const MIN_LIGHT_LUMA: f32 = 0.55;
    const TARGET_LIGHT_LUMA: f32 = 0.74;

    const MIN_NORMAL_LUMA: f32 = 0.3;
    const TARGET_NORMAL_LUMA: f32 = 0.5;
    const MAX_NORMAL_LUMA: f32 = 0.7;

    const TARGET_MUTED_SATURATION: f32 = 0.3;
    const MAX_MUTED_SATURATION: f32 = 0.4;

    const TARGET_VIBRANT_SATURATION: f32 = 1.0;
    const MIN_VIBRANT_SATURATION: f32 = 0.35;

    const WEIGHT_SATURATION: f32 = 0.24;
    const WEIGHT_LUMA: f32 = 0.52;
    const WEIGHT_POPULATION: f32 = 0.24;

    pub(crate) const INDEX_MIN: usize = 0;
    pub(crate) const INDEX_TARGET: usize = 1;
    pub(crate) const INDEX_MAX: usize = 2;

    pub(crate) const INDEX_WEIGHT_SAT: usize = 0;
    pub(crate) const INDEX_WEIGHT_LUMA: usize = 1;
    pub(crate) const INDEX_WEIGHT_POP: usize = 2;

    fn set_target_default_values(&mut self) {
        self.m_lightness_targets[Target::INDEX_MIN] = 0.0;
        self.m_lightness_targets[Target::INDEX_TARGET] = 0.5;
        self.m_lightness_targets[Target::INDEX_MAX] = 1.0;

        self.m_saturation_targets[Target::INDEX_MIN] = 0.0;
        self.m_saturation_targets[Target::INDEX_TARGET] = 0.5;
        self.m_saturation_targets[Target::INDEX_MAX] = 1.0;
    }

    fn set_default_weights(&mut self) {
        self.m_weights[Target::INDEX_WEIGHT_SAT] = Target::WEIGHT_SATURATION;
        self.m_weights[Target::INDEX_WEIGHT_LUMA] = Target::WEIGHT_LUMA;
        self.m_weights[Target::INDEX_WEIGHT_POP] = Target::WEIGHT_POPULATION;
    }

    pub(crate) fn normalize_weights(&mut self) {
        let mut sum = 0f32;
        let mut i = 0;
        let z = self.m_weights.len();
        while i < z {
            let weight = self.m_weights[i];
            if weight > 0f32 {
                sum += weight;
            }
            i += 1;
        }
        if sum != 0f32 {
            let mut i = 0;
            let z = self.m_weights.len();
            while i < z {
                if self.m_weights[i] > 0f32 {
                    self.m_weights[i] /= sum
                }
                i += 1;
            }
        }
    }

    fn set_default_dark_lightness_values(&mut self) {
        self.m_lightness_targets[Target::INDEX_TARGET] = Target::TARGET_DARK_LUMA;
        self.m_lightness_targets[Target::INDEX_MAX] = Target::MAX_DARK_LUMA;
    }

    fn set_default_normal_lightness_values(&mut self) {
        self.m_lightness_targets[Target::INDEX_MIN] = Target::MIN_NORMAL_LUMA;
        self.m_lightness_targets[Target::INDEX_TARGET] = Target::TARGET_NORMAL_LUMA;
        self.m_lightness_targets[Target::INDEX_MAX] = Target::MAX_NORMAL_LUMA;
    }

    fn set_default_light_lightness_values(&mut self) {
        self.m_lightness_targets[Target::INDEX_MIN] = Target::MIN_LIGHT_LUMA;
        self.m_lightness_targets[Target::INDEX_TARGET] = Target::TARGET_LIGHT_LUMA;
    }

    fn set_default_vibrant_saturation_values(&mut self) {
        self.m_saturation_targets[Target::INDEX_MIN] = Target::MIN_VIBRANT_SATURATION;
        self.m_saturation_targets[Target::INDEX_TARGET] = Target::TARGET_VIBRANT_SATURATION;
    }

    fn set_default_muted_saturation_values(&mut self) {
        self.m_saturation_targets[Target::INDEX_TARGET] = Target::TARGET_MUTED_SATURATION;
        self.m_saturation_targets[Target::INDEX_MAX] = Target::MAX_MUTED_SATURATION;
    }

    pub fn new(kind: TargetKind) -> Target {
        let mut target = Target::default();

        use TargetKind::*;
        match kind {
            LightVibrant => {
                target.set_default_light_lightness_values();
                target.set_default_vibrant_saturation_values()
            }
            Vibrant => {
                target.set_default_normal_lightness_values();
                target.set_default_vibrant_saturation_values();
            }
            DarkVibrant => {
                target.set_default_dark_lightness_values();
                target.set_default_vibrant_saturation_values();
            }
            LightMuted => {
                target.set_default_light_lightness_values();
                target.set_default_muted_saturation_values();
            }
            Muted => {
                target.set_default_normal_lightness_values();
                target.set_default_muted_saturation_values();
            }
            DarkMuted => {
                target.set_default_dark_lightness_values();
                target.set_default_muted_saturation_values();
            }
        }
        target
    }

    /// The minimum saturation value for this target.
    pub fn get_minimum_saturation(&self) -> f32 {
        self.m_saturation_targets[Target::INDEX_MIN]
    }

    /// The target saturation value for this target.
    pub fn get_target_saturation(&self) -> f32 {
        self.m_saturation_targets[Target::INDEX_TARGET]
    }

    /// The maximum saturation value for this target.
    pub fn get_maximum_saturation(&self) -> f32 {
        self.m_saturation_targets[Target::INDEX_MAX]
    }

    /// The minimum lightness value for this target.
    pub fn get_minimum_lightness(&self) -> f32 {
        self.m_lightness_targets[Target::INDEX_MIN]
    }

    /// The target lightness value for this target.
    pub fn get_target_lightness(&self) -> f32 {
        self.m_lightness_targets[Target::INDEX_TARGET]
    }

    /// The maximum lightness value for this target.
    pub fn get_maximum_lightness(&self) -> f32 {
        self.m_lightness_targets[Target::INDEX_MAX]
    }

    /// Returns the weight of importance that this target places on a color's saturation within
    /// the image.
    ///
    /// <p>The larger the weight, relative to the other weights, the more important that a color
    /// being close to the target value has on selection.</p>
    ///
    /// See also [get_target_saturation][Target::get_target_saturation()]
    pub fn get_saturation_weight(&self) -> f32 {
        self.m_weights[Target::INDEX_WEIGHT_SAT]
    }

    /// Returns the weight of importance that this target places on a color's lightness within
    /// the image.
    ///
    /// <p>The larger the weight, relative to the other weights, the more important that a color
    ///  being close to the target value has on selection.</p>
    ///
    /// See also [get_target_lightness][Target::get_target_lightness]
    pub fn get_lightness_weight(&self) -> f32 {
        self.m_weights[Target::INDEX_WEIGHT_LUMA]
    }

    /// Returns the weight of importance that this target places on a color's population within
    /// the image.
    ///
    /// <p>The larger the weight, relative to the other weights, the more important that a
    /// color's population being close to the most populous has on selection.</p>
    pub fn get_population_weight(&self) -> f32 {
        self.m_weights[Target::INDEX_WEIGHT_POP]
    }

    /// Returns whether any color selected for this target is exclusive for this target only.
    ///
    /// <p>If false, then the color can be selected for other targets.</p>
    pub fn is_exclusive(&self) -> bool {
        self.m_is_exclusive
    }


}

/// TargetBuilder struct for generating custom [Target] instances.
pub struct TargetBuilder {
    m_target: Target
}

impl Default for TargetBuilder {
    /// Create a new [Target] builder from scratch.
    fn default() -> Self {
        Self { m_target: Target::default() }
    }
}

impl TargetBuilder {
    /// Create a new builder based on an existing [Target].
    pub fn new(kind: TargetKind) -> TargetBuilder {
        let target = Target::new(kind);
        TargetBuilder { m_target: target }
    }

    /// Set the minimum saturation value for this target.
    pub fn set_minimum_saturation(mut self, value: f32) -> TargetBuilder {
        self.m_target.m_saturation_targets[Target::INDEX_MIN] = value;
        self
    }

    /// Set the target/ideal saturation value for this target.
    pub fn set_target_saturation(mut self, value: f32) -> TargetBuilder {
        self.m_target.m_saturation_targets[Target::INDEX_TARGET] = value;
        self
    }

    /// Set the maximum saturation value for this target.
    pub fn set_maximum_saturation(mut self, value: f32) -> TargetBuilder {
        self.m_target.m_saturation_targets[Target::INDEX_MAX] = value;
        self
    }

    /// Set the minimum lightness value for this target.
    pub fn set_minimum_lightness(mut self, value: f32) -> TargetBuilder {
        self.m_target.m_lightness_targets[Target::INDEX_MIN] = value;
        self
    }

    /// Set the target/ideal lightness value for this target.
    pub fn set_target_lightness(mut self, value: f32) -> TargetBuilder {
        self.m_target.m_lightness_targets[Target::INDEX_TARGET] = value;
        self
    }

    /// Set the maximum lightness value for this target.
    pub fn set_maximum_lightness(mut self, value: f32) -> TargetBuilder {
        self.m_target.m_lightness_targets[Target::INDEX_MAX] = value;
        self
    }

    /// Set the weight of importance that this target will place on saturation values.
    ///
    /// <p>The larger the weight, relative to the other weights, the more important that a color
    ///  being close to the target value has on selection.</p>
    ///
    /// <p>A weight of 0 means that it has no weight, and thus has no
    /// bearing on the selection.</p>
    ///
    ///  See also [set_target_saturation][TargetBuilder::set_target_saturation]
    pub fn set_saturation_weight(mut self, weight: f32) -> TargetBuilder {
        self.m_target.m_weights[Target::INDEX_WEIGHT_SAT] = weight;
        self
    }

    /// Set the weight of importance that this target will place on lightness values.
    ///
    /// <p>The larger the weight, relative to the other weights, the more important that a color
    /// being close to the target value has on selection.</p>
    ///
    /// <p>A weight of 0 means that it has no weight, and thus has no
    /// bearing on the selection.</p>
    ///
    /// See also [set_target_lightness][TargetBuilder::set_target_lightness]
    pub fn set_lightness_weight(mut self, weight: f32) -> TargetBuilder {
        self.m_target.m_weights[Target::INDEX_WEIGHT_LUMA] = weight;
        self
    }

    /// Set the weight of importance that this target will place on a color's population within
    /// the image.
    ///
    /// <p>The larger the weight, relative to the other weights, the more important that a
    /// color's population being close to the most populous has on selection.</p>
    ///
    /// <p>A weight of 0 means that it has no weight, and thus has no
    /// bearing on the selection.</p>
    pub fn set_population_weight(mut self, weight: f32) -> TargetBuilder {
        self.m_target.m_weights[Target::INDEX_WEIGHT_POP] = weight;
        self
    }

     /// Set whether any color selected for this target is exclusive to this target only.
    /// Defaults to true.
    ///
    /// @param exclusive true if any the color is exclusive to this target, or false if the
    /// color can be selected for other targets.
    pub fn set_exclusive(mut self, exclusive: bool) -> TargetBuilder {
        self.m_target.m_is_exclusive = exclusive;
        self
    }

    /// Builds and returns the resulting [Target].
    pub fn build(self) -> Target {
        self.m_target
    }
}

