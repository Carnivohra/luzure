pub struct MeshPipelineContract;

impl MeshPipelineContract {
    pub const VERTEX_ENTRY: &str = "vertex";
    pub const FRAGMENT_ENTRY: &str = "fragment";

    pub const CAMERA_GROUP: u32 = 0;
    pub const CAMERA_BINDING: u32 = 0;

    pub const POSITION_LOCATION: u32 = 0;
    pub const NORMAL_LOCATION: u32 = 1;
    pub const TEXTURE_COORDINATE_LOCATION: u32 = 2;
    pub const MODEL_COLUMN_0_LOCATION: u32 = 3;
    pub const MODEL_COLUMN_1_LOCATION: u32 = 4;
    pub const MODEL_COLUMN_2_LOCATION: u32 = 5;
    pub const MODEL_COLUMN_3_LOCATION: u32 = 6;
    pub const COLOR_TARGET_LOCATION: u32 = 0;
}
