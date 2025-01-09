use clap::Parser;
use libotp::totp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    secret: String,
    #[arg(short, long, default_value_t = 6)]
    digits: u32,
    #[arg(short, long, default_value_t = 30)]
    timestep: u64
}

fn main() {
    let args = Args::parse();

    match totp(&args.secret, args.digits, args.timestep, 0) {
        Some(otp) => println!("{}", otp),
        None => println!("Failed to generate OTP"),
    };
}
