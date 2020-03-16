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

    let stb_col = if args.len() >= 3 {
        args[2].parse::<i32>().unwrap_or(1) + 1
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

        let mut buf = Vec::new();

        if object_idx >= zsc.objects.len() {
            continue;
        }
        let zsc_obj = &zsc.objects[object_idx];

        writeln!(buf, "numObj {}", zsc_obj.parts.len())?;
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

            writeln!(buf, "obj {}", part_idx + 1)?;
            writeln!(buf, "\tmesh {}", mesh.to_string_lossy())?;
            writeln!(buf, "\tmat {}", mat.path.to_string_lossy())?;
            writeln!(buf, "\tisskin {}", mat.is_skin as u32)?;
            writeln!(buf, "\talpha  {}", mat.alpha_enabled as u32)?;
            writeln!(buf, "\ttwoside {}", mat.two_sided as u32)?;
            writeln!(buf, "\tparent {}", part.parent)?;
            writeln!(
                buf,
                "\tpos {} {} {}",
                part.position.x, part.position.y, part.position.z
            )?;
            writeln!(
                buf,
                "\trot {} {} {} {}",
                part.rotation.w, part.rotation.x, part.rotation.y, part.rotation.z
            )?;
            writeln!(
                buf,
                "\tscale {} {} {}",
                part.scale.x, part.scale.y, part.scale.z
            )?;
            writeln!(buf, "\tcollision {}", part.collision)?;
            writeln!(buf, "\tuselightmap {}", part.use_lightmap as u32)?;
            writeln!(buf, "\trangeset {}", part.range)?;
        }

        writeln!(buf, "numpoint {}", zsc_obj.effects.len())?;
        for (effect_idx, effect) in zsc_obj.effects.iter().enumerate() {
            let effect_path = if effect.effect_id as usize >= zsc.effects.len() {
                PathBuf::from("")
            } else {
                PathBuf::from(&zsc.effects[effect.effect_id as usize])
            };

            writeln!(buf, "point {}", effect_idx + 1)?;
            writeln!(buf, "\teffect {}", effect_path.to_string_lossy())?;
            if effect.effect_id == 65535 {
                writeln!(buf, "\ttype {}", 0)?;
            } else {
                writeln!(buf, "\ttype {}", effect.effect_id)?;
            }
            writeln!(buf, "\tparent {}", effect.parent)?;
            writeln!(
                buf,
                "\tpos {} {} {}",
                effect.position.x, effect.position.y, effect.position.z
            )?;
            writeln!(
                buf,
                "\trot {} {} {} {}",
                effect.rotation.w, effect.rotation.x, effect.rotation.y, effect.rotation.z
            )?;
            writeln!(
                buf,
                "\tscale {} {} {}",
                effect.scale.x, effect.scale.y, effect.scale.z
            )?;
        }

        let mut s = String::from_utf8(buf).unwrap();
        s = s.replace("\n", "\r\n");

        let mut f = File::create(out_file).unwrap();
        f.write_all(s.as_bytes()).unwrap();
    }

    Ok(())
}
