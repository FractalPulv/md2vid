use regex::Regex;
use std::error::Error;

pub fn process_sentence(sentence: &str) -> String {
    // Regex for footnotes syntax (e.g. ^[{footnote text}])
    let re = Regex::new(r"\^\[\{(.*?)\}\]").unwrap();
    let result = re.replace_all(sentence, |caps: &regex::Captures| {
        format!("{{\\up1\\c&HFFFFFF&}}{}{{\\up0\\c&HFFFFFF&}}", &caps[1])
    });

    // Regex for md image syntax, replace with blue bold IMG text
    // For local images: ![[image.png]]
    let re = Regex::new(r"!\[\[(.*?)\]\]").unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        format!("{{\\c&H0000FF&}}{{\\b1\\c&HFFFFFF&}}LOCAL IMG{{\\b0\\c&HFFFFFF&}}")
    });
    // For hosted images: ![image](https://example.com/image.png)
    let re = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        format!("{{\\c&H0000FF&}}{{\\b1\\c&HFFFFFF&}}HOSTED IMG{{\\b0\\c&HFFFFFF&}}")
    });

    // Regex for text within double square brackets, remove brackets and make the text purple
    let re = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        let text = &caps[1];
        if let Some((_, alias)) = text.split_once(" |") {
            format!("{{\\c&H800080&}}{}{{\\c&HFFFFFF&}}", alias.trim())
        } else {
            format!("{{\\c&H800080&}}{}{{\\c&HFFFFFF&}}", text)
        }
    });

    // // Regex of md link syntax, replace with underlined blue text
    // // Sytnax: [link text](https://example.com)
    // let re = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    // let result = re.replace_all(&result, |caps: &regex::Captures| {
    //     format!("{{\\ul1\\c&H0000FF&}}{}{{\\ul0\\c&HFFFFFF&}}", &caps[1])
    // });

    // Regex for text wrapped with underscores, make it italic and green, then remove the underscores
    let re = Regex::new(r"_([^_]+)_").unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        format!("{{\\i1\\c&H00FF00&}}{}{{\\i0\\c&HFFFFFF&}}", &caps[1])
    });

    // Regex for text wrapped with asterisks, make it bold and red, then remove the asterisks
    let re = Regex::new(r"\*([^*]+)\*").unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        format!("{{\\b1\\c&HFF0000&}}{}{{\\b0\\c&HFFFFFF&}}", &caps[1])
    });

    // Regex for text wrapped with single backticks, make it monospace with gray background, then remove the backticks
    let re = Regex::new(r"`([^`]+)`").unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        format!("{{\\c&H808080&}}{}{{\\c&HFFFFFF&}}", &caps[1])
    });

    result.into_owned()
}





pub fn generate_ass_content_bottom(sentence: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let ass_content = format!(
        r#"[Script Info]
        Title: Default Aegisub file
        ScriptType: v4.00+
        WrapStyle: 0
        PlayResX: 1280
        PlayResY: 720
        ScaledBorderAndShadow: yes
        YCbCr Matrix: None

        [V4+ Styles]
        Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
        Style: Default, Vera, 28, &HFFFFFF, &HFFFFFF, &H000000, &H000000, -1, 0, 0, 0, 100, 100, 0, 0, 1, 1, 1, 2, 10, 10, 30, 1

        [Events]
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
        Dialogue: 0,0:00:00.00,0:00:05.00,Default,,10,10,30,,{}"#,
        sentence
    );

    Ok(ass_content)
}

pub fn generate_ass_content_centered(sentence: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let ass_content = format!(
        r#"[Script Info]
        Title: Default Aegisub file
        ScriptType: v4.00+
        WrapStyle: 0
        PlayResX: 1280
        PlayResY: 720
        ScaledBorderAndShadow: yes
        YCbCr Matrix: None
        
        [V4+ Styles]
        Format: Name, Fontname, Fontsize, PrimaryColour, Alignment
        Style: Default, Vera, 42, &HFFFFFF, 8
        
        [Events]
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
        Dialogue: 0,0:00:00.00,0:00:05.00,Default,,320,320,355,,{}"#,
        sentence
    );

    Ok(ass_content)
}