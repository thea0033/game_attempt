// layering
pub const LAYER_DEPTH: u32 = 3; // how deep the layer table is made
pub const LAYERS: u64 = 2u64.pow(LAYER_DEPTH); // 8 layers are generated
pub const LAYER_SIZE: u64 = (u64::MAX / 2 + 1) / (if LAYER_DEPTH == 0 {1} else {2u64.pow(LAYER_DEPTH - 1)}); // how big each layer is
pub const FRONT_LAYER: u64 = LAYERS - 1; // the front layer
pub const BACK_LAYER: u64 = 0; // the back layer
pub const CONTENT_LAYER: u64 = LAYERS / 2;
pub const UI_LAYER: u64 = 3 * LAYERS / 4;
