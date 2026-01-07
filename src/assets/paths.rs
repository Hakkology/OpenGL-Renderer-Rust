//! Asset Paths - Centralized static paths for all game assets

pub const SHADERS_DIR: &str = "assets/shaders";
pub const TEXTURES_DIR: &str = "assets/resources/textures";
pub const MODELS_DIR: &str = "assets/resources/models";

pub mod shaders {
    // Lit shaders (with lighting)
    pub const LIT_VERT: &str = concat!("assets/shaders", "/lit.vert");
    pub const LIT_COLOR_FRAG: &str = concat!("assets/shaders", "/lit_color.frag");
    pub const LIT_TEXTURED_FRAG: &str = concat!("assets/shaders", "/lit_textured.frag");

    // UI shaders
    pub const UI_VERT: &str = concat!("assets/shaders", "/ui.vert");
    pub const UI_TEXT_FRAG: &str = concat!("assets/shaders", "/ui_text.frag");
    pub const UI_COLOR_FRAG: &str = concat!("assets/shaders", "/ui_color.frag");

    // Skybox shaders
    pub const SKYBOX_VERT: &str = concat!("assets/shaders", "/skybox.vert");
    pub const SKYBOX_FRAG: &str = concat!("assets/shaders", "/skybox.frag");

    // Shadow shaders (directional light)
    pub const SHADOW_DEPTH_VERT: &str = concat!("assets/shaders", "/shadow_depth.vert");
    pub const SHADOW_DEPTH_FRAG: &str = concat!("assets/shaders", "/shadow_depth.frag");

    // Point shadow shaders (point light cubemap)
    pub const POINT_SHADOW_VERT: &str = concat!("assets/shaders", "/point_shadow_depth.vert");
    pub const POINT_SHADOW_GEOM: &str = concat!("assets/shaders", "/point_shadow_depth.geom");
    pub const POINT_SHADOW_FRAG: &str = concat!("assets/shaders", "/point_shadow_depth.frag");
}

pub mod textures {
    pub const GRASS: &str =
        "assets/resources/textures/Poliigon_GrassPatchyGround_4585_BaseColor.jpg";
    pub const STONE_BRICKS: &str = "assets/resources/textures/StoneBricks_1K.tiff";
    pub const SKYBOX: &str = "assets/resources/textures/Cubemap_Sky_22-512x512.png";
}

pub mod models {
    pub const TREE: &str = "assets/resources/models/Tree2/trees9.obj";
    pub const XWING: &str = "assets/resources/models/Xwing/x-wing.obj";
    pub const STATUE: &str = "assets/resources/models/Statue/12334_statue_v1_l3.obj";
}

pub mod names {
    // Shaders
    pub const SHADER_COLORED: &str = "colored";
    pub const SHADER_TEXTURED: &str = "textured";
    pub const SHADER_UI_TEXT: &str = "ui_text";
    pub const SHADER_UI_COLOR: &str = "ui_color";
    pub const SHADER_SKYBOX: &str = "skybox";

    // Textures
    pub const TEX_GRASS: &str = "grass";
    pub const TEX_STONE: &str = "stone";
    pub const TEX_SKYBOX: &str = "skybox";

    // Models
    pub const MODEL_TREE: &str = "tree";
    pub const MODEL_XWING: &str = "xwing";
    pub const MODEL_STATUE: &str = "statue";
}
