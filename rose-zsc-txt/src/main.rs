use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use roselib::files::{STB, ZSC};
use roselib::files::zsc::SceneMaterial;
use roselib::io::RoseFile;

fn bail(msg: &str) -> Result<(), ::std::io::Error> {
    println!("{}", msg);
    ::std::process::exit(1);
}

fn main() -> Result<(), ::std::io::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        bail("Usage: rose-zsc-txt <stb_path> <zsc_path> [stb_col]")?;
    }

    let stb_path = PathBuf::from(&args[0]);
    if !stb_path.exists() {
        bail(&format!(
            "STB file does not exist: {}",
            stb_path.to_string_lossy()
        ))?;
    }

    let zsc_path = PathBuf::from(&args[1]);
    if !zsc_path.exists() {
        bail(&format!(
            "ZSC file does not exist: {}",
            zsc_path.to_string_lossy()
        ))?;
    }

    let stb = match STB::from_path(&stb_path) {
        Ok(s) => s,
        Err(e) => return bail(&format!("Failed to read STB file: {}", e)),
    };

    let zsc = match ZSC::from_path(&zsc_path) {
        Ok(s) => s,
        Err(e) => return bail(&format!("Failed to read ZSC file: {}", e)),
    };

    let stb_col = if args.len() > 3 {
        args[2].parse::<i32>().unwrap_or(3) - 1
    } else {
        2
    };

    if stb_col > stb.cols() as i32 {
        bail(&format!(
            "STB column exceeds column count: {} ({} max)",
            stb_col,
            stb.cols()
        ))?;
    }

    for (object_idx, stb_row) in stb.data.iter().enumerate() {
        let txt_file = &stb_row[stb_col as usize];
        if txt_file.is_empty() {
            println!(
                "No txt file name in column {} for row {}. Skipping.",
                stb_col, object_idx
            );
            continue;
        }

        let out_file = PathBuf::from(&txt_file);
        create_dir_all(out_file.parent().unwrap_or(&PathBuf::new())).unwrap();

        let mut f = File::create(out_file).unwrap();

        if object_idx >= zsc.objects.len() {
            continue;
        }
        let zsc_obj = &zsc.objects[object_idx];

        writeln!(f, "numObj {}", zsc_obj.parts.len())?;
        for (part_idx, part) in zsc_obj.parts.iter().enumerate() {
            let mesh = if part.mesh_id as usize >= zsc.meshes.len() {
                PathBuf::from("")
            } else {
                PathBuf::from(&zsc.meshes[part.mesh_id as usize])
            };

            let dmat = SceneMaterial::default();
            let mat = if part.material_id as usize >= zsc.materials.len() {
                &dmat
            } else {
                &zsc.materials[part.material_id as usize]
            };

            writeln!(f, "obj {}", part_idx + 1)?;
            writeln!(f, "\tmesh {}", mesh.to_string_lossy())?;
            writeln!(f, "\tmat {}", mat.path.to_string_lossy())?;
            writeln!(f, "\tisskin {}", mat.is_skin as u32)?;
            writeln!(f, "\talpha  {}", mat.alpha_enabled as u32)?;
            writeln!(f, "\ttwoside {}", mat.two_sided as u32)?;
            writeln!(f, "\tparent {}", part.parent)?;
            writeln!(
                f,
                "\tpos {} {} {}",
                part.position.x, part.position.y, part.position.z
            )?;
            writeln!(
                f,
                "\trot {} {} {} {}",
                part.rotation.w, part.rotation.x, part.rotation.y, part.rotation.z
            )?;
            writeln!(
                f,
                "\tscale {} {} {}",
                part.scale.x, part.scale.y, part.scale.z
            )?;
            writeln!(f, "\tcollision {}", part.collision)?;
            writeln!(f, "\tuselightmap {}", part.use_lightmap)?;
            writeln!(f, "\trangeset {}", part.range)?;
        }

        writeln!(f, "numpoint {}", zsc_obj.effects.len())?;
        for (effect_idx, effect) in zsc_obj.effects.iter().enumerate() {
            let effect_path = if effect.effect_id as usize >= zsc.effects.len() {
                PathBuf::from("")
            } else {
                PathBuf::from(&zsc.effects[effect.effect_id as usize])
            };

            writeln!(f, "point {}", effect_idx)?;
            writeln!(f, "\teffect {}", effect_path.to_string_lossy())?;
            writeln!(f, "\ttype {}", effect.effect_id)?;
            writeln!(f, "\tparent {}", effect.parent)?;
            writeln!(
                f,
                "\tpos {} {} {}",
                effect.position.x, effect.position.y, effect.position.z
            )?;
            writeln!(
                f,
                "\trot {} {} {} {}",
                effect.rotation.w, effect.rotation.x, effect.rotation.y, effect.rotation.z
            )?;
            writeln!(
                f,
                "\tscale {} {} {}",
                effect.scale.x, effect.scale.y, effect.scale.z
            )?;
        }
    }

    Ok(())
}
