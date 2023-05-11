use reqwest;
use structopt::StructOpt;

mod model;

#[derive(Debug, PartialEq, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    region: Option<String>,

    #[structopt(short, long)]
    service: Option<String>,
}

const URL: &str = "https://ip-ranges.amazonaws.com/ip-ranges.json";

fn main() {
    let opt = Opt::from_args();
    let response = reqwest::blocking::get(URL);

    match response {
        Ok(response) => {
            let data = response.json::<model::IpRanges>();
            match data {
                Ok(data) => {
                    let prefixes: Vec<&model::Prefix> = if let Some(region) = opt.region {
                        data.prefixes
                            .iter()
                            .filter(|prefix| {
                                prefix
                                    .region
                                    .to_lowercase()
                                    .contains(&region.to_lowercase())
                            })
                            .collect()
                    } else {
                        data.prefixes.iter().collect()
                    };

                    let prefixes: Vec<&model::Prefix> = if let Some(service) = opt.service {
                        prefixes
                            .into_iter()
                            .filter(|prefix| {
                                prefix
                                    .service
                                    .to_lowercase()
                                    .contains(&service.to_lowercase())
                            })
                            .collect()
                    } else {
                        prefixes.into_iter().collect()
                    };

                    println!("{:#?}", prefixes)
                }
                Err(error) => println!("{}", error),
            }
        }
        Err(error) => println!("{}", error),
    }
}
