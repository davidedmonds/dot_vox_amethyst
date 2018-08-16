use amethyst_assets::{Result, SimpleFormat};
use amethyst_renderer::{Mesh, MeshData, PosColor};
use dot_vox;
use dot_vox::Voxel;

#[derive(Clone)]
pub struct DotVoxFormat;

impl SimpleFormat<Mesh> for DotVoxFormat {
    const NAME: &'static str = "MAGICAVOXEL_DOT_VOX";
    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<MeshData> {
        let data = dot_vox::load_bytes(&bytes)?;
        let vertices = data.models[0].voxels.iter()
            .flat_map(|voxel| voxel_to_cube(voxel, &data.palette))
            .collect();
        Ok(MeshData::PosColor(vertices))
    }
}

lazy_static! {
    static ref CUBE_OFFSETS: Vec<(f32, f32, f32)> = vec!(
        (-0.5, -0.5, -0.5),
        (-0.5, -0.5, 0.5),
        (-0.5, 0.5, 0.5),
        (0.5, 0.5, -0.5),
        (-0.5, -0.5, -0.5),
        (-0.5, 0.5, -0.5),
        (0.5, -0.5, 0.5),
        (-0.5, -0.5, -0.5),
        (0.5, -0.5, -0.5),
        (0.5, 0.5, -0.5),
        (0.5, -0.5, -0.5),
        (-0.5, -0.5, -0.5),
        (-0.5, -0.5, -0.5),
        (-0.5, 0.5, 0.5),
        (-0.5, 0.5, -0.5),
        (0.5, -0.5, 0.5),
        (-0.5, -0.5, 0.5),
        (-0.5, -0.5, -0.5),
        (-0.5, 0.5, 0.5),
        (-0.5, -0.5, 0.5),
        (0.5, -0.5, 0.5),
        (0.5, 0.5, 0.5),
        (0.5, -0.5, -0.5),
        (0.5, 0.5, -0.5),
        (0.5, -0.5, -0.5),
        (0.5, 0.5, 0.5),
        (0.5, -0.5, 0.5),
        (0.5, 0.5, 0.5),
        (0.5, 0.5, -0.5),
        (-0.5, 0.5, -0.5),
        (0.5, 0.5, 0.5),
        (-0.5, 0.5, -0.5),
        (-0.5, 0.5, 0.5),
        (0.5, 0.5, 0.5),
        (-0.5, 0.5, 0.5),
        (0.5, -0.5, 0.5)
);

}

fn palette_to_rgba(palette: &Vec<u32>, index: usize) -> [f32; 4] {
    let color = palette.get(index).unwrap_or(&0);
    let (a, b, g, r) = (color >> 24u32 & 0xFF, color >> 16u32 & 0xFF, color >> 8u32 & 0xFF, color & 0xFF);
    [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0
    ]
}

fn voxel_to_cube(voxel: &Voxel, palette: &Vec<u32>) -> Vec<PosColor> {
    let color = palette_to_rgba(palette, voxel.i as usize);
    CUBE_OFFSETS.iter()
        .map(|cube_vertex| {
            let (vx, vy, vz) = cube_vertex;
            PosColor {
                position: [
                    voxel.x as f32 + vx,
                    voxel.y as f32 + vy,
                    voxel.z as f32 + vz
                ],
                color
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use avow::vec;
    use super::*;

    const PLACEHOLDER: &'static [u8] = include_bytes!("../resources/mesh/placeholder.vox");

    fn pos(x: f32, y: f32, z: f32) -> PosColor {
        PosColor {
            position: [x, y, z],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    fn pos_color(position: [f32; 3], color: [f32; 4]) -> PosColor {
        PosColor { position, color }
    }

    #[test]
    fn can_import_a_dot_vox_file() {
        let format = DotVoxFormat;
        let red = [0.93333334, 0.0, 0.0, 1.0];
        let green = [0.0, 0.93333334, 0.0, 1.0];
        let blue = [0.0, 0.0, 0.93333334, 1.0];
        let yellow = [1.0, 1.0, 0.0, 1.0];
        match format.import(PLACEHOLDER.to_vec(), ()).unwrap() {
            MeshData::PosColor(result) =>
                vec::are_eq(result, vec!(pos_color([-0.5, -0.5, -0.5], green),
                                           pos_color([-0.5, -0.5, 0.5], green),
                                           pos_color([-0.5, 0.5, 0.5], green),
                                           pos_color([0.5, 0.5, -0.5], green),
                                           pos_color([-0.5, -0.5, -0.5], green),
                                           pos_color([-0.5, 0.5, -0.5], green),
                                           pos_color([0.5, -0.5, 0.5], green),
                                           pos_color([-0.5, -0.5, -0.5], green),
                                           pos_color([0.5, -0.5, -0.5], green),
                                           pos_color([0.5, 0.5, -0.5], green),
                                           pos_color([0.5, -0.5, -0.5], green),
                                           pos_color([-0.5, -0.5, -0.5], green),
                                           pos_color([-0.5, -0.5, -0.5], green),
                                           pos_color([-0.5, 0.5, 0.5], green),
                                           pos_color([-0.5, 0.5, -0.5], green),
                                           pos_color([0.5, -0.5, 0.5], green),
                                           pos_color([-0.5, -0.5, 0.5], green),
                                           pos_color([-0.5, -0.5, -0.5], green),
                                           pos_color([-0.5, 0.5, 0.5], green),
                                           pos_color([-0.5, -0.5, 0.5], green),
                                           pos_color([0.5, -0.5, 0.5], green),
                                           pos_color([0.5, 0.5, 0.5], green),
                                           pos_color([0.5, -0.5, -0.5], green),
                                           pos_color([0.5, 0.5, -0.5], green),
                                           pos_color([0.5, -0.5, -0.5], green),
                                           pos_color([0.5, 0.5, 0.5], green),
                                           pos_color([0.5, -0.5, 0.5], green),
                                           pos_color([0.5, 0.5, 0.5], green),
                                           pos_color([0.5, 0.5, -0.5], green),
                                           pos_color([-0.5, 0.5, -0.5], green),
                                           pos_color([0.5, 0.5, 0.5], green),
                                           pos_color([-0.5, 0.5, -0.5], green),
                                           pos_color([-0.5, 0.5, 0.5], green),
                                           pos_color([0.5, 0.5, 0.5], green),
                                           pos_color([-0.5, 0.5, 0.5], green),
                                           pos_color([0.5, -0.5, 0.5], green),

                                           pos_color([-0.5, 0.5, 0.5], red),
                                           pos_color([-0.5, 0.5, 1.5], red),
                                           pos_color([-0.5, 1.5, 1.5], red),
                                           pos_color([0.5, 1.5, 0.5], red),
                                           pos_color([-0.5, 0.5, 0.5], red),
                                           pos_color([-0.5, 1.5, 0.5], red),
                                           pos_color([0.5, 0.5, 1.5], red),
                                           pos_color([-0.5, 0.5, 0.5], red),
                                           pos_color([0.5, 0.5, 0.5], red),
                                           pos_color([0.5, 1.5, 0.5], red),
                                           pos_color([0.5, 0.5, 0.5], red),
                                           pos_color([-0.5, 0.5, 0.5], red),
                                           pos_color([-0.5, 0.5, 0.5], red),
                                           pos_color([-0.5, 1.5, 1.5], red),
                                           pos_color([-0.5, 1.5, 0.5], red),
                                           pos_color([0.5, 0.5, 1.5], red),
                                           pos_color([-0.5, 0.5, 1.5], red),
                                           pos_color([-0.5, 0.5, 0.5], red),
                                           pos_color([-0.5, 1.5, 1.5], red),
                                           pos_color([-0.5, 0.5, 1.5], red),
                                           pos_color([0.5, 0.5, 1.5], red),
                                           pos_color([0.5, 1.5, 1.5], red),
                                           pos_color([0.5, 0.5, 0.5], red),
                                           pos_color([0.5, 1.5, 0.5], red),
                                           pos_color([0.5, 0.5, 0.5], red),
                                           pos_color([0.5, 1.5, 1.5], red),
                                           pos_color([0.5, 0.5, 1.5], red),
                                           pos_color([0.5, 1.5, 1.5], red),
                                           pos_color([0.5, 1.5, 0.5], red),
                                           pos_color([-0.5, 1.5, 0.5], red),
                                           pos_color([0.5, 1.5, 1.5], red),
                                           pos_color([-0.5, 1.5, 0.5], red),
                                           pos_color([-0.5, 1.5, 1.5], red),
                                           pos_color([0.5, 1.5, 1.5], red),
                                           pos_color([-0.5, 1.5, 1.5], red),
                                           pos_color([0.5, 0.5, 1.5], red),

                                           pos_color([0.5, -0.5, 0.5], blue),
                                           pos_color([0.5, -0.5, 1.5], blue),
                                           pos_color([0.5, 0.5, 1.5], blue),
                                           pos_color([1.5, 0.5, 0.5], blue),
                                           pos_color([0.5, -0.5, 0.5], blue),
                                           pos_color([0.5, 0.5, 0.5], blue),
                                           pos_color([1.5, -0.5, 1.5], blue),
                                           pos_color([0.5, -0.5, 0.5], blue),
                                           pos_color([1.5, -0.5, 0.5], blue),
                                           pos_color([1.5, 0.5, 0.5], blue),
                                           pos_color([1.5, -0.5, 0.5], blue),
                                           pos_color([0.5, -0.5, 0.5], blue),
                                           pos_color([0.5, -0.5, 0.5], blue),
                                           pos_color([0.5, 0.5, 1.5], blue),
                                           pos_color([0.5, 0.5, 0.5], blue),
                                           pos_color([1.5, -0.5, 1.5], blue),
                                           pos_color([0.5, -0.5, 1.5], blue),
                                           pos_color([0.5, -0.5, 0.5], blue),
                                           pos_color([0.5, 0.5, 1.5], blue),
                                           pos_color([0.5, -0.5, 1.5], blue),
                                           pos_color([1.5, -0.5, 1.5], blue),
                                           pos_color([1.5, 0.5, 1.5], blue),
                                           pos_color([1.5, -0.5, 0.5], blue),
                                           pos_color([1.5, 0.5, 0.5], blue),
                                           pos_color([1.5, -0.5, 0.5], blue),
                                           pos_color([1.5, 0.5, 1.5], blue),
                                           pos_color([1.5, -0.5, 1.5], blue),
                                           pos_color([1.5, 0.5, 1.5], blue),
                                           pos_color([1.5, 0.5, 0.5], blue),
                                           pos_color([0.5, 0.5, 0.5], blue),
                                           pos_color([1.5, 0.5, 1.5], blue),
                                           pos_color([0.5, 0.5, 0.5], blue),
                                           pos_color([0.5, 0.5, 1.5], blue),
                                           pos_color([1.5, 0.5, 1.5], blue),
                                           pos_color([0.5, 0.5, 1.5], blue),
                                           pos_color([1.5, -0.5, 1.5], blue),

                                           pos_color([0.5, 0.5, -0.5], yellow),
                                           pos_color([0.5, 0.5, 0.5], yellow),
                                           pos_color([0.5, 1.5, 0.5], yellow),
                                           pos_color([1.5, 1.5, -0.5], yellow),
                                           pos_color([0.5, 0.5, -0.5], yellow),
                                           pos_color([0.5, 1.5, -0.5], yellow),
                                           pos_color([1.5, 0.5, 0.5], yellow),
                                           pos_color([0.5, 0.5, -0.5], yellow),
                                           pos_color([1.5, 0.5, -0.5], yellow),
                                           pos_color([1.5, 1.5, -0.5], yellow),
                                           pos_color([1.5, 0.5, -0.5], yellow),
                                           pos_color([0.5, 0.5, -0.5], yellow),
                                           pos_color([0.5, 0.5, -0.5], yellow),
                                           pos_color([0.5, 1.5, 0.5], yellow),
                                           pos_color([0.5, 1.5, -0.5], yellow),
                                           pos_color([1.5, 0.5, 0.5], yellow),
                                           pos_color([0.5, 0.5, 0.5], yellow),
                                           pos_color([0.5, 0.5, -0.5], yellow),
                                           pos_color([0.5, 1.5, 0.5], yellow),
                                           pos_color([0.5, 0.5, 0.5], yellow),
                                           pos_color([1.5, 0.5, 0.5], yellow),
                                           pos_color([1.5, 1.5, 0.5], yellow),
                                           pos_color([1.5, 0.5, -0.5], yellow),
                                           pos_color([1.5, 1.5, -0.5], yellow),
                                           pos_color([1.5, 0.5, -0.5], yellow),
                                           pos_color([1.5, 1.5, 0.5], yellow),
                                           pos_color([1.5, 0.5, 0.5], yellow),
                                           pos_color([1.5, 1.5, 0.5], yellow),
                                           pos_color([1.5, 1.5, -0.5], yellow),
                                           pos_color([0.5, 1.5, -0.5], yellow),
                                           pos_color([1.5, 1.5, 0.5], yellow),
                                           pos_color([0.5, 1.5, -0.5], yellow),
                                           pos_color([0.5, 1.5, 0.5], yellow),
                                           pos_color([1.5, 1.5, 0.5], yellow),
                                           pos_color([0.5, 1.5, 0.5], yellow),
                                           pos_color([1.5, 0.5, 0.5], yellow))),
            result => panic!("Expected miracle, received {:?}", result)
        }
    }

    #[test]
    fn one_voxel_becomes_a_12_triangle_cube() {
        let result = voxel_to_cube(&Voxel { x: 1, y: 1, z: 1, i: 0 },
                                   &dot_vox::DEFAULT_PALETTE.to_vec());
        vec::are_eq(result, vec!(pos(0.5, 0.5, 0.5),
                                 pos(0.5, 0.5, 1.5),
                                 pos(0.5, 1.5, 1.5),
                                 pos(1.5, 1.5, 0.5),
                                 pos(0.5, 0.5, 0.5),
                                 pos(0.5, 1.5, 0.5),
                                 pos(1.5, 0.5, 1.5),
                                 pos(0.5, 0.5, 0.5),
                                 pos(1.5, 0.5, 0.5),
                                 pos(1.5, 1.5, 0.5),
                                 pos(1.5, 0.5, 0.5),
                                 pos(0.5, 0.5, 0.5),
                                 pos(0.5, 0.5, 0.5),
                                 pos(0.5, 1.5, 1.5),
                                 pos(0.5, 1.5, 0.5),
                                 pos(1.5, 0.5, 1.5),
                                 pos(0.5, 0.5, 1.5),
                                 pos(0.5, 0.5, 0.5),
                                 pos(0.5, 1.5, 1.5),
                                 pos(0.5, 0.5, 1.5),
                                 pos(1.5, 0.5, 1.5),
                                 pos(1.5, 1.5, 1.5),
                                 pos(1.5, 0.5, 0.5),
                                 pos(1.5, 1.5, 0.5),
                                 pos(1.5, 0.5, 0.5),
                                 pos(1.5, 1.5, 1.5),
                                 pos(1.5, 0.5, 1.5),
                                 pos(1.5, 1.5, 1.5),
                                 pos(1.5, 1.5, 0.5),
                                 pos(0.5, 1.5, 0.5),
                                 pos(1.5, 1.5, 1.5),
                                 pos(0.5, 1.5, 0.5),
                                 pos(0.5, 1.5, 1.5),
                                 pos(1.5, 1.5, 1.5),
                                 pos(0.5, 1.5, 1.5),
                                 pos(1.5, 0.5, 1.5)));
    }

    #[test]
    fn palette_and_index_becomes_a_color() {
        assert_eq!(palette_to_rgba(&dot_vox::DEFAULT_PALETTE.to_vec(), 215),
                   [0.93333334, 0.0, 0.0, 1.0]);
    }
}