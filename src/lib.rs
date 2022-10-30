use pyo3::prelude::*;
use std::process::Command;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn get_filename(filepath: &str, file_format: &str) -> String {
    let filename: Vec<&str> = filepath.split(".").collect();
    let after_filename = filename[0].to_owned() + "." + file_format;

    return after_filename;
}

fn ffmpeg_cmd(filepath: &str, file_format: &str) -> String {
    let after_filename = get_filename(filepath, file_format);
    let cmd = "ffmpeg".to_owned() + " -i " + filepath + " " + &after_filename;

    return cmd;
}
#[pyfunction]
#[pyo3(name = "get_filename")]
fn python_get_filename(filepath: &str, file_format: &str) -> PyResult<String> {
    let after_filename = get_filename(filepath, file_format);

    Ok(after_filename)
}

#[pyclass]
struct FFmpeg {
    filepath: String,
    file_format: String,
}

#[pymethods]
impl FFmpeg {
    #[new]
    fn new(filepath: String, file_format: String) -> Self {
        FFmpeg {
            filepath,
            file_format,
        }
    }

    #[pyo3(name = "ffmpeg_cmd")]
    fn python_ffmpeg_cmd(&self) -> PyResult<String> {
        let cmd = ffmpeg_cmd(&self.filepath, &self.file_format);

        Ok(cmd.to_string())
    }

    fn ffmpeg(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .args([
                    "ffmpeg",
                    "-i",
                    &self.filepath,
                    &get_filename(&self.filepath, &self.file_format),
                ])
                .output()
                .expect("ffmpeg command failed to start");
        } else {
            Command::new("sh")
                .arg("-c")
                .args([
                    "ffmpeg",
                    "-i",
                    &self.filepath,
                    &get_filename(&self.filepath, &self.file_format),
                ])
                .output()
                .expect("ffmpeg command failed to start");
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ffmpegwithpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(python_get_filename, m)?)?;
    m.add_class::<FFmpeg>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filename() {
        assert_eq!(get_filename("test.mp4", "mp3"), "test.mp3")
    }

    #[test]
    fn test_ffmpeg_cmd() {
        assert_eq!(ffmpeg_cmd("test.mp4", "mp3"), "ffmpeg -i test.mp4 test.mp3")
    }
}
