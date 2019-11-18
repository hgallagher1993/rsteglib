use structopt::StructOpt;
use rsteglib::image_encoder::CoverImage;
use rsteglib::image_decoder::StegObject;

#[derive(StructOpt, Debug)]
#[structopt(name = "rsteglib_cli")]
enum Opt {
    /// Encode a message onto a picture
    Encode {
        /// Image path
        #[structopt(short, long)]
        path: String,

        /// Message
        #[structopt(short, long)]
        message: String,

        /// Optional output file name
        #[structopt(short, long)]
        output_file_name: String
    },

    /// Decode a message from a picture
    Decode {
        /// Image path
        #[structopt(short, long)]
        path: String
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode {
            path,
            message,
            output_file_name
        } => {
            encode_message(path, message, output_file_name)
        },

        Opt::Decode {
            path,
        } => {
            decode_message(path)
        },

    };
}

fn encode_message(path: String, message: String, output_file_name: String) {
    let mut cover_image = CoverImage::new();
    cover_image.set_cover_image(path);
    cover_image.set_message(message);
    cover_image.set_output_image_path(output_file_name + ".png");
    cover_image.encode();
}

fn decode_message(path: String) {
    let mut steg_object = StegObject::new();
    steg_object.set_steg_image(path);
    steg_object.set_message_length(32);
    let message = steg_object.decode();

    println!("{:?}", message.as_str());
}