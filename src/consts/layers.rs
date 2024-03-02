
// how deep the layer table is made
pub const LAYER_DEPTH: u32 = 3;

// how many layers there are - 3 binary digits -> 8 layers are generated
pub const LAYERS: u64 = 2u64.pow(LAYER_DEPTH); 

// how big each layer is - the maximum size if there's only one layer, or 2^64 (number of possible numbers) divided by the number of layers. 
pub const LAYER_SIZE: u64 = (u64::MAX / 2 + 1)
    / (if LAYER_DEPTH == 0 {
        1
    } else {
        2u64.pow(LAYER_DEPTH - 1)
    }); // how big each layer is

// different layers. The front is always the front. The back is always the back. 
pub const FRONT_LAYER: u64 = LAYERS - 1; // the front layer
pub const BACK_LAYER: u64 = 0; // the back layer
pub const CONTENT_LAYER: u64 = LAYERS / 2;
pub const UI_LAYER: u64 = 3 * LAYERS / 4;
