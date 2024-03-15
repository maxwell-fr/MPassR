use regex::Regex;

use mpass::specifier::Specifier;

fn main() {
    println!("Hello, world!");

    let hay = "<html>\
    <head><title>Test</title>\
    </head>\
    <body>\
    <<flarg:wW#$>>
    <<frobz:#w W #$>>
    <<ppwd:W w w w ####>>
    </body></html>";

    let re = Regex::new(r"<<(?<name>[a-zA-Z]+):(?<spec>[a-zA-Z#$ ]+)>>").unwrap();
    let caps = re.captures_iter(hay);

    for cap in caps {
        let name = cap.name("name").unwrap().as_str();
        let spec = cap.name("spec").unwrap().as_str();

        let specifier = Specifier::try_parse(spec).unwrap();
        println!("{} ({}): {}", name, spec, specifier.get_passphrase());

    }

}
