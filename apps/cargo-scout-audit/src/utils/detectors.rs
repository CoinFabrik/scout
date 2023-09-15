use anyhow::bail;
use anyhow::Result;

fn get_parsed_detectors(detectors: String) -> Vec<String> {
    detectors
        .to_lowercase()
        .trim()
        .replace('_', "-")
        .split(',')
        .map(|detector| detector.trim().to_string())
        .collect::<Vec<String>>()
}

pub fn get_filtered_detectors(filter: String, detectors_names: Vec<String>) -> Result<Vec<String>> {
    let mut used_detectors: Vec<String> = Vec::new();
    let parsed_detectors = get_parsed_detectors(filter);
    for detector in parsed_detectors {
        if detectors_names.contains(&detector.to_string()) {
            used_detectors.push(detector.to_string());
        } else {
            bail!("The detector '{}' doesn't exist", detector);
        }
    }
    Ok(used_detectors)
}

pub fn get_excluded_detectors(
    excluded: String,
    detectors_names: Vec<String>,
) -> Result<Vec<String>> {
    let mut used_detectors = detectors_names.clone();
    let parsed_detectors = get_parsed_detectors(excluded);
    for detector in parsed_detectors {
        if detectors_names.contains(&detector.to_string()) {
            let index = used_detectors.iter().position(|x| x == &detector).unwrap();
            used_detectors.remove(index);
        } else {
            bail!("The detector '{}' doesn't exist", detector);
        }
    }
    Ok(used_detectors)
}

pub fn list_detectors(detectors_names: Vec<String>) {
    let separator = "─".repeat(48);
    let upper_border = format!("┌{}┐", separator);
    let lower_border = format!("└{}┘", separator);
    let empty_line = format!("│{:48}│", "");

    println!("{}", upper_border);
    println!("│{:^47}│", "🔍 Available detectors:");
    println!("{}", empty_line);

    for (index, detector_name) in detectors_names.iter().enumerate() {
        println!("│ {:>2}. {:<43}│", index + 1, detector_name);
    }

    println!("{}", empty_line);
    println!("{}", lower_border);
}
