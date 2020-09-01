#[cfg(test)]
mod tests {
	use std::fs::{create_dir_all, remove_dir, remove_file};
	use std::path::{Path, PathBuf};

	use rs_sdf::data::{Cell, CellLayer, DistanceField};
	use rs_sdf::data::output::TransformationOutputWriter;
	use rs_sdf::data::transformation::{DistanceTransformation, TransformationResult, TransformOutputGenerator};
	use rs_sdf::distance::{DistanceLayer, DistanceType};
	use rs_sdf::export::image::{ImageFileWriter, PngOutput};

	const TEMP_DIR: &str = r"__tmp__output__dir__/";
	const TEMP_IMAGE_FILE: &str = r"image.png";

	fn create_temp_dir() {
		let p = Path::new(TEMP_DIR);
		create_dir_all(p).unwrap();
	}

	fn delete_temp_dir() {
		let p = Path::new(TEMP_DIR);
		remove_dir(p).unwrap();
	}

	fn delete_temp_image_file() {
		remove_file(get_temp_image_path()).unwrap();
	}

	fn get_temp_image_path() -> PathBuf {
		let mut b = PathBuf::new();
		b.push(Path::new(TEMP_DIR));
		b.push(Path::new(TEMP_IMAGE_FILE));
		b
	}

	#[test]
	fn generates_png_file() {
		// should generate a 1x1 pixel grey image
		let d: DistanceField = DistanceField {
			data: vec![Cell::new(CellLayer::Foreground, 0, 90, 90); 1], // Foreground(Distance::new(180, 180))
			width: 1,
			height: 1,
		};

		create_temp_dir();

		let out = PngOutput::new(
			&get_temp_image_path().into_os_string().into_string().unwrap());

		let mut dt: DistanceTransformation = DistanceTransformation::from(d);
		dt.filter(DistanceLayer::Combined);
		dt.distance_type(DistanceType::EuclideanDistance);
		dt.scale(0.9); // u8 -> 0 = orig, 1 = 2^1 = orig / 2, 2 = 2^2 = orig / 4, etc...

		let trans: TransformationResult<u8> = dt.transform();
		out.write(&trans);

		assert!(get_temp_image_path().is_file());

		delete_temp_image_file();
		delete_temp_dir();
	}
}